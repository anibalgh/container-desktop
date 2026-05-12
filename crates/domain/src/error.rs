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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_failed_display() {
        let err = DomainError::ConnectionFailed("timeout".into());
        assert!(err.to_string().contains("Docker connection failed"));
        assert!(err.to_string().contains("timeout"));
    }

    #[test]
    fn docker_api_display() {
        let err = DomainError::DockerApi("400 Bad Request".into());
        assert!(err.to_string().contains("Docker API error"));
        assert!(err.to_string().contains("400 Bad Request"));
    }

    #[test]
    fn not_found_display() {
        let err = DomainError::NotFound("container abc123".into());
        assert!(err.to_string().contains("Resource not found"));
        assert!(err.to_string().contains("abc123"));
    }

    #[test]
    fn operation_failed_display() {
        let err = DomainError::OperationFailed("cannot stop".into());
        assert!(err.to_string().contains("Operation failed"));
    }

    #[test]
    fn config_display() {
        let err = DomainError::Config("missing field".into());
        assert!(err.to_string().contains("Configuration error"));
        assert!(err.to_string().contains("missing field"));
    }

    #[test]
    fn io_error_from() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let domain_err: DomainError = io_err.into();
        assert!(matches!(domain_err, DomainError::Io(_)));
        assert!(domain_err.to_string().contains("IO error"));
    }

    #[test]
    fn serialization_display() {
        let err = DomainError::Serialization("invalid JSON".into());
        assert!(err.to_string().contains("Serialization error"));
    }

    #[test]
    fn compose_display() {
        let err = DomainError::Compose("yaml parse error".into());
        assert!(err.to_string().contains("Compose error"));
        assert!(err.to_string().contains("yaml parse error"));
    }
}
