use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Task not found: {0}")]
    TaskNotFound(u64),

    #[error("Invalid date format: {0}")]
    InvalidDate(String),
}

pub type Result<T> = std::result::Result<T, TodoError>;
