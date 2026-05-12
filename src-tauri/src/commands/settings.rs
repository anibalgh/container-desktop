use domain::entities::AppSettings;
use domain::repository::SettingsRepository;
use tauri::State;

use super::connection::validate_docker_endpoint;
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
    if state.docker_client.endpoint() != settings.endpoint {
        validate_docker_endpoint(&settings.endpoint)?;
        state
            .docker_client
            .reconfigure(settings.endpoint.clone())
            .await
            .map_err(|e| e.to_string())?;
    }

    state
        .config_manager
        .save_settings(&settings)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_fonts() -> Result<Vec<String>, String> {
    list_fonts_platform()
}

/// Platform-specific font listing.
///
/// - **Linux**: uses `fc-list` (fontconfig).
/// - **macOS**: uses `system_profiler SPFontsDataType`.
/// - **Windows**: returns a curated list of common monospace fonts
///   (DirectWrite enumeration would require winapi; CLI equivalents
///   are unreliable, so we ship a sensible default list).
#[cfg(target_os = "linux")]
fn list_fonts_platform() -> Result<Vec<String>, String> {
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

#[cfg(target_os = "macos")]
fn list_fonts_platform() -> Result<Vec<String>, String> {
    let output = std::process::Command::new("system_profiler")
        .args(["SPFontsDataType"])
        .output()
        .map_err(|e| format!("Failed to enumerate fonts: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut fonts: Vec<String> = stdout
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("Family:") {
                Some(trimmed.trim_start_matches("Family:").trim().to_string())
            } else {
                None
            }
        })
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    if fonts.is_empty() {
        fonts.push("Menlo".to_string());
        fonts.push("Monaco".to_string());
        fonts.push("SF Mono".to_string());
    }
    Ok(fonts)
}

#[cfg(target_os = "windows")]
fn list_fonts_platform() -> Result<Vec<String>, String> {
    // Windows has no reliable CLI for font enumeration without admin rights.
    // DirectWrite enumeration requires winapi — for now, return a curated list
    // of well-known monospace fonts available on most Windows installs.
    Ok(vec![
        "Cascadia Code".to_string(),
        "Cascadia Mono".to_string(),
        "Consolas".to_string(),
        "Courier New".to_string(),
        "Fira Code".to_string(),
        "Hack".to_string(),
        "JetBrains Mono".to_string(),
        "Lucida Console".to_string(),
        "Monospace".to_string(),
        "Source Code Pro".to_string(),
    ])
}
