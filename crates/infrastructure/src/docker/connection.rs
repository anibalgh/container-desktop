use super::DockerClient;
use async_trait::async_trait;
use bollard::query_parameters::DataUsageOptions;
use domain::entities::{DockerCleanupSummary, DockerInfo};
use domain::repository::DockerConnectionRepository;
use domain::{DomainError, DomainResult};
use std::process::Command;

#[async_trait]
impl DockerConnectionRepository for DockerClient {
    async fn test_connection(&self) -> DomainResult<DockerInfo> {
        let docker = self.get_docker().await?;

        let info = docker
            .info()
            .await
            .map_err(|e| DomainError::ConnectionFailed(format!("Cannot get Docker info: {e}")))?;

        let version = docker.version().await.map_err(|e| {
            DomainError::ConnectionFailed(format!("Cannot get Docker version: {e}"))
        })?;

        Ok(DockerInfo {
            server_version: version.version.unwrap_or_default(),
            containers_running: info.containers_running.unwrap_or(0) as u64,
            containers_paused: info.containers_paused.unwrap_or(0) as u64,
            containers_stopped: info.containers_stopped.unwrap_or(0) as u64,
            images: info.images.unwrap_or(0) as u64,
            os_type: info.operating_system.unwrap_or_default(),
            architecture: info.architecture.unwrap_or_default(),
            endpoint: self.endpoint().host_url,
        })
    }

    fn endpoint_url(&self) -> String {
        self.endpoint().host_url
    }

    async fn ping(&self) -> DomainResult<()> {
        let docker = self.get_docker().await?;
        docker
            .ping()
            .await
            .map(|_| ())
            .map_err(|e| DomainError::ConnectionFailed(format!("Ping failed: {e}")))
    }

    async fn cleanup_summary(&self) -> DomainResult<DockerCleanupSummary> {
        let docker = self.get_docker().await?;
        let usage = docker.df(None::<DataUsageOptions>).await.map_err(|e| {
            DomainError::OperationFailed(format!("Failed to inspect Docker disk usage: {e}"))
        })?;
        let reclaimable_bytes = total_reclaimable_bytes(&usage);

        Ok(DockerCleanupSummary {
            reclaimable_mb: bytes_to_mb(reclaimable_bytes),
            reclaimable_bytes,
        })
    }

    async fn system_prune(&self) -> DomainResult<()> {
        self.ping().await?;

        let output = docker_cli_command(&self.endpoint())
            .args(["system", "prune", "-f"])
            .output()
            .map_err(|e| {
                DomainError::OperationFailed(format!("Failed to run docker system prune: {e}"))
            })?;

        if !output.status.success() {
            return Err(DomainError::OperationFailed(format!(
                "Failed to run docker system prune: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            )));
        }

        Ok(())
    }
}

fn docker_cli_command(endpoint: &domain::entities::DockerEndpoint) -> Command {
    let mut command = Command::new("docker");
    command.env("DOCKER_HOST", &endpoint.host_url);
    command.env_remove("DOCKER_TLS_VERIFY");
    command.env_remove("DOCKER_CERT_PATH");
    command
}

fn total_reclaimable_bytes(usage: &bollard::models::SystemDataUsageResponse) -> u64 {
    [
        usage
            .image_usage
            .as_ref()
            .and_then(|usage| usage.reclaimable),
        usage
            .container_usage
            .as_ref()
            .and_then(|usage| usage.reclaimable),
        usage
            .volume_usage
            .as_ref()
            .and_then(|usage| usage.reclaimable),
        usage
            .build_cache_usage
            .as_ref()
            .and_then(|usage| usage.reclaimable),
    ]
    .into_iter()
    .flatten()
    .fold(0_u64, |acc, value| {
        acc.saturating_add(non_negative_bytes(value))
    })
}

fn non_negative_bytes(value: i64) -> u64 {
    u64::try_from(value).unwrap_or_default()
}

#[cfg(test)]
fn parse_reclaimable_size(value: &str) -> DomainResult<u64> {
    let token = value
        .split_whitespace()
        .next()
        .ok_or_else(|| DomainError::Serialization("Missing reclaimable size".to_string()))?;
    parse_size_token(token)
}

#[cfg(test)]
fn parse_reclaimable_rows(stdout: &str) -> DomainResult<u64> {
    stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .try_fold(0_u64, |acc, line| {
            let reclaimable = line
                .split("\"Reclaimable\":\"")
                .nth(1)
                .and_then(|part| part.split('"').next())
                .ok_or_else(|| {
                    DomainError::Serialization(
                        "Missing reclaimable field in docker df row".to_string(),
                    )
                })?;
            let bytes = parse_reclaimable_size(reclaimable)?;
            Ok(acc.saturating_add(bytes))
        })
}

#[cfg(test)]
fn parse_size_token(token: &str) -> DomainResult<u64> {
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return Err(DomainError::Serialization(
            "Empty reclaimable size".to_string(),
        ));
    }

    let number_len = trimmed
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .count();
    if number_len == 0 {
        return Err(DomainError::Serialization(format!(
            "Invalid reclaimable size token: {trimmed}"
        )));
    }

    let number = trimmed[..number_len].parse::<f64>().map_err(|e| {
        DomainError::Serialization(format!("Invalid reclaimable size number {trimmed}: {e}"))
    })?;
    let unit = trimmed[number_len..].trim().to_ascii_lowercase();
    let multiplier = match unit.as_str() {
        "b" | "" => 1_f64,
        "kb" | "kib" => 1024_f64,
        "mb" | "mib" => 1024_f64.powi(2),
        "gb" | "gib" => 1024_f64.powi(3),
        "tb" | "tib" => 1024_f64.powi(4),
        _ => {
            return Err(DomainError::Serialization(format!(
                "Unsupported reclaimable size unit: {unit}"
            )))
        }
    };

    Ok((number * multiplier).round() as u64)
}

fn bytes_to_mb(bytes: u64) -> f64 {
    ((bytes as f64 / 1_000_000_f64) * 10.0).round() / 10.0
}

#[cfg(test)]
mod tests {
    use super::{
        bytes_to_mb, parse_reclaimable_rows, parse_reclaimable_size, parse_size_token,
        total_reclaimable_bytes,
    };
    use bollard::models::{
        BuildCacheDiskUsage, ContainersDiskUsage, ImagesDiskUsage, SystemDataUsageResponse,
        VolumesDiskUsage,
    };

    #[test]
    fn parses_reclaimable_size_with_percentage() {
        assert_eq!(
            parse_reclaimable_size("8.04GB (28%)").unwrap(),
            (8.04_f64 * 1024_f64.powi(3)).round() as u64
        );
    }

    #[test]
    fn parses_small_units() {
        assert_eq!(parse_size_token("483.3kB").unwrap(), 494_899);
        assert_eq!(parse_size_token("512B").unwrap(), 512);
    }

    #[test]
    fn parses_docker_df_rows_total() {
        let stdout = r#"{"Type":"Images","Reclaimable":"8.04GB (28%)"}
{"Type":"Containers","Reclaimable":"0B (0%)"}
{"Type":"Build Cache","Reclaimable":"483.3kB"}
"#;

        let total = parse_reclaimable_rows(stdout).unwrap();
        assert_eq!(
            total,
            (8.04_f64 * 1024_f64.powi(3)).round() as u64 + 494_899
        );
    }

    #[test]
    fn rounds_mb_display() {
        assert_eq!(bytes_to_mb(1_000_000), 1.0);
        assert_eq!(bytes_to_mb(1_500_000), 1.5);
    }

    #[test]
    fn sums_reclaimable_bytes_from_data_usage_response() {
        let usage = SystemDataUsageResponse {
            image_usage: Some(ImagesDiskUsage {
                reclaimable: Some(1_000_000),
                ..Default::default()
            }),
            container_usage: Some(ContainersDiskUsage {
                reclaimable: Some(2_000_000),
                ..Default::default()
            }),
            volume_usage: Some(VolumesDiskUsage {
                reclaimable: Some(-1),
                ..Default::default()
            }),
            build_cache_usage: Some(BuildCacheDiskUsage {
                reclaimable: Some(500_000),
                ..Default::default()
            }),
        };

        assert_eq!(total_reclaimable_bytes(&usage), 3_500_000);
    }
}
