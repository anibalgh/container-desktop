use async_trait::async_trait;
use domain::entities::{LogLine, LogStream};
use domain::repository::ComposeRepository;
use domain::{DomainError, DomainResult};
use futures::Stream;
use std::path::{Path, PathBuf};
use tokio::process::Command;

pub struct ComposeClient;

impl ComposeClient {
    pub fn new() -> Self {
        Self
    }

    /// Returns the Docker Compose command to use.
    ///
    /// Uses `docker compose` (plugin, available since Docker 20.10+)
    /// which is the modern standard across all platforms.
    fn make_compose_command() -> Command {
        let mut cmd = Command::new("docker");
        cmd.arg("compose");
        cmd
    }
}

/// Validates a user-supplied compose file path to prevent path traversal attacks.
///
/// Returns the canonicalized path if valid, or a `DomainError` if the path is
/// suspicious (contains `..` traversal, doesn't exist, isn't a .yml/.yaml file, etc.).
fn validate_compose_path(file_path: &str) -> DomainResult<PathBuf> {
    let path = Path::new(file_path);

    // Reject empty paths
    if file_path.is_empty() {
        return Err(DomainError::Config(
            "Compose file path cannot be empty".to_string(),
        ));
    }

    // Reject paths with null bytes (used to bypass extension checks)
    if file_path.contains('\0') {
        return Err(DomainError::Config(
            "Compose file path contains null byte".to_string(),
        ));
    }

    // Check for path traversal sequences before canonicalization
    // (canonicalization resolves symlinks but we also want to catch raw `..`)
    let raw_str = path.to_string_lossy();
    if raw_str.contains("..") {
        return Err(DomainError::Config(
            "Path traversal detected in compose file path".to_string(),
        ));
    }

    // Verify the file exists and is a regular file
    let metadata = std::fs::metadata(path).map_err(|e| {
        DomainError::Config(format!("Cannot access compose file: {e}"))
    })?;

    if !metadata.is_file() {
        return Err(DomainError::Config(
            "Compose file path is not a regular file".to_string(),
        ));
    }

    // Restrict to .yml / .yaml extensions
    let valid_extensions = ["yml", "yaml"];
    let has_valid_extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| valid_extensions.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false);

    if !has_valid_extension {
        return Err(DomainError::Config(
            "Compose file must have .yml or .yaml extension".to_string(),
        ));
    }

    // Verify the file size is reasonable (prevent OOM on huge files)
    let file_size = metadata.len();
    const MAX_COMPOSE_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10 MB
    if file_size > MAX_COMPOSE_FILE_SIZE {
        return Err(DomainError::Config(format!(
            "Compose file too large ({} bytes, max {} bytes)",
            file_size, MAX_COMPOSE_FILE_SIZE
        )));
    }

    // Canonicalize to resolve symlinks and relative paths
    let canonical = path.canonicalize().map_err(|e| {
        DomainError::Config(format!("Cannot resolve compose file path: {e}"))
    })?;

    Ok(canonical)
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
        let canonical = validate_compose_path(file_path)?;
        run_compose_command(canonical, &["up", "-d"]).await
    }

    async fn compose_down(&self, file_path: &str) -> DomainResult<()> {
        let canonical = validate_compose_path(file_path)?;
        let file_str = canonical.to_string_lossy();
        let dir = canonical.parent().unwrap_or_else(|| Path::new("."));
        let output = Self::make_compose_command()
            .args(["-f", &file_str, "down"])
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
        let canonical = validate_compose_path(file_path)?;
        run_compose_command(canonical, &["logs", "-f", "--no-color"]).await
    }

    async fn compose_ps(&self, file_path: &str) -> DomainResult<Vec<String>> {
        let canonical = validate_compose_path(file_path)?;
        let file_str = canonical.to_string_lossy();
        let dir = canonical.parent().unwrap_or_else(|| Path::new("."));
        let output = Self::make_compose_command()
            .args(["-f", &file_str, "ps", "--no-trunc"])
            .current_dir(dir)
            .output()
            .await
            .map_err(|e| DomainError::Compose(format!("Failed: {e}")))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().map(|l| l.to_string()).collect())
    }
}

async fn run_compose_command(
    canonical_path: PathBuf,
    args: &[&str],
) -> DomainResult<Box<dyn Stream<Item = DomainResult<LogLine>> + Unpin + Send>> {
    let dir = canonical_path.parent().unwrap_or_else(|| Path::new("."));
    let file_str = canonical_path.to_string_lossy();

    let mut cmd = ComposeClient::make_compose_command();
    cmd.arg("-f").arg(file_str.as_ref());
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

#[cfg(test)]
mod tests {
    use super::validate_compose_path;
    use std::fs;
    use std::io::Write;

    fn create_temp_yml(name: &str, content: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir();
        let path = dir.join(name);
        let mut f = fs::File::create(&path).expect("Failed to create temp file");
        f.write_all(content.as_bytes())
            .expect("Failed to write temp file");
        path
    }

    #[test]
    fn valid_yml_path() {
        let p = create_temp_yml("test_valid.yml", "version: '3'\nservices:\n  web:\n    image: nginx\n");
        let result = validate_compose_path(p.to_str().unwrap());
        p.parent().map(|_| fs::remove_file(&p).ok());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_yaml_extension() {
        let p = create_temp_yml("test_valid.yaml", "version: '3'\n");
        let result = validate_compose_path(p.to_str().unwrap());
        fs::remove_file(&p).ok();
        assert!(result.is_ok());
    }

    #[test]
    fn empty_path_rejected() {
        let result = validate_compose_path("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("cannot be empty"));
    }

    #[test]
    fn null_byte_rejected() {
        let result = validate_compose_path("file.yml\0hidden");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("null byte"));
    }

    #[test]
    fn path_traversal_rejected() {
        let result = validate_compose_path("../etc/passwd");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Path traversal"));

        let result = validate_compose_path("sub/../../../root.yml");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Path traversal"));
    }

    #[test]
    fn nonexistent_file_rejected() {
        let path = std::env::temp_dir().join("nonexistent_compose_file_test.yml");
        // Ensure it doesn't exist
        std::fs::remove_file(&path).ok();
        let result = validate_compose_path(path.to_str().unwrap());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Cannot access"));
    }

    #[test]
    fn wrong_extension_rejected() {
        let p = create_temp_yml("test.txt", "not a compose file");
        let result = validate_compose_path(p.to_str().unwrap());
        fs::remove_file(&p).ok();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains(".yml or .yaml"));
    }

    #[test]
    fn no_extension_rejected() {
        let p = create_temp_yml("testfile", "no extension");
        let result = validate_compose_path(p.to_str().unwrap());
        fs::remove_file(&p).ok();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains(".yml or .yaml"));
    }

    #[test]
    fn directory_not_file_rejected() {
        let dir = std::env::temp_dir().join("compose_test_dir.yml");
        fs::create_dir_all(&dir).ok();
        let result = validate_compose_path(dir.to_str().unwrap());
        fs::remove_dir_all(&dir).ok();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("not a regular file"));
    }

    #[test]
    fn yml_extension_case_insensitive() {
        let p = create_temp_yml("test_case.YML", "version: '3'\n");
        let result = validate_compose_path(p.to_str().unwrap());
        fs::remove_file(&p).ok();
        assert!(result.is_ok());
        let p = create_temp_yml("test_case.Yaml", "version: '3'\n");
        let result = validate_compose_path(p.to_str().unwrap());
        fs::remove_file(&p).ok();
        assert!(result.is_ok());
    }
}
