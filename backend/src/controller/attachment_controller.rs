use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{
    web::{self},
    HttpRequest, HttpResponse, HttpResponseBuilder, Responder,
};
use futures_util::TryStreamExt as _;
use rusqlite::params;
use uuid::Uuid;

use crate::{
    application::{
        config::Configuration,
        error::{HomeworkError, InternalError},
    },
    entity::attachment::Attachment,
};

/// Lists all attachments saved in the database.
pub async fn all_attachments() -> actix_web::Result<impl Responder> {
    let conn = Configuration::database_connection()?;
    let mut stmt = conn
        .prepare("SELECT id, name, creation_time FROM attachment")
        .map_err(HomeworkError::from)?;
    let attachments_sql = stmt
        .query_map([], |row| Ok(Attachment::try_from(row)?))
        .map_err(HomeworkError::from)?;
    let mut attachments: Vec<Attachment> = Vec::new();
    for attachment in attachments_sql {
        attachments.push(attachment.map_err(HomeworkError::from)?);
    }
    Ok(web::Json(attachments))
}

pub async fn add_attachment(
    request: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, HomeworkError> {
    // Open a databse connection first so the file is not saved in case of connection errors.
    let conn = Configuration::database_connection()?;

    // Generate a new UUID for the attachment.
    let uuid = Configuration::generate_uuid();

    // Load the backup service.
    let backup_service = Configuration::backup_service_from_request(&request);

    // Create the attachment folder.
    let app_config = request
        .app_data::<Arc<Configuration>>()
        .expect("The configuration must be accessible.");
    let mut file_path = app_config.application_attachments_folder_path();
    std::fs::create_dir_all(&file_path)?;
    file_path.push(uuid.to_string());
    let file_path: Arc<Path> = file_path.into();
    let mut file_name: Option<String> = None;

    // Iterate over the multipart stream and save the file.
    while let Some(mut field) = payload.try_next().await? {
        // Set the file name if available.
        if let Some(name) = field.content_disposition().get_filename() {
            file_name = Some(sanitize_filename::sanitize(name));
        }
        // Write the file to disk.
        let file_path_ref = Arc::clone(&file_path);
        let mut file = web::block(|| std::fs::File::create(file_path_ref)).await??;
        while let Some(chunk) = field.try_next().await? {
            file = web::block(move || file.write_all(&chunk).map(|_| file)).await??;
        }
    }

    // Save the attachment into the database.
    conn.execute(
        "INSERT INTO attachment (id, name, creation_time) VALUES (?1, ?2, ?3)",
        params![
            uuid,
            file_name.unwrap_or(uuid.to_string()),
            chrono::Utc::now()
        ],
    )?;

    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();

    // Return the UUID of the created attachment.
    Ok(HttpResponse::Created().body(uuid.to_string()))
}

pub async fn delete_attachment(
    request: HttpRequest,
    id: web::Path<Uuid>,
) -> Result<HttpResponseBuilder, HomeworkError> {
    let uuid: Uuid = id.into_inner();

    // Load the backup service.
    let backup_service = Configuration::backup_service_from_request(&request);

    // Open a databse connection first so the file is not deleted in case of connection errors
    // and check that the attachmnet indeed exists.
    let conn = Configuration::database_connection()?;
    Attachment::exists_in_database_by_id_throw_not_found(uuid, &conn)?;

    // Access the configuration.
    let app_config = request
        .app_data::<Arc<Configuration>>()
        .expect("The configuration must be accessible.");

    // Get the attachment file path.
    let file_path = attachment_path(&request, uuid);

    // Remove the attachment from the disk.
    std::fs::remove_file(file_path)?;

    // Remove potential thumbnails from the disk.
    for entry in std::fs::read_dir(app_config.application_thumbnail_folder_path())? {
        let entry = entry?;
        if let Some(file_name) = entry.file_name().to_str() {
            if file_name.contains(&uuid.to_string()) {
                std::fs::remove_file(entry.path())?;
            }
        }
    }

    // Remove the attachment from the database.
    conn.execute("DELETE FROM attachment WHERE id = ?1", params![uuid,])?;

    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();

    // Return the UUID of the created attachment.
    Ok(HttpResponse::Ok())
}

pub async fn download_attachment(
    request: HttpRequest,
    id: web::Path<Uuid>,
) -> Result<NamedFile, HomeworkError> {
    let uuid: Uuid = id.into_inner();
    let file_path = attachment_path(&request, uuid);
    let file_name = attachment_file_name_from_db(uuid)?;
    Ok(NamedFile::from_file(File::open(file_path)?, file_name)?)
}

/// Return the thumbnail with the specified width for the specified image attachment.
pub async fn thumbnail_image_attachment(
    request: HttpRequest,
    path: web::Path<(Uuid, u32)>,
) -> Result<NamedFile, HomeworkError> {
    let (uuid, width) = path.into_inner();
    if !Configuration::thumbnail_widths().contains(&width) {
        return Err(HomeworkError::BadRequestError(InternalError::new(
            "Unsupported thumbnail size",
            format!(
                "The requested thumbnail size of {}px is not supported. Supported values are: {:?}",
                width,
                Configuration::thumbnail_widths()
            ),
            "The requested thumbnail size is not supported.",
        )));
    }
    let app_config = request
        .app_data::<Arc<Configuration>>()
        .expect("The configuration must be accessible.");

    let mut thumbnail_path = app_config.application_thumbnail_folder_path();
    std::fs::create_dir_all(&thumbnail_path)?;
    thumbnail_path.push(format!("{}_{}.webp", uuid, width));
    if !thumbnail_path.try_exists()? {
        generate_thumbnail(uuid, width, Arc::clone(app_config)).await?;
    }

    Ok(NamedFile::from_file(File::open(&thumbnail_path)?, thumbnail_path)?)
}

/// Generates a thumbnail based on the specified ID and image width and writes it to
/// a file.
/// This method will fail if the specified ID does not belong to an image attachment.
/// If a width of `0` is specified the original image dimensions are used.
///
/// # Parameters
/// * `attachment_uuid` - the ID of the attachment image
/// * `width` - the width in px of the generated thumbnail
async fn generate_thumbnail(
    attachment_uuid: Uuid,
    width: u32,
    config: Arc<Configuration>,
) -> Result<(), HomeworkError> {
    let mut attachment_path = config.application_attachments_folder_path();
    attachment_path.push(attachment_uuid.to_string());
    let image_attachment = image::io::Reader::open(attachment_path)?
        .with_guessed_format()?
        .decode()?;
    let thumbnail = if width == 0 {
        image_attachment
    } else {
        image_attachment.thumbnail(width, width)
    }
    .into_rgba8();
    let mut thumbnail_path = config.application_thumbnail_folder_path();
    thumbnail_path.push(format!("{}_{}.webp", attachment_uuid, width));
    web::block(move || thumbnail.save_with_format(thumbnail_path, image::ImageFormat::WebP))
        .await??;
    Ok(())
}

fn attachment_path(request: &HttpRequest, uuid: Uuid) -> PathBuf {
    let app_config = request
        .app_data::<Arc<Configuration>>()
        .expect("The configuration must be accessible.");
    let mut file_path = app_config.application_attachments_folder_path();
    file_path.push(uuid.to_string());
    file_path
}

fn attachment_file_name_from_db(uuid: Uuid) -> Result<String, HomeworkError> {
    let conn = Configuration::database_connection()?;
    let mut stmt = conn
        .prepare("SELECT id, name, creation_time FROM attachment WHERE id = ?1")
        .map_err(HomeworkError::from)?;
    let attachment_option = stmt
        .query_map(params![uuid,], |row| Ok(Attachment::try_from(row)?))
        .map_err(HomeworkError::from)?
        .take(1)
        .last();
    let file_name = if attachment_option.is_some() {
        attachment_option.unwrap()?.name().to_string()
    } else {
        uuid.to_string()
    };
    Ok(file_name)
}
