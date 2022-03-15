use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::error::HomeworkError;

use super::{attachment::Attachment, ingredient::Ingredient};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    id: Uuid,
    title: String,
    instructions: String,
    reference: String,
    rating: u8,
    tags: Vec<String>,
    attachments: Vec<Attachment>,
    ingredients: Vec<Ingredient>,
    creation_time: DateTime<Utc>,
}

impl Recipe {
    pub fn select_from_database_by_id(
        recipe_id: Uuid,
        connection: &Connection,
    ) -> Result<Recipe, HomeworkError> {
        if !Recipe::exists_in_database_by_id(recipe_id, &connection)? {
            return Err(HomeworkError::NotFoundError(Some(format!(
                "The recipe {} does not exist.",
                recipe_id
            ))));
        }
        let mut stmt_recipe = connection
            .prepare("SELECT id, title, instructions, reference, rating, creation_time FROM recipe WHERE id = ?1")?;
        let recipe = stmt_recipe
            .query_map([recipe_id], |row| Ok(Recipe::try_from((row, connection))?))?
            .last()
            .expect("The validity of the query was checked before.")?;

        Ok(recipe)
    }

    pub fn select_all_from_database(connection: &Connection) -> Result<Vec<Recipe>, HomeworkError> {
        let mut stmt_recipe = connection.prepare(
            "SELECT id, title, instructions, reference, rating, creation_time FROM recipe",
        )?;
        let recipe_query =
            stmt_recipe.query_map([], |row| Ok(Recipe::try_from((row, connection))?))?;
        let mut recipes = Vec::new();
        for recipe in recipe_query {
            recipes.push(recipe?);
        }
        Ok(recipes)
    }

    pub fn insert_into_database_new_entry(
        id: Uuid,
        title: &str,
        connection: &Connection,
    ) -> Result<(), rusqlite::Error> {
        connection.execute(
            "INSERT INTO recipe (id, title, instructions, reference, rating, creation_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                id,
                title,
                "",
                "",
                0,
                chrono::Utc::now()
            ],
        )?;
        Ok(())
    }

    pub fn update_in_database_string_column(
        id: Uuid,
        column: &str,
        value: &str,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        if !Recipe::exists_in_database_by_id(id, connection)? {
            return Err(HomeworkError::NotFoundError(Some(format!(
                "The recipe {} does not exist.",
                id
            ))));
        }

        let column = StringColumns::try_from(column)?;
        connection.execute(
            &format!("UPDATE recipe SET {} = ?1 WHERE id = ?2", column),
            params![value, id],
        )?;
        Ok(())
    }

    pub fn update_in_database_rating(
        id: Uuid,
        value: u8,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        if !Recipe::exists_in_database_by_id(id, connection)? {
            return Err(HomeworkError::NotFoundError(Some(format!(
                "The recipe {} does not exist.",
                id
            ))));
        }

        connection.execute("UPDATE recipe SET rating = ?1 WHERE id = ?2", params![value, id])?;
        Ok(())
    }

    pub fn update_in_database_insert_tag(
        id: Uuid,
        tag: &str,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        if !Recipe::exists_in_database_by_id(id, connection)? {
            return Err(HomeworkError::NotFoundError(Some(format!(
                "The recipe {} does not exist.",
                id
            ))));
        }

        connection.execute(
            "INSERT INTO tag_recipe_mapping (tag, recipe_id) VALUES (?1, ?2)",
            params![tag.trim(), id],
        )?;
        Ok(())
    }

    pub fn update_in_database_delete_tag(
        id: Uuid,
        tag: &str,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        if !Recipe::exists_in_database_by_id(id, connection)? {
            return Err(HomeworkError::NotFoundError(Some(format!(
                "The recipe {} does not exist.",
                id
            ))));
        }

        connection.execute(
            "DELETE FROM tag_recipe_mapping WHERE tag = ?1 AND recipe_id = ?2",
            params![tag.trim(), id],
        )?;
        Ok(())
    }

    pub fn update_in_database_insert_attachment(
        recipe_id: Uuid,
        attachment_id: Uuid,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        if !Recipe::exists_in_database_by_id(recipe_id, connection)? {
            return Err(HomeworkError::NotFoundError(Some(format!(
                "The recipe {} does not exist.",
                recipe_id
            ))));
        }

        if !Attachment::exists_in_database_by_id(attachment_id, connection)? {
            return Err(HomeworkError::NotFoundError(Some(format!(
                "The attachment {} does not exist.",
                attachment_id
            ))));
        }

        connection.execute(
            "INSERT INTO attachment_recipe_mapping (recipe_id, attachment_id) VALUES (?1, ?2)",
            params![recipe_id, attachment_id],
        )?;
        Ok(())
    }

    pub fn tags_by_id(
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

    pub fn attachments_by_id(
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

    pub fn ingredients_by_id(
        recipe_id: Uuid,
        connection: &Connection,
    ) -> Result<Vec<Ingredient>, rusqlite::Error> {
        let mut ingredient_stmt = connection.prepare(
            "
                SELECT ingredient.id, ingredient.amount, ingredient.unit, ingredient.recipe_reference, ingredient.creation_time 
                FROM ingredient 
                INNER JOIN ingredient_recipe_mapping 
                    ON ingredient.id = ingredient_recipe_mapping.ingredient_id      
                WHERE ingredient_recipe_mapping.recipe_id = ?1",
        )?;
        let ingredient_rows =
            ingredient_stmt.query_map([recipe_id], |row| Ok(Ingredient::try_from(row)?))?;

        let mut ingredients = Vec::new();
        for ingredient in ingredient_rows {
            ingredients.push(ingredient?);
        }
        Ok(ingredients)
    }

    pub fn exists_in_database_by_id(
        recipe_id: Uuid,
        connection: &Connection,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = connection.prepare("SELECT 1 FROM recipe WHERE id = ?1")?;
        stmt.exists([recipe_id])
    }
}

impl TryFrom<(&Row<'_>, &Connection)> for Recipe {
    type Error = rusqlite::Error;

    fn try_from(data: (&Row, &Connection)) -> Result<Self, Self::Error> {
        let (row, connection) = data;
        let id = row.get(0)?;
        Ok(Recipe {
            id,
            title: row.get(1)?,
            instructions: row.get(2)?,
            reference: row.get(3)?,
            rating: row.get(4)?,
            tags: Recipe::tags_by_id(id, connection)?,
            attachments: Recipe::attachments_by_id(id, connection)?,
            ingredients: Recipe::ingredients_by_id(id, connection)?,
            creation_time: row.get(5)?,
        })
    }
}

const COLUMN_STRING_TITLE: &str = "title";
const COLUMN_STRING_INSTRUCTIONS: &str = "instructions";
const COLUMN_STRING_REFERENCE: &str = "reference";

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
            _ => Err(HomeworkError::NotFoundError(Some(format!(
                "\"{}\" is not a valid value.",
                value
            )))),
        }
    }
}
