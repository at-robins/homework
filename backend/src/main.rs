use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpServer};
use application::{config::Configuration, error::HomeworkError};
use controller::{
    attachment_controller::{attachment_routing, attachments_routing},
    recipe_controller::{
        recipe_rating_routing, recipe_routing, recipe_string_routing, recipes_routing, recipe_tags_routing, recipe_tag_delete_routing, recipes_tags_routing, recipe_attachments_routing,
    },
};

async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static_dist/index.html")?)
}

#[actix_web::main]
async fn main() -> Result<(), HomeworkError> {
    let app_config = init_config();
    Configuration::initialise_database()?;
    Ok(HttpServer::new(|| {
        App::new()
            // Registers single page app routes.
            .route("/", web::get().to(index))
            .route("/ui", web::get().to(index))
            .route("/ui/{rest:.*}", web::get().to(index))
            .service(attachments_routing())
            .service(attachment_routing())
            .service(recipes_routing())
            .service(recipe_routing())
            .service(recipe_string_routing())
            .service(recipe_rating_routing())
            .service(recipe_tags_routing())
            .service(recipe_tag_delete_routing())
            .service(recipes_tags_routing())
            .service(recipe_attachments_routing())
            // Registers static frontend resources. Needs to be last to not overwrite other routes.
            .service(Files::new("/", "./static_dist").show_files_listing())
    })
    .bind(app_config.server_address_and_port())?
    .run()
    .await?)
}

fn init_config() -> Configuration {
    let config = Configuration::load_from_file()
        .map_err(|config_loading_error| {
            println!("Configuration could not be loaded using default: {:?}", config_loading_error);
            config_loading_error
        })
        .unwrap_or_default();
    if !Configuration::exists() {
        if let Err(config_save_error) = config.save_to_file() {
            println!("Configuration could not be saved: {:?}", config_save_error);
        }
    }
    config
}

mod application;
mod controller;
mod entity;
