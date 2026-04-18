mod config;
mod error;
mod manager;

use config::Config;
use error::Result;
use manager::{DataLoader, FileManager};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting sync-bfu...");
    
    // Load configuration
    let config = Config::from_env()?;
    println!("Configuration loaded successfully");
    
    // Create data directory if it doesn't exist
    tokio::fs::create_dir_all(&config.data_dir).await?;
    println!("Data directory: {:?}", config.data_dir);
    
    // Initialize S3 client and managers
    let s3_client = config.create_s3_client()?;
    let data_loader = DataLoader::new(s3_client);
    let mut file_manager = FileManager::new(data_loader);
    
    println!("sync-bfu initialized successfully");
    println!("Ready to sync files from bucket: {}", config.bucket);
    
    // Example usage (commented out for now)
    // let objects = file_manager.data_loader
    //     .list_objects(&config.bucket, "")
    //     .await?;
    // println!("Found {} objects", objects.len());
    
    Ok(())
}
