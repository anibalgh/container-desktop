/// Domain-level error types shared across the application.
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Docker connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Docker API error: {0}")]
    DockerApi(String),
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Operation failed: {0}")]
    OperationFailed(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Compose error: {0}")]
    Compose(String),
}

/// Convenience result type using `DomainError`.
pub type DomainResult<T> = Result<T, DomainError>;
