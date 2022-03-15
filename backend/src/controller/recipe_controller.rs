use std::collections::HashSet;

use actix_web::{web, HttpResponse, HttpResponseBuilder, Resource, Responder};
use rusqlite::params;
use uuid::Uuid;

use crate::{
    application::{config::Configuration, error::HomeworkError},
    entity::{ingredient::Ingredient, recipe::Recipe},
};

use super::attachment_controller;

pub fn recipes_routing() -> Resource {
    web::resource("/api/recipes")
        .route(web::get().to(all_recipes))
        .route(web::post().to(create_recipe))
}

pub fn recipe_routing() -> Resource {
    web::resource("/api/recipe/{id}").route(web::get().to(single_recipe))
}

pub fn recipe_string_routing() -> Resource {
    web::resource("/api/recipe/{id}/string/{string_param}")
        .route(web::post().to(change_recipe_string_column))
}

pub fn recipe_rating_routing() -> Resource {
    web::resource("/api/recipe/{id}/rating").route(web::post().to(change_rating))
}

pub fn recipe_tags_routing() -> Resource {
    web::resource("/api/recipe/{id}/tags").route(web::post().to(add_tag_to_recipe))
}

pub fn recipe_tag_delete_routing() -> Resource {
    web::resource("/api/recipe/{id}/tag/{tag_name}").route(web::delete().to(remove_tag_from_recipe))
}

pub fn recipes_tags_routing() -> Resource {
    web::resource("/api/recipes/tags").route(web::get().to(all_recipe_tags))
}

pub fn recipe_attachments_routing() -> Resource {
    web::resource("/api/recipe/{id}/attachments").route(web::post().to(add_attachment_to_recipe))
}

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

pub async fn create_recipe(title: web::Json<String>) -> Result<HttpResponse, HomeworkError> {
    let conn = Configuration::database_connection()?;
    let title = title.into_inner();
    // Generate a new UUID for the recipe.
    let uuid = Configuration::generate_uuid();
    Recipe::insert_into_database_new_entry(uuid, &title, &conn);
    // Return the UUID of the created recipe.
    Ok(HttpResponse::Created().body(uuid.to_string()))
}

pub async fn change_recipe_string_column(
    value: web::Json<String>,
    path: web::Path<(Uuid, String)>,
) -> Result<HttpResponseBuilder, HomeworkError> {
    let (uuid, column) = path.into_inner();
    let value = value.into_inner();
    let conn = Configuration::database_connection()?;
    Recipe::update_in_database_string_column(uuid, &column, &value, &conn)?;
    Ok(HttpResponse::Ok())
}

pub async fn change_rating(
    rating: web::Json<u8>,
    id: web::Path<Uuid>,
) -> Result<HttpResponseBuilder, HomeworkError> {
    let uuid = id.into_inner();
    let conn = Configuration::database_connection()?;
    Recipe::update_in_database_rating(uuid, rating.into_inner(), &conn)?;
    Ok(HttpResponse::Ok())
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
) -> Result<HttpResponseBuilder, HomeworkError> {
    let uuid = path.into_inner();
    let conn = Configuration::database_connection()?;
    let tag = tag.into_inner();
    Recipe::update_in_database_insert_tag(uuid, &tag, &conn)?;
    Ok(HttpResponse::Created())
}

pub async fn remove_tag_from_recipe(
    path: web::Path<(Uuid, String)>,
) -> Result<HttpResponseBuilder, HomeworkError> {
    let (uuid, tag_name) = path.into_inner();
    let conn = Configuration::database_connection()?;
    Recipe::update_in_database_delete_tag(uuid, &tag_name, &conn)?;
    Ok(HttpResponse::Ok())
}

pub async fn add_attachment_to_recipe(
    attachment: web::Json<Uuid>,
    path: web::Path<Uuid>,
) -> Result<HttpResponseBuilder, HomeworkError> {
    let uuid_recipe = path.into_inner();
    let uuid_attachment = attachment.into_inner();
    let conn = Configuration::database_connection()?;
    Recipe::update_in_database_insert_attachment(uuid_recipe, uuid_attachment, &conn)?;
    Ok(HttpResponse::Created())
}

pub async fn add_ingredient_to_recipe(
    ingredient: web::Json<Ingredient>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, HomeworkError> {
    let uuid_recipe = path.into_inner();
    let mut ingredient = ingredient.into_inner();
    // Overwrite the UUID.
    let uuid_ingredient = Configuration::generate_uuid();
    ingredient.set_id(uuid_ingredient);
    let conn = Configuration::database_connection()?;
    ingredient.insert_into_database(&conn)?;
    Ok(HttpResponse::Created().body(uuid_ingredient.to_string()))
}

pub async fn modify_ingredient(
    ingredient: web::Json<Ingredient>,
    path: web::Path<Uuid>,
) -> Result<HttpResponseBuilder, HomeworkError> {
    let uuid_recipe = path.into_inner();
    let ingredient = ingredient.into_inner();
    let conn = Configuration::database_connection()?;
    ingredient.update_in_database(&conn)?;
    Ok(HttpResponse::Ok())
}
