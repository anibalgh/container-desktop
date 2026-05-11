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
    /// Port mappings (container_port -> host_port).
    pub port_mappings: Vec<(String, String)>,
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

/// A line of container log output.
#[derive(Debug, Clone)]
pub struct LogLine {
    /// The output stream: stdout or stderr.
    pub stream: LogStream,
    /// The log content.
    pub content: String,
    /// Timestamp of the log entry, if available.
    pub timestamp: Option<String>,
}

/// Log output stream type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
