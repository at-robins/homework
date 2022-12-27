//! The `config` module provides configuration setting for application parameters.

/// The folder in which all information relevant for application configuration is stored.
const DEFAULT_FOLDER_APPLICATION: &str = "application";
/// The folder in which all attachments are stored.
const DEFAULT_FOLDER_APPLICATION_ATTACHMENTS: &str = "attachments";
/// The folder in which all thumbnails are stored.
const DEFAULT_FOLDER_APPLICATION_THUMBNAILS: &str = "thumbnails";
/// The name of the default configuration file.
const DEFAULT_FILE_APPLICATION_CONFIGURATION: &str = "configuration";
/// The name of the default backup.
const DEFAULT_FOLDER_APPLICATION_BACKUP: &str = "backups";
/// The maximum number of backups stored.
const DEFAULT_MAXIMUM_STORED_BACKUPS: usize = 3;
/// The default log level.
const DEFAULT_LOG_LEVEL: log::Level = log::Level::Warn;
/// The name of the default database file.
const DEFAULT_FILE_APPLICATION_DATABASE: &str = "database";
/// The file extension of a database.
const EXTENSION_DATABASE: &str = "sqlite";
/// The file extension of a JSON.
const EXTENSION_JSON: &str = "json";
/// The default server address.
const DEFAULT_CONFIG_SERVER_ADDRESS: &str = "127.0.0.1";
/// The default server port.
const DEFAULT_CONFIG_SERVER_PORT: &str = "8080";
/// The context for UUID generation.
const UUID_CONTEXT: Context = Context::new(0);
/// The node ID for UUID generation.
const UUID_NODE_ID: &[u8; 6] = &[12, 21, 33, 4, 35, 116];
/// The available thumbnail widths.
const THUMBNAIL_WIDTHS: &[u32; 7] = &[0, 100, 200, 400, 600, 800, 1000];

use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::SystemTime,
};

use actix_web::HttpRequest;
use parking_lot::Mutex;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use uuid::{
    v1::{Context, Timestamp},
    Uuid,
};

use crate::service::backup_service::BackupService;

use super::error::HomeworkError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// A configuration that defines basic parameters of the application.
pub struct Configuration {
    server_address: Option<String>,
    server_port: Option<String>,
    attachment_path: Option<String>,
    thumbnail_path: Option<String>,
    log_level: Option<String>,
    backup_path: Option<String>,
    maximum_backups: Option<usize>,
}

impl Configuration {
    /// Tries to load a previously stored configuration from the default location.
    pub fn load_from_file() -> Result<Self, HomeworkError> {
        let file = File::open(Configuration::application_configuration_file_path())?;
        Ok(serde_json::from_reader(file)?)
    }

    /// Tries to save the current configuration to the default file location.
    pub fn save_to_file(&self) -> Result<(), HomeworkError> {
        let path = Configuration::application_configuration_file_path();
        if let Some(parent_path) = path.parent() {
            std::fs::create_dir_all(parent_path)?;
        }
        let file = OpenOptions::new()
            .create(true)
            .append(false)
            .write(true)
            .open(path)?;
        Ok(serde_json::to_writer_pretty(file, self)?)
    }

    /// Opens a database connection if possible.
    pub fn database_connection() -> Result<Connection, HomeworkError> {
        let connection = Connection::open(Configuration::application_database_file_path())?;
        connection.execute("PRAGMA foreign_keys = ON;", [])?;
        Ok(connection)
    }

    pub fn initialise_database() -> Result<(), HomeworkError> {
        let connection = Configuration::database_connection()?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS attachment (
                id              TEXT PRIMARY KEY,
                name            TEXT NOT NULL,
                creation_time   TEXT NOT NULL
                    )",
            [],
        )?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS recipe (
                id              BLOB PRIMARY KEY,
                title           TEXT NOT NULL,
                instructions    TEXT NOT NULL,                    
                reference       TEXT NOT NULL,
                rating          RATING NOT NULL,
                creation_time   TEXT NOT NULL,
                thumbnail       BLOB,
                FOREIGN KEY (thumbnail) REFERENCES attachment (id) 
                        ON UPDATE CASCADE
                        ON DELETE SET NULL
            )",
            [],
        )?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS ingredient (
                    id                              BLOB PRIMARY KEY,
                    amount                          TEXT NOT NULL,
                    unit                            TEXT NOT NULL,                    
                    text                            TEXT NOT NULL,
                    creation_time                   TEXT NOT NULL,
                    recipe_reference                BLOB,
                    recipe_id                       BLOB NOT NULL,
                    ordering                        INTEGER NOT NULL,
                    filter_text                     TEXT,
                    FOREIGN KEY (recipe_id) REFERENCES recipe (id) 
                        ON UPDATE CASCADE
                        ON DELETE CASCADE,
                    FOREIGN KEY (recipe_reference) REFERENCES recipe (id) 
                        ON UPDATE CASCADE
                        ON DELETE SET NULL
                      )",
            [],
        )?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS payment (
                    id                              BLOB PRIMARY KEY,
                    target                          TEXT NOT NULL,
                    note                            TEXT NOT NULL,
                    paid                            TEXT NOT NULL,                    
                    involved                        TEXT NOT NULL,
                    payment_type                    TEXT NOT NULL,
                    creation_time                   TEXT NOT NULL
                      )",
            [],
        )?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS attachment_recipe_mapping (
                    id                          INTEGER PRIMARY KEY,
                    recipe_id                   BLOB NOT NULL,
                    attachment_id               BLOB NOT NULL,
                    FOREIGN KEY (recipe_id)     REFERENCES recipe (id) 
                        ON UPDATE CASCADE
                        ON DELETE CASCADE,
                    FOREIGN KEY (attachment_id) REFERENCES attachment (id) 
                        ON UPDATE CASCADE
                        ON DELETE CASCADE
                      )",
            [],
        )?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS tag_recipe_mapping (
                    id                      INTEGER PRIMARY KEY,
                    tag                     TEXT NOT NULL,
                    recipe_id               BLOB NOT NULL,
                    FOREIGN KEY (recipe_id) REFERENCES recipe (id) 
                        ON UPDATE CASCADE
                        ON DELETE CASCADE
                      )",
            [],
        )?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS attachment_payment_mapping (
                    id                          INTEGER PRIMARY KEY,
                    payment_id                  BLOB NOT NULL,
                    attachment_id               BLOB NOT NULL,
                    FOREIGN KEY (payment_id)       REFERENCES payment (id) 
                        ON UPDATE CASCADE
                        ON DELETE CASCADE,
                    FOREIGN KEY (attachment_id) REFERENCES attachment (id) 
                        ON UPDATE CASCADE
                        ON DELETE CASCADE
                      )",
            [],
        )?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS tag_payment_mapping (
                    id                          INTEGER PRIMARY KEY,
                    tag                         TEXT NOT NULL,
                    payment_id                  BLOB NOT NULL,
                    FOREIGN KEY (payment_id)    REFERENCES payment (id) 
                        ON UPDATE CASCADE
                        ON DELETE CASCADE
                      )",
            [],
        )?;
        Ok(())
    }

    /// Checks if the configuration exists as a physical file.
    pub fn exists() -> bool {
        Configuration::application_configuration_file_path().exists()
    }

    /// Generates a V1 UUID.
    pub fn generate_uuid() -> Uuid {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("The UNIX epoch must be the earliest possible time point.");
        let timestamp = Timestamp::from_unix(UUID_CONTEXT, now.as_secs(), now.subsec_nanos());
        Uuid::new_v1(timestamp, UUID_NODE_ID)
    }

    /// The path to the application configuration folder.
    pub fn application_configuration_folder_path() -> PathBuf {
        PathBuf::from(DEFAULT_FOLDER_APPLICATION)
    }

    /// The path to the application configuration file.
    pub fn application_configuration_file_path() -> PathBuf {
        let mut path = Configuration::application_configuration_folder_path();
        path.push(DEFAULT_FILE_APPLICATION_CONFIGURATION);
        path.set_extension(EXTENSION_JSON);
        path
    }

    /// The path to the application database file.
    pub fn application_database_file_path() -> PathBuf {
        let mut path = Configuration::application_configuration_folder_path();
        path.push(DEFAULT_FILE_APPLICATION_DATABASE);
        path.set_extension(EXTENSION_DATABASE);
        path
    }

    /// Extracts the backup service from a request.
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

    /// The path to the attachments folder.
    pub fn application_attachments_folder_path(&self) -> PathBuf {
        if let Some(configured_path_string) = self.attachment_path.clone() {
            if let Ok(configured_path) = PathBuf::try_from(configured_path_string) {
                return configured_path;
            } else {
                log::warn!("{:?} is not a valid path.", self.attachment_path);
            }
        }
        let mut default_path = Configuration::application_configuration_folder_path();
        default_path.push(DEFAULT_FOLDER_APPLICATION_ATTACHMENTS);
        default_path
    }

    /// The path to the thumbnail folder.
    pub fn application_thumbnail_folder_path(&self) -> PathBuf {
        if let Some(configured_path_string) = self.thumbnail_path.clone() {
            if let Ok(configured_path) = PathBuf::try_from(configured_path_string) {
                return configured_path;
            } else {
                log::warn!("{:?} is not a valid path.", self.thumbnail_path);
            }
        }
        let mut default_path = Configuration::application_configuration_folder_path();
        default_path.push(DEFAULT_FOLDER_APPLICATION_THUMBNAILS);
        default_path
    }

    /// The available thumbnail widths.
    pub fn thumbnail_widths() -> &'static [u32; 7] {
        THUMBNAIL_WIDTHS
    }

    /// Returns the address of the server.
    pub fn server_address(&self) -> String {
        self.server_address
            .clone()
            .unwrap_or(DEFAULT_CONFIG_SERVER_ADDRESS.to_string())
    }

    /// Returns the port of the server.
    pub fn server_port(&self) -> String {
        self.server_port
            .clone()
            .unwrap_or(DEFAULT_CONFIG_SERVER_PORT.to_string())
    }

    /// Returns the full server address including port information.
    pub fn server_address_and_port(&self) -> String {
        format!("{}:{}", self.server_address(), self.server_port())
    }

    /// Returns the logging level of the server.
    pub fn log_level(&self) -> log::Level {
        self.log_level
            .clone()
            .and_then(|log_level_string| {
                log::Level::from_str(&log_level_string)
                    .map_err(|err| {
                        eprintln!(
                            "Parsing log level {} failed with error: {}",
                            log_level_string, err
                        )
                    })
                    .ok()
            })
            .unwrap_or(DEFAULT_LOG_LEVEL)
    }

    /// Returns the maximum number of backups.
    pub fn maximum_backups(&self) -> usize {
        self.maximum_backups
            .clone()
            .unwrap_or(DEFAULT_MAXIMUM_STORED_BACKUPS)
    }

    /// The path to the backup folder.
    pub fn application_backup_folder_path(&self) -> PathBuf {
        if let Some(configured_path_string) = self.backup_path.clone() {
            if let Ok(configured_path) = PathBuf::try_from(configured_path_string) {
                return configured_path;
            } else {
                log::warn!("{:?} is not a valid path.", self.backup_path);
            }
        }
        let mut default_path = Configuration::application_configuration_folder_path();
        default_path.push(DEFAULT_FOLDER_APPLICATION_BACKUP);
        default_path
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            server_address: None,
            server_port: None,
            attachment_path: None,
            thumbnail_path: None,
            log_level: None,
            backup_path: None,
            maximum_backups: None,
        }
    }
}

#[cfg(test)]
mod test;
