use super::icon::{icon, Icon};
use iced::widget::{button, column, container, image, row, text};
use iced::{Element, Length, Padding, Theme};

use crate::typography::FontScale;

/// Navigation item for the sidebar.
pub struct NavItem {
    pub label: &'static str,
    pub icon: Icon,
}

/// All sidebar navigation items in display order.
pub const NAV_ITEMS: &[NavItem] = &[
    NavItem {
        label: "Dashboard",
        icon: Icon::Dashboard,
    },
    NavItem {
        label: "Containers",
        icon: Icon::Containers,
    },
    NavItem {
        label: "Images",
        icon: Icon::Images,
    },
    NavItem {
        label: "Volumes",
        icon: Icon::Volumes,
    },
    NavItem {
        label: "Networks",
        icon: Icon::Networks,
    },
    NavItem {
        label: "Compose",
        icon: Icon::Compose,
    },
    NavItem {
        label: "Settings",
        icon: Icon::Settings,
    },
];

/// Creates the sidebar navigation widget.
pub fn sidebar<'a, Message: Clone + 'a>(
    active_index: usize,
    dark_mode: bool,
    on_select: impl Fn(usize) -> Message + 'a,
    docker_running: bool,
    font_size: u16,
) -> Element<'a, Message, Theme, iced::Renderer> {
    let fs = FontScale::new(font_size);
    let sidebar_icon_bytes: &[u8] = if dark_mode {
        include_bytes!("../../../../assets/icons/dark/icon16x16.png")
    } else {
        include_bytes!("../../../../assets/icons/light/icon16x16.png")
    };
    let header = container(
        row![
            image::Image::new(image::Handle::from_bytes(sidebar_icon_bytes.to_vec()))
                .width(28)
                .height(28),
            text("Container").size(fs.size(14)),
            text("Desktop").size(fs.size(18)),
        ]
        .align_y(iced::Alignment::Center)
        .spacing(6),
    )
    .padding(Padding::new(12.0))
    .width(Length::Fill);

    let nav_buttons = column(
        NAV_ITEMS
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_active = i == active_index;
                let btn = button(
                    row![
                        icon(item.icon, dark_mode, 18.0),
                        text(item.label).size(fs.size(13)),
                    ]
                    .spacing(8)
                    .align_y(iced::Alignment::Center)
                    .padding(Padding::new(8.0)),
                )
                .width(Length::Fill);

                if is_active {
                    btn.style(|theme: &Theme, status| active_nav_style(theme, status))
                } else {
                    btn.style(|theme: &Theme, status| inactive_nav_style(theme, status))
                }
                .on_press(on_select(i))
                .into()
            })
            .collect::<Vec<Element<'_, Message, Theme, iced::Renderer>>>(),
    )
    .spacing(2)
    .width(Length::Fill);

    let status_text = if docker_running {
        text("Connected").size(fs.size(11))
    } else {
        text("Disconnected").size(fs.size(11))
    };

    let status = container(status_text)
        .padding(Padding::new(8.0))
        .width(Length::Fill);

    let spacer = container(text("")).height(Length::Fill);

    let content = column![header, nav_buttons, spacer, status]
        .spacing(4)
        .width(200);

    container(content)
        .width(200)
        .height(Length::Fill)
        .style(container::bordered_box)
        .into()
}

fn active_nav_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let mut style = button::Style::default();
    match status {
        button::Status::Active | button::Status::Pressed => {
            style.background = Some(iced::Background::Color(palette.primary.strong.color));
            style.text_color = palette.primary.strong.text;
        }
        button::Status::Hovered => {
            style.background = Some(iced::Background::Color(palette.primary.base.color));
            style.text_color = palette.primary.base.text;
        }
        button::Status::Disabled => {}
    }
    style
}

fn inactive_nav_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let mut style = button::Style::default();
    match status {
        button::Status::Hovered => {
            style.background = Some(iced::Background::Color(palette.background.strong.color));
        }
        _ => {}
    }
    style.text_color = palette.background.base.text;
    style
}
