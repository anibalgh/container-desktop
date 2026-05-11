use async_trait::async_trait;
use directories::ProjectDirs;
use domain::entities::AppSettings;
use domain::repository::SettingsRepository;
use domain::{DomainError, DomainResult};
use std::path::PathBuf;

/// Manages application settings via JSON files in the user config directory.
pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    /// Creates a new `ConfigManager` using the platform-appropriate config directory.
    pub fn new() -> DomainResult<Self> {
        let proj_dirs = ProjectDirs::from("com", "container-desktop", "ContainerDesktop")
            .ok_or_else(|| {
                DomainError::Config("Could not determine config directory".to_string())
            })?;

        let config_dir = proj_dirs.config_dir().to_path_buf();
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| DomainError::Config(format!("Failed to create config dir: {e}")))?;

        let config_path = config_dir.join("settings.json");

        Ok(Self { config_path })
    }

    /// Returns the path to the settings file.
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }
}

#[async_trait]
impl SettingsRepository for ConfigManager {
    /// Loads settings from the JSON config file, or returns defaults if not found.
    async fn load_settings(&self) -> DomainResult<AppSettings> {
        if !self.config_path.exists() {
            let defaults = AppSettings::default();
            self.save_settings(&defaults).await?;
            return Ok(defaults);
        }

        let content = tokio::fs::read_to_string(&self.config_path)
            .await
            .map_err(|e| DomainError::Config(format!("Cannot read settings file: {e}")))?;

        let settings: AppSettings = serde_json::from_str(&content)
            .map_err(|e| DomainError::Serialization(format!("Invalid settings JSON: {e}")))?;

        Ok(settings)
    }

    /// Saves settings to the JSON config file.
    async fn save_settings(&self, settings: &AppSettings) -> DomainResult<()> {
        let content = serde_json::to_string_pretty(settings)
            .map_err(|e| DomainError::Serialization(format!("Cannot serialize settings: {e}")))?;

        tokio::fs::write(&self.config_path, content)
            .await
            .map_err(|e| DomainError::Config(format!("Cannot write settings file: {e}")))?;

        Ok(())
    }
}
