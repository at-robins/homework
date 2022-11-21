use std::collections::HashMap;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::error::{HomeworkError, InternalError};

use super::attachment::Attachment;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    id: Uuid,
    target: String,
    note: String,
    paid: HashMap<String, BigDecimal>,
    involved: HashMap<String, BigDecimal>,
    payment_type: PaymentType,
    tags: Vec<String>,
    attachments: Vec<Attachment>,
    creation_time: DateTime<Utc>,
}

impl Payment {
    pub fn select_from_database_by_id(
        payment_id: Uuid,
        connection: &Connection,
    ) -> Result<Payment, HomeworkError> {
        if !Payment::exists_in_database_by_id(payment_id, &connection)? {
            return Err(HomeworkError::NotFoundError(InternalError::new(
                "Payment not found",
                format!("The payment {} does not exist.", payment_id),
                "The payment does not exist.",
            )));
        }
        connection
            .query_row_and_then("SELECT id, target, note, paid, involved, payment_type, creation_time FROM payment WHERE id = ?1",
            [payment_id],
            |row| Payment::try_from((row, connection)))
    }

    pub fn select_all_from_database(
        connection: &Connection,
    ) -> Result<Vec<Payment>, HomeworkError> {
        let mut stmt_payment = connection.prepare(
            "SELECT id, target, note, paid, involved, payment_type, creation_time FROM payment",
        )?;
        let payment_query =
            stmt_payment.query_and_then([], |row| Payment::try_from((row, connection)))?;
        let mut payments = Vec::new();
        for payment in payment_query {
            payments.push(payment?);
        }
        Ok(payments)
    }

    pub fn insert_into_database_new_entry(
        id: Uuid,
        target: &str,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        let paid_empty: HashMap<String, BigDecimal> = HashMap::new();
        let involved_empty: HashMap<String, BigDecimal> = HashMap::new();
        connection.execute(
            "INSERT INTO payment (id, target, note, paid, involved, payment_type, creation_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                id,
                target,
                "",
                serde_json::to_value(&paid_empty)?,
                serde_json::to_value(&involved_empty)?,
                serde_json::to_value(&PaymentType::OneOff{ start: chrono::Utc::now() })?,
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
        if !Payment::exists_in_database_by_id(id, connection)? {
            return Err(HomeworkError::NotFoundError(InternalError::new(
                "Payment not found",
                format!("The payment {} does not exist.", id),
                "The payment does not exist.",
            )));
        }

        let column = StringColumns::try_from(column)?;
        connection.execute(
            &format!("UPDATE payment SET {} = ?1 WHERE id = ?2", column),
            params![value, id],
        )?;
        Ok(())
    }

    pub fn update_in_database_insert_tag(
        id: Uuid,
        tag: &str,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        if !Payment::exists_in_database_by_id(id, connection)? {
            return Err(HomeworkError::NotFoundError(InternalError::new(
                "Payment not found",
                format!("The payment {} does not exist.", id),
                "The payment does not exist.",
            )));
        }

        connection.execute(
            "INSERT INTO tag_payment_mapping (tag, payment_id) VALUES (?1, ?2)",
            params![tag.trim(), id],
        )?;
        Ok(())
    }

    pub fn update_in_database_delete_tag(
        id: Uuid,
        tag: &str,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        if !Payment::exists_in_database_by_id(id, connection)? {
            return Err(HomeworkError::NotFoundError(InternalError::new(
                "Payment not found",
                format!("The payment {} does not exist.", id),
                "The payment does not exist.",
            )));
        }

        connection.execute(
            "DELETE FROM tag_payment_mapping WHERE tag = ?1 AND payment_id = ?2",
            params![tag.trim(), id],
        )?;
        Ok(())
    }

    pub fn update_in_database_insert_attachment(
        payment_id: Uuid,
        attachment_id: Uuid,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        if !Payment::exists_in_database_by_id(payment_id, connection)? {
            return Err(HomeworkError::NotFoundError(InternalError::new(
                "Payment not found",
                format!("The payment {} does not exist.", payment_id),
                "The payment does not exist.",
            )));
        }

        if !Attachment::exists_in_database_by_id(attachment_id, connection)? {
            return Err(HomeworkError::NotFoundError(InternalError::new(
                "Attachment not found",
                format!("The payment {} does not exist.", attachment_id),
                "The attachment does not exist.",
            )));
        }

        connection.execute(
            "INSERT INTO attachment_payment_mapping (payment_id, attachment_id) VALUES (?1, ?2)",
            params![payment_id, attachment_id],
        )?;
        Ok(())
    }

    pub fn delete_from_database_by_id(
        id: Uuid,
        connection: &Connection,
    ) -> Result<(), HomeworkError> {
        if !Payment::exists_in_database_by_id(id, connection)? {
            return Err(HomeworkError::NotFoundError(InternalError::new(
                "Payment not found",
                format!("The payment {} does not exist.", id),
                "The payment does not exist.",
            )));
        }

        connection.execute("DELETE FROM payment WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn tags_by_id(
        payment_id: Uuid,
        connection: &Connection,
    ) -> Result<Vec<String>, rusqlite::Error> {
        let mut tag_stmt =
            connection.prepare("SELECT tag FROM tag_payment_mapping WHERE payment_id = ?1")?;
        let tag_rows = tag_stmt.query_map([payment_id], |row| Ok(row.get(0)?))?;
        let mut tags = Vec::new();
        for tag in tag_rows {
            tags.push(tag?);
        }
        Ok(tags)
    }

    pub fn attachments_by_id(
        payment_id: Uuid,
        connection: &Connection,
    ) -> Result<Vec<Attachment>, rusqlite::Error> {
        let mut attachment_stmt = connection.prepare(
            "
                SELECT attachment.id, attachment.name, attachment.creation_time 
                FROM attachment 
                INNER JOIN attachment_payment_mapping 
                    ON attachment.id = attachment_payment_mapping.attachment_id      
                WHERE attachment_payment_mapping.payment_id = ?1",
        )?;
        let attachment_rows =
            attachment_stmt.query_map([payment_id], |row| Ok(Attachment::try_from(row)?))?;

        let mut attachments = Vec::new();
        for attachment in attachment_rows {
            attachments.push(attachment?);
        }
        Ok(attachments)
    }

    pub fn exists_in_database_by_id(
        payment_id: Uuid,
        connection: &Connection,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = connection.prepare("SELECT 1 FROM payment WHERE id = ?1")?;
        stmt.exists([payment_id])
    }
}

impl TryFrom<(&Row<'_>, &Connection)> for Payment {
    type Error = HomeworkError;

    fn try_from(data: (&Row, &Connection)) -> Result<Self, Self::Error> {
        let (row, connection) = data;
        let id = row.get(0)?;
        let paid = serde_json::from_value(row.get(3)?)?;
        let involved = serde_json::from_value(row.get(4)?)?;
        let payment_type = serde_json::from_value(row.get(5)?)?;
        Ok(Payment {
            id,
            target: row.get(1)?,
            note: row.get(2)?,
            paid,
            involved,
            payment_type,
            tags: Payment::tags_by_id(id, connection)?,
            attachments: Payment::attachments_by_id(id, connection)?,
            creation_time: row.get(6)?,
        })
    }
}

const COLUMN_STRING_TARGET: &str = "target";
const COLUMN_STRING_NOTE: &str = "note";

enum StringColumns {
    Target,
    Note,
}

impl std::fmt::Display for StringColumns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StringColumns::Target => COLUMN_STRING_TARGET,
                StringColumns::Note => COLUMN_STRING_NOTE,
            }
        )
    }
}

impl TryFrom<&str> for StringColumns {
    type Error = HomeworkError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            COLUMN_STRING_TARGET => Ok(StringColumns::Target),
            COLUMN_STRING_NOTE => Ok(StringColumns::Note),
            _ => Err(HomeworkError::NotFoundError(InternalError::new(
                "Invalid value",
                format!("\"{}\" is not a valid payment column value.", value),
                "An invalid value was supplied.",
            ))),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PaymentType {
    OneOff {
        start: DateTime<Utc>,
    },
    Daily {
        distance: u64,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
    Weekly {
        distance: u64,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
    Monthly {
        distance: u64,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
    Annualy {
        distance: u64,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
}
