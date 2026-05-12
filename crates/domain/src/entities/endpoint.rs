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

#[cfg(test)]
mod tests {
    use super::DockerEndpoint;

    #[test]
    fn default_unix_socket_on_linux() {
        let endpoint = DockerEndpoint::default();
        #[cfg(not(target_os = "windows"))]
        assert_eq!(endpoint.host_url, "unix:///var/run/docker.sock");
        #[cfg(target_os = "windows")]
        assert_eq!(endpoint.host_url, "npipe:////./pipe/docker_engine");
        assert_eq!(endpoint.timeout_secs, 30);
        assert!(endpoint.tls_ca.is_none());
        assert!(endpoint.tls_cert.is_none());
        assert!(endpoint.tls_key.is_none());
    }

    #[test]
    fn connection_type_unix() {
        let ep = DockerEndpoint { host_url: "unix:///var/run/docker.sock".into(), ..Default::default() };
        assert_eq!(ep.connection_type(), "Local socket");
    }

    #[test]
    fn connection_type_tcp() {
        let ep = DockerEndpoint { host_url: "tcp://192.168.1.1:2375".into(), ..Default::default() };
        assert_eq!(ep.connection_type(), "Remote (plain)");
    }

    #[test]
    fn connection_type_tcp_tls() {
        let ep = DockerEndpoint {
            host_url: "tcp://192.168.1.1:2376".into(),
            tls_ca: Some("/ca.pem".into()),
            ..Default::default()
        };
        assert_eq!(ep.connection_type(), "Remote (TLS)");
    }

    #[test]
    fn connection_type_npipe() {
        let ep = DockerEndpoint { host_url: "npipe:////./pipe/docker_engine".into(), ..Default::default() };
        assert_eq!(ep.connection_type(), "Named pipe");
    }

    #[test]
    fn connection_type_unknown() {
        let ep = DockerEndpoint { host_url: "ssh://host".into(), ..Default::default() };
        assert_eq!(ep.connection_type(), "Unknown");
    }

    #[test]
    fn serialization_roundtrip() {
        let ep = DockerEndpoint {
            host_url: "tcp://host:2376".into(),
            tls_ca: Some("/ca.pem".into()),
            tls_cert: Some("/cert.pem".into()),
            tls_key: Some("/key.pem".into()),
            timeout_secs: 60,
        };
        let json = serde_json::to_string(&ep).unwrap();
        let decoded: DockerEndpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.host_url, ep.host_url);
        assert_eq!(decoded.tls_ca, ep.tls_ca);
        assert_eq!(decoded.timeout_secs, 60);
    }
}
