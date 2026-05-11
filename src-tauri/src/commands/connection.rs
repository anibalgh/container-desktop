use domain::entities::DockerEndpoint;
use domain::repository::DockerConnectionRepository;
use tauri::State;

use crate::AppState;

#[tauri::command]
pub async fn connect(
    _state: State<'_, AppState>,
    endpoint: DockerEndpoint,
) -> Result<domain::entities::DockerInfo, String> {
    // Create a new client for the given endpoint
    let client = infrastructure::DockerClient::new(endpoint);
    client.connect().await.map_err(|e| e.to_string())?;
    client.test_connection().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_connection(
    state: State<'_, AppState>,
) -> Result<domain::entities::DockerInfo, String> {
    state
        .docker_client
        .test_connection()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ping(state: State<'_, AppState>) -> Result<bool, String> {
    match state.docker_client.ping().await {
        Ok(()) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}
