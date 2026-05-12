use domain::entities::Volume;
use domain::repository::VolumeRepository;
use tauri::State;

use crate::AppState;
use super::validate_docker_id;

#[tauri::command]
pub async fn list_volumes(state: State<'_, AppState>) -> Result<Vec<Volume>, String> {
    state
        .docker_client
        .list_volumes()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_volume(
    state: State<'_, AppState>,
    name: String,
) -> Result<Volume, String> {
    validate_docker_id(&name, "Volume")?;
    state
        .docker_client
        .create_volume(&name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_volume(state: State<'_, AppState>, name: String) -> Result<(), String> {
    validate_docker_id(&name, "Volume")?;
    state
        .docker_client
        .remove_volume(&name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn inspect_volume(
    state: State<'_, AppState>,
    name: String,
) -> Result<String, String> {
    validate_docker_id(&name, "Volume")?;
    state
        .docker_client
        .inspect_volume(&name)
        .await
        .map_err(|e| e.to_string())
}
