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

#[cfg(test)]
mod tests {
    use super::Image;

    #[test]
    fn full_name_with_repo() {
        let img = Image {
            id: "sha256:abc".into(),
            repo_name: "nginx".into(),
            tag: "latest".into(),
            size: "187 MB".into(),
            created: "2026-01-01".into(),
            labels: vec![],
        };
        assert_eq!(img.full_name(), "nginx:latest");
    }

    #[test]
    fn full_name_empty_repo() {
        let img = Image {
            id: "sha256:def".into(),
            repo_name: "".into(),
            tag: "latest".into(),
            size: "0 B".into(),
            created: "".into(),
            labels: vec![],
        };
        assert_eq!(img.full_name(), "latest");
    }

    #[test]
    fn serialization_roundtrip() {
        let img = Image {
            id: "sha256:abc123".into(),
            repo_name: "redis".into(),
            tag: "7-alpine".into(),
            size: "32.5 MB".into(),
            created: "2026-05-01 12:00:00".into(),
            labels: vec!["env=prod".into(), "team=backend".into()],
        };
        let json = serde_json::to_string(&img).unwrap();
        let decoded: Image = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.repo_name, "redis");
        assert_eq!(decoded.tag, "7-alpine");
        assert_eq!(decoded.labels.len(), 2);
        assert_eq!(decoded.full_name(), "redis:7-alpine");
    }
}
