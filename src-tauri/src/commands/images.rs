use domain::entities::Image;
use domain::repository::ImageRepository;
use futures::StreamExt;
use tauri::{Emitter, State};

use crate::AppState;
use super::validate_docker_id;

#[tauri::command]
pub async fn list_images(state: State<'_, AppState>) -> Result<Vec<Image>, String> {
    state
        .docker_client
        .list_images()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pull_image(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    name: String,
    tag: Option<String>,
) -> Result<(), String> {
    let stream = state
        .docker_client
        .pull_image(&name, tag.as_deref())
        .await
        .map_err(|e| e.to_string())?;

    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        futures::pin_mut!(stream);
        while let Some(result) = stream.next().await {
            match result {
                Ok(msg) => {
                    if app_clone.emit("image-pull-progress", &msg).is_err() {
                        break;
                    }
                }
                Err(e) => {
                    let _ = app_clone.emit("image-pull-progress", &format!("Error: {e}"));
                    break;
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn remove_image(state: State<'_, AppState>, id: String) -> Result<(), String> {
    validate_docker_id(&id, "Image")?;
    state
        .docker_client
        .remove_image(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn tag_image(
    state: State<'_, AppState>,
    id: String,
    repo: String,
    tag: String,
) -> Result<(), String> {
    validate_docker_id(&id, "Image")?;
    state
        .docker_client
        .tag_image(&id, &repo, &tag)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn inspect_image(state: State<'_, AppState>, id: String) -> Result<String, String> {
    validate_docker_id(&id, "Image")?;
    state
        .docker_client
        .inspect_image(&id)
        .await
        .map_err(|e| e.to_string())
}
