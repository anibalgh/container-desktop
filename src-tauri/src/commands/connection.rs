use domain::entities::DockerEndpoint;
use domain::repository::DockerConnectionRepository;
use tauri::State;

use crate::AppState;

/// Validates that the Docker endpoint URL uses an allowed scheme and is non-empty.
fn validate_endpoint_url(url: &str) -> Result<(), String> {
    if url.is_empty() {
        return Err("Endpoint URL cannot be empty".to_string());
    }

    // Reject URLs with null bytes (can bypass validation checks)
    if url.contains('\0') {
        return Err("Endpoint URL contains null byte".to_string());
    }

    // Only allow known Docker transport schemes
    let allowed_prefixes = ["unix://", "tcp://", "npipe://"];
    let has_allowed_prefix = allowed_prefixes
        .iter()
        .any(|prefix| url.starts_with(prefix));

    if !has_allowed_prefix {
        return Err(format!(
            "Invalid endpoint scheme. Allowed: unix://, tcp://, npipe://. Got: {url}"
        ));
    }

    // Enforce maximum URL length to prevent abuse
    const MAX_URL_LENGTH: usize = 4096;
    if url.len() > MAX_URL_LENGTH {
        return Err(format!(
            "Endpoint URL too long ({} bytes, max {} bytes)",
            url.len(),
            MAX_URL_LENGTH
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_endpoint_url;

    #[test]
    fn valid_endpoint_urls() {
        assert!(validate_endpoint_url("unix:///var/run/docker.sock").is_ok());
        assert!(validate_endpoint_url("tcp://192.168.1.10:2376").is_ok());
        assert!(validate_endpoint_url("npipe:////./pipe/docker_engine").is_ok());
        assert!(validate_endpoint_url("tcp://localhost:2375").is_ok());
    }

    #[test]
    fn empty_url_rejected() {
        let err = validate_endpoint_url("").unwrap_err();
        assert!(err.contains("cannot be empty"));
    }

    #[test]
    fn null_byte_rejected() {
        let err = validate_endpoint_url("unix://\0/path").unwrap_err();
        assert!(err.contains("null byte"));
    }

    #[test]
    fn unknown_scheme_rejected() {
        let err = validate_endpoint_url("http://example.com").unwrap_err();
        assert!(err.contains("Invalid endpoint scheme"));

        let err = validate_endpoint_url("ssh://host").unwrap_err();
        assert!(err.contains("Invalid endpoint scheme"));

        let err = validate_endpoint_url("fd://socket").unwrap_err();
        assert!(err.contains("Invalid endpoint scheme"));
    }

    #[test]
    fn no_scheme_rejected() {
        let err = validate_endpoint_url("/var/run/docker.sock").unwrap_err();
        assert!(err.contains("Invalid endpoint scheme"));
        let err = validate_endpoint_url("just-a-string").unwrap_err();
        assert!(err.contains("Invalid endpoint scheme"));
    }

    #[test]
    fn too_long_url_rejected() {
        let long_url = format!("unix://{}", "a".repeat(4100));
        let err = validate_endpoint_url(&long_url).unwrap_err();
        assert!(err.contains("too long"));
        assert!(err.contains("4096"));
    }

    #[test]
    fn max_length_accepted() {
        let max_url = format!("tcp://{}", "b".repeat(4090));
        assert!(validate_endpoint_url(&max_url).is_ok());
    }
}

#[tauri::command]
pub async fn connect(
    _state: State<'_, AppState>,
    endpoint: DockerEndpoint,
) -> Result<domain::entities::DockerInfo, String> {
    // Validate the endpoint URL before attempting connection
    validate_endpoint_url(&endpoint.host_url)?;

    // Create a new client for the given endpoint
    let client = infrastructure::DockerClient::new(endpoint);
    client.connect().await.map_err(|e| e.to_string())?;
    client.test_connection().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_connection(
    state: State<'_, AppState>,
) -> Result<domain::entities::DockerInfo, String> {
    state
        .docker_client
        .test_connection()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ping(state: State<'_, AppState>) -> Result<bool, String> {
    match state.docker_client.ping().await {
        Ok(()) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}
