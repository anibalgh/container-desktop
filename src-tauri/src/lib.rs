mod commands;

use std::collections::HashMap;
use std::sync::Arc;

use tauri::{Emitter, Manager};
use tokio::sync::Mutex;

use domain::repository::{DockerConnectionRepository, SettingsRepository};
use infrastructure::{ConfigManager, DockerClient, SecurityService};

type ExecInputWriter = Arc<Mutex<Box<dyn tokio::io::AsyncWrite + Unpin + Send>>>;

pub struct AppState {
    pub docker_client: Arc<DockerClient>,
    pub config_manager: Arc<ConfigManager>,
    pub security_service: Arc<SecurityService>,
    pub exec_inputs: Arc<Mutex<HashMap<String, ExecInputWriter>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let config_manager =
                Arc::new(ConfigManager::new().expect("failed to create config manager"));

            let settings =
                tauri::async_runtime::block_on(config_manager.load_settings()).unwrap_or_default();

            let docker_client = Arc::new(DockerClient::new(settings.endpoint.clone()));
            let security_service = Arc::new(SecurityService::new(
                docker_client.clone(),
                config_manager
                    .security_dir()
                    .expect("failed to create security data dir"),
            ));

            let state = AppState {
                docker_client: docker_client.clone(),
                config_manager: config_manager.clone(),
                security_service: security_service.clone(),
                exec_inputs: Arc::new(Mutex::new(HashMap::new())),
            };

            app.manage(state);

            let handle = app.handle().clone();
            let dc = docker_client.clone();
            let security = security_service.clone();
            let selected_tools = settings.security.selected_tools.clone();
            tauri::async_runtime::spawn(async move {
                let connected = if let Err(e) = dc.connect().await {
                    tracing::warn!("Initial Docker connection failed: {e}");
                    false
                } else {
                    true
                };
                match dc.test_connection().await {
                    Ok(info) => {
                        let _ = handle.emit("docker-connected", info);
                    }
                    Err(e) => {
                        let _ = handle.emit("docker-error", e.to_string());
                    }
                }

                if connected && !selected_tools.is_empty() {
                    let event_handle = handle.clone();
                    let notifier = Arc::new(move |progress| {
                        let _ = event_handle.emit("security-scan-progress", progress);
                    });
                    if let Err(error) = security.schedule_scans(selected_tools, notifier).await {
                        tracing::warn!("Initial security scan scheduling failed: {error}");
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::connection::connect,
            commands::connection::test_connection,
            commands::connection::ping,
            commands::containers::list_containers,
            commands::containers::start_container,
            commands::containers::stop_container,
            commands::containers::restart_container,
            commands::containers::remove_container,
            commands::containers::container_logs,
            commands::containers::inspect_container,
            commands::containers::container_stats,
            commands::containers::exec_create,
            commands::containers::exec_start,
            commands::containers::exec_input,
            commands::containers::exec_resize,
            commands::images::list_images,
            commands::images::pull_image,
            commands::images::remove_image,
            commands::images::tag_image,
            commands::images::inspect_image,
            commands::security::security_overview,
            commands::security::image_security_report,
            commands::security::configure_security_tools,
            commands::security::open_external_link,
            commands::volumes::list_volumes,
            commands::volumes::create_volume,
            commands::volumes::remove_volume,
            commands::volumes::inspect_volume,
            commands::networks::list_networks,
            commands::networks::create_network,
            commands::networks::remove_network,
            commands::networks::inspect_network,
            commands::compose::compose_up,
            commands::compose::compose_down,
            commands::compose::compose_logs,
            commands::compose::compose_ps,
            commands::settings::load_settings,
            commands::settings::save_settings,
            commands::settings::list_fonts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
