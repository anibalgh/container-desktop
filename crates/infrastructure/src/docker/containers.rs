use super::DockerClient;
use async_trait::async_trait;
use bollard::query_parameters::{
    ListContainersOptionsBuilder, LogsOptionsBuilder, ResizeExecOptionsBuilder, StatsOptions,
};
use domain::entities::{
    Container, ContainerConfig, ContainerState, ContainerStats, ExecId, ExecOutput, ExecSession,
    LogLine, LogStream, Mount, PortMapping,
};
use domain::repository::ContainerRepository;
use domain::{DomainError, DomainResult};
use futures::Stream;
use futures::StreamExt;

/// Validates a container name against Docker's naming rules.
///
/// Docker container names must match `[a-zA-Z0-9][a-zA-Z0-9_.-]+`.
fn validate_container_name(name: &str) -> DomainResult<()> {
    if name.is_empty() {
        return Ok(()); // empty name is fine, Docker will auto-generate one
    }

    // Docker container names are limited to 255 characters
    if name.len() > 255 {
        return Err(DomainError::Config(
            "Container name exceeds maximum length of 255 characters".to_string(),
        ));
    }

    // Reject null bytes (can bypass validation)
    if name.contains('\0') {
        return Err(DomainError::Config(
            "Container name contains null byte".to_string(),
        ));
    }

    // First character must be alphanumeric, underscore, or dot
    let first_char = name.chars().next().unwrap();
    if !first_char.is_ascii_alphanumeric() && first_char != '_' && first_char != '.' {
        return Err(DomainError::Config(format!(
            "Container name must start with alphanumeric, underscore, or dot: '{name}'"
        )));
    }

    // All characters must be alphanumeric, underscore, dot, or hyphen
    for (i, c) in name.chars().enumerate() {
        if !c.is_ascii_alphanumeric() && c != '_' && c != '.' && c != '-' {
            return Err(DomainError::Config(format!(
                "Invalid character '{c}' at position {i} in container name '{name}'"
            )));
        }
    }

    Ok(())
}

fn validate_container_create_config(config: &ContainerConfig) -> DomainResult<()> {
    if config.image.trim().is_empty() {
        return Err(DomainError::Config(
            "Container image cannot be empty".to_string(),
        ));
    }

    for mapping in &config.port_mappings {
        if mapping.container_port.trim().is_empty() {
            return Err(DomainError::Config(
                "Container port cannot be empty".to_string(),
            ));
        }
        if mapping.host_port.trim().is_empty() {
            return Err(DomainError::Config("Host port cannot be empty".to_string()));
        }

        let protocol = if mapping.protocol.trim().is_empty() {
            "tcp"
        } else {
            mapping.protocol.trim()
        };

        if !matches!(protocol, "tcp" | "udp" | "sctp") {
            return Err(DomainError::Config(format!(
                "Unsupported port mapping protocol: {protocol}"
            )));
        }
    }

    Ok(())
}

#[async_trait]
impl ContainerRepository for DockerClient {
    async fn list_containers(&self, all: bool) -> DomainResult<Vec<Container>> {
        let docker = self.get_docker().await?;
        let options = ListContainersOptionsBuilder::default().all(all).build();
        let containers = docker
            .list_containers(Some(options))
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot list containers: {e}")))?;

        let result = containers
            .into_iter()
            .map(|c| {
                let ports: Vec<PortMapping> = c
                    .ports
                    .map(|p_list| {
                        p_list
                            .into_iter()
                            .map(|p| PortMapping {
                                host_ip: p.ip.unwrap_or_default(),
                                host_port: p
                                    .public_port
                                    .map(|pp| pp.to_string())
                                    .unwrap_or_default(),
                                container_port: p.private_port.to_string(),
                                protocol: match p.typ {
                                    Some(t) => {
                                        format!("{t:?}").to_lowercase().trim_matches('"').to_owned()
                                    }
                                    None => "tcp".to_string(),
                                },
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                let mounts: Vec<Mount> = c
                    .mounts
                    .map(|m_list| {
                        m_list
                            .into_iter()
                            .map(|m| Mount {
                                source: m.source.unwrap_or_default(),
                                destination: m.destination.unwrap_or_default(),
                                mount_type: match m.typ {
                                    Some(t) => {
                                        format!("{t:?}").to_lowercase().trim_matches('"').to_owned()
                                    }
                                    None => "bind".to_string(),
                                },
                                read_only: !m.rw.unwrap_or(true),
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                let state = c
                    .state
                    .map(|s| {
                        parse_container_state(format!("{s:?}").to_lowercase().trim_matches('"'))
                    })
                    .unwrap_or(ContainerState::Created);

                Container {
                    id: c.id.unwrap_or_default(),
                    name: c
                        .names
                        .and_then(|n| n.first().map(|s| s.trim_start_matches('/').to_string()))
                        .unwrap_or_default(),
                    image: c.image.unwrap_or_default(),
                    status: c.status.unwrap_or_default(),
                    state,
                    ports,
                    mounts,
                    created: c.created.map(format_created).unwrap_or_default(),
                    command: c.command.unwrap_or_default(),
                }
            })
            .collect();

        Ok(result)
    }

    async fn create_container(&self, config: &ContainerConfig) -> DomainResult<String> {
        let docker = self.get_docker().await?;
        validate_container_create_config(config)?;

        // Validate the container name if provided
        if let Some(ref name) = config.name {
            validate_container_name(name)?;
        }

        let mut hc = bollard::config::HostConfig::default();
        if !config.volumes.is_empty() {
            hc.binds = Some(
                config
                    .volumes
                    .iter()
                    .map(|(s, d)| format!("{s}:{d}"))
                    .collect(),
            );
        }
        if !config.port_mappings.is_empty() {
            let mut pb = std::collections::HashMap::new();
            for mapping in &config.port_mappings {
                let protocol = if mapping.protocol.trim().is_empty() {
                    "tcp"
                } else {
                    mapping.protocol.trim()
                };
                let host_ip = if mapping.host_ip.trim().is_empty() {
                    "127.0.0.1"
                } else {
                    mapping.host_ip.trim()
                };
                pb.insert(
                    format!("{}/{}", mapping.container_port.trim(), protocol),
                    Some(vec![bollard::config::PortBinding {
                        host_ip: Some(host_ip.to_string()),
                        host_port: Some(mapping.host_port.trim().to_string()),
                    }]),
                );
            }
            hc.port_bindings = Some(pb);
        }
        hc.auto_remove = Some(config.auto_remove);

        let opts = bollard::query_parameters::CreateContainerOptionsBuilder::default()
            .name(&config.name.clone().unwrap_or_default())
            .build();

        let body = bollard::config::ContainerCreateBody {
            image: Some(config.image.clone()),
            cmd: config.command.clone(),
            env: if config.env.is_empty() {
                None
            } else {
                Some(config.env.clone())
            },
            host_config: Some(hc),
            ..Default::default()
        };

        docker
            .create_container(Some(opts), body)
            .await
            .map(|r| r.id)
            .map_err(|e| DomainError::DockerApi(format!("Cannot create container: {e}")))
    }

    async fn start_container(&self, id: &str) -> DomainResult<()> {
        let docker = self.get_docker().await?;
        docker
            .start_container(id, None)
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot start container: {e}")))
    }
    async fn stop_container(&self, id: &str) -> DomainResult<()> {
        let docker = self.get_docker().await?;
        docker
            .stop_container(id, None)
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot stop container: {e}")))
    }
    async fn restart_container(&self, id: &str) -> DomainResult<()> {
        let docker = self.get_docker().await?;
        docker
            .restart_container(id, None)
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot restart container: {e}")))
    }
    async fn remove_container(&self, id: &str) -> DomainResult<()> {
        let docker = self.get_docker().await?;
        let opts = bollard::query_parameters::RemoveContainerOptionsBuilder::default()
            .force(true)
            .build();
        docker
            .remove_container(id, Some(opts))
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot remove container: {e}")))
    }

    async fn container_logs(
        &self,
        id: &str,
        tail: Option<u32>,
        follow: bool,
        since: Option<i32>,
        until: Option<i32>,
    ) -> DomainResult<Box<dyn Stream<Item = DomainResult<LogLine>> + Unpin + Send>> {
        let docker = self.get_docker().await?;
        let mut b = LogsOptionsBuilder::default()
            .stdout(true)
            .stderr(true)
            .follow(follow);
        let tail_str: String = match tail {
            Some(t) => t.to_string(),
            None => "all".to_string(),
        };
        b = b.tail(&tail_str);
        if let Some(s) = since {
            b = b.since(s);
        }
        if let Some(u) = until {
            b = b.until(u);
        }
        let stream = docker.logs(id, Some(b.build()));

        Ok(Box::new(stream.map(|r| {
            r.map_err(|e| DomainError::DockerApi(format!("Log error: {e}")))
                .map(|o| {
                    use bollard::container::LogOutput;
                    let (st, data) = match o {
                        LogOutput::StdOut { message } => (LogStream::Stdout, message),
                        LogOutput::StdErr { message } => (LogStream::Stderr, message),
                        LogOutput::StdIn { message } => (LogStream::Stdout, message),
                        LogOutput::Console { message } => (LogStream::Stdout, message),
                    };
                    LogLine {
                        stream: st,
                        content: String::from_utf8_lossy(&data).to_string(),
                        timestamp: None,
                    }
                })
        })))
    }

    async fn create_exec(
        &self,
        id: &str,
        cmd: &[String],
        user: Option<&str>,
    ) -> DomainResult<ExecId> {
        let docker = self.get_docker().await?;
        let r = docker
            .create_exec(
                id,
                bollard::exec::CreateExecOptions {
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    attach_stdin: Some(true),
                    tty: Some(true),
                    cmd: Some(cmd.to_vec()),
                    user: user.map(|value| value.to_string()),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot create exec: {e}")))?;
        Ok(ExecId(r.id))
    }

    async fn start_exec_interactive(&self, exec_id: &ExecId) -> DomainResult<ExecSession> {
        let docker = self.get_docker().await?;
        let r = docker
            .start_exec(
                &exec_id.0,
                Some(bollard::exec::StartExecOptions {
                    tty: true,
                    detach: false,
                    output_capacity: None,
                }),
            )
            .await
            .map_err(|e| DomainError::DockerApi(format!("Start exec: {e}")))?;

        match r {
            bollard::exec::StartExecResults::Attached { output, input } => {
                let mapped = output.map(|r| {
                    r.map_err(|e| DomainError::DockerApi(format!("Exec: {e}")))
                        .map(|lo| {
                            use bollard::container::LogOutput;
                            let data = match lo {
                                LogOutput::StdOut { message } => message,
                                LogOutput::StdErr { message } => message,
                                LogOutput::Console { message } => message,
                                LogOutput::StdIn { message } => message,
                            };
                            ExecOutput {
                                data: data.to_vec(),
                            }
                        })
                });
                Ok(ExecSession {
                    output: Box::new(mapped),
                    input: Box::new(input),
                })
            }
            bollard::exec::StartExecResults::Detached => Err(DomainError::OperationFailed(
                "Exec detached unexpectedly".to_string(),
            )),
        }
    }

    async fn start_exec(
        &self,
        exec_id: &ExecId,
    ) -> DomainResult<Box<dyn Stream<Item = DomainResult<ExecOutput>> + Unpin + Send>> {
        self.start_exec_interactive(exec_id).await.map(|s| s.output)
    }

    async fn resize_exec(&self, exec_id: &ExecId, width: u16, height: u16) -> DomainResult<()> {
        let docker = self.get_docker().await?;
        docker
            .resize_exec(
                &exec_id.0,
                ResizeExecOptionsBuilder::default()
                    .w(width as i32)
                    .h(height as i32)
                    .build(),
            )
            .await
            .map_err(|e| DomainError::DockerApi(format!("Resize exec: {e}")))
    }

    async fn inspect_container(&self, id: &str) -> DomainResult<String> {
        let docker = self.get_docker().await?;
        let info = docker
            .inspect_container(id, None)
            .await
            .map_err(|e| DomainError::DockerApi(format!("Inspect container: {e}")))?;
        serde_json::to_string_pretty(&info)
            .map_err(|e| DomainError::Serialization(format!("Format error: {e}")))
    }

    async fn container_stats(
        &self,
        id: &str,
    ) -> DomainResult<Box<dyn Stream<Item = DomainResult<ContainerStats>> + Unpin + Send>> {
        let docker = self.get_docker().await?;
        let opts = StatsOptions {
            stream: false,
            one_shot: true,
        };
        let stream = docker.stats(id, Some(opts));

        Ok(Box::new(stream.map(|r| {
            r.map_err(|e| DomainError::DockerApi(format!("Stats: {e}")))
                .map(|stat| {
                    let cpu_d = stat
                        .cpu_stats
                        .as_ref()
                        .and_then(|c| c.cpu_usage.as_ref())
                        .map(|u| u.total_usage.unwrap_or_default())
                        .unwrap_or_default();
                    let sys_d = stat
                        .precpu_stats
                        .as_ref()
                        .and_then(|c| c.cpu_usage.as_ref())
                        .map(|u| u.total_usage.unwrap_or_default())
                        .unwrap_or_default();
                    let cpu = if sys_d > 0 {
                        (cpu_d as f64) / (sys_d as f64) * 100.0
                    } else {
                        0.0
                    };
                    let mem = stat
                        .memory_stats
                        .as_ref()
                        .and_then(|m| m.usage)
                        .unwrap_or_default();
                    let mem_l = stat
                        .memory_stats
                        .as_ref()
                        .and_then(|m| m.limit)
                        .unwrap_or_default();
                    let nrx = stat
                        .networks
                        .as_ref()
                        .and_then(|n| n.values().next())
                        .and_then(|x| x.rx_bytes)
                        .unwrap_or_default();
                    let ntx = stat
                        .networks
                        .as_ref()
                        .and_then(|n| n.values().next())
                        .and_then(|x| x.tx_bytes)
                        .unwrap_or_default();
                    let br = stat
                        .blkio_stats
                        .as_ref()
                        .and_then(|b| b.io_service_bytes_recursive.as_ref())
                        .map(|v| {
                            v.iter()
                                .filter(|e| e.op.as_deref() == Some("read"))
                                .map(|e| e.value.unwrap_or_default())
                                .sum::<u64>()
                        })
                        .unwrap_or_default();
                    let bw = stat
                        .blkio_stats
                        .as_ref()
                        .and_then(|b| b.io_service_bytes_recursive.as_ref())
                        .map(|v| {
                            v.iter()
                                .filter(|e| e.op.as_deref() == Some("write"))
                                .map(|e| e.value.unwrap_or_default())
                                .sum::<u64>()
                        })
                        .unwrap_or_default();
                    ContainerStats {
                        cpu_percent: (cpu * 100.0).round() / 100.0,
                        memory_usage: super::images::format_bytes(mem as i64),
                        memory_usage_bytes: mem,
                        memory_limit_bytes: mem_l,
                        network_rx: super::images::format_bytes(nrx as i64),
                        network_tx: super::images::format_bytes(ntx as i64),
                        block_read: super::images::format_bytes(br as i64),
                        block_write: super::images::format_bytes(bw as i64),
                        pids: stat
                            .pids_stats
                            .as_ref()
                            .and_then(|p| p.current)
                            .unwrap_or_default(),
                    }
                })
        })))
    }
}

fn parse_container_state(s: &str) -> ContainerState {
    match s {
        "running" => ContainerState::Running,
        "exited" => ContainerState::Exited,
        "paused" => ContainerState::Paused,
        "restarting" => ContainerState::Restarting,
        "created" => ContainerState::Created,
        "removing" => ContainerState::Removing,
        "dead" => ContainerState::Dead,
        _ => ContainerState::Created,
    }
}

fn format_created(unix: i64) -> String {
    chrono::DateTime::from_timestamp(unix, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::{validate_container_create_config, validate_container_name};
    use domain::entities::{ContainerConfig, PortMapping};

    #[test]
    fn valid_names() {
        assert!(validate_container_name("nginx").is_ok());
        assert!(validate_container_name("my-app").is_ok());
        assert!(validate_container_name("web_server.prod").is_ok());
        assert!(validate_container_name("_internal").is_ok());
        assert!(validate_container_name(".hidden").is_ok());
        assert!(validate_container_name("a").is_ok());
        assert!(validate_container_name("A_b-C.D").is_ok());
        assert!(validate_container_name("abc123").is_ok());
    }

    #[test]
    fn empty_name_accepted() {
        // Empty name is allowed — Docker auto-generates one
        assert!(validate_container_name("").is_ok());
    }

    #[test]
    fn null_byte_rejected() {
        let result = validate_container_name("web\0hidden");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("null byte"));
    }

    #[test]
    fn too_long_rejected() {
        let long = "a".repeat(256);
        assert!(validate_container_name(&long).is_err());
        let ok = "a".repeat(255);
        assert!(validate_container_name(&ok).is_ok());
    }

    #[test]
    fn invalid_first_character() {
        let result = validate_container_name("-badstart");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("must start with"));

        let result = validate_container_name("$invalid");
        assert!(result.is_err());
    }

    #[test]
    fn invalid_characters() {
        let result = validate_container_name("my container");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid character"));

        let result = validate_container_name("web/app");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid character"));

        let result = validate_container_name("db@host");
        assert!(result.is_err());
    }

    #[test]
    fn position_reported() {
        let result = validate_container_name("abc def");
        let err = result.unwrap_err().to_string();
        assert!(err.contains("position 3"));
    }

    #[test]
    fn unicode_rejected() {
        // Docker names are ASCII-only
        let result = validate_container_name("café");
        assert!(result.is_err());
    }

    #[test]
    fn empty_image_rejected() {
        let config = ContainerConfig::default();
        let err = validate_container_create_config(&config).unwrap_err();
        assert!(err.to_string().contains("image cannot be empty"));
    }

    #[test]
    fn invalid_port_mapping_rejected() {
        let config = ContainerConfig {
            image: "nginx:latest".into(),
            port_mappings: vec![PortMapping {
                host_ip: String::new(),
                host_port: "8080".into(),
                container_port: String::new(),
                protocol: "tcp".into(),
            }],
            ..Default::default()
        };
        let err = validate_container_create_config(&config).unwrap_err();
        assert!(err.to_string().contains("Container port cannot be empty"));
    }

    #[test]
    fn unsupported_protocol_rejected() {
        let config = ContainerConfig {
            image: "nginx:latest".into(),
            port_mappings: vec![PortMapping {
                host_ip: String::new(),
                host_port: "8080".into(),
                container_port: "80".into(),
                protocol: "icmp".into(),
            }],
            ..Default::default()
        };
        let err = validate_container_create_config(&config).unwrap_err();
        assert!(err
            .to_string()
            .contains("Unsupported port mapping protocol"));
    }
}
