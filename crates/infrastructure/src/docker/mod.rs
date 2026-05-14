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
use std::sync::RwLock;
use tokio::sync::Mutex;

/// High-level Docker client that wraps `bollard::Docker` and implements all repository traits.
pub struct DockerClient {
    docker: Arc<Mutex<Option<Docker>>>,
    endpoint: Arc<RwLock<DockerEndpoint>>,
}

impl DockerClient {
    /// Creates a new `DockerClient` but does not connect yet.
    pub fn new(endpoint: DockerEndpoint) -> Self {
        Self {
            docker: Arc::new(Mutex::new(None)),
            endpoint: Arc::new(RwLock::new(endpoint)),
        }
    }

    /// Attempts to connect to the configured Docker endpoint.
    pub async fn connect(&self) -> DomainResult<()> {
        let endpoint = self.endpoint();
        let new_docker = connect_to_endpoint(&endpoint)?;
        let mut guard = self.docker.lock().await;
        *guard = Some(new_docker);
        Ok(())
    }

    /// Atomically attempts to swap the active endpoint and connected client.
    ///
    /// The new endpoint is only persisted in memory if the connection succeeds.
    pub async fn reconfigure(&self, endpoint: DockerEndpoint) -> DomainResult<()> {
        let new_docker = connect_to_endpoint(&endpoint)?;
        {
            let mut guard = self.docker.lock().await;
            *guard = Some(new_docker);
        }
        *self
            .endpoint
            .write()
            .expect("docker endpoint lock poisoned") = endpoint;
        Ok(())
    }

    /// Returns the configured endpoint.
    pub fn endpoint(&self) -> DockerEndpoint {
        self.endpoint
            .read()
            .expect("docker endpoint lock poisoned")
            .clone()
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
    let timeout_secs = endpoint.timeout_secs.max(1);
    let docker = if endpoint.host_url.starts_with("tcp://") {
        if endpoint.tls_ca.is_some() {
            // TLS connection
            return Err(DomainError::Config(
                "TLS connections not yet implemented".to_string(),
            ));
        } else {
            Docker::connect_with_http(
                &endpoint.host_url,
                timeout_secs,
                bollard::API_DEFAULT_VERSION,
            )
            .map_err(|e| DomainError::ConnectionFailed(format!("HTTP connection failed: {e}")))?
        }
    } else if endpoint.host_url.starts_with("npipe://") {
        #[cfg(target_os = "windows")]
        {
            Docker::connect_with_named_pipe(
                &endpoint.host_url,
                timeout_secs,
                bollard::API_DEFAULT_VERSION,
            )
            .map_err(|e| {
                DomainError::ConnectionFailed(format!("Named pipe connection failed: {e}"))
            })?
        }
        #[cfg(not(target_os = "windows"))]
        {
            return Err(DomainError::Config(
                "Named pipe connections are only supported on Windows".to_string(),
            ));
        }
    } else {
        connect_local_socket(&endpoint.host_url, timeout_secs)?
    };

    Ok(docker)
}

#[cfg(target_os = "windows")]
fn connect_local_socket(endpoint_url: &str, _timeout_secs: u64) -> DomainResult<Docker> {
    Err(DomainError::Config(format!(
        "Unix socket connections are not supported on Windows: {endpoint_url}"
    )))
}

#[cfg(not(target_os = "windows"))]
fn connect_local_socket(endpoint_url: &str, timeout_secs: u64) -> DomainResult<Docker> {
    Docker::connect_with_unix(endpoint_url, timeout_secs, bollard::API_DEFAULT_VERSION)
        .map_err(|e| DomainError::ConnectionFailed(format!("Local connection failed: {e}")))
}
