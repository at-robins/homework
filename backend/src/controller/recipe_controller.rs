use std::{collections::HashSet, sync::Arc};

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::info;
use uuid::Uuid;

use crate::{
    application::{
        config::Configuration,
        error::{HomeworkError, InternalError},
    },
    entity::{ingredient::Ingredient, recipe::Recipe}, service::application_service::{backup_service_from_request, configuration_from_request},
};

use super::attachment_controller;

/// Lists all recipes saved in the database.
pub async fn all_recipes() -> Result<impl Responder, HomeworkError> {
    let conn = Configuration::database_connection()?;
    Ok(web::Json(Recipe::select_all_from_database(&conn)?))
}

pub async fn single_recipe(id: web::Path<Uuid>) -> Result<impl Responder, HomeworkError> {
    let uuid = id.into_inner();
    let conn = Configuration::database_connection()?;
    Ok(web::Json(Recipe::select_from_database_by_id(uuid, &conn)?))
}

pub async fn create_recipe(
    title: web::Json<String>,
    request: HttpRequest,
) -> Result<HttpResponse, HomeworkError> {
    // Load the backup service.
    let backup_service = backup_service_from_request(&request);
    let conn = Configuration::database_connection()?;
    let title = title.into_inner();
    // Generate a new UUID for the recipe.
    let uuid = Configuration::generate_uuid();
    Recipe::insert_into_database_new_entry(uuid, &title, &conn)?;
    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();
    // Return the UUID of the created recipe.
    Ok(HttpResponse::Created().body(uuid.to_string()))
}

pub async fn remove_recipe(
    path: web::Path<Uuid>,
    request: HttpRequest,
) -> Result<HttpResponse, HomeworkError> {
    // Load the backup service and configuration.
    let backup_service = backup_service_from_request(&request);
    let config  = configuration_from_request(&request);
    let uuid_recipe = path.into_inner();
    let conn = Configuration::database_connection()?;
    let recipe = Recipe::select_from_database_by_id(uuid_recipe, &conn)?;
    // Delete all corresponding attachments.
    for recipe_attachment in recipe.attachments() {
        attachment_controller::delete_attachment(Arc::clone(&config), recipe_attachment.id())?;
    }
    Recipe::delete_from_database_by_id(uuid_recipe, &conn)?;
    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();
    info!("Removed recipe {}.", uuid_recipe);
    Ok(HttpResponse::Ok().finish())
}

pub async fn change_recipe_string_column(
    value: web::Json<String>,
    path: web::Path<(Uuid, String)>,
    request: HttpRequest,
) -> Result<HttpResponse, HomeworkError> {
    // Load the backup service.
    let backup_service = backup_service_from_request(&request);
    let (uuid, column) = path.into_inner();
    let value = value.into_inner();
    let conn = Configuration::database_connection()?;
    Recipe::update_in_database_string_column(uuid, &column, &value, &conn)?;
    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();
    Ok(HttpResponse::Ok().finish())
}

pub async fn change_rating(
    rating: web::Json<u8>,
    id: web::Path<Uuid>,
    request: HttpRequest,
) -> Result<HttpResponse, HomeworkError> {
    // Load the backup service.
    let backup_service = backup_service_from_request(&request);
    let uuid = id.into_inner();
    let conn = Configuration::database_connection()?;
    Recipe::update_in_database_rating(uuid, rating.into_inner(), &conn)?;
    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();
    Ok(HttpResponse::Ok().finish())
}

pub async fn all_recipe_tags() -> Result<impl Responder, HomeworkError> {
    let conn = Configuration::database_connection()?;
    let mut stmt = conn.prepare("SELECT tag FROM tag_recipe_mapping")?;
    let tag_rows = stmt.query_map([], |row| Ok(row.get(0)?))?;
    let mut tags: HashSet<String> = HashSet::new();
    for tag in tag_rows {
        tags.insert(tag?);
    }
    Ok(web::Json(tags))
}

pub async fn add_tag_to_recipe(
    tag: web::Json<String>,
    path: web::Path<Uuid>,
    request: HttpRequest,
) -> Result<HttpResponse, HomeworkError> {
    // Load the backup service.
    let backup_service = backup_service_from_request(&request);
    let uuid = path.into_inner();
    let conn = Configuration::database_connection()?;
    let tag = tag.into_inner();
    Recipe::update_in_database_insert_tag(uuid, &tag, &conn)?;
    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();
    Ok(HttpResponse::Created().finish())
}

pub async fn remove_tag_from_recipe(
    path: web::Path<(Uuid, String)>,
    request: HttpRequest,
) -> Result<HttpResponse, HomeworkError> {
    // Load the backup service.
    let backup_service = backup_service_from_request(&request);
    let (uuid, tag_name) = path.into_inner();
    let conn = Configuration::database_connection()?;
    Recipe::update_in_database_delete_tag(uuid, &tag_name, &conn)?;
    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();
    Ok(HttpResponse::Ok().finish())
}

pub async fn add_attachment_to_recipe(
    attachment: web::Json<Uuid>,
    path: web::Path<Uuid>,
    request: HttpRequest,
) -> Result<HttpResponse, HomeworkError> {
    // Load the backup service.
    let backup_service = backup_service_from_request(&request);
    let uuid_recipe = path.into_inner();
    let uuid_attachment = attachment.into_inner();
    let conn = Configuration::database_connection()?;
    Recipe::update_in_database_insert_attachment(uuid_recipe, uuid_attachment, &conn)?;
    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();
    Ok(HttpResponse::Created().finish())
}

pub async fn set_thumbnail_for_recipe(
    attachment: web::Json<Option<Uuid>>,
    path: web::Path<Uuid>,
    request: HttpRequest,
) -> Result<HttpResponse, HomeworkError> {
    // Load the backup service.
    let backup_service = backup_service_from_request(&request);
    let uuid_recipe = path.into_inner();
    let uuid_attachment = attachment.into_inner();
    let conn = Configuration::database_connection()?;
    Recipe::update_in_database_thumbnail(uuid_recipe, uuid_attachment, &conn)?;
    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();
    Ok(HttpResponse::Ok().finish())
}

pub async fn add_ingredient_to_recipe(
    ingredient: web::Json<Ingredient>,
    path: web::Path<Uuid>,
    request: HttpRequest,
) -> Result<HttpResponse, HomeworkError> {
    // Load the backup service.
    let backup_service = backup_service_from_request(&request);
    let uuid_recipe = path.into_inner();
    let mut ingredient = ingredient.into_inner();
    // Overwrite the UUID.
    let uuid_ingredient = Configuration::generate_uuid();
    ingredient.set_id(uuid_ingredient);
    ingredient.set_recipe_id(uuid_recipe);
    let conn = Configuration::database_connection()?;
    ingredient.insert_into_database(&conn)?;
    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();
    Ok(HttpResponse::Created().body(uuid_ingredient.to_string()))
}

pub async fn modify_ingredient(
    ingredient: web::Json<Ingredient>,
    path: web::Path<Uuid>,
    request: HttpRequest,
) -> Result<HttpResponse, HomeworkError> {
    // Load the backup service.
    let backup_service = backup_service_from_request(&request);
    let uuid_recipe = path.into_inner();
    let ingredient = ingredient.into_inner();
    if uuid_recipe != ingredient.recipe_id() {
        return Err(HomeworkError::BadRequestError(InternalError::new(
            "ID missmatch",
            format!(
            "The requested recipe {} does not match the recipe referenced by the ingredient {}.",
            uuid_recipe,
            ingredient.recipe_id()
        ),
            "Mismatching recipe and ingredient during ingredient modification.",
        )));
    }
    let conn = Configuration::database_connection()?;
    ingredient.update_in_database(&conn)?;
    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();
    Ok(HttpResponse::Ok().finish())
}

pub async fn modify_ingredients_ordering(
    ingredients: web::Json<Vec<Uuid>>,
    path: web::Path<Uuid>,
    request: HttpRequest,
) -> Result<HttpResponse, HomeworkError> {
    // Load the backup service.
    let backup_service = backup_service_from_request(&request);
    let uuid_recipe = path.into_inner();
    let ingredient_uuids = ingredients.into_inner();
    let conn = Configuration::database_connection()?;
    let ingredient_uuids_recipe: Vec<Uuid> =
        Ingredient::select_from_database_by_recipe_id(uuid_recipe, &conn)?
            .iter()
            .map(|ingredient| ingredient.id())
            .collect();
    if ingredient_uuids
        .iter()
        .any(|id| !ingredient_uuids_recipe.contains(id))
    {
        return Err(HomeworkError::BadRequestError(InternalError::new(
            "Ingredient missmatch",
            format!(
            "The requested recipe {} does not contain the specified ingredients {:?}, only the following ingredients are contained: {:?} ",
            uuid_recipe,
            ingredient_uuids,
            ingredient_uuids_recipe
        ),
            "The requested recipe does not contain the specified ingredients.",
        )));
    }
    for (i, ingredient_uuid) in ingredient_uuids.iter().enumerate() {
        Ingredient::update_ordering_by_id(i as i32, *ingredient_uuid, &conn)?;
    }
    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();
    Ok(HttpResponse::Ok().finish())
}

pub async fn remove_ingredient_from_recipe(
    path: web::Path<(Uuid, Uuid)>,
    request: HttpRequest,
) -> Result<HttpResponse, HomeworkError> {
    // Load the backup service.
    let backup_service = backup_service_from_request(&request);
    let (_, uuid_ingredient) = path.into_inner();
    let conn = Configuration::database_connection()?;
    Ingredient::delete_from_database_by_id(uuid_ingredient, &conn)?;
    // Request a backup as internal data changed.
    backup_service.lock().request_timed_backup();
    Ok(HttpResponse::Ok().finish())
}
