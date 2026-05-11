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
                                    Some(t) => format!("{t:?}")
                                        .to_lowercase()
                                        .trim_matches('"')
                                        .to_string(),
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
                                    Some(t) => format!("{t:?}")
                                        .to_lowercase()
                                        .trim_matches('"')
                                        .to_string(),
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
                        parse_container_state(
                            &format!("{s:?}")
                                .to_lowercase()
                                .trim_matches('"')
                                .to_string(),
                        )
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
                    created: c.created.map(|ts| format_created(ts)).unwrap_or_default(),
                    command: c.command.unwrap_or_default(),
                }
            })
            .collect();

        Ok(result)
    }

    async fn create_container(&self, config: &ContainerConfig) -> DomainResult<String> {
        let docker = self.get_docker().await?;

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
            for (cp, hp) in &config.port_mappings {
                pb.insert(
                    format!("{cp}/tcp"),
                    Some(vec![bollard::config::PortBinding {
                        host_ip: Some("0.0.0.0".to_string()),
                        host_port: Some(hp.clone()),
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

    async fn create_exec(&self, id: &str, cmd: &[String]) -> DomainResult<ExecId> {
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
            ..Default::default()
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
                        memory_limit_bytes: mem_l as u64,
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
