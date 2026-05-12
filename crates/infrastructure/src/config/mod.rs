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

    /// Creates a `ConfigManager` pointed at a specific file path.
    /// Only available in tests to isolate from the real user config directory.
    #[cfg(test)]
    pub(crate) fn with_path(path: PathBuf) -> Self {
        Self { config_path: path }
    }

    /// Synchronously reads font family and font size from the settings file.
    ///
    /// Used at startup (before the async runtime is available) to configure
    /// the application's default font via `iced::Settings`.
    /// Returns defaults if the file is missing or unreadable.
    pub fn load_font_settings_sync(&self) -> (String, u16) {
        use domain::entities::AppSettings;

        let defaults = AppSettings::default();
        let (default_family, default_size) = (defaults.font_family, defaults.font_size);

        let content = match std::fs::read_to_string(&self.config_path) {
            Ok(c) => c,
            Err(_) => return (default_family, default_size),
        };

        let settings: AppSettings = match serde_json::from_str(&content) {
            Ok(s) => s,
            Err(_) => return (default_family, default_size),
        };

        (settings.font_family, settings.font_size)
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

#[cfg(test)]
mod tests {
    use super::*;
    use domain::entities::{AppSettings, LanguageSetting, ThemeSetting};

    fn test_dir() -> PathBuf {
        std::env::temp_dir().join("container_desktop_test")
    }

    #[tokio::test]
    async fn save_and_load_settings() {
        let dir = test_dir();
        std::fs::create_dir_all(&dir).ok();
        let path = dir.join("save_load_test.json");
        std::fs::remove_file(&path).ok();

        let mgr = ConfigManager::with_path(path.clone());

        let mut settings = AppSettings::default();
        settings.window_width = 1920;
        settings.window_height = 1080;
        settings.font_size = 18;

        mgr.save_settings(&settings).await.unwrap();
        let loaded = mgr.load_settings().await.unwrap();

        assert_eq!(loaded.window_width, 1920);
        assert_eq!(loaded.window_height, 1080);
        assert_eq!(loaded.font_size, 18);
        assert_eq!(loaded.language_setting, LanguageSetting::Auto);

        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    async fn load_returns_defaults_when_file_missing() {
        let dir = test_dir();
        std::fs::create_dir_all(&dir).ok();
        let path = dir.join("load_defaults_test.json");
        std::fs::remove_file(&path).ok();

        let mgr = ConfigManager::with_path(path.clone());
        let loaded = mgr.load_settings().await.unwrap();

        assert_eq!(loaded.window_width, 1280);
        assert_eq!(loaded.window_height, 800);
        assert_eq!(loaded.font_family, "Monospace");
        assert_eq!(loaded.font_size, 14);
        assert_eq!(loaded.theme_setting, ThemeSetting::Auto);
        assert_eq!(loaded.language_setting, LanguageSetting::Auto);

        // It should also create the file with defaults
        assert!(path.exists());
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    async fn load_font_settings_sync_with_valid_file() {
        let dir = test_dir();
        std::fs::create_dir_all(&dir).ok();
        let path = dir.join("font_sync_test.json");
        std::fs::remove_file(&path).ok();

        let mgr = ConfigManager::with_path(path.clone());

        let mut settings = AppSettings::default();
        settings.font_family = "Fira Code".into();
        settings.font_size = 16;
        mgr.save_settings(&settings).await.unwrap();

        let (family, size) = mgr.load_font_settings_sync();
        assert_eq!(family, "Fira Code");
        assert_eq!(size, 16);

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn load_font_settings_sync_missing_file_returns_defaults() {
        let path = test_dir().join("nonexistent_font_settings.json");
        std::fs::remove_file(&path).ok();

        let mgr = ConfigManager::with_path(path);
        let (family, size) = mgr.load_font_settings_sync();
        assert_eq!(family, "Monospace");
        assert_eq!(size, 14);
    }

    #[test]
    fn load_font_settings_sync_invalid_json_returns_defaults() {
        let dir = test_dir();
        std::fs::create_dir_all(&dir).ok();
        let path = dir.join("invalid_json_test.json");
        std::fs::write(&path, "not valid json").unwrap();

        let mgr = ConfigManager::with_path(path.clone());
        let (family, size) = mgr.load_font_settings_sync();
        assert_eq!(family, "Monospace");
        assert_eq!(size, 14);

        std::fs::remove_file(&path).ok();
    }
}
