pub mod compose;
mod connection;
mod containers;
mod images;
mod networks;
mod volumes;

use bollard::Docker;
use domain::entities::DockerEndpoint;
use domain::{DomainError, DomainResult};
use std::sync::Arc;
use tokio::sync::Mutex;

/// High-level Docker client that wraps `bollard::Docker` and implements all repository traits.
pub struct DockerClient {
    docker: Arc<Mutex<Option<Docker>>>,
    endpoint: DockerEndpoint,
}

impl DockerClient {
    /// Creates a new `DockerClient` but does not connect yet.
    pub fn new(endpoint: DockerEndpoint) -> Self {
        Self {
            docker: Arc::new(Mutex::new(None)),
            endpoint,
        }
    }

    /// Attempts to connect to the configured Docker endpoint.
    pub async fn connect(&self) -> DomainResult<()> {
        let mut guard = self.docker.lock().await;
        let new_docker = connect_to_endpoint(&self.endpoint)?;
        *guard = Some(new_docker);
        Ok(())
    }

    /// Returns the configured endpoint.
    pub fn endpoint(&self) -> &DockerEndpoint {
        &self.endpoint
    }

    /// Convenience: locks and returns the bollard Docker client, or an error if not connected.
    pub(crate) async fn get_docker(&self) -> DomainResult<bollard::Docker> {
        let guard = self.docker.lock().await;
        guard.clone().ok_or_else(|| {
            DomainError::ConnectionFailed("Not connected to Docker daemon".to_string())
        })
    }
}

/// Connect to a Docker endpoint using the bollard library.
fn connect_to_endpoint(endpoint: &DockerEndpoint) -> DomainResult<Docker> {
    let docker = if endpoint.host_url.starts_with("tcp://") {
        if endpoint.tls_ca.is_some() {
            // TLS connection
            return Err(DomainError::Config(
                "TLS connections not yet implemented".to_string(),
            ));
        } else {
            let url = endpoint.host_url.trim_start_matches("tcp://").to_string();
            Docker::connect_with_http(&url, 120, bollard::API_DEFAULT_VERSION).map_err(|e| {
                DomainError::ConnectionFailed(format!("HTTP connection failed: {e}"))
            })?
        }
    } else if endpoint.host_url.starts_with("npipe://") {
        #[cfg(target_os = "windows")]
        {
            let pipe = endpoint.host_url.trim_start_matches("npipe://").to_string();
            Docker::connect_with_named_pipe(&pipe, 120, bollard::API_DEFAULT_VERSION).map_err(
                |e| DomainError::ConnectionFailed(format!("Named pipe connection failed: {e}")),
            )?
        }
        #[cfg(not(target_os = "windows"))]
        {
            return Err(DomainError::Config(
                "Named pipe connections are only supported on Windows".to_string(),
            ));
        }
    } else {
        // Default: Unix socket
        Docker::connect_with_local_defaults()
            .map_err(|e| DomainError::ConnectionFailed(format!("Local connection failed: {e}")))?
    };

    Ok(docker)
}
