use serde::{Deserialize, Serialize};

/// Represents a Docker image.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Image {
    /// The image ID (digest hash).
    pub id: String,
    /// The repository name (e.g. "nginx").
    pub repo_name: String,
    /// The image tag (e.g. "latest", "1.25").
    pub tag: String,
    /// Human-readable size (e.g. "187 MB").
    pub size: String,
    /// Timestamp when the image was created.
    pub created: String,
    /// DNS-namespace labels attached to the image.
    pub labels: Vec<String>,
}

impl Image {
    /// Returns the full `repository:tag` string for this image.
    pub fn full_name(&self) -> String {
        if self.repo_name.is_empty() {
            self.tag.clone()
        } else {
            format!("{}:{}", self.repo_name, self.tag)
        }
    }
}
