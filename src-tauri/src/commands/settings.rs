use domain::entities::AppSettings;
use domain::repository::SettingsRepository;
use tauri::State;

use crate::AppState;

#[tauri::command]
pub async fn load_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    state
        .config_manager
        .load_settings()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings(
    state: State<'_, AppState>,
    settings: AppSettings,
) -> Result<(), String> {
    state
        .config_manager
        .save_settings(&settings)
        .await
        .map_err(|e| e.to_string())
}
