use async_trait::async_trait;
use futures::Stream;

use crate::entities::{ComposeStack, LogLine};
use crate::error::DomainResult;

/// Repository for managing Docker Compose stacks.
#[async_trait]
pub trait ComposeRepository: Send + Sync {
    /// List all discovered Compose stacks.
    async fn list_stacks(&self) -> DomainResult<Vec<ComposeStack>>;

    /// Bring up a Compose stack from a file.
    /// Returns a stream of output lines.
    async fn compose_up(
        &self,
        file_path: &str,
    ) -> DomainResult<Box<dyn Stream<Item = DomainResult<LogLine>> + Unpin + Send>>;

    /// Bring down a Compose stack.
    async fn compose_down(&self, file_path: &str) -> DomainResult<()>;

    /// Stream logs from a Compose stack.
    async fn compose_logs(
        &self,
        file_path: &str,
    ) -> DomainResult<Box<dyn Stream<Item = DomainResult<LogLine>> + Unpin + Send>>;

    /// Show the status of services in a Compose stack.
    async fn compose_ps(&self, file_path: &str) -> DomainResult<Vec<String>>;
}
