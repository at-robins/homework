use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::error::{HomeworkError, InternalError};

use super::recipe::Recipe;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    id: Uuid,
    amount: String,
    unit: String,
    text: String,
    creation_time: DateTime<Utc>,
    recipe_reference: Option<Uuid>,
    recipe_id: Uuid,
    ordering: i32,
    filter_text: Option<String>,
}

impl Ingredient {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn amount(&self) -> &str {
        &self.amount
    }

    pub fn unit(&self) -> &str {
        &self.unit
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn recipe_reference(&self) -> Option<Uuid> {
        self.recipe_reference
    }

    pub fn recipe_id(&self) -> Uuid {
        self.recipe_id
    }

    pub fn ordering(&self) -> i32 {
        self.ordering
    }

    pub fn filter_text(&self) -> Option<&str> {
        self.filter_text.as_ref().map(|value| value.as_str())
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn set_recipe_id(&mut self, recipe_id: Uuid) {
        self.recipe_id = recipe_id;
    }

    pub fn select_from_database_by_recipe_id(
        recipe_id: Uuid,
        connection: &Connection,
    ) -> Result<Vec<Ingredient>, rusqlite::Error> {
        let mut ingredient_stmt = connection.prepare(
            "
                SELECT id, amount, unit, text, creation_time, recipe_reference, recipe_id, ordering, filter_text
                FROM ingredient
                WHERE recipe_id = ?1",
        )?;
        let ingredient_rows =
            ingredient_stmt.query_map([recipe_id], |row| Ok(Ingredient::try_from(row)?))?;

        let mut ingredients = Vec::new();
        for ingredient in ingredient_rows {
            ingredients.push(ingredient?);
        }
        Ok(ingredients)
    }

    pub fn insert_into_database(&self, connection: &Connection) -> Result<(), HomeworkError> {
        if Self::exists_in_database_by_id(self.id(), connection)? {
            return Err(HomeworkError::NotFoundError(InternalError::new(
                "Ingredient already exists",
                format!("The ingredient {} already exists.", self.id()),
                "The ingredient already exists.",
            )));
        }

        if !Recipe::exists_in_database_by_id(self.recipe_id(), connection)? {
            return Err(HomeworkError::NotFoundError(InternalError::new(
                "Reference error",
                format!(
                    "The recipe {} referenced by ingredient {} does not exist.",
                    self.recipe_id(),
                    self.id()
                ),
                "The recipe referenced by the ingredient does not exist.",
            )));
        }

        connection.execute(
            "INSERT INTO ingredient (id, amount, unit, text, creation_time, recipe_reference, recipe_id, ordering, filter_text) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![self.id(), self.amount(), self.unit(), self.text(), chrono::Utc::now(), self.recipe_reference(), self.recipe_id(), self.ordering(), self.filter_text()],
        )?;

        Ok(())
    }

    pub fn update_in_database(&self, connection: &Connection) -> Result<(), HomeworkError> {
        Self::exists_in_database_by_id_throw_not_found(self.id(), &connection)?;

        if !Recipe::exists_in_database_by_id(self.recipe_id(), connection)? {
            return Err(HomeworkError::NotFoundError(InternalError::new(
                "Reference error",
                format!(
                    "The recipe {} referenced by ingredient {} does not exist.",
                    self.recipe_id(),
                    self.id()
                ),
                "The recipe referenced by the ingredient does not exist.",
            )));
        }

        connection.execute(
            "UPDATE ingredient SET amount = ?1, unit = ?2, text = ?3, recipe_reference = ?4, ordering = ?5, filter_text = ?6 WHERE id = ?7",
            params![self.amount(), self.unit(), self.text(), self.recipe_reference(), self.ordering(), self.filter_text(), self.id()],
        )?;

        Ok(())
    }

    pub fn update_ordering_by_id(
        ordering: i32,
        id: Uuid,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        Self::exists_in_database_by_id_throw_not_found(id, &connection)?;

        connection
            .execute("UPDATE ingredient SET ordering = ?1 WHERE id = ?2", params![ordering, id])?;

        Ok(())
    }

    pub fn delete_from_database_by_id(
        id: Uuid,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        Self::exists_in_database_by_id_throw_not_found(id, &connection)?;

        connection.execute("DELETE FROM ingredient WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn exists_in_database_by_id(
        ingredient_id: Uuid,
        connection: &Connection,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = connection.prepare("SELECT 1 FROM ingredient WHERE id = ?1")?;
        stmt.exists([ingredient_id])
    }

    /// Automatically throws a ```Not Found``` if the entry does not exist.
    /// Returns an ```Ok``` otherwise.
    ///
    /// Parameters
    ///
    /// * ```ingredient_id``` - the ID of the ingredient
    /// * ```connection``` - the database connection
    pub fn exists_in_database_by_id_throw_not_found(
        ingredient_id: Uuid,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        if !Self::exists_in_database_by_id(ingredient_id, &connection)? {
            Err(HomeworkError::NotFoundError(InternalError::new(
                "Ingredient not found",
                format!("The ingredient {} does not exist.", ingredient_id),
                "The ingredient does not exist.",
            )))
        } else {
            Ok(())
        }
    }
}

impl TryFrom<&Row<'_>> for Ingredient {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Ingredient {
            id: row.get(0)?,
            amount: row.get(1)?,
            unit: row.get(2)?,
            text: row.get(3)?,
            creation_time: row.get(4)?,
            recipe_reference: row.get(5)?,
            recipe_id: row.get(6)?,
            ordering: row.get(7)?,
            filter_text: row.get(8)?,
        })
    }
}
