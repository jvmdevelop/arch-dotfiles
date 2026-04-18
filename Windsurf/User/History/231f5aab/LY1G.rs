use std::path::{Path, PathBuf};

use bytes::Bytes;
use s3::Client;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::error::{Result, SyncError};

const DATA_DIR: &str = ".sync-bfu";

pub struct DataLoader {
    s3_client: Client,
}

pub struct FullFile {
    pub file: File,
    pub path: PathBuf,
}

impl DataLoader {
    pub fn new(s3_client: Client) -> Self {
        Self { s3_client }
    }

    /// List objects in S3 bucket with given prefix
    pub async fn list_objects(
        &self,
        bucket: &str,
        prefix: &str,
    ) -> Result<Vec<String>> {
        let response = self
            .s3_client
            .objects()
            .list_v2(bucket)
            .prefix(prefix)
            .send()
            .await?;

        Ok(response.common_prefixes)
    }

    /// Download file content from S3 and save to local filesystem
    pub async fn download_file(
        &self,
        bucket: &str,
        key: &str,
        base_path: &Path,
    ) -> Result<FullFile> {
        let file_path = base_path.join(key);
        
        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let response = self.s3_client.objects().get(bucket, key).send().await?;
        let bytes = response.bytes().await?;

        let mut file = File::create(&file_path).await?;
        file.write_all(&bytes).await?;
        file.flush().await?;

        Ok(FullFile {
            file,
            path: file_path,
        })
    }

    /// Get the default data directory path
    pub fn get_data_dir() -> PathBuf {
        home::home_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join(DATA_DIR)
    }
}
