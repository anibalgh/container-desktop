use iced::widget::{column, container, image, row, text, Space};
use iced::{Color, Element, Length, Theme};

/// Dashboard screen state.
#[derive(Default)]
pub struct DashboardScreen;

impl DashboardScreen {
    /// Returns the view for the dashboard screen.
    pub fn view<'a, Message: Clone + 'a>(
        &self,
        connected: bool,
        dark_mode: bool,
    ) -> Element<'a, Message, Theme, iced::Renderer> {
        let status_text = if connected {
            text("Docker is connected").color(Color::from_rgb(0.2, 0.7, 0.3))
        } else {
            text("Docker is not connected").color(Color::from_rgb(0.8, 0.3, 0.3))
        };

        let logo_bytes: &[u8] = if dark_mode {
            include_bytes!("../../../../assets/icons/dark/container-desktop.png")
        } else {
            include_bytes!("../../../../assets/icons/light/container-desktop.png")
        };
        let logo = image::Image::new(image::Handle::from_bytes(logo_bytes.to_vec()))
            .width(128)
            .height(128);

        let content = column![
            text("Dashboard").size(24),
            Space::new().height(8),
            container(logo).width(Length::Fill).center_x(Length::Fill),
            Space::new().height(12),
            row![status_text.size(14)],
        ]
        .spacing(16)
        .padding(24);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
