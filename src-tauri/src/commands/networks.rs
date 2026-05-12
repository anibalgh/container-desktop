use domain::entities::Network;
use domain::repository::NetworkRepository;
use tauri::State;

use crate::AppState;
use super::validate_docker_id;

#[tauri::command]
pub async fn list_networks(state: State<'_, AppState>) -> Result<Vec<Network>, String> {
    state
        .docker_client
        .list_networks()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_network(
    state: State<'_, AppState>,
    name: String,
    driver: Option<String>,
) -> Result<String, String> {
    validate_docker_id(&name, "Network")?;
    state
        .docker_client
        .create_network(&name, driver.as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_network(state: State<'_, AppState>, id: String) -> Result<(), String> {
    validate_docker_id(&id, "Network")?;
    state
        .docker_client
        .remove_network(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn inspect_network(
    state: State<'_, AppState>,
    id: String,
) -> Result<String, String> {
    validate_docker_id(&id, "Network")?;
    state
        .docker_client
        .inspect_network(&id)
        .await
        .map_err(|e| e.to_string())
}
