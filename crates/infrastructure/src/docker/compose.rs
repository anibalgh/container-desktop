use async_trait::async_trait;
use domain::entities::{LogLine, LogStream};
use domain::repository::ComposeRepository;
use domain::{DomainError, DomainResult};
use futures::Stream;
use std::path::Path;
use tokio::process::Command;

pub struct ComposeClient;

impl ComposeClient {
    pub fn new() -> Self {
        Self
    }
    fn compose_bin() -> &'static str {
        "docker-compose"
    }
}

#[async_trait]
impl ComposeRepository for ComposeClient {
    async fn list_stacks(&self) -> DomainResult<Vec<domain::entities::ComposeStack>> {
        Ok(Vec::new())
    }

    async fn compose_up(
        &self,
        file_path: &str,
    ) -> DomainResult<Box<dyn Stream<Item = DomainResult<LogLine>> + Unpin + Send>> {
        run_compose_command(file_path, &["up", "-d"]).await
    }

    async fn compose_down(&self, file_path: &str) -> DomainResult<()> {
        let path = Path::new(file_path);
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let output = Command::new(Self::compose_bin())
            .args(["-f", file_path, "down"])
            .current_dir(dir)
            .output()
            .await
            .map_err(|e| DomainError::Compose(format!("Failed: {e}")))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(DomainError::Compose(format!(
                "compose down failed: {stderr}"
            )));
        }
        Ok(())
    }

    async fn compose_logs(
        &self,
        file_path: &str,
    ) -> DomainResult<Box<dyn Stream<Item = DomainResult<LogLine>> + Unpin + Send>> {
        run_compose_command(file_path, &["logs", "-f", "--no-color"]).await
    }

    async fn compose_ps(&self, file_path: &str) -> DomainResult<Vec<String>> {
        let path = Path::new(file_path);
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let output = Command::new(Self::compose_bin())
            .args(["-f", file_path, "ps", "--no-trunc"])
            .current_dir(dir)
            .output()
            .await
            .map_err(|e| DomainError::Compose(format!("Failed: {e}")))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().map(|l| l.to_string()).collect())
    }
}

async fn run_compose_command(
    file_path: &str,
    args: &[&str],
) -> DomainResult<Box<dyn Stream<Item = DomainResult<LogLine>> + Unpin + Send>> {
    let path = Path::new(file_path);
    let dir = path.parent().unwrap_or_else(|| Path::new("."));

    let mut cmd = Command::new(ComposeClient::compose_bin());
    cmd.arg("-f").arg(file_path);
    for a in args {
        cmd.arg(a);
    }
    cmd.current_dir(dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    let mut child = cmd
        .spawn()
        .map_err(|e| DomainError::Compose(format!("Failed to spawn compose: {e}")))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| DomainError::Compose("No stdout".to_string()))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| DomainError::Compose("No stderr".to_string()))?;

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let tx2 = tx.clone();

    tokio::spawn(async move {
        use tokio::io::AsyncBufReadExt;
        let mut reader = tokio::io::BufReader::new(stdout);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    if tx
                        .send(Ok(LogLine {
                            stream: LogStream::Stdout,
                            content: line.trim_end().to_string(),
                            timestamp: None,
                        }))
                        .is_err()
                    {
                        break;
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(DomainError::Compose(format!("Read error: {e}"))));
                    break;
                }
            }
        }
    });

    tokio::spawn(async move {
        use tokio::io::AsyncBufReadExt;
        let mut reader = tokio::io::BufReader::new(stderr);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    if tx2
                        .send(Ok(LogLine {
                            stream: LogStream::Stderr,
                            content: line.trim_end().to_string(),
                            timestamp: None,
                        }))
                        .is_err()
                    {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    Ok(Box::new(
        tokio_stream::wrappers::UnboundedReceiverStream::new(rx),
    ))
}
