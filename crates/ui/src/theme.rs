use domain::entities::{ThemeSetting, ThemeVariant};
use iced::theme::Base;
use std::sync::OnceLock;

/// Cached auto-detected OS theme. Detected once on first call.
static AUTO_DETECTED: OnceLock<iced::Theme> = OnceLock::new();

/// Maps domain theme settings to iced themes.
pub struct ThemeManager;

impl ThemeManager {
    /// Resolves a `ThemeSetting` to an `iced::Theme`.
    pub fn resolve(setting: &ThemeSetting) -> iced::Theme {
        match setting {
            ThemeSetting::Auto => AUTO_DETECTED
                .get_or_init(|| Self::detect_os_theme())
                .clone(),
            ThemeSetting::Manual(variant) => Self::variant_to_theme(*variant),
        }
    }

    fn detect_os_theme() -> iced::Theme {
        use dark_light::Mode;
        match dark_light::detect() {
            Mode::Dark => iced::Theme::TokyoNight,
            Mode::Light => iced::Theme::CatppuccinLatte,
            Mode::Default => {
                if Self::system_looks_dark() {
                    iced::Theme::TokyoNight
                } else {
                    iced::Theme::CatppuccinLatte
                }
            }
        }
    }

    fn system_looks_dark() -> bool {
        if let Ok(theme) = std::env::var("GTK_THEME") {
            if theme.to_lowercase().contains("dark") {
                return true;
            }
        }
        if let Ok(scheme) = std::env::var("COLOR_SCHEME") {
            let s = scheme.to_lowercase();
            if s == "dark" || s == "prefer-dark" {
                return true;
            }
        }
        false
    }

    /// Converts a domain `ThemeVariant` to an `iced::Theme`.
    pub fn variant_to_theme(variant: ThemeVariant) -> iced::Theme {
        match variant {
            ThemeVariant::Light => iced::Theme::Light,
            ThemeVariant::Dark => iced::Theme::Dark,
            ThemeVariant::Dracula => iced::Theme::Dracula,
            ThemeVariant::Nord => iced::Theme::Nord,
            ThemeVariant::SolarizedLight => iced::Theme::SolarizedLight,
            ThemeVariant::SolarizedDark => iced::Theme::SolarizedDark,
            ThemeVariant::GruvboxLight => iced::Theme::GruvboxLight,
            ThemeVariant::GruvboxDark => iced::Theme::GruvboxDark,
            ThemeVariant::CatppuccinLatte => iced::Theme::CatppuccinLatte,
            ThemeVariant::CatppuccinFrappe => iced::Theme::CatppuccinFrappe,
            ThemeVariant::CatppuccinMacchiato => iced::Theme::CatppuccinMacchiato,
            ThemeVariant::CatppuccinMocha => iced::Theme::CatppuccinMocha,
            ThemeVariant::TokyoNight => iced::Theme::TokyoNight,
            ThemeVariant::TokyoNightStorm => iced::Theme::TokyoNightStorm,
            ThemeVariant::TokyoNightLight => iced::Theme::TokyoNightLight,
            ThemeVariant::KanagawaWave => iced::Theme::KanagawaWave,
            ThemeVariant::KanagawaDragon => iced::Theme::KanagawaDragon,
            ThemeVariant::KanagawaLotus => iced::Theme::KanagawaLotus,
            ThemeVariant::Moonfly => iced::Theme::Moonfly,
            ThemeVariant::Nightfly => iced::Theme::Nightfly,
            ThemeVariant::Oxocarbon => iced::Theme::Oxocarbon,
            ThemeVariant::Ferra => iced::Theme::Ferra,
        }
    }

    /// Returns whether the current theme is dark mode.
    pub fn is_dark(theme: &iced::Theme) -> bool {
        theme.mode() == iced::theme::Mode::Dark
    }

    /// Detects whether the OS is currently in dark mode.
    /// Returns true if the OS theme preference is dark.
    pub fn os_is_dark() -> bool {
        use dark_light::Mode;
        match dark_light::detect() {
            Mode::Dark => true,
            Mode::Light => false,
            Mode::Default => Self::system_looks_dark(),
        }
    }
}
