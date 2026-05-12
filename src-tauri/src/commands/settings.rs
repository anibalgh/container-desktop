use domain::entities::AppSettings;
use domain::repository::SettingsRepository;
use tauri::State;

use crate::AppState;

#[tauri::command]
pub async fn load_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    state.config_manager.load_settings().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings(state: State<'_, AppState>, settings: AppSettings) -> Result<(), String> {
    state.config_manager.save_settings(&settings).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_fonts() -> Result<Vec<String>, String> {
    let output = std::process::Command::new("fc-list")
        .args([":spacing=mono", "family"])
        .output()
        .map_err(|e| format!("Failed to run fc-list: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let fonts: Vec<String> = stdout
        .lines()
        .filter_map(|line| line.split(',').next())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && !s.starts_with('.'))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    if fonts.is_empty() {
        return Ok(vec!["Monospace".to_string()]);
    }
    Ok(fonts)
}
