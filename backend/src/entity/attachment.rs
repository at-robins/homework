use chrono::{DateTime, Utc};
use rusqlite::{Row, Connection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    id: Uuid,
    name: String,
    creation_time: DateTime<Utc>,
}

impl Attachment {
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
