mod config;
mod error;
mod manager;

use config::Config;
use error::Result;
use manager::{DataLoader, FileManager};

#[tokio::main]
async fn main() -> Result<()> {    
    let config = Config::from_env()?;
    
    tokio::fs::create_dir_all(&config.data_dir).await?;
    println!("Data directory: {:?}", config.data_dir);
    
    let s3_client = config.create_s3_client()?;
    let data_loader = DataLoader::new(s3_client);
    let mut file_manager = FileManager::new(data_loader);
    
    println!("sync-bfu initialized successfully");
    println!("Ready to sync files from bucket: {}", config.bucket);
    
    Ok(())
}
