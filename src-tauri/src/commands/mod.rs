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
