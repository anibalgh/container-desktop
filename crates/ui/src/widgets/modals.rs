use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Padding, Theme};

/// A modal overlay for dialogs.
pub fn modal<'a, Message: Clone + 'a>(
    title: &'a str,
    body: Element<'a, Message, Theme, iced::Renderer>,
    primary_label: &'a str,
    primary_msg: Message,
    secondary_label: Option<(&'a str, Message)>,
    cancel_msg: Option<Message>,
) -> Element<'a, Message, Theme, iced::Renderer> {
    let cancel_btn: Element<'a, Message, Theme, iced::Renderer> =
        if let Some(cancel) = cancel_msg.clone() {
            button(text("✕").size(14))
                .style(transparent_button)
                .on_press(cancel)
                .into()
        } else {
            Space::new().width(0).into()
        };

    let title_bar = container(
        row![
            text(title).size(16),
            Space::new().width(Length::Fill),
            cancel_btn,
        ]
        .align_y(Alignment::Center),
    )
    .padding(Padding::new(12.0))
    .width(Length::Fill);

    let secondary: Element<'a, Message, Theme, iced::Renderer> =
        if let Some((label, msg)) = secondary_label {
            button(text(label)).on_press(msg).into()
        } else {
            Space::new().width(0).into()
        };

    let primary_btn: Element<'a, Message, Theme, iced::Renderer> =
        button(text(primary_label)).on_press(primary_msg).into();

    let buttons = row![secondary, Space::new().width(Length::Fill), primary_btn,]
        .spacing(8)
        .padding(Padding::new(12.0))
        .align_y(Alignment::Center);

    let content = column![
        title_bar,
        container(body).padding(Padding::new(12.0).left(16.0).right(16.0)),
        buttons,
    ]
    .spacing(0)
    .width(520);

    container(content).style(modal_style).padding(0).into()
}

/// A pull image modal dialog.
pub fn pull_image_modal<'a>(
    image_name: &'a str,
    tag: &'a str,
    pull_progress: &'a [String],
) -> Element<'a, PullImageMessage, Theme, iced::Renderer> {
    let body = column![
        text("Pull an image from a registry").size(12),
        Space::new().height(8),
        text("Image name:").size(11),
        text_input("e.g. nginx, alpine, ubuntu", image_name)
            .on_input(PullImageMessage::ImageNameChanged)
            .padding(8),
        Space::new().height(8),
        text("Tag (optional):").size(11),
        text_input("latest", tag)
            .on_input(PullImageMessage::TagChanged)
            .padding(8),
        Space::new().height(12),
        {
            let progress_elem: Element<'_, PullImageMessage, Theme, iced::Renderer> =
                if !pull_progress.is_empty() {
                    container(
                    column(
                        pull_progress
                            .iter()
                            .map(|line| text(line).size(11).into())
                            .collect::<Vec<Element<'_, PullImageMessage, Theme, iced::Renderer>>>()
                    )
                    .spacing(2),
                )
                .height(100)
                .style(container::bordered_box)
                .into()
                } else {
                    Space::new().height(0).into()
                };
            progress_elem
        },
    ]
    .spacing(2)
    .width(Length::Fill);

    modal(
        "Pull Image",
        body.into(),
        "Pull",
        PullImageMessage::Pull,
        None,
        Some(PullImageMessage::Cancel),
    )
}

#[derive(Debug, Clone)]
pub enum PullImageMessage {
    ImageNameChanged(String),
    TagChanged(String),
    Pull,
    Cancel,
}

/// A confirmation modal dialog.
pub fn confirm_modal<'a, Message: Clone + 'a>(
    title: &'a str,
    message: &'a str,
    confirm_msg: Message,
    cancel_msg: Message,
) -> Element<'a, Message, Theme, iced::Renderer> {
    let body = container(text(message).size(13))
        .padding(Padding::new(8.0))
        .width(Length::Fill);

    modal(
        title,
        body.into(),
        "Confirm",
        confirm_msg,
        None,
        Some(cancel_msg),
    )
}

/// A create container modal.
#[derive(Debug, Default, Clone)]
pub struct CreateContainerData {
    pub name: String,
    pub image: String,
    pub ports: String,
    pub volumes: String,
    pub env: String,
}

#[derive(Debug, Clone)]
pub enum CreateContainerMessage {
    NameChanged(String),
    ImageChanged(String),
    PortsChanged(String),
    VolumesChanged(String),
    EnvChanged(String),
    Create,
    Cancel,
}

pub fn create_container_modal<'a>(
    data: &CreateContainerData,
) -> Element<'a, CreateContainerMessage, Theme, iced::Renderer> {
    let body = column![
        text("Image name:").size(11),
        text_input("e.g. nginx:latest", &data.image)
            .on_input(CreateContainerMessage::ImageChanged)
            .padding(8),
        Space::new().height(6),
        text("Container name (optional):").size(11),
        text_input("my-container", &data.name)
            .on_input(CreateContainerMessage::NameChanged)
            .padding(8),
        Space::new().height(6),
        text("Ports (e.g. 8080:80):").size(11),
        text_input("", &data.ports)
            .on_input(CreateContainerMessage::PortsChanged)
            .padding(8),
        Space::new().height(6),
        text("Volumes (e.g. /host:/container):").size(11),
        text_input("", &data.volumes)
            .on_input(CreateContainerMessage::VolumesChanged)
            .padding(8),
        Space::new().height(6),
        text("Environment (KEY=VALUE, one per line):").size(11),
        text_input("", &data.env)
            .on_input(CreateContainerMessage::EnvChanged)
            .padding(8),
    ]
    .spacing(2)
    .width(Length::Fill);

    modal(
        "Create Container",
        body.into(),
        "Create",
        CreateContainerMessage::Create,
        None,
        Some(CreateContainerMessage::Cancel),
    )
}

fn modal_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(iced::Background::Color(palette.background.base.color)),
        border: iced::Border {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

fn transparent_button(
    theme: &Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    let palette = theme.extended_palette();
    let mut style = iced::widget::button::Style::default();
    match status {
        iced::widget::button::Status::Hovered => {
            style.background = Some(iced::Background::Color(palette.danger.base.color));
        }
        _ => {}
    }
    style
}
