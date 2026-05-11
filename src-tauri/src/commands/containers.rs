use domain::entities::Container;
use domain::repository::ContainerRepository;
use futures::StreamExt;
use tauri::{Emitter, State};

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
    state
        .docker_client
        .start_container(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_container(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state
        .docker_client
        .stop_container(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn restart_container(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state
        .docker_client
        .restart_container(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_container(state: State<'_, AppState>, id: String) -> Result<(), String> {
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
) -> Result<(), String> {
    let stream = state
        .docker_client
        .container_logs(&id, tail, follow)
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
pub async fn inspect_container(
    state: State<'_, AppState>,
    id: String,
) -> Result<String, String> {
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
    let mut stream = state
        .docker_client
        .container_stats(&id)
        .await
        .map_err(|e| e.to_string())?;

    use futures::StreamExt;
    match stream.next().await {
        Some(Ok(stats)) => Ok(stats),
        Some(Err(e)) => Err(e.to_string()),
        None => Err("No stats available".to_string()),
    }
}
