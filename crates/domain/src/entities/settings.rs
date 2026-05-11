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
