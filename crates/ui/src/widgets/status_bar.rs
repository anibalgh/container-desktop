use domain::entities::DockerInfo;
use iced::widget::{container, row, text};
use iced::{Element, Length, Padding, Theme};

use crate::typography::FontScale;

/// Creates a status bar widget showing Docker connection info.
pub fn status_bar<'a, Message: Clone + 'a>(
    docker_info: &'a Option<DockerInfo>,
    active_screen: &'a str,
    font_size: u16,
) -> Element<'a, Message, Theme, iced::Renderer> {
    let fs = FontScale::new(font_size);
    let palette = iced::Theme::CatppuccinMocha.extended_palette();

    let connection_status = if docker_info.is_some() {
        let info = docker_info.as_ref().unwrap();
        row![
            text("●")
                .color(palette.success.strong.color)
                .size(fs.size(12)),
            text(format!(
                "Connected to Docker {} | {} | {} containers, {} images",
                info.server_version, info.os_type, info.containers_running, info.images
            ))
            .size(fs.size(11))
            .color(palette.background.weak.text),
        ]
        .spacing(6)
        .align_y(iced::Alignment::Center)
    } else {
        row![
            text("●")
                .color(palette.danger.strong.color)
                .size(fs.size(12)),
            text("Docker not connected")
                .size(fs.size(11))
                .color(palette.background.weak.text),
        ]
        .spacing(6)
        .align_y(iced::Alignment::Center)
    };

    container(
        row![
            connection_status,
            text("").width(Length::Fill),
            text(active_screen)
                .size(fs.size(11))
                .color(palette.background.weak.text),
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .padding(Padding::new(6.0).left(8.0).right(12.0))
    .style(move |theme: &Theme| {
        let p = theme.extended_palette();
        container::Style {
            background: Some(iced::Background::Color(p.background.strong.color)),
            ..Default::default()
        }
    })
    .into()
}

/// Creates the status bar for the given screen title.
pub fn status_bar_with_title<'a, Message: Clone + 'a>(
    docker_info: &'a Option<DockerInfo>,
    title: &'a str,
    font_size: u16,
) -> Element<'a, Message, Theme, iced::Renderer> {
    status_bar(docker_info, title, font_size)
}
