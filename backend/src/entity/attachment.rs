use chrono::{DateTime, Utc};
use rusqlite::Row;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    id: Uuid,
    name: String,
    creation_time: DateTime<Utc>,
}

impl Attachment {
    pub fn name(&self) -> &str {
        &self.name
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
