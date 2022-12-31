use chrono::{DateTime, Utc};
use rusqlite::{Connection, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::error::{HomeworkError, InternalError};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    id: Uuid,
    name: String,
    creation_time: DateTime<Utc>,
}

impl Attachment {
    /// Returns the ID of this `Attachment`.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Returns the name of this `Attachment`.
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn exists_in_database_by_id(
        id: Uuid,
        connection: &Connection,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = connection.prepare("SELECT 1 FROM attachment WHERE id = ?1")?;
        stmt.exists([id])
    }

    /// Automatically throws a ```Not Found``` if the entry does not exist.
    /// Returns an ```Ok``` otherwise.
    /// 
    /// Parameters
    /// 
    /// * ```attachment_id``` - the ID of the attachment
    /// * ```connection``` - the database connection
    pub fn exists_in_database_by_id_throw_not_found(
        attachment_id: Uuid,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        if !Self::exists_in_database_by_id(attachment_id, &connection)? {
            Err(HomeworkError::NotFoundError(InternalError::new(
                "Attachment not found",
                format!("The attachment {} does not exist.", attachment_id),
                "The attachment does not exist.",
            )))
        } else {
            Ok(())
        }
    }
}

impl TryFrom<&Row<'_>> for Attachment {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Attachment {
            id: row.get(0)?,
            name: row.get(1)?,
            creation_time: row.get(2)?,
        })
    }
}
