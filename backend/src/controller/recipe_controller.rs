use std::collections::HashSet;

use actix_web::{web, HttpResponse, HttpResponseBuilder, Resource, Responder};
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::{
    application::{config::Configuration, error::HomeworkError},
    entity::{recipe::Recipe, attachment::Attachment},
};

use super::attachment_controller;

const COLUMN_STRING_TITLE: &str = "title";
const COLUMN_STRING_INSTRUCTIONS: &str = "instructions";
const COLUMN_STRING_REFERENCE: &str = "reference";

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
    let mut stmt = conn
        .prepare("SELECT id, title, instructions, reference, rating, creation_time FROM recipe")
        .map_err(HomeworkError::from)?;
    let recipes_sql = stmt
        .query_map([], |row| Ok(Recipe::try_from(row)?))
        .map_err(HomeworkError::from)?;
    let mut recipes: Vec<Recipe> = Vec::new();
    for recipe in recipes_sql {
        let mut recipe = recipe.map_err(HomeworkError::from)?;
        recipe.set_tags(tags_for_recipe(recipe.id(), &conn)?);
        recipe.set_attachments(attachments_for_recipe(recipe.id(), &conn)?);
        recipes.push(recipe);
    }
    Ok(web::Json(recipes))
}

pub async fn single_recipe(id: web::Path<Uuid>) -> Result<impl Responder, HomeworkError> {
    let uuid = id.into_inner();
    let conn = Configuration::database_connection()?;
    if !exists_in_database(uuid, &conn)? {
        return Err(HomeworkError::NotFoundError(Some("The recipe does not exist.".to_string())));
    }
    let mut stmt_recipe = conn
        .prepare("SELECT id, title, instructions, reference, rating, creation_time FROM recipe WHERE id = ?1")
        .map_err(HomeworkError::from)?;
    let mut recipe = stmt_recipe
        .query_map([uuid], |row| Ok(Recipe::try_from(row)?))
        .map_err(HomeworkError::from)?
        .last()
        .expect("The validity of the query was checked before.")?;
    recipe.set_tags(tags_for_recipe(uuid, &conn)?);
    recipe.set_attachments(attachments_for_recipe(recipe.id(), &conn)?);
    Ok(web::Json(recipe))
}

pub async fn create_recipe(title: web::Json<String>) -> Result<HttpResponse, HomeworkError> {
    let conn = Configuration::database_connection()?;
    // Generate a new UUID for the recipe.
    let uuid = Configuration::generate_uuid();

    // Save the recipe into the database.
    conn.execute(
        "INSERT INTO recipe (id, title, instructions, reference, rating, creation_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            uuid,
            title.as_str(),
            "",
            "",
            0,
            chrono::Utc::now()
        ],
    )?;

    // Return the UUID of the created recipe.
    Ok(HttpResponse::Created().body(uuid.to_string()))
}

pub async fn change_recipe_string_column(
    value: web::Json<String>,
    path: web::Path<(Uuid, String)>,
) -> Result<HttpResponseBuilder, HomeworkError> {
    let (uuid, column) = path.into_inner();
    let column = StringColumns::try_from(column)?;
    let conn = Configuration::database_connection()?;

    if !exists_in_database(uuid, &conn)? {
        return Err(HomeworkError::NotFoundError(Some("The recipe does not exist.".to_string())));
    }

    // Update the recipe in the database.
    conn.execute(
        &format!("UPDATE recipe SET {} = ?1 WHERE id = ?2", column),
        params![value.into_inner(), uuid],
    )?;

    Ok(HttpResponse::Ok())
}

pub async fn change_rating(
    rating: web::Json<u8>,
    id: web::Path<Uuid>,
) -> Result<HttpResponseBuilder, HomeworkError> {
    let uuid = id.into_inner();
    let conn = Configuration::database_connection()?;

    if !exists_in_database(uuid, &conn)? {
        return Err(HomeworkError::NotFoundError(Some("The recipe does not exist.".to_string())));
    }

    // Update the recipe in the database.
    conn.execute(
        "UPDATE recipe SET rating = ?1 WHERE id = ?2",
        params![rating.into_inner(), uuid],
    )?;

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

    if !exists_in_database(uuid, &conn)? {
        return Err(HomeworkError::NotFoundError(Some("The recipe does not exist.".to_string())));
    }

    // Add the tag to the database.
    conn.execute(
        "INSERT INTO tag_recipe_mapping (tag, recipe_id) VALUES (?1, ?2)",
        params![tag.into_inner().trim(), uuid],
    )?;

    Ok(HttpResponse::Created())
}

pub async fn remove_tag_from_recipe(
    path: web::Path<(Uuid, String)>,
) -> Result<HttpResponseBuilder, HomeworkError> {
    let (uuid, tag_name) = path.into_inner();
    let conn = Configuration::database_connection()?;

    if !exists_in_database(uuid, &conn)? {
        return Err(HomeworkError::NotFoundError(Some("The recipe does not exist.".to_string())));
    }

    // Add the tag to the database.
    conn.execute(
        "DELETE FROM tag_recipe_mapping WHERE tag = ?1 AND recipe_id = ?2",
        params![tag_name.trim(), uuid],
    )?;

    Ok(HttpResponse::Ok())
}

pub async fn add_attachment_to_recipe(
    attachment: web::Json<Uuid>,
    path: web::Path<Uuid>,
) -> Result<HttpResponseBuilder, HomeworkError> {
    let uuid_recipe = path.into_inner();
    let uuid_attachment = attachment.into_inner();
    let conn = Configuration::database_connection()?;

    if !exists_in_database(uuid_recipe, &conn)? {
        return Err(HomeworkError::NotFoundError(Some("The recipe does not exist.".to_string())));
    }

    if !attachment_controller::exists_in_database(uuid_attachment, &conn)? {
        return Err(HomeworkError::NotFoundError(Some(
            "The attachment does not exist.".to_string(),
        )));
    }

    // Add the association to the database.
    conn.execute(
        "INSERT INTO attachment_recipe_mapping (recipe_id, attachment_id) VALUES (?1, ?2)",
        params![uuid_recipe, uuid_attachment],
    )?;

    Ok(HttpResponse::Created())
}

pub fn exists_in_database(
    recipe_id: Uuid,
    connection: &Connection,
) -> Result<bool, rusqlite::Error> {
    let mut stmt = connection.prepare("SELECT 1 FROM recipe WHERE id = ?1")?;
    stmt.exists([recipe_id])
}

pub fn tags_for_recipe(
    recipe_id: Uuid,
    connection: &Connection,
) -> Result<Vec<String>, rusqlite::Error> {
    let mut tag_stmt =
        connection.prepare("SELECT tag FROM tag_recipe_mapping WHERE recipe_id = ?1")?;
    let tag_rows = tag_stmt.query_map([recipe_id], |row| Ok(row.get(0)?))?;
    let mut tags = Vec::new();
    for tag in tag_rows {
        tags.push(tag?);
    }
    Ok(tags)
}

pub fn attachments_for_recipe(
    recipe_id: Uuid,
    connection: &Connection,
) -> Result<Vec<Attachment>, rusqlite::Error> {
    let mut attachment_stmt = connection.prepare(
        "
            SELECT attachment.id, attachment.name, attachment.creation_time 
            FROM attachment 
            INNER JOIN attachment_recipe_mapping 
                ON attachment.id = attachment_recipe_mapping.attachment_id      
            WHERE attachment_recipe_mapping.recipe_id = ?1",
    )?;
    let attachment_rows =
        attachment_stmt.query_map([recipe_id], |row| Ok(Attachment::try_from(row)?))?;

    let mut attachments = Vec::new();
    for attachment in attachment_rows {
        attachments.push(attachment?);
    }
    Ok(attachments)
}

enum StringColumns {
    Title,
    Instructions,
    Reference,
}

impl std::fmt::Display for StringColumns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StringColumns::Title => COLUMN_STRING_TITLE,
                StringColumns::Instructions => COLUMN_STRING_INSTRUCTIONS,
                StringColumns::Reference => COLUMN_STRING_REFERENCE,
            }
        )
    }
}

impl TryFrom<&str> for StringColumns {
    type Error = HomeworkError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            COLUMN_STRING_TITLE => Ok(StringColumns::Title),
            COLUMN_STRING_INSTRUCTIONS => Ok(StringColumns::Instructions),
            COLUMN_STRING_REFERENCE => Ok(StringColumns::Reference),
            _ => Err(HomeworkError::NotFoundError(None)),
        }
    }
}

impl TryFrom<String> for StringColumns {
    type Error = HomeworkError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        StringColumns::try_from(value.as_str())
    }
}
