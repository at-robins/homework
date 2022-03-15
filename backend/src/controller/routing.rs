use actix_files::{Files, NamedFile};
use actix_web::web::{self, ServiceConfig};

use super::{
    attachment_controller::{
        add_attachment, all_attachments, delete_attachment, download_attachment,
    },
    recipe_controller::{
        add_attachment_to_recipe, add_tag_to_recipe, all_recipe_tags, all_recipes, change_rating,
        change_recipe_string_column, create_recipe, remove_tag_from_recipe, single_recipe, modify_ingredient, add_ingredient_to_recipe,
    },
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

    // Recipe controller routing
    .service(
        web::resource("/api/recipes")
            .route(web::get().to(all_recipes))
            .route(web::post().to(create_recipe))
    )
    .route("/api/recipes/tags", web::get().to(all_recipe_tags))
    .route("/api/recipe/{id}", web::get().to(single_recipe))
    .route("/api/recipe/{id}/string/{string_param}", web::post().to(change_recipe_string_column))
    .route("/api/recipe/{id}/rating", web::post().to(change_rating))
    .route("/api/recipe/{id}/tags", web::post().to(add_tag_to_recipe))
    .route("/api/recipe/{id}/tag/{tag_name}", web::delete().to(remove_tag_from_recipe))
    .route("/api/recipe/{id}/attachments", web::post().to(add_attachment_to_recipe))
    .route("/api/recipe/{id}", web::get().to(single_recipe))
    .service(
        web::resource("/api/recipe/{id}/ingredients")
        .route(web::post().to(add_ingredient_to_recipe))
        .route(web::patch().to(modify_ingredient))
    )
    // Registers static frontend resources. Needs to be last to not overwrite other routes.
    .service(Files::new("/", "./static_dist").show_files_listing());
}
