use std::{io::Write, path::PathBuf, sync::Arc};

use chrono::{DateTime, Utc};
use log::info;
use zip::write::FileOptions;

use crate::application::config::Configuration;

/// The file extension of backup archives.
const BACKUP_FILE_EXTENSION: &str = "zip";
/// The database file inside the archive.
const ARCHIVE_DATABASE_FILE: &str = "database.sqlite";
/// The attachment folder inside the archive.
const ARCHIVE_ATTACHMENT_FOLDER: &str = "attachments";
/// The intervall in hours that backups are checked.
const BACKUP_UPDATE_INTERVALL: i64 = 3;

#[derive(Debug)]
/// A service that automatically creates backups of the application.
pub struct BackupService {
    configuration: Arc<Configuration>,
    next_timed_backup: Option<DateTime<Utc>>,
}

impl BackupService {
    /// Creates a new `BackupService` with the specified configuration.
    ///
    /// #Parameters
    ///
    /// * `configuration` - the [`Configuration`] to use for creating backups
    pub fn new(configuration: Arc<Configuration>) -> Self {
        BackupService {
            configuration,
            next_timed_backup: None,
        }
    }

    /// Schedules or re-schedules a timed backup.
    pub fn request_timed_backup(&mut self) {
        self.next_timed_backup =
            Some(chrono::Utc::now() + chrono::Duration::hours(BACKUP_UPDATE_INTERVALL));
        info!("New backup scheduled: {:?}", self.next_timed_backup);
    }

    /// Checks if a timed backup is scheduled and executes the backup if necessary.
    /// Deletes old backups afterwards.
    pub fn check_timed_backup(&mut self) -> Result<(), std::io::Error> {
        info!("Performing timed backup check...");
        if let Some(scheduled_backup_time) = self.next_timed_backup {
            if chrono::Utc::now() >= scheduled_backup_time {
                self.next_timed_backup = None;
                self.create_backup()?;
                self.clean_backups()?;
            }
        }
        Ok(())
    }

    /// Creates a backup of the current state.
    pub fn create_backup(&self) -> Result<(), std::io::Error> {
        let attachments_path = self.configuration().application_attachments_folder_path();
        let database_path = Configuration::application_database_file_path();
        let timestamp = chrono::Utc::now().timestamp();
        info!("Creating new backup {}.", timestamp);
        let mut backup_file_path = self.configuration().application_backup_folder_path();
        std::fs::create_dir_all(&backup_file_path)?;
        backup_file_path.push(timestamp.to_string());
        backup_file_path.set_extension(BACKUP_FILE_EXTENSION);
        let mut backup_archive = zip::ZipWriter::new(std::fs::File::create(backup_file_path)?);
        let archive_options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Zstd)
            .compression_level(Some(3));
        backup_archive.start_file(ARCHIVE_DATABASE_FILE, archive_options)?;
        backup_archive.write(&std::fs::read(database_path)?)?;
        backup_archive.add_directory(ARCHIVE_ATTACHMENT_FOLDER, archive_options)?;
        for attachment_entry in std::fs::read_dir(attachments_path)? {
            let attachment_entry = attachment_entry?;
            let attachment_file_name = format!(
                "{}/{}",
                ARCHIVE_ATTACHMENT_FOLDER,
                attachment_entry.file_name().to_string_lossy()
            );
            backup_archive.start_file(attachment_file_name, archive_options)?;
            backup_archive.write(&std::fs::read(attachment_entry.path())?)?;
        }
        backup_archive.finish()?;
        Ok(())
    }

    /// Removes old backup archives.
    pub fn clean_backups(&self) -> Result<(), std::io::Error> {
        let mut backups = self.current_backups()?;
        if backups.len() > self.configuration().maximum_backups() {
            // Backup archives are named by timestamp.
            backups.sort();
            for backup_path in &backups[0..(backups.len() - self.configuration().maximum_backups())]
            {
                info!("Removing old backup archive: {}", backup_path.to_string_lossy());
                std::fs::remove_file(backup_path)?;
            }
        }
        Ok(())
    }

    /// Returns the currently present backup archives.
    pub fn current_backups(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let backup_folder = self.configuration().application_backup_folder_path();
        if !backup_folder.exists() {
            return Ok(Vec::new());
        } else {
            let mut backups = Vec::new();
            for entry in std::fs::read_dir(backup_folder)? {
                let entry_path = entry?.path();
                if entry_path.is_file()
                    && entry_path.extension() == Some(BACKUP_FILE_EXTENSION.as_ref())
                {
                    backups.push(entry_path);
                }
            }
            Ok(backups)
        }
    }

    /// Returns a reference to the underlying [`Configuration`].
    fn configuration(&self) -> Arc<Configuration> {
        Arc::clone(&self.configuration)
    }
}
