use std::{collections::HashMap, path::PathBuf};

use tokio::fs;

use crate::error::{Result, SyncError};
use crate::manager::data_manager::DataLoader;

pub struct FileManager {
    file_map: HashMap<String, PathBuf>, // file_name -> file_path
    data_loader: DataLoader,
}

impl FileManager {
    pub fn new(data_loader: DataLoader) -> Self {
        FileManager {
            file_map: HashMap::new(),
            data_loader,
        }
    }

    /// Add a file to the managed files
    pub fn add_file(&mut self, file_name: String, path_to_file: PathBuf) {
        self.file_map.insert(file_name, path_to_file);
    }

    /// Remove a file from management
    pub fn remove_file(&mut self, file_name: &str) -> Option<PathBuf> {
        self.file_map.remove(file_name)
    }

    /// Get the file path for a managed file
    pub fn get_file_path(&self, file_name: &str) -> Option<&PathBuf> {
        self.file_map.get(file_name)
    }

    /// Check if a file is being managed
    pub fn is_file_managed(&self, file_name: &str) -> bool {
        self.file_map.contains_key(file_name)
    }

    /// Load file content from S3 and add to management
    pub async fn load_file_from_s3(
        &mut self,
        file_name: &str,
        bucket: &str,
        base_path: &std::path::Path,
    ) -> Result<()> {
        let content = self
            .data_loader
            .download_file(bucket, file_name, base_path)
            .await?;
        
        self.add_file(file_name.to_string(), content.path);
        Ok(())
    }

    /// Get all managed files
    pub fn get_managed_files(&self) -> impl Iterator<Item = (&String, &PathBuf)> {
        self.file_map.iter()
    }

    /// Check if a managed file exists on disk
    pub async fn file_exists_on_disk(&self, file_name: &str) -> Result<bool> {
        if let Some(path) = self.get_file_path(file_name) {
            Ok(fs::metadata(path).await.is_ok())
        } else {
            Ok(false)
        }
    }
}
