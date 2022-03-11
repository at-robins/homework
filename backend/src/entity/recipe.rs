use chrono::{DateTime, Utc};
use rusqlite::Row;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::attachment::Attachment;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    id: Uuid,
    title: String,
    instructions: String,
    reference: String,
    rating: u8,
    tags: Vec<String>,
    attachments: Vec<Attachment>,
    creation_time: DateTime<Utc>,
}

impl Recipe {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    pub fn set_attachments(&mut self, attachments: Vec<Attachment>) {
        self.attachments = attachments;
    }
}


impl TryFrom<&Row<'_>> for Recipe {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Recipe {
            id: row.get(0)?,
            title: row.get(1)?,
            instructions: row.get(2)?,
            reference: row.get(3)?,
            rating: row.get(4)?,
            tags: Vec::new(),
            attachments: Vec::new(),
            creation_time: row.get(5)?,
        })
    }
}
/*pub struct Recipe {
    id: Uuid,
    title: String,
    ingredients: Vec<IngredientEntry>,
    instructions: String,
    reference: String,
    tags: Vec<String>,
    attachments: Vec<Uuid>,
    creation_time: DateTime<Utc>,
}*/

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct IngredientEntry {
    unit: String,
    amount: String,
    ingredient_text: String,
    ingredient_reference: Option<Uuid>,
}
