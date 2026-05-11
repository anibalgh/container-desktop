use async_trait::async_trait;

use crate::entities::Network;
use crate::error::DomainResult;

/// Repository for managing Docker networks.
#[async_trait]
pub trait NetworkRepository: Send + Sync {
    /// List all networks.
    async fn list_networks(&self) -> DomainResult<Vec<Network>>;

    /// Create a new network with the given name and optional driver.
    async fn create_network(&self, name: &str, driver: Option<&str>) -> DomainResult<String>;

    /// Remove a network by ID.
    async fn remove_network(&self, id: &str) -> DomainResult<()>;

    /// Inspect detailed information about a network.
    async fn inspect_network(&self, id: &str) -> DomainResult<String>;
}
