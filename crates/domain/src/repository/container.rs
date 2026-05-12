use async_trait::async_trait;
use futures::Stream;

use crate::entities::{
    Container, ContainerConfig, ContainerStats, ExecId, ExecOutput, ExecSession, LogLine,
};
use crate::error::DomainResult;

/// Repository for managing Docker containers.
#[async_trait]
pub trait ContainerRepository: Send + Sync {
    /// List all containers, optionally including stopped ones.
    async fn list_containers(&self, all: bool) -> DomainResult<Vec<Container>>;

    /// Create a new container from the given configuration.
    async fn create_container(&self, config: &ContainerConfig) -> DomainResult<String>;

    /// Start a container by ID.
    async fn start_container(&self, id: &str) -> DomainResult<()>;

    /// Stop a container by ID.
    async fn stop_container(&self, id: &str) -> DomainResult<()>;

    /// Restart a container by ID.
    async fn restart_container(&self, id: &str) -> DomainResult<()>;

    /// Remove a container by ID.
    async fn remove_container(&self, id: &str) -> DomainResult<()>;

    /// Stream logs from a container.
    /// `since` and `until` are Unix timestamps in seconds.
    async fn container_logs(
        &self,
        id: &str,
        tail: Option<u32>,
        follow: bool,
        since: Option<i32>,
        until: Option<i32>,
    ) -> DomainResult<Box<dyn Stream<Item = DomainResult<LogLine>> + Unpin + Send>>;

    /// Create an exec session inside a running container.
    async fn create_exec(&self, id: &str, cmd: &[String]) -> DomainResult<ExecId>;

    /// Start an exec session and return its output stream and writable input.
    /// Returns a tuple of (output_stream, input_writer).
    async fn start_exec_interactive(&self, exec_id: &ExecId) -> DomainResult<ExecSession>;

    /// Start an exec session non-interactively and return output stream.
    async fn start_exec(
        &self,
        exec_id: &ExecId,
    ) -> DomainResult<Box<dyn Stream<Item = DomainResult<ExecOutput>> + Unpin + Send>>;

    /// Resize the TTY of an exec session.
    async fn resize_exec(&self, exec_id: &ExecId, width: u16, height: u16) -> DomainResult<()>;

    /// Inspect detailed information about a container.
    async fn inspect_container(&self, id: &str) -> DomainResult<String>;

    /// Stream resource usage statistics for a container.
    async fn container_stats(
        &self,
        id: &str,
    ) -> DomainResult<Box<dyn Stream<Item = DomainResult<ContainerStats>> + Unpin + Send>>;
}
