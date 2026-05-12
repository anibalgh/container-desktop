use domain::entities::{Container, ExecId};
use domain::repository::ContainerRepository;
use futures::StreamExt;
use tauri::{Emitter, State};

use super::validate_docker_id;
use crate::AppState;

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
    tail: Option<u32>,
    follow: bool,
    since: Option<i32>,
    until: Option<i32>,
) -> Result<(), String> {
    validate_docker_id(&id, "Container")?;
    let stream = state
        .docker_client
        .container_logs(&id, tail, follow, since, until)
        .await
        .map_err(|e| e.to_string())?;

    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        futures::pin_mut!(stream);
        while let Some(result) = stream.next().await {
            match result {
                Ok(log_line) => {
                    if app_clone.emit("container-log-line", &log_line).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
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
    let exec_id = state
        .docker_client
        .create_exec(&id, &cmd)
        .await
        .map_err(|e| e.to_string())?;
    Ok(exec_id.0)
}

#[tauri::command]
pub async fn exec_start(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    exec_id_str: String,
) -> Result<(), String> {
    validate_docker_id(&exec_id_str, "Exec")?;
    let exec_id = ExecId(exec_id_str.clone());
    let session = state
        .docker_client
        .start_exec_interactive(&exec_id)
        .await
        .map_err(|e| e.to_string())?;

    // Split: move output stream to background task, keep input writer in state
    let mut output = session.output;
    let input = session.input;
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        while let Some(result) = output.next().await {
            match result {
                Ok(out) => {
                    let text = String::from_utf8_lossy(&out.data).to_string();
                    if app_clone.emit("exec-output", &text).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
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
    let mut inputs = state.exec_inputs.lock().await;
    if let Some(writer) = inputs.get_mut(&exec_id_str) {
        writer.write_all(&data).await.map_err(|e| e.to_string())?;
    }
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
