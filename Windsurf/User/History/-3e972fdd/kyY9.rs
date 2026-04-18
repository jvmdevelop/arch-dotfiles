use std::path::PathBuf;

use crate::error::{Result, SyncError};

#[derive(Debug, Clone)]
pub struct Config {
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub endpoint: Option<String>,
    pub data_dir: PathBuf,
}

impl Config {
    pub fn new(
        bucket: String,
        region: String,
        access_key: String,
        secret_key: String,
        endpoint: Option<String>,
    ) -> Self {
        Self {
            bucket,
            region,
            access_key,
            secret_key,
            endpoint,
            data_dir: crate::manager::DataLoader::get_data_dir(),
        }
    }

    pub fn from_env() -> Result<Self> {
        let bucket = std::env::var("SYNC_BFU_BUCKET")
            .map_err(|_| SyncError::ConfigError("SYNC_BFU_BUCKET not set".to_string()))?;
        
        let region = std::env::var("SYNC_BFU_REGION")
            .map_err(|_| SyncError::ConfigError("SYNC_BFU_REGION not set".to_string()))?;
        
        let access_key = std::env::var("SYNC_BFU_ACCESS_KEY")
            .map_err(|_| SyncError::ConfigError("SYNC_BFU_ACCESS_KEY not set".to_string()))?;
        
        let secret_key = std::env::var("SYNC_BFU_SECRET_KEY")
            .map_err(|_| SyncError::ConfigError("SYNC_BFU_SECRET_KEY not set".to_string()))?;
        
        let endpoint = std::env::var("SYNC_BFU_ENDPOINT").ok();

        Ok(Self::new(bucket, region, access_key, secret_key, endpoint))
    }

    pub fn create_s3_client(&self) -> Result<s3::Client> {
        let endpoint = self.endpoint.as_deref().unwrap_or("https://s3.amazonaws.com");
        
        let client = s3::Client::builder(endpoint)?
            .access_key(&self.access_key)
            .secret_key(&self.secret_key)
            .region(&self.region)
            .build()?;

        Ok(client)
    }
}
