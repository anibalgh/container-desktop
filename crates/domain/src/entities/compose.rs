use serde::{Deserialize, Serialize};

/// Represents a Docker Compose stack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComposeStack {
    /// The stack/project name (derived from directory or compose `name`).
    pub name: String,
    /// Path to the compose file.
    pub file_path: String,
    /// List of service names in this stack.
    pub services: Vec<ComposeService>,
    /// Overall stack status.
    pub status: ComposeStackStatus,
}

/// A single service within a Compose stack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComposeService {
    /// The service name as defined in docker-compose.yml.
    pub name: String,
    /// Container IDs or names belonging to this service.
    pub containers: Vec<String>,
    /// Human-readable status (e.g. "Up 2 hours").
    pub status: String,
    /// Ports published by this service.
    pub ports: Vec<String>,
}

/// Status of a Compose stack.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComposeStackStatus {
    /// All services are up and running.
    Running,
    /// Some or all services are stopped.
    Stopped,
    /// Some services are partially up.
    Partial,
    /// The stack status is unknown.
    Unknown,
}
