use std::collections::BTreeSet;
use std::process::Command;
use std::sync::Arc;

use domain::entities::{ImageSecurityReport, SecurityOverview, SecurityTool};
use domain::repository::{SecurityRepository, SettingsRepository};
use tauri::{Emitter, State};

use super::validate_docker_id;
use crate::AppState;

fn normalize_tools(tools: Vec<SecurityTool>) -> Vec<SecurityTool> {
    tools
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[tauri::command]
pub async fn security_overview(state: State<'_, AppState>) -> Result<SecurityOverview, String> {
    let settings = state
        .config_manager
        .load_settings()
        .await
        .map_err(|e| e.to_string())?;

    state
        .security_service
        .security_overview(&settings.security.selected_tools)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn image_security_report(
    state: State<'_, AppState>,
    image_id: String,
) -> Result<ImageSecurityReport, String> {
    validate_docker_id(&image_id, "Image")?;
    state
        .security_service
        .image_security_report(&image_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn configure_security_tools(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    tools: Vec<SecurityTool>,
) -> Result<SecurityOverview, String> {
    let selected_tools = normalize_tools(tools);
    let mut settings = state
        .config_manager
        .load_settings()
        .await
        .map_err(|e| e.to_string())?;
    settings.security.selected_tools = selected_tools.clone();

    state
        .config_manager
        .save_settings(&settings)
        .await
        .map_err(|e| e.to_string())?;

    let notifier = Arc::new(move |progress| {
        let _ = app.emit("security-scan-progress", progress);
    });

    state
        .security_service
        .schedule_scans(selected_tools.clone(), notifier)
        .await
        .map_err(|e| e.to_string())?;

    state
        .security_service
        .security_overview(&selected_tools)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_external_link(url: String) -> Result<(), String> {
    validate_external_url(&url)?;
    open_url_in_browser(&url).map_err(|e| e.to_string())
}

fn validate_external_url(url: &str) -> Result<(), String> {
    if url.is_empty() {
        return Err("URL cannot be empty".into());
    }
    if url.contains('\0') {
        return Err("URL contains null byte".into());
    }
    const MAX_URL_LENGTH: usize = 4096;
    if url.len() > MAX_URL_LENGTH {
        return Err(format!(
            "URL too long ({} bytes, max {MAX_URL_LENGTH} bytes)",
            url.len()
        ));
    }
    let normalized = url.trim().to_ascii_lowercase();
    if !(normalized.starts_with("https://") || normalized.starts_with("http://")) {
        return Err("Only http:// and https:// URLs are allowed".into());
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn open_url_in_browser(url: &str) -> Result<(), std::io::Error> {
    let status = Command::new("xdg-open").arg(url).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "xdg-open exited with status {status}"
        )))
    }
}

#[cfg(target_os = "macos")]
fn open_url_in_browser(url: &str) -> Result<(), std::io::Error> {
    let status = Command::new("open").arg(url).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "open exited with status {status}"
        )))
    }
}

#[cfg(target_os = "windows")]
fn open_url_in_browser(url: &str) -> Result<(), std::io::Error> {
    let status = Command::new("cmd")
        .args(["/C", "start", "", url])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "start exited with status {status}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::validate_external_url;

    #[test]
    fn accepts_http_and_https_urls() {
        assert!(validate_external_url("https://example.com").is_ok());
        assert!(validate_external_url("http://example.com").is_ok());
    }

    #[test]
    fn rejects_non_http_schemes() {
        assert!(validate_external_url("file:///tmp/test").is_err());
        assert!(validate_external_url("javascript:alert(1)").is_err());
    }
}
