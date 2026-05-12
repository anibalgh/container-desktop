use domain::entities::DockerEndpoint;
use domain::repository::DockerConnectionRepository;
use std::net::IpAddr;
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

fn tcp_host(url: &str) -> Result<&str, String> {
    let authority = url
        .strip_prefix("tcp://")
        .ok_or_else(|| "Invalid TCP endpoint".to_string())?
        .split('/')
        .next()
        .unwrap_or_default();

    if authority.is_empty() {
        return Err("TCP endpoint must include a host".to_string());
    }

    if authority.starts_with('[') {
        let end = authority
            .find(']')
            .ok_or_else(|| "Invalid IPv6 TCP endpoint".to_string())?;
        return Ok(&authority[1..end]);
    }

    Ok(authority
        .rsplit_once(':')
        .map(|(host, _)| host)
        .unwrap_or(authority))
}

fn is_loopback_host(host: &str) -> bool {
    if host.eq_ignore_ascii_case("localhost") {
        return true;
    }

    host.parse::<IpAddr>()
        .map(|ip| ip.is_loopback())
        .unwrap_or(false)
}

pub(crate) fn validate_docker_endpoint(endpoint: &DockerEndpoint) -> Result<(), String> {
    validate_endpoint_url(&endpoint.host_url)?;

    if endpoint.tls_ca.is_some() || endpoint.tls_cert.is_some() || endpoint.tls_key.is_some() {
        return Err("TLS Docker endpoints are not implemented yet".to_string());
    }

    if endpoint.host_url.starts_with("tcp://") {
        let host = tcp_host(&endpoint.host_url)?;
        if !is_loopback_host(host) {
            return Err(format!(
                "Plain TCP Docker endpoints must use a local loopback host. Got: {host}"
            ));
        }
    }

    if endpoint.timeout_secs == 0 {
        return Err("Endpoint timeout must be at least 1 second".to_string());
    }

    #[cfg(not(target_os = "windows"))]
    if endpoint.host_url.starts_with("npipe://") {
        return Err("Named pipe connections are only supported on Windows".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{validate_docker_endpoint, validate_endpoint_url};
    use domain::entities::DockerEndpoint;

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

    #[test]
    fn loopback_tcp_endpoint_allowed() {
        let endpoint = DockerEndpoint {
            host_url: "tcp://127.0.0.1:2375".into(),
            timeout_secs: 5,
            ..Default::default()
        };
        assert!(validate_docker_endpoint(&endpoint).is_ok());

        let endpoint = DockerEndpoint {
            host_url: "tcp://localhost:2375".into(),
            timeout_secs: 5,
            ..Default::default()
        };
        assert!(validate_docker_endpoint(&endpoint).is_ok());

        let endpoint = DockerEndpoint {
            host_url: "tcp://[::1]:2375".into(),
            timeout_secs: 5,
            ..Default::default()
        };
        assert!(validate_docker_endpoint(&endpoint).is_ok());
    }

    #[test]
    fn remote_tcp_endpoint_rejected() {
        let endpoint = DockerEndpoint {
            host_url: "tcp://192.168.1.10:2375".into(),
            timeout_secs: 5,
            ..Default::default()
        };
        let err = validate_docker_endpoint(&endpoint).unwrap_err();
        assert!(err.contains("loopback"));
    }

    #[test]
    fn tls_endpoint_rejected() {
        let endpoint = DockerEndpoint {
            host_url: "tcp://localhost:2376".into(),
            tls_ca: Some("/tmp/ca.pem".into()),
            timeout_secs: 5,
            ..Default::default()
        };
        let err = validate_docker_endpoint(&endpoint).unwrap_err();
        assert!(err.contains("not implemented"));
    }

    #[test]
    fn zero_timeout_rejected() {
        let endpoint = DockerEndpoint {
            timeout_secs: 0,
            ..Default::default()
        };
        let err = validate_docker_endpoint(&endpoint).unwrap_err();
        assert!(err.contains("at least 1 second"));
    }
}

#[tauri::command]
pub async fn connect(
    state: State<'_, AppState>,
    endpoint: DockerEndpoint,
) -> Result<domain::entities::DockerInfo, String> {
    validate_docker_endpoint(&endpoint)?;
    state
        .docker_client
        .reconfigure(endpoint)
        .await
        .map_err(|e| e.to_string())?;
    state
        .docker_client
        .test_connection()
        .await
        .map_err(|e| e.to_string())
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
