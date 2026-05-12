use domain::repository::ComposeRepository;
use futures::StreamExt;
use infrastructure::ComposeClient;
use tauri::Emitter;

use super::{LogStreamEvent, StreamStatus, StreamStatusEvent};

#[tauri::command]
pub async fn compose_up(
    app: tauri::AppHandle,
    file_path: String,
    request_id: String,
) -> Result<(), String> {
    let client = ComposeClient::new();
    let stream = client
        .compose_up(&file_path)
        .await
        .map_err(|e| e.to_string())?;

    app.emit(
        "compose-status",
        StreamStatusEvent {
            request_id: request_id.clone(),
            status: StreamStatus::Started,
            error: None,
        },
    )
    .map_err(|e| e.to_string())?;

    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        futures::pin_mut!(stream);
        let mut failed = None;
        while let Some(result) = stream.next().await {
            match result {
                Ok(log_line) => {
                    if app_clone
                        .emit(
                            "compose-output",
                            LogStreamEvent {
                                request_id: request_id.clone(),
                                line: log_line,
                            },
                        )
                        .is_err()
                    {
                        break;
                    }
                }
                Err(err) => {
                    failed = Some(err.to_string());
                    break;
                }
            }
        }

        let _ = app_clone.emit(
            "compose-status",
            StreamStatusEvent {
                request_id,
                status: if failed.is_some() {
                    StreamStatus::Failed
                } else {
                    StreamStatus::Completed
                },
                error: failed,
            },
        );
    });

    Ok(())
}

#[tauri::command]
pub async fn compose_down(file_path: String) -> Result<(), String> {
    let client = ComposeClient::new();
    client
        .compose_down(&file_path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn compose_logs(
    app: tauri::AppHandle,
    file_path: String,
    request_id: String,
) -> Result<(), String> {
    let client = ComposeClient::new();
    let stream = client
        .compose_logs(&file_path)
        .await
        .map_err(|e| e.to_string())?;

    app.emit(
        "compose-status",
        StreamStatusEvent {
            request_id: request_id.clone(),
            status: StreamStatus::Started,
            error: None,
        },
    )
    .map_err(|e| e.to_string())?;

    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        futures::pin_mut!(stream);
        let mut failed = None;
        while let Some(result) = stream.next().await {
            match result {
                Ok(log_line) => {
                    if app_clone
                        .emit(
                            "compose-output",
                            LogStreamEvent {
                                request_id: request_id.clone(),
                                line: log_line,
                            },
                        )
                        .is_err()
                    {
                        break;
                    }
                }
                Err(err) => {
                    failed = Some(err.to_string());
                    break;
                }
            }
        }

        let _ = app_clone.emit(
            "compose-status",
            StreamStatusEvent {
                request_id,
                status: if failed.is_some() {
                    StreamStatus::Failed
                } else {
                    StreamStatus::Completed
                },
                error: failed,
            },
        );
    });

    Ok(())
}

#[tauri::command]
pub async fn compose_ps(file_path: String) -> Result<Vec<String>, String> {
    let client = ComposeClient::new();
    client
        .compose_ps(&file_path)
        .await
        .map_err(|e| e.to_string())
}
