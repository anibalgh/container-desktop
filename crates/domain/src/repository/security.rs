use async_trait::async_trait;

use crate::entities::{ImageSecurityReport, SecurityOverview, SecurityTool};
use crate::error::DomainResult;

/// Repository for reading consolidated security results.
#[async_trait]
pub trait SecurityRepository: Send + Sync {
    /// Builds the current security overview for all Docker images.
    async fn security_overview(
        &self,
        selected_tools: &[SecurityTool],
    ) -> DomainResult<SecurityOverview>;

    /// Loads the detailed security report for one image.
    async fn image_security_report(&self, image_id: &str) -> DomainResult<ImageSecurityReport>;
}
