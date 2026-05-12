use domain::entities::{Container, ExecId};
use domain::repository::ContainerRepository;
use futures::StreamExt;
use serde::Deserialize;
use tauri::{Emitter, State};

use super::{validate_docker_id, LogStreamEvent, StreamStatus, StreamStatusEvent, TextStreamEvent};
use crate::AppState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerLogOptions {
    tail: Option<u32>,
    follow: bool,
    since: Option<i32>,
    until: Option<i32>,
    request_id: String,
}

#[tauri::command]
pub async fn list_containers(
    state: State<'_, AppState>,
    all: bool,
) -> Result<Vec<Container>, String> {
    state
        .docker_client
        .list_containers(all)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_container(state: State<'_, AppState>, id: String) -> Result<(), String> {
    validate_docker_id(&id, "Container")?;
    state
        .docker_client
        .start_container(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_container(state: State<'_, AppState>, id: String) -> Result<(), String> {
    validate_docker_id(&id, "Container")?;
    state
        .docker_client
        .stop_container(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn restart_container(state: State<'_, AppState>, id: String) -> Result<(), String> {
    validate_docker_id(&id, "Container")?;
    state
        .docker_client
        .restart_container(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_container(state: State<'_, AppState>, id: String) -> Result<(), String> {
    validate_docker_id(&id, "Container")?;
    state
        .docker_client
        .remove_container(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn container_logs(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    id: String,
    options: ContainerLogOptions,
) -> Result<(), String> {
    validate_docker_id(&id, "Container")?;
    let stream = state
        .docker_client
        .container_logs(
            &id,
            options.tail,
            options.follow,
            options.since,
            options.until,
        )
        .await
        .map_err(|e| e.to_string())?;

    app.emit(
        "container-log-status",
        StreamStatusEvent {
            request_id: options.request_id.clone(),
            status: StreamStatus::Started,
            error: None,
        },
    )
    .map_err(|e| e.to_string())?;

    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        futures::pin_mut!(stream);
        let mut failed = None;
        while let Some(result) = stream.next().await {
            match result {
                Ok(log_line) => {
                    if app_clone
                        .emit(
                            "container-log-line",
                            LogStreamEvent {
                                request_id: options.request_id.clone(),
                                line: log_line,
                            },
                        )
                        .is_err()
                    {
                        break;
                    }
                }
                Err(err) => {
                    failed = Some(err.to_string());
                    break;
                }
            }
        }

        let _ = app_clone.emit(
            "container-log-status",
            StreamStatusEvent {
                request_id: options.request_id,
                status: if failed.is_some() {
                    StreamStatus::Failed
                } else {
                    StreamStatus::Completed
                },
                error: failed,
            },
        );
    });

    Ok(())
}

#[tauri::command]
pub async fn inspect_container(state: State<'_, AppState>, id: String) -> Result<String, String> {
    validate_docker_id(&id, "Container")?;
    state
        .docker_client
        .inspect_container(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn container_stats(
    state: State<'_, AppState>,
    id: String,
) -> Result<domain::entities::ContainerStats, String> {
    validate_docker_id(&id, "Container")?;
    let mut stream = state
        .docker_client
        .container_stats(&id)
        .await
        .map_err(|e| e.to_string())?;
    match stream.next().await {
        Some(Ok(stats)) => Ok(stats),
        Some(Err(e)) => Err(e.to_string()),
        None => Err("No stats available".to_string()),
    }
}

#[tauri::command]
pub async fn exec_create(
    state: State<'_, AppState>,
    id: String,
    cmd: Vec<String>,
    user: Option<String>,
) -> Result<String, String> {
    validate_docker_id(&id, "Container")?;
    // Sanitize command arguments: reject empty commands and overly long arguments
    if cmd.is_empty() {
        return Err("Exec command cannot be empty".to_string());
    }
    for arg in &cmd {
        if arg.contains('\0') {
            return Err("Exec command argument contains null byte".to_string());
        }
        if arg.len() > 4096 {
            return Err("Exec command argument too long".to_string());
        }
    }

    let user = match user {
        Some(user) if user.is_empty() => return Err("Exec user cannot be empty".to_string()),
        Some(user) if user.contains('\0') => return Err("Exec user contains null byte".to_string()),
        Some(user) if user.len() > 256 => return Err("Exec user too long".to_string()),
        other => other,
    };
    let exec_id = state
        .docker_client
        .create_exec(&id, &cmd, user.as_deref())
        .await
        .map_err(|e| e.to_string())?;
    Ok(exec_id.0)
}

#[tauri::command]
pub async fn exec_start(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    exec_id_str: String,
    request_id: String,
) -> Result<(), String> {
    validate_docker_id(&exec_id_str, "Exec")?;
    let exec_id = ExecId(exec_id_str.clone());
    let session = state
        .docker_client
        .start_exec_interactive(&exec_id)
        .await
        .map_err(|e| e.to_string())?;

    app.emit(
        "exec-status",
        StreamStatusEvent {
            request_id: request_id.clone(),
            status: StreamStatus::Started,
            error: None,
        },
    )
    .map_err(|e| e.to_string())?;

    // Split: move output stream to background task, keep input writer in state
    let mut output = session.output;
    let input = std::sync::Arc::new(tokio::sync::Mutex::new(session.input));
    let app_clone = app.clone();
    let exec_inputs = state.exec_inputs.clone();
    let cleanup_exec_id = exec_id_str.clone();
    tauri::async_runtime::spawn(async move {
        let mut failed = None;
        while let Some(result) = output.next().await {
            match result {
                Ok(out) => {
                    let text = String::from_utf8_lossy(&out.data).to_string();
                    if app_clone
                        .emit(
                            "exec-output",
                            TextStreamEvent {
                                request_id: request_id.clone(),
                                text,
                            },
                        )
                        .is_err()
                    {
                        break;
                    }
                }
                Err(err) => {
                    failed = Some(err.to_string());
                    break;
                }
            }
        }

        exec_inputs.lock().await.remove(&cleanup_exec_id);

        let _ = app_clone.emit(
            "exec-status",
            StreamStatusEvent {
                request_id,
                status: if failed.is_some() {
                    StreamStatus::Failed
                } else {
                    StreamStatus::Completed
                },
                error: failed,
            },
        );
    });

    // Store just the input writer
    state.exec_inputs.lock().await.insert(exec_id_str, input);
    Ok(())
}

#[tauri::command]
pub async fn exec_input(
    state: State<'_, AppState>,
    exec_id_str: String,
    data: Vec<u8>,
) -> Result<(), String> {
    validate_docker_id(&exec_id_str, "Exec")?;
    // Limit input data size to prevent memory exhaustion
    const MAX_EXEC_INPUT_SIZE: usize = 65536; // 64 KB
    if data.len() > MAX_EXEC_INPUT_SIZE {
        return Err(format!(
            "Exec input too large ({} bytes, max {MAX_EXEC_INPUT_SIZE} bytes)",
            data.len()
        ));
    }
    use tokio::io::AsyncWriteExt;
    let writer = {
        let inputs = state.exec_inputs.lock().await;
        inputs.get(&exec_id_str).cloned()
    }
    .ok_or_else(|| format!("Exec session not found: {exec_id_str}"))?;

    let mut writer = writer.lock().await;
    writer.write_all(&data).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn exec_resize(
    state: State<'_, AppState>,
    exec_id_str: String,
    width: u16,
    height: u16,
) -> Result<(), String> {
    validate_docker_id(&exec_id_str, "Exec")?;
    let exec_id = ExecId(exec_id_str);
    state
        .docker_client
        .resize_exec(&exec_id, width, height)
        .await
        .map_err(|e| e.to_string())
}
