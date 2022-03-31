use std::collections::HashSet;

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    application::{config::Configuration, error::HomeworkError},
    entity::payment::Payment,
};

/// Lists all payments saved in the database.
pub async fn all_payments() -> Result<impl Responder, HomeworkError> {
    let conn = Configuration::database_connection()?;
    Ok(web::Json(Payment::select_all_from_database(&conn)?))
}

pub async fn single_payment(id: web::Path<Uuid>) -> Result<impl Responder, HomeworkError> {
    let uuid = id.into_inner();
    let conn = Configuration::database_connection()?;
    Ok(web::Json(Payment::select_from_database_by_id(uuid, &conn)?))
}

pub async fn create_payment(title: web::Json<String>) -> Result<HttpResponse, HomeworkError> {
    let conn = Configuration::database_connection()?;
    let title = title.into_inner();
    // Generate a new UUID for the payment.
    let uuid = Configuration::generate_uuid();
    Payment::insert_into_database_new_entry(uuid, &title, &conn)?;
    // Return the UUID of the created payment.
    Ok(HttpResponse::Created().body(uuid.to_string()))
}

pub async fn remove_payment(path: web::Path<Uuid>) -> Result<HttpResponse, HomeworkError> {
    let uuid_payment = path.into_inner();
    let conn = Configuration::database_connection()?;
    Payment::delete_from_database_by_id(uuid_payment, &conn)?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn remove_multiple_payments(ids: web::Json<Vec<Uuid>>) -> Result<HttpResponse, HomeworkError> {
    let payment_uuids = ids.into_inner();
    let conn = Configuration::database_connection()?;
    for uuid_payment in payment_uuids {
        Payment::delete_from_database_by_id(uuid_payment, &conn)?;
    }
    Ok(HttpResponse::Ok().finish())
}

pub async fn change_payment_string_column(
    value: web::Json<String>,
    path: web::Path<(Uuid, String)>,
) -> Result<HttpResponse, HomeworkError> {
    let (uuid, column) = path.into_inner();
    let value = value.into_inner();
    let conn = Configuration::database_connection()?;
    Payment::update_in_database_string_column(uuid, &column, &value, &conn)?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn all_payment_tags() -> Result<impl Responder, HomeworkError> {
    let conn = Configuration::database_connection()?;
    let mut stmt = conn.prepare("SELECT tag FROM tag_payment_mapping")?;
    let tag_rows = stmt.query_map([], |row| Ok(row.get(0)?))?;
    let mut tags: HashSet<String> = HashSet::new();
    for tag in tag_rows {
        tags.insert(tag?);
    }
    Ok(web::Json(tags))
}

pub async fn add_tag_to_payment(
    tag: web::Json<String>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, HomeworkError> {
    let uuid = path.into_inner();
    let conn = Configuration::database_connection()?;
    let tag = tag.into_inner();
    Payment::update_in_database_insert_tag(uuid, &tag, &conn)?;
    Ok(HttpResponse::Created().finish())
}

pub async fn remove_tag_from_payment(
    path: web::Path<(Uuid, String)>,
) -> Result<HttpResponse, HomeworkError> {
    let (uuid, tag_name) = path.into_inner();
    let conn = Configuration::database_connection()?;
    Payment::update_in_database_delete_tag(uuid, &tag_name, &conn)?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn add_attachment_to_payment(
    attachment: web::Json<Uuid>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, HomeworkError> {
    let uuid_payment = path.into_inner();
    let uuid_attachment = attachment.into_inner();
    let conn = Configuration::database_connection()?;
    Payment::update_in_database_insert_attachment(uuid_payment, uuid_attachment, &conn)?;
    Ok(HttpResponse::Created().finish())
}
