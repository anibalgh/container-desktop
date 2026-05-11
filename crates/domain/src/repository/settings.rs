use async_trait::async_trait;

use crate::entities::AppSettings;
use crate::error::DomainResult;

/// Repository for persisting and loading application settings.
#[async_trait]
pub trait SettingsRepository: Send + Sync {
    /// Load settings from persistent storage.
    async fn load_settings(&self) -> DomainResult<AppSettings>;

    /// Save settings to persistent storage.
    async fn save_settings(&self, settings: &AppSettings) -> DomainResult<()>;
}
