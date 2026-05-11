use serde::{Deserialize, Serialize};

/// Represents a Docker volume.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Volume {
    /// The volume name.
    pub name: String,
    /// The volume driver (e.g. "local", "nfs").
    pub driver: String,
    /// The mount point on the host filesystem.
    pub mountpoint: String,
    /// The scope of the volume: "local" or "global".
    pub scope: String,
    /// Creation timestamp.
    pub created: String,
}
