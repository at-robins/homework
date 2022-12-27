use std::sync::Arc;

use actix_web::HttpRequest;
use parking_lot::Mutex;

use crate::application::config::Configuration;

use super::backup_service::BackupService;


    /// Extracts the [`BackupService`] from a request.
    /// 
    /// # Parameters
    /// 
    /// * `request` - the HTTP request to extract the backup service from
    ///
    /// # Panics
    /// 
    /// If the backup service was not defined in the app configuration.
    pub fn backup_service_from_request(request: &HttpRequest) -> Arc<Mutex<BackupService>> {
        Arc::clone(
            request
                .app_data::<Arc<Mutex<BackupService>>>()
                .expect("The backup service must be accessible."),
        )
    }

    /// Extracts the app's [`Configuration`] from a request.
    /// 
    /// # Parameters
    /// 
    /// * `request` - the HTTP request to extract the configuration from
    ///
    /// # Panics
    /// 
    /// If the configuration was not defined in the app configuration.
    pub fn configuration_from_request(request: &HttpRequest) -> Arc<Configuration> {
        Arc::clone(
            request
                .app_data::<Arc<Configuration>>()
                .expect("The backup service must be accessible."),
        )
    }