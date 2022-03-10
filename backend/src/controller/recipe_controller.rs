use actix_web::{web, HttpResponse, HttpResponseBuilder, Resource, Responder};
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::{
    application::{config::Configuration, error::HomeworkError},
    entity::recipe::Recipe,
};

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

/// Lists all recipes saved in the database.
pub async fn all_recipes() -> actix_web::Result<impl Responder> {
    let conn = Configuration::database_connection()?;
    let mut stmt = conn
        .prepare("SELECT id, title, instructions, reference, rating, creation_time FROM recipe")
        .map_err(HomeworkError::from)?;
    let recipes_sql = stmt
        .query_map([], |row| Ok(Recipe::try_from(row)?))
        .map_err(HomeworkError::from)?;
    let mut recipes: Vec<Recipe> = Vec::new();
    for recipe in recipes_sql {
        recipes.push(recipe.map_err(HomeworkError::from)?);
    }
    Ok(web::Json(recipes))
}

pub async fn single_recipe(id: web::Path<Uuid>) -> Result<impl Responder, HomeworkError> {
    let uuid = id.into_inner();
    let conn = Configuration::database_connection()?;
    if !exists_in_database(uuid, &conn)? {
        return Err(HomeworkError::NotFoundError(Some("The recipe does not exist.".to_string())));
    }
    let mut stmt = conn
        .prepare("SELECT id, title, instructions, reference, rating, creation_time FROM recipe WHERE id = ?1")
        .map_err(HomeworkError::from)?;
    let recipe = stmt
        .query_map([uuid], |row| Ok(Recipe::try_from(row)?))
        .map_err(HomeworkError::from)?
        .last()
        .expect("The validity of the query was checked before.")?;
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

pub fn exists_in_database(
    recipe_id: Uuid,
    connection: &Connection,
) -> Result<bool, rusqlite::Error> {
    let mut stmt = connection.prepare("SELECT 1 FROM recipe WHERE id = ?1")?;
    stmt.exists([recipe_id])
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
