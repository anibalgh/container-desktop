use serde::{Deserialize, Serialize};

/// Represents a container port mapping.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortMapping {
    /// The host-side IP address (e.g. "0.0.0.0", "127.0.0.1").
    pub host_ip: String,
    /// The host-side port number.
    pub host_port: String,
    /// The container-side port number.
    pub container_port: String,
    /// The protocol (tcp, udp, sctp).
    pub protocol: String,
}

impl std::fmt::Display for PortMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}->{}/{}",
            self.host_ip, self.host_port, self.container_port, self.protocol
        )
    }
}

/// Represents a container mount (bind mount or volume).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Mount {
    /// The source path or volume name.
    pub source: String,
    /// The destination path inside the container.
    pub destination: String,
    /// The mount type: "bind", "volume", or "tmpfs".
    pub mount_type: String,
    /// Whether the mount is read-only.
    pub read_only: bool,
}

/// Represents a Docker container.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Container {
    /// The container ID.
    pub id: String,
    /// The container name.
    pub name: String,
    /// The image used by this container.
    pub image: String,
    /// The lifecycle status (e.g. "running", "exited", "created").
    pub status: String,
    /// The runtime state (running, stopped, paused, restarting, removing, dead).
    pub state: ContainerState,
    /// Port mappings published by the container.
    pub ports: Vec<PortMapping>,
    /// Volume/bind mounts attached to the container.
    pub mounts: Vec<Mount>,
    /// Timestamp when the container was created.
    pub created: String,
    /// The command being executed inside the container.
    pub command: String,
}

/// Runtime state of a container.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContainerState {
    /// Container is actively running.
    Running,
    /// Container has exited.
    Exited,
    /// Container is paused.
    Paused,
    /// Container is being restarted.
    Restarting,
    /// Container was created but not started.
    Created,
    /// Container is being removed.
    Removing,
    /// Container is in an unrecoverable state.
    Dead,
}

impl std::fmt::Display for ContainerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerState::Running => write!(f, "running"),
            ContainerState::Exited => write!(f, "exited"),
            ContainerState::Paused => write!(f, "paused"),
            ContainerState::Restarting => write!(f, "restarting"),
            ContainerState::Created => write!(f, "created"),
            ContainerState::Removing => write!(f, "removing"),
            ContainerState::Dead => write!(f, "dead"),
        }
    }
}

/// Configuration for creating a new container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    /// The image name (e.g. "nginx:latest").
    pub image: String,
    /// The container name.
    pub name: Option<String>,
    /// The command to run (overrides image CMD).
    pub command: Option<Vec<String>>,
    /// Environment variables in KEY=VALUE format.
    pub env: Vec<String>,
    /// Port mappings to publish when creating the container.
    pub port_mappings: Vec<PortMapping>,
    /// Volume mounts.
    pub volumes: Vec<(String, String)>,
    /// Whether to run in detached mode.
    pub detached: bool,
    /// Whether to auto-remove when stopped.
    pub auto_remove: bool,
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            image: String::new(),
            name: None,
            command: None,
            env: Vec::new(),
            port_mappings: Vec::new(),
            volumes: Vec::new(),
            detached: true,
            auto_remove: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_state_display() {
        assert_eq!(ContainerState::Running.to_string(), "running");
        assert_eq!(ContainerState::Exited.to_string(), "exited");
        assert_eq!(ContainerState::Paused.to_string(), "paused");
        assert_eq!(ContainerState::Restarting.to_string(), "restarting");
        assert_eq!(ContainerState::Created.to_string(), "created");
        assert_eq!(ContainerState::Removing.to_string(), "removing");
        assert_eq!(ContainerState::Dead.to_string(), "dead");
    }

    #[test]
    fn port_mapping_display() {
        let pm = PortMapping {
            host_ip: "0.0.0.0".into(),
            host_port: "8080".into(),
            container_port: "80".into(),
            protocol: "tcp".into(),
        };
        assert_eq!(pm.to_string(), "0.0.0.0:8080->80/tcp");
    }

    #[test]
    fn container_config_default() {
        let config = ContainerConfig::default();
        assert!(config.image.is_empty());
        assert!(config.name.is_none());
        assert!(config.command.is_none());
        assert!(config.env.is_empty());
        assert!(config.port_mappings.is_empty());
        assert!(config.volumes.is_empty());
        assert!(config.detached);
        assert!(!config.auto_remove);
    }

    #[test]
    fn container_serialization_roundtrip() {
        let container = Container {
            id: "abc123".into(),
            name: "web-server".into(),
            image: "nginx:latest".into(),
            status: "Up 3 hours".into(),
            state: ContainerState::Running,
            ports: vec![PortMapping {
                host_ip: "0.0.0.0".into(),
                host_port: "8080".into(),
                container_port: "80".into(),
                protocol: "tcp".into(),
            }],
            mounts: vec![],
            created: "2026-01-01 00:00:00".into(),
            command: "nginx -g daemon off;".into(),
        };
        let json = serde_json::to_string(&container).unwrap();
        let decoded: Container = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.id, "abc123");
        assert_eq!(decoded.name, "web-server");
        assert_eq!(decoded.state, ContainerState::Running);
        assert_eq!(decoded.ports.len(), 1);
        assert_eq!(decoded.ports[0].host_port, "8080");
    }

    #[test]
    fn container_state_serialization() {
        let state = ContainerState::Running;
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("Running"));
        let decoded: ContainerState = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, ContainerState::Running);
    }

    #[test]
    fn log_stream_serialization() {
        let line = LogLine {
            stream: LogStream::Stdout,
            content: "Hello world".into(),
            timestamp: Some("2026-01-01T00:00:00Z".into()),
        };
        let json = serde_json::to_string(&line).unwrap();
        let decoded: LogLine = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.content, "Hello world");
        assert_eq!(decoded.stream, LogStream::Stdout);
        assert_eq!(decoded.timestamp.unwrap(), "2026-01-01T00:00:00Z");
    }
}

/// A line of container log output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogLine {
    /// The output stream: stdout or stderr.
    pub stream: LogStream,
    /// The log content.
    pub content: String,
    /// Timestamp of the log entry, if available.
    pub timestamp: Option<String>,
}

/// Log output stream type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogStream {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
}

/// Output from a container exec session.
#[derive(Debug, Clone)]
pub struct ExecOutput {
    /// The output bytes.
    pub data: Vec<u8>,
}

/// Resource usage statistics for a container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStats {
    /// CPU usage percentage.
    pub cpu_percent: f64,
    /// Memory usage in human-readable form.
    pub memory_usage: String,
    /// Memory usage in bytes.
    pub memory_usage_bytes: u64,
    /// Total system memory in bytes.
    pub memory_limit_bytes: u64,
    /// Network input in human-readable form.
    pub network_rx: String,
    /// Network output in human-readable form.
    pub network_tx: String,
    /// Block input in human-readable form.
    pub block_read: String,
    /// Block output in human-readable form.
    pub block_write: String,
    /// Number of PIDs running in the container.
    pub pids: u64,
}

/// General Docker daemon information (returned on connection).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerInfo {
    /// Docker daemon version.
    pub server_version: String,
    /// Number of running containers.
    pub containers_running: u64,
    /// Number of paused containers.
    pub containers_paused: u64,
    /// Number of stopped containers.
    pub containers_stopped: u64,
    /// Number of images.
    pub images: u64,
    /// Server OS type.
    pub os_type: String,
    /// Server architecture.
    pub architecture: String,
    /// Active Docker endpoint URL.
    pub endpoint: String,
}

/// Unique identifier for an exec session.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExecId(pub String);

/// An active exec session with read/write handles.
pub struct ExecSession {
    /// Stream of output from the exec session.
    pub output:
        Box<dyn futures::Stream<Item = crate::error::DomainResult<ExecOutput>> + Unpin + Send>,
    /// Writable input to the exec session (using tokio's AsyncWrite).
    pub input: Box<dyn tokio::io::AsyncWrite + Unpin + Send>,
}
