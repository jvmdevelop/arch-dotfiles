mod error;
mod manager;

use error::Result;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting sync-bfu...");
    
    // Create data directory if it doesn't exist
    let data_dir = home::home_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join(".sync-bfu");
    
    fs::create_dir_all(&data_dir).await?;
    
    println!("Data directory: {:?}", data_dir);
    println!("sync-bfu initialized successfully");
    
    Ok(())
}
