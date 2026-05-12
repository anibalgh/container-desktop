use serde::{Deserialize, Serialize};

use super::endpoint::DockerEndpoint;

/// All 23 built-in Iced theme variants.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThemeVariant {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

impl ThemeVariant {
    /// Returns the display name for this theme variant.
    pub fn display_name(&self) -> &'static str {
        match self {
            ThemeVariant::Light => "Light",
            ThemeVariant::Dark => "Dark",
            ThemeVariant::Dracula => "Dracula",
            ThemeVariant::Nord => "Nord",
            ThemeVariant::SolarizedLight => "Solarized Light",
            ThemeVariant::SolarizedDark => "Solarized Dark",
            ThemeVariant::GruvboxLight => "Gruvbox Light",
            ThemeVariant::GruvboxDark => "Gruvbox Dark",
            ThemeVariant::CatppuccinLatte => "Catppuccin Latte",
            ThemeVariant::CatppuccinFrappe => "Catppuccin Frappe",
            ThemeVariant::CatppuccinMacchiato => "Catppuccin Macchiato",
            ThemeVariant::CatppuccinMocha => "Catppuccin Mocha",
            ThemeVariant::TokyoNight => "Tokyo Night",
            ThemeVariant::TokyoNightStorm => "Tokyo Night Storm",
            ThemeVariant::TokyoNightLight => "Tokyo Night Light",
            ThemeVariant::KanagawaWave => "Kanagawa Wave",
            ThemeVariant::KanagawaDragon => "Kanagawa Dragon",
            ThemeVariant::KanagawaLotus => "Kanagawa Lotus",
            ThemeVariant::Moonfly => "Moonfly",
            ThemeVariant::Nightfly => "Nightfly",
            ThemeVariant::Oxocarbon => "Oxocarbon",
            ThemeVariant::Ferra => "Ferra",
        }
    }

    /// All theme variants.
    pub fn all() -> &'static [ThemeVariant] {
        &[
            ThemeVariant::Light,
            ThemeVariant::Dark,
            ThemeVariant::Dracula,
            ThemeVariant::Nord,
            ThemeVariant::SolarizedLight,
            ThemeVariant::SolarizedDark,
            ThemeVariant::GruvboxLight,
            ThemeVariant::GruvboxDark,
            ThemeVariant::CatppuccinLatte,
            ThemeVariant::CatppuccinFrappe,
            ThemeVariant::CatppuccinMacchiato,
            ThemeVariant::CatppuccinMocha,
            ThemeVariant::TokyoNight,
            ThemeVariant::TokyoNightStorm,
            ThemeVariant::TokyoNightLight,
            ThemeVariant::KanagawaWave,
            ThemeVariant::KanagawaDragon,
            ThemeVariant::KanagawaLotus,
            ThemeVariant::Moonfly,
            ThemeVariant::Nightfly,
            ThemeVariant::Oxocarbon,
            ThemeVariant::Ferra,
        ]
    }

    /// Returns whether this theme variant is dark.
    pub fn is_dark(&self) -> bool {
        matches!(
            self,
            ThemeVariant::Dark
                | ThemeVariant::Dracula
                | ThemeVariant::Nord
                | ThemeVariant::SolarizedDark
                | ThemeVariant::GruvboxDark
                | ThemeVariant::CatppuccinFrappe
                | ThemeVariant::CatppuccinMacchiato
                | ThemeVariant::CatppuccinMocha
                | ThemeVariant::TokyoNight
                | ThemeVariant::TokyoNightStorm
                | ThemeVariant::KanagawaWave
                | ThemeVariant::KanagawaDragon
                | ThemeVariant::Moonfly
                | ThemeVariant::Nightfly
                | ThemeVariant::Oxocarbon
                | ThemeVariant::Ferra
        )
    }
}

impl std::fmt::Display for ThemeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Theme mode setting: auto-detect or manual override.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThemeSetting {
    /// Automatically follow the OS theme preference.
    Auto,
    /// Manually pick a specific theme.
    Manual(ThemeVariant),
}

impl Default for ThemeSetting {
    fn default() -> Self {
        ThemeSetting::Auto
    }
}

/// Application settings persisted to user config directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// The theme mode configuration.
    pub theme_setting: ThemeSetting,
    /// The Docker endpoint configuration.
    pub endpoint: DockerEndpoint,
    /// Window width in pixels.
    pub window_width: u32,
    /// Window height in pixels.
    pub window_height: u32,
    /// Monospace font family name (e.g. "Fira Code", "JetBrains Mono").
    /// Use "Monospace" for the system default monospace font.
    #[serde(default = "default_font_family")]
    pub font_family: String,
    /// Base font size in pixels for the entire UI.
    #[serde(default = "default_font_size")]
    pub font_size: u16,
}

fn default_font_family() -> String {
    "Monospace".to_string()
}

fn default_font_size() -> u16 {
    14
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme_setting: ThemeSetting::default(),
            endpoint: DockerEndpoint::default(),
            window_width: 1280,
            window_height: 800,
            font_family: default_font_family(),
            font_size: default_font_size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── ThemeVariant tests ──

    #[test]
    fn theme_variant_display_names() {
        assert_eq!(ThemeVariant::Light.display_name(), "Light");
        assert_eq!(ThemeVariant::Dark.display_name(), "Dark");
        assert_eq!(ThemeVariant::Dracula.display_name(), "Dracula");
        assert_eq!(ThemeVariant::Nord.display_name(), "Nord");
        assert_eq!(ThemeVariant::TokyoNight.display_name(), "Tokyo Night");
        assert_eq!(ThemeVariant::CatppuccinMocha.display_name(), "Catppuccin Mocha");
        assert_eq!(ThemeVariant::Ferra.display_name(), "Ferra");
    }

    #[test]
    fn theme_variant_display_trait() {
        assert_eq!(format!("{}", ThemeVariant::Dark), "Dark");
        assert_eq!(format!("{}", ThemeVariant::SolarizedDark), "Solarized Dark");
    }

    #[test]
    fn theme_variant_is_dark() {
        assert!(!ThemeVariant::Light.is_dark());
        assert!(ThemeVariant::Dark.is_dark());
        assert!(ThemeVariant::Dracula.is_dark());
        assert!(ThemeVariant::Nord.is_dark());
        assert!(!ThemeVariant::SolarizedLight.is_dark());
        assert!(ThemeVariant::SolarizedDark.is_dark());
        assert!(ThemeVariant::TokyoNight.is_dark());
        assert!(ThemeVariant::TokyoNightStorm.is_dark());
        assert!(!ThemeVariant::TokyoNightLight.is_dark());
        assert!(ThemeVariant::Moonfly.is_dark());
        assert!(ThemeVariant::Nightfly.is_dark());
        assert!(!ThemeVariant::CatppuccinLatte.is_dark());
        assert!(ThemeVariant::CatppuccinMocha.is_dark());
        assert!(!ThemeVariant::GruvboxLight.is_dark());
        assert!(ThemeVariant::GruvboxDark.is_dark());
        assert!(!ThemeVariant::KanagawaLotus.is_dark());
        assert!(ThemeVariant::KanagawaWave.is_dark());
        assert!(ThemeVariant::KanagawaDragon.is_dark());
        assert!(ThemeVariant::Oxocarbon.is_dark());
        assert!(ThemeVariant::Ferra.is_dark());
    }

    #[test]
    fn theme_variant_all_count() {
        assert_eq!(ThemeVariant::all().len(), 22);
    }

    #[test]
    fn theme_variant_serialization() {
        let v = ThemeVariant::CatppuccinFrappe;
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("CatppuccinFrappe"));
        let decoded: ThemeVariant = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, ThemeVariant::CatppuccinFrappe);
    }

    // ── ThemeSetting tests ──

    #[test]
    fn theme_setting_default_is_auto() {
        assert_eq!(ThemeSetting::default(), ThemeSetting::Auto);
    }

    #[test]
    fn theme_setting_serialization() {
        let auto = ThemeSetting::Auto;
        let json = serde_json::to_string(&auto).unwrap();
        assert!(json.contains("Auto"));
        let decoded: ThemeSetting = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, ThemeSetting::Auto);

        let manual = ThemeSetting::Manual(ThemeVariant::Dracula);
        let json = serde_json::to_string(&manual).unwrap();
        let decoded: ThemeSetting = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, ThemeSetting::Manual(ThemeVariant::Dracula));
    }

    // ── AppSettings tests ──

    #[test]
    fn app_settings_defaults() {
        let settings = AppSettings::default();
        assert_eq!(settings.theme_setting, ThemeSetting::Auto);
        assert_eq!(settings.window_width, 1280);
        assert_eq!(settings.window_height, 800);
        assert_eq!(settings.font_family, "Monospace");
        assert_eq!(settings.font_size, 14);
    }

    #[test]
    fn app_settings_serialization_roundtrip() {
        let settings = AppSettings::default();
        let json = serde_json::to_string(&settings).unwrap();
        let decoded: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.window_width, 1280);
        assert_eq!(decoded.window_height, 800);
        assert_eq!(decoded.font_family, "Monospace");
        assert_eq!(decoded.font_size, 14);
        assert_eq!(decoded.theme_setting, ThemeSetting::Auto);
    }

    #[test]
    fn app_settings_serde_default_font() {
        // Build JSON dynamically so the endpoint URL matches the current platform default
        let default_endpoint = DockerEndpoint::default();
        let json = format!(
            r#"{{"theme_setting":"Auto","endpoint":{{"host_url":"{}","tls_ca":null,"tls_cert":null,"tls_key":null,"timeout_secs":30}},"window_width":1024,"window_height":768}}"#,
            default_endpoint.host_url
        );
        let settings: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings.font_family, "Monospace");
        assert_eq!(settings.font_size, 14);
    }
}
