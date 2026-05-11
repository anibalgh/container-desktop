use async_trait::async_trait;
use futures::Stream;

use crate::entities::Image;
use crate::error::DomainResult;

/// Repository for managing Docker images.
#[async_trait]
pub trait ImageRepository: Send + Sync {
    /// List all images on the Docker daemon.
    async fn list_images(&self) -> DomainResult<Vec<Image>>;

    /// Pull an image from a registry.
    /// Returns a stream of pull progress messages.
    async fn pull_image(
        &self,
        name: &str,
        tag: Option<&str>,
    ) -> DomainResult<Box<dyn Stream<Item = DomainResult<String>> + Unpin + Send>>;

    /// Remove an image by ID.
    async fn remove_image(&self, id: &str) -> DomainResult<()>;

    /// Tag an existing image with a new repository:tag.
    async fn tag_image(&self, id: &str, repo: &str, tag: &str) -> DomainResult<()>;

    /// Inspect detailed information about an image.
    async fn inspect_image(&self, id: &str) -> DomainResult<String>;
}
