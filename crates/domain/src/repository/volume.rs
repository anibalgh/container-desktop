use async_trait::async_trait;

use crate::entities::Volume;
use crate::error::DomainResult;

/// Repository for managing Docker volumes.
#[async_trait]
pub trait VolumeRepository: Send + Sync {
    /// List all volumes.
    async fn list_volumes(&self) -> DomainResult<Vec<Volume>>;

    /// Create a new volume with the given name.
    async fn create_volume(&self, name: &str) -> DomainResult<Volume>;

    /// Remove a volume by name.
    async fn remove_volume(&self, name: &str) -> DomainResult<()>;

    /// Inspect detailed information about a volume.
    async fn inspect_volume(&self, name: &str) -> DomainResult<String>;
}
