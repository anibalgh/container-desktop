use serde::{Deserialize, Serialize};

/// Docker daemon connection endpoint configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerEndpoint {
    /// The connection URL.
    ///
    /// Examples:
    /// - `unix:///var/run/docker.sock` (local Linux/macOS)
    /// - `npipe:////./pipe/docker_engine` (local Windows)
    /// - `tcp://192.168.1.10:2376` (remote Docker)
    pub host_url: String,
    /// Path to the TLS CA certificate, if using TLS.
    pub tls_ca: Option<String>,
    /// Path to the TLS client certificate, if using TLS.
    pub tls_cert: Option<String>,
    /// Path to the TLS client key, if using TLS.
    pub tls_key: Option<String>,
    /// Connection timeout in seconds.
    pub timeout_secs: u64,
}

impl Default for DockerEndpoint {
    fn default() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            host_url: "npipe:////./pipe/docker_engine".to_string(),
            #[cfg(not(target_os = "windows"))]
            host_url: "unix:///var/run/docker.sock".to_string(),
            tls_ca: None,
            tls_cert: None,
            tls_key: None,
            timeout_secs: 30,
        }
    }
}

impl DockerEndpoint {
    /// Returns the display-friendly connection type label.
    pub fn connection_type(&self) -> &str {
        if self.host_url.starts_with("unix://") {
            "Local socket"
        } else if self.host_url.starts_with("npipe://") {
            "Named pipe"
        } else if self.host_url.starts_with("tcp://") {
            if self.tls_ca.is_some() {
                "Remote (TLS)"
            } else {
                "Remote (plain)"
            }
        } else {
            "Unknown"
        }
    }
}
