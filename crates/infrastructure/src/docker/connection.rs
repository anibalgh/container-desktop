use super::DockerClient;
use async_trait::async_trait;
use domain::entities::DockerInfo;
use domain::repository::DockerConnectionRepository;
use domain::{DomainError, DomainResult};

#[async_trait]
impl DockerConnectionRepository for DockerClient {
    async fn test_connection(&self) -> DomainResult<DockerInfo> {
        let docker = self.get_docker().await?;

        let info = docker
            .info()
            .await
            .map_err(|e| DomainError::ConnectionFailed(format!("Cannot get Docker info: {e}")))?;

        let version = docker.version().await.map_err(|e| {
            DomainError::ConnectionFailed(format!("Cannot get Docker version: {e}"))
        })?;

        Ok(DockerInfo {
            server_version: version.version.unwrap_or_default(),
            containers_running: info.containers_running.unwrap_or(0) as u64,
            containers_paused: info.containers_paused.unwrap_or(0) as u64,
            containers_stopped: info.containers_stopped.unwrap_or(0) as u64,
            images: info.images.unwrap_or(0) as u64,
            os_type: info.operating_system.unwrap_or_default(),
            architecture: info.architecture.unwrap_or_default(),
            endpoint: self.endpoint().host_url,
        })
    }

    fn endpoint_url(&self) -> String {
        self.endpoint().host_url
    }

    async fn ping(&self) -> DomainResult<()> {
        let docker = self.get_docker().await?;
        docker
            .ping()
            .await
            .map(|_| ())
            .map_err(|e| DomainError::ConnectionFailed(format!("Ping failed: {e}")))
    }
}
