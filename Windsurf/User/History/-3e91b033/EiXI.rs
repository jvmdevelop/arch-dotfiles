use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Parsing error: {0}")]
    Parsing(String),

    #[error("HuggingFace API error: {0}")]
    HuggingFace(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Timeout error: operation timed out after {0} seconds")]
    Timeout(u64),

    #[error("Empty input error: {0}")]
    EmptyInput(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
