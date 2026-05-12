pub mod compose;
pub mod connection;
pub mod containers;
pub mod images;
pub mod networks;
pub mod settings;
pub mod volumes;

/// Validates a Docker resource ID for length and null bytes to prevent DoS attacks.
///
/// Docker IDs (container, image, volume, network) are typically SHA256 hashes
/// (64 hex chars) or short 12-char prefixes. We allow up to 1024 chars for
/// edge cases but reject obviously malicious payloads.
pub(crate) fn validate_docker_id(id: &str, resource_type: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err(format!("{resource_type} ID cannot be empty"));
    }
    if id.contains('\0') {
        return Err(format!("{resource_type} ID contains null byte"));
    }
    const MAX_ID_LENGTH: usize = 1024;
    if id.len() > MAX_ID_LENGTH {
        return Err(format!(
            "{resource_type} ID too long ({} bytes, max {MAX_ID_LENGTH} bytes)",
            id.len()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_docker_id;

    #[test]
    fn valid_docker_ids() {
        assert!(validate_docker_id("abc123", "Container").is_ok());
        assert!(validate_docker_id("sha256:abcd1234ef567890abcd1234ef567890abcd1234ef567890abcd1234ef567890", "Image").is_ok());
        assert!(validate_docker_id("a", "Network").is_ok());
        // 12-char short ID
        assert!(validate_docker_id("a1b2c3d4e5f6", "Container").is_ok());
    }

    #[test]
    fn empty_id_rejected() {
        let err = validate_docker_id("", "Container").unwrap_err();
        assert!(err.contains("cannot be empty"));
        assert!(err.contains("Container"));
    }

    #[test]
    fn null_byte_rejected() {
        let err = validate_docker_id("abc\0def", "Image").unwrap_err();
        assert!(err.contains("null byte"));
        assert!(err.contains("Image"));
    }

    #[test]
    fn too_long_id_rejected() {
        let long_id = "a".repeat(1025);
        let err = validate_docker_id(&long_id, "Volume").unwrap_err();
        assert!(err.contains("too long"));
        assert!(err.contains("1024"));
        assert!(err.contains("Volume"));
    }

    #[test]
    fn exactly_max_length_accepted() {
        let max_id = "b".repeat(1024);
        assert!(validate_docker_id(&max_id, "Network").is_ok());
    }

    #[test]
    fn resource_type_in_error_message() {
        let err = validate_docker_id("", "Exec").unwrap_err();
        assert!(err.contains("Exec"));
        let err = validate_docker_id("\0", "Container").unwrap_err();
        assert!(err.contains("Container"));
    }

    #[test]
    fn unicode_ids_accepted() {
        // Docker names can contain Unicode in some contexts
        assert!(validate_docker_id("café", "Volume").is_ok());
        assert!(validate_docker_id("ネットワーク", "Network").is_ok());
    }
}
