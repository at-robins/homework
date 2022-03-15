use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::error::HomeworkError;

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

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn exists_in_database(&self, connection: &Connection) -> Result<bool, rusqlite::Error> {
        Ingredient::exists_in_database_by_id(self.id(), connection)
    }

    pub fn insert_into_database(&self, connection: &Connection) -> Result<(), HomeworkError> {
        if self.exists_in_database(connection)? {
            return Err(HomeworkError::GenericError(format!(
                "The ingredient {} already exists.",
                self.id()
            )));
        }

        if !Recipe::exists_in_database_by_id(self.recipe_id(), connection)? {
            return Err(HomeworkError::NotFoundError(Some(format!(
                "The recipe {} referenced by ingredient {} does not exist.",
                self.recipe_id(),
                self.id()
            ))));
        }

        connection.execute(
            "INSERT INTO ingredient (id, amount, unit, text, creation_time, recipe_reference, recipe_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![self.id(), self.amount(), self.unit(), self.text(), chrono::Utc::now(), self.recipe_reference(), self.recipe_id()],
        )?;

        Ok(())
    }

    pub fn update_in_database(&self, connection: &Connection) -> Result<(), HomeworkError> {
        if !self.exists_in_database(connection)? {
            return Err(HomeworkError::NotFoundError(Some(format!(
                "The ingredient {} does not exists.",
                self.id()
            ))));
        }

        if !Recipe::exists_in_database_by_id(self.recipe_id(), connection)? {
            return Err(HomeworkError::NotFoundError(Some(format!(
                "The recipe {} referenced by ingredient {} does not exist.",
                self.recipe_id(),
                self.id()
            ))));
        }

        connection.execute(
            "UPDATE ingredient SET amount = ?1, unit = ?2, text = ?3, recipe_reference = ?4 WHERE id = ?5",
            params![self.amount(), self.unit(), self.text(), self.recipe_reference(), self.id()],
        )?;

        Ok(())
    }

    pub fn exists_in_database_by_id(
        ingredient_id: Uuid,
        connection: &Connection,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = connection.prepare("SELECT 1 FROM ingredient WHERE id = ?1")?;
        stmt.exists([ingredient_id])
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
        })
    }
}
