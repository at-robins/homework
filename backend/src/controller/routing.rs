use actix_files::{Files, NamedFile};
use actix_web::web::{self, ServiceConfig};

use super::{
    attachment_controller::{
        add_attachment, all_attachments, delete_attachment, download_attachment, thumbnail_image_attachment,
    },
    payment_controller::{
        add_attachment_to_payment, add_tag_to_payment, all_payment_tags, all_payments,
        change_payment_string_column, create_payment, remove_payment, remove_tag_from_payment,
        single_payment, remove_multiple_payments,
    },
    recipe_controller::{
        add_attachment_to_recipe, add_ingredient_to_recipe, add_tag_to_recipe, all_recipe_tags,
        all_recipes, change_rating, change_recipe_string_column, create_recipe, modify_ingredient,
        remove_ingredient_from_recipe, remove_recipe, remove_tag_from_recipe, single_recipe,
    }, resources_controller::favicon,
};

async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static_dist/index.html")?)
}

pub fn routing_config(cfg: &mut ServiceConfig) {
    cfg

    // UI redirect
    .route("/", web::get().to(index))
    .route("/ui", web::get().to(index))
    .route("/ui/{rest:.*}", web::get().to(index))

    // Attachment controller routing
    .service(
        web::resource("/api/attachments")
            .route(web::get().to(all_attachments))
            .route(web::post().to(add_attachment)),
    )
    .service(
        web::resource("/api/attachment/{id}")
            .route(web::get().to(download_attachment))
            .route(web::delete().to(delete_attachment)),
    )
    .route("/api/attachment/{id}/{width}", web::get().to(thumbnail_image_attachment))

    // Recipe controller routing
    .service(
        web::resource("/api/recipes")
            .route(web::get().to(all_recipes))
            .route(web::post().to(create_recipe))
    )
    .route("/api/recipes/tags", web::get().to(all_recipe_tags))
    .service(
        web::resource("/api/recipe/{id}")
        .route(web::get().to(single_recipe))
        .route(web::delete().to(remove_recipe))
    )
    .route("/api/recipe/{id}/string/{string_param}", web::post().to(change_recipe_string_column))
    .route("/api/recipe/{id}/rating", web::post().to(change_rating))
    .route("/api/recipe/{id}/tags", web::post().to(add_tag_to_recipe))
    .route("/api/recipe/{id}/tag/{tag_name}", web::delete().to(remove_tag_from_recipe))
    .route("/api/recipe/{id}/attachments", web::post().to(add_attachment_to_recipe))
    .service(
        web::resource("/api/recipe/{id}/ingredients")
        .route(web::post().to(add_ingredient_to_recipe))
        .route(web::patch().to(modify_ingredient))
    )
    .route("/api/recipe/{recipe_id}/ingredient/{ingredient_id}", web::delete().to(remove_ingredient_from_recipe))

    // Payment controller routing
    .service(
        web::resource("/api/payments")
            .route(web::get().to(all_payments))
            .route(web::patch().to(remove_multiple_payments))
            .route(web::post().to(create_payment))
    )
    .route("/api/payment/tags", web::get().to(all_payment_tags))
    .service(
        web::resource("/api/payment/{id}")
        .route(web::get().to(single_payment))
        .route(web::delete().to(remove_payment))
    )
    .route("/api/payment/{id}/string/{string_param}", web::post().to(change_payment_string_column))
    .route("/api/payment/{id}/tags", web::post().to(add_tag_to_payment))
    .route("/api/payment/{id}/tag/{tag_name}", web::delete().to(remove_tag_from_payment))
    .route("/api/payment/{id}/attachments", web::post().to(add_attachment_to_payment))

    // Redirects the favicon route.
    .route("/favicon.ico", web::get().to(favicon))
    // Registers static frontend resources. Needs to be last to not overwrite other routes.
    .service(Files::new("/", "./static_dist").show_files_listing());
}
