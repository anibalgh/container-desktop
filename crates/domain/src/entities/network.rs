use serde::{Deserialize, Serialize};

/// Represents a Docker network.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Network {
    /// The network ID.
    pub id: String,
    /// The network name.
    pub name: String,
    /// The network driver (e.g. "bridge", "overlay", "host").
    pub driver: String,
    /// The network scope: "local", "swarm", "global".
    pub scope: String,
    /// The subnet/gateway used by this network.
    pub subnet: Option<String>,
    /// The gateway IP address.
    pub gateway: Option<String>,
    /// Whether this network is internal-only.
    pub internal: bool,
    /// Number of containers attached to this network.
    pub containers_count: u32,
    /// Creation timestamp.
    pub created: String,
}
