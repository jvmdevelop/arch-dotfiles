use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("S3 operation failed: {0}")]
    S3Error(#[from] s3::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("File path conversion error: {0}")]
    PathError(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, SyncError>;
