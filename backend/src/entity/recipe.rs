use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::error::{HomeworkError, InternalError};

use super::{attachment::Attachment, ingredient::Ingredient};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    id: Uuid,
    title: String,
    instructions: String,
    reference: String,
    rating: u8,
    thumbnail: Option<Attachment>,
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
        Self::exists_in_database_by_id_throw_not_found(recipe_id, &connection)?;
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
        Self::exists_in_database_by_id_throw_not_found(id, &connection)?;

        let column = StringColumns::try_from(column)?;
        connection.execute(
            &format!("UPDATE recipe SET {} = ?1 WHERE id = ?2", column),
            params![value, id],
        )?;
        Ok(())
    }

    pub fn update_in_database_thumbnail(
        id: Uuid,
        attachment_id: Option<Uuid>,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        Self::exists_in_database_by_id_throw_not_found(id, &connection)?;

        if let Some(thumbnail_id) = attachment_id {
            Attachment::exists_in_database_by_id_throw_not_found(thumbnail_id, &connection)?;
            connection.execute(
                "UPDATE recipe SET thumbnail = ?1 WHERE id = ?2",
                params![thumbnail_id, id],
            )?;
        } else {
            connection.execute("UPDATE recipe SET thumbnail = NULL WHERE id = ?1", params![id])?;
        }

        Ok(())
    }

    pub fn update_in_database_rating(
        id: Uuid,
        value: u8,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        Self::exists_in_database_by_id_throw_not_found(id, &connection)?;

        connection.execute("UPDATE recipe SET rating = ?1 WHERE id = ?2", params![value, id])?;
        Ok(())
    }

    pub fn update_in_database_insert_tag(
        id: Uuid,
        tag: &str,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        Self::exists_in_database_by_id_throw_not_found(id, &connection)?;

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
        Self::exists_in_database_by_id_throw_not_found(id, &connection)?;

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
        Self::exists_in_database_by_id_throw_not_found(recipe_id, &connection)?;
        Attachment::exists_in_database_by_id_throw_not_found(attachment_id, connection)?;

        connection.execute(
            "INSERT INTO attachment_recipe_mapping (recipe_id, attachment_id) VALUES (?1, ?2)",
            params![recipe_id, attachment_id],
        )?;
        Ok(())
    }

    pub fn delete_from_database_by_id(
        id: Uuid,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        Self::exists_in_database_by_id_throw_not_found(id, &connection)?;

        connection.execute("DELETE FROM recipe WHERE id = ?1", params![id])?;
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

    pub fn thumbnail_by_id(
        recipe_id: Uuid,
        connection: &Connection,
    ) -> Result<Option<Attachment>, rusqlite::Error> {
        let mut attachment_stmt = connection.prepare(
            "
                SELECT id, name, creation_time 
                FROM attachment 
                WHERE id = (
                    SELECT thumbnail FROM recipe WHERE id = ?1
                )",
        )?;
        let attachment_option = attachment_stmt
            .query_map([recipe_id], |row| Ok(Attachment::try_from(row)?))?
            .next();

        if attachment_option.is_some() {
            Ok(Some(attachment_option.unwrap()?))
        } else {
            Ok(None)
        }
    }

    pub fn exists_in_database_by_id(
        recipe_id: Uuid,
        connection: &Connection,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = connection.prepare("SELECT 1 FROM recipe WHERE id = ?1")?;
        stmt.exists([recipe_id])
    }

    /// Automatically throws a ```Not Found``` if the entry does not exist.
    /// Returns an ```Ok``` otherwise.
    ///
    /// Parameters
    ///
    /// * ```recipe_id``` - the ID of the recipe
    /// * ```connection``` - the database connection
    pub fn exists_in_database_by_id_throw_not_found(
        recipe_id: Uuid,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        if !Self::exists_in_database_by_id(recipe_id, &connection)? {
            Err(HomeworkError::NotFoundError(InternalError::new(
                "Recipe not found",
                format!("The recipe {} does not exist.", recipe_id),
                "The recipe does not exist.",
            )))
        } else {
            Ok(())
        }
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
            thumbnail: Recipe::thumbnail_by_id(id, connection)?,
            attachments: Recipe::attachments_by_id(id, connection)?,
            ingredients: Ingredient::select_from_database_by_recipe_id(id, connection)?,
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
            _ => Err(HomeworkError::NotFoundError(InternalError::new(
                "Invalid value",
                format!("\"{}\" is not a valid recipe column value.", value),
                "An invalid value was supplied.",
            ))),
        }
    }
}
