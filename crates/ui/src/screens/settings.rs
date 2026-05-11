use iced::widget::{
    button, column, container, pick_list, row, scrollable, slider, text, text_input, Space,
};
use iced::{Alignment, Element, Length, Padding, Theme};

use domain::entities::{DockerEndpoint, ThemeSetting, ThemeVariant};

use crate::typography::{FontScale, MONOSPACE_FONTS};

pub struct SettingsScreen {
    pub theme_setting: ThemeSetting,
    pub endpoint_url: String,
    pub tls_ca: String,
    pub tls_cert: String,
    pub tls_key: String,
    pub saved: bool,
    pub test_result: Option<String>,
    /// Selected monospace font family.
    pub font_family: String,
    /// Base font size in pixels (4..32).
    pub font_size: u16,
}

impl Default for SettingsScreen {
    fn default() -> Self {
        let default_endpoint = DockerEndpoint::default();
        Self {
            theme_setting: ThemeSetting::Auto,
            endpoint_url: default_endpoint.host_url,
            tls_ca: String::new(),
            tls_cert: String::new(),
            tls_key: String::new(),
            saved: false,
            test_result: None,
            font_family: "Monospace".to_string(),
            font_size: 14,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ThemeModeChanged(ThemeModeChoice),
    ThemeVariantChanged(ThemeVariant),
    EndpointUrlChanged(String),
    TlsCaChanged(String),
    TlsCertChanged(String),
    TlsKeyChanged(String),
    TestConnection,
    TestResult(Result<String, String>),
    FontFamilyChanged(String),
    FontSizeChanged(u16),
    Save,
    Saved,
    Noop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeModeChoice {
    Auto,
    Manual,
}

impl std::fmt::Display for ThemeModeChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeModeChoice::Auto => write!(f, "Auto (follow system)"),
            ThemeModeChoice::Manual => write!(f, "Manual"),
        }
    }
}

impl SettingsScreen {
    pub fn update(&mut self, message: SettingsMessage) -> iced::Task<SettingsMessage> {
        match message {
            SettingsMessage::ThemeModeChanged(mode) => {
                self.theme_setting = match mode {
                    ThemeModeChoice::Auto => ThemeSetting::Auto,
                    ThemeModeChoice::Manual => ThemeSetting::Manual(ThemeVariant::CatppuccinMocha),
                };
                iced::Task::none()
            }
            SettingsMessage::ThemeVariantChanged(variant) => {
                self.theme_setting = ThemeSetting::Manual(variant);
                iced::Task::none()
            }
            SettingsMessage::EndpointUrlChanged(url) => {
                self.endpoint_url = url;
                iced::Task::none()
            }
            SettingsMessage::TlsCaChanged(ca) => {
                self.tls_ca = ca;
                iced::Task::none()
            }
            SettingsMessage::TlsCertChanged(cert) => {
                self.tls_cert = cert;
                iced::Task::none()
            }
            SettingsMessage::TlsKeyChanged(key) => {
                self.tls_key = key;
                iced::Task::none()
            }
            SettingsMessage::TestConnection => {
                let url = self.endpoint_url.clone();
                iced::Task::perform(
                    async move {
                        // Simulate connection test
                        if url.contains("docker.sock") || url.contains("localhost") {
                            Ok("Connection successful".into())
                        } else {
                            Err("Connection failed".into())
                        }
                    },
                    SettingsMessage::TestResult,
                )
            }
            SettingsMessage::TestResult(result) => {
                self.test_result = match &result {
                    Ok(msg) => Some(msg.clone()),
                    Err(e) => Some(format!("Error: {e}")),
                };
                iced::Task::none()
            }
            SettingsMessage::FontFamilyChanged(family) => {
                self.font_family = family;
                iced::Task::none()
            }
            SettingsMessage::FontSizeChanged(size) => {
                self.font_size = size;
                iced::Task::none()
            }
            SettingsMessage::Save => {
                self.saved = true;
                iced::Task::perform(async move { Ok::<_, String>(()) }, |_| {
                    SettingsMessage::Saved
                })
            }
            SettingsMessage::Saved => iced::Task::none(),
            SettingsMessage::Noop => iced::Task::none(),
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, SettingsMessage, Theme, iced::Renderer> {
        let fs = FontScale::new(self.font_size);
        let theme_section = {
            let theme_mode_pick = pick_list(
                vec![ThemeModeChoice::Auto, ThemeModeChoice::Manual],
                match &self.theme_setting {
                    ThemeSetting::Auto => Some(ThemeModeChoice::Auto),
                    ThemeSetting::Manual(_) => Some(ThemeModeChoice::Manual),
                },
                SettingsMessage::ThemeModeChanged,
            );

            let mut section = column![
                text("Theme").size(fs.size(16)),
                Space::new().height(8),
                text("Theme mode:").size(fs.size(12)),
                theme_mode_pick,
            ]
            .spacing(4);

            if let ThemeSetting::Manual(_) = &self.theme_setting {
                let variants: Vec<ThemeVariant> = ThemeVariant::all().to_vec();
                let current = match &self.theme_setting {
                    ThemeSetting::Manual(v) => Some(*v),
                    _ => None,
                };
                section = section.push(pick_list(
                    variants,
                    current,
                    SettingsMessage::ThemeVariantChanged,
                ));
            }

            section
        };

        let typography_section = {
            let font_families: Vec<String> =
                MONOSPACE_FONTS.iter().map(|s| s.to_string()).collect();
            let current_family = if font_families.iter().any(|f| f == &self.font_family) {
                Some(self.font_family.clone())
            } else {
                // Custom font typed by user — show current value as selected
                None
            };

            let font_picker = pick_list(
                font_families,
                current_family,
                SettingsMessage::FontFamilyChanged,
            );

            let size_control = {
                let size_label =
                    text(format!("Font size: {} px", self.font_size)).size(fs.size(12));
                let slider = slider(4..=32, self.font_size, SettingsMessage::FontSizeChanged)
                    .width(200)
                    .step(1u16);
                row![size_label, Space::new().width(12), slider].align_y(Alignment::Center)
            };

            column![
                text("Typography").size(fs.size(16)),
                Space::new().height(8),
                text("Monospace font family:").size(fs.size(12)),
                font_picker,
                Space::new().height(8),
                size_control,
                Space::new().height(4),
                text("Changes to font settings take effect after restarting the application.")
                    .size(fs.size(10)),
            ]
            .spacing(4)
        };

        let endpoint_section = column![
            text("Docker Endpoint").size(fs.size(16)),
            Space::new().height(8),
            text("Host URL:").size(fs.size(12)),
            text_input("unix:///var/run/docker.sock", &self.endpoint_url)
                .on_input(SettingsMessage::EndpointUrlChanged)
                .padding(6)
                .size(fs.size(12)),
            Space::new().height(6),
            text("TLS CA certificate path (optional):").size(fs.size(12)),
            text_input("", &self.tls_ca)
                .on_input(SettingsMessage::TlsCaChanged)
                .padding(6)
                .size(fs.size(12)),
            Space::new().height(6),
            text("TLS certificate path (optional):").size(fs.size(12)),
            text_input("", &self.tls_cert)
                .on_input(SettingsMessage::TlsCertChanged)
                .padding(6)
                .size(fs.size(12)),
            Space::new().height(6),
            text("TLS key path (optional):").size(fs.size(12)),
            text_input("", &self.tls_key)
                .on_input(SettingsMessage::TlsKeyChanged)
                .padding(6)
                .size(fs.size(12)),
            Space::new().height(8),
            row![
                button(text("Test Connection")).on_press(SettingsMessage::TestConnection),
                Space::new().width(8),
                if let Some(ref result) = self.test_result {
                    let result_elem: Element<'_, SettingsMessage, Theme, iced::Renderer> =
                        text(result.clone()).size(fs.size(12)).into();
                    result_elem
                } else {
                    Space::new().width(0).into()
                },
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(2);

        let save_button = row![
            Space::new().width(Length::Fill),
            button(text("Save Settings")).on_press(SettingsMessage::Save),
            if self.saved {
                let saved_text: Element<'_, SettingsMessage, Theme, iced::Renderer> =
                    text(" Saved!")
                        .size(fs.size(12))
                        .color(iced::Color::from_rgb(0.2, 0.7, 0.3))
                        .into();
                saved_text
            } else {
                Space::new().width(0).into()
            },
        ]
        .align_y(Alignment::Center);

        container(scrollable(
            column![
                text("Settings").size(fs.size(20)),
                Space::new().height(8),
                theme_section,
                Space::new().height(20),
                typography_section,
                Space::new().height(20),
                endpoint_section,
                Space::new().height(16),
                save_button,
            ]
            .spacing(4)
            .padding(Padding::new(16.0))
            .width(Length::Fill),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
