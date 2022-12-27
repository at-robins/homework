use std::sync::Arc;

use actix_web::{middleware, App, HttpServer};
use application::{config::Configuration, error::HomeworkError};
use controller::routing::routing_config;
use log::error;
use parking_lot::Mutex;
use service::backup_service::BackupService;

#[actix_web::main]
async fn main() -> Result<(), HomeworkError> {
    let app_config = Arc::new(init_config());
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(app_config.log_level().as_str()),
    )
    .init();
    let app_config_internal = Arc::clone(&app_config);
    Configuration::initialise_database()?;
    // Create a backup service and check on a regular basis if any backups need to be performed.
    let backup_service = Arc::new(Mutex::new(BackupService::new(Arc::clone(&app_config))));
    let backup_service_schedule = Arc::clone(&backup_service);
    actix_rt::spawn(async move {
        let mut interval = actix_rt::time::interval(std::time::Duration::from_secs(3600));
        loop {
            interval.tick().await;
            let mut service = backup_service_schedule.lock();
            if let Err(err) = service.check_timed_backup() {
                error!("{}", err);
            }
        }
    });

    Ok(HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Arc::clone(&app_config_internal))
            .app_data(Arc::clone(&backup_service))
            .configure(routing_config)
    })
    .bind(app_config.server_address_and_port())?
    .run()
    .await?)
}

fn init_config() -> Configuration {
    let config = Configuration::load_from_file()
        .map_err(|config_loading_error| {
            eprintln!(
                "Configuration could not be loaded using default: {:?}",
                config_loading_error
            );
            config_loading_error
        })
        .unwrap_or_default();
    if !Configuration::exists() {
        if let Err(config_save_error) = config.save_to_file() {
            eprintln!("Configuration could not be saved: {:?}", config_save_error);
        }
    }
    config
}

mod application;
mod controller;
mod entity;
mod service;
