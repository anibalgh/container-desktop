use async_trait::async_trait;

use crate::entities::{DockerCleanupSummary, DockerInfo};
use crate::error::DomainResult;

/// Repository for testing and managing Docker daemon connections.
#[async_trait]
pub trait DockerConnectionRepository: Send + Sync {
    /// Test whether the configured endpoint is reachable and return daemon info.
    async fn test_connection(&self) -> DomainResult<DockerInfo>;

    /// Get the current connection endpoint URL.
    fn endpoint_url(&self) -> String;

    /// Verify the daemon is reachable (health check).
    async fn ping(&self) -> DomainResult<()>;

    /// Get reclaimable Docker disk usage available for cleanup.
    async fn cleanup_summary(&self) -> DomainResult<DockerCleanupSummary>;

    /// Run Docker system prune on the configured endpoint.
    async fn system_prune(&self) -> DomainResult<()>;
}
