use std::{fs::File, io::Write, path::Path, sync::Arc};

use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, HttpResponseBuilder, Responder};
use futures_util::TryStreamExt as _;
use rusqlite::params;
use uuid::Uuid;

use crate::{
    application::{config::Configuration, error::HomeworkError},
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

pub async fn add_attachment(mut payload: Multipart) -> Result<HttpResponse, HomeworkError> {
    // Open a databse connection first so the file is not saved in case of connection errors.
    let conn = Configuration::database_connection()?;

    // Generate a new UUID for the attachment.
    let uuid = Configuration::generate_uuid();

    // Create the attachment folder.
    let mut file_path = Configuration::application_attachments_folder_path();
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

    // Return the UUID of the created attachment.
    Ok(HttpResponse::Created().body(uuid.to_string()))
}

pub async fn delete_attachment(id: web::Path<Uuid>) -> Result<HttpResponseBuilder, HomeworkError> {
    let uuid: Uuid = id.into_inner();
    // Open a databse connection first so the file is not deleted in case of connection errors.
    let conn = Configuration::database_connection()?;

    // Get the attachment file path.
    let mut file_path = Configuration::application_attachments_folder_path();
    file_path.push(uuid.to_string());

    // Remove the attachment from the disk.
    std::fs::remove_file(file_path)?;

    // Remove the attachment from the database.
    conn.execute("DELETE FROM attachment WHERE id = ?1", params![uuid,])?;

    // Return the UUID of the created attachment.
    Ok(HttpResponse::Ok())
}

pub async fn download_attachment(id: web::Path<Uuid>) -> Result<NamedFile, HomeworkError> {
    let uuid: Uuid = id.into_inner();
    // Get the attachment file path.
    let mut file_path = Configuration::application_attachments_folder_path();
    file_path.push(uuid.to_string());
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
    Ok(NamedFile::from_file(File::open(file_path)?, file_name)?)
}
