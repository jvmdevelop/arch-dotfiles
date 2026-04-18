use std::path::{Path, PathBuf};

use aws_sdk_s3::Client;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::core::error::Result;

const DATA_DIR: &str = ".sync-bfu";

pub struct DataManager {
    s3_client: Client,
}

pub struct FullFile {
    pub file: File,
    pub path: PathBuf,
}

impl DataManager {
    pub fn new(s3_client: Client) -> Self {
        Self { s3_client }
    }

    pub async fn list_objects(&self, bucket: &str, prefix: &str) -> Result<Vec<String>> {
        let response = self
            .s3_client
            .list_objects_v2()
            .bucket(bucket)
            .prefix(prefix)
            .send()
            .await?;

        let objects = response
            .contents()
            .unwrap_or_default()
            .iter()
            .filter_map(|obj| obj.key().map(|key| key.to_string()))
            .collect();

        Ok(objects)
    }

    pub async fn download_file(
        &self,
        bucket: &str,
        key: &str,
        base_path: &Path,
    ) -> Result<FullFile> {
        let file_path = base_path.join(key);

        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let response = self
            .s3_client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;
        
        let bytes = response.body.collect().await?.into_bytes();

        let mut file = File::create(&file_path).await?;
        file.write_all(&bytes).await?;
        file.flush().await?;

        Ok(FullFile {
            file,
            path: file_path,
        })
    }

    pub fn get_data_dir() -> PathBuf {
        home::home_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join(DATA_DIR)
    }
}
