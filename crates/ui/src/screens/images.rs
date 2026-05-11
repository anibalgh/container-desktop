use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Length, Padding, Theme};

use crate::typography::FontScale;
use crate::widgets::data_table::{data_table, Column, DataTableConfig};
use crate::widgets::modals::{pull_image_modal, PullImageMessage};
use domain::entities::Image;

pub struct ImagesScreen {
    pub font_size: u16,
    pub images: Vec<Image>,
    pub selected_index: Option<usize>,
    pub showing_pull_modal: bool,
    pub pull_image_name: String,
    pub pull_image_tag: String,
    pub pull_progress: Vec<String>,
    pub is_loading: bool,
    pub error_message: Option<String>,
}

impl Default for ImagesScreen {
    fn default() -> Self {
        Self {
            font_size: 14,
            images: Vec::new(),
            selected_index: None,
            showing_pull_modal: false,
            pull_image_name: String::new(),
            pull_image_tag: String::new(),
            pull_progress: Vec::new(),
            is_loading: false,
            error_message: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ImagesMessage {
    LoadImages,
    ImagesLoaded(Vec<Image>),
    SelectImage(usize),
    ShowPullModal,
    CloseModal,
    PullImageNameChanged(String),
    PullImageTagChanged(String),
    PullImage,
    PullProgress(String),
    RemoveImage(String),
    TagImage(String, String, String),
    Error(String),
    Noop,
}

impl ImagesScreen {
    pub fn update(&mut self, message: ImagesMessage) -> iced::Task<ImagesMessage> {
        match message {
            ImagesMessage::LoadImages => {
                self.is_loading = true;
                iced::Task::perform(
                    async move { Ok::<Vec<Image>, String>(Vec::new()) },
                    |r| match r {
                        Ok(imgs) => ImagesMessage::ImagesLoaded(imgs),
                        Err(e) => ImagesMessage::Error(e),
                    },
                )
            }
            ImagesMessage::ImagesLoaded(images) => {
                self.images = images;
                self.is_loading = false;
                iced::Task::none()
            }
            ImagesMessage::SelectImage(i) => {
                self.selected_index = Some(i);
                iced::Task::none()
            }
            ImagesMessage::ShowPullModal => {
                self.showing_pull_modal = true;
                self.pull_image_name.clear();
                self.pull_image_tag.clear();
                self.pull_progress.clear();
                iced::Task::none()
            }
            ImagesMessage::CloseModal => {
                self.showing_pull_modal = false;
                iced::Task::none()
            }
            ImagesMessage::PullImageNameChanged(name) => {
                self.pull_image_name = name;
                iced::Task::none()
            }
            ImagesMessage::PullImageTagChanged(tag) => {
                self.pull_image_tag = tag;
                iced::Task::none()
            }
            ImagesMessage::PullImage => {
                let name = self.pull_image_name.clone();
                iced::Task::perform(
                    async move { Ok::<_, String>(format!("Pulling {}...", name)) },
                    |r| match r {
                        Ok(msg) => ImagesMessage::PullProgress(msg),
                        Err(e) => ImagesMessage::Error(e),
                    },
                )
            }
            ImagesMessage::PullProgress(_) => {
                self.pull_progress.push("Pulling image...".into());
                iced::Task::none()
            }
            ImagesMessage::RemoveImage(_id) => iced::Task::none(),
            ImagesMessage::TagImage(_, _, _) => iced::Task::none(),
            ImagesMessage::Error(e) => {
                self.error_message = Some(e);
                iced::Task::none()
            }
            ImagesMessage::Noop => iced::Task::none(),
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, ImagesMessage, Theme, iced::Renderer> {
        let fs = FontScale::new(self.font_size);
        if self.showing_pull_modal {
            return self.view_pull_modal();
        }

        let table_config = DataTableConfig {
            columns: vec![
                Column {
                    header: "REPOSITORY".into(),
                    width: 200.0,
                },
                Column {
                    header: "TAG".into(),
                    width: 120.0,
                },
                Column {
                    header: "IMAGE ID".into(),
                    width: 130.0,
                },
                Column {
                    header: "SIZE".into(),
                    width: 100.0,
                },
                Column {
                    header: "CREATED".into(),
                    width: 150.0,
                },
            ],
            row_height: 28.0,
        };

        let rows: Vec<Vec<String>> = self
            .images
            .iter()
            .map(|img| {
                vec![
                    img.repo_name.clone(),
                    img.tag.clone(),
                    img.id.chars().take(12).collect(),
                    img.size.clone(),
                    img.created.clone(),
                ]
            })
            .collect();

        let table = data_table(
            table_config,
            rows,
            self.selected_index,
            ImagesMessage::SelectImage,
            None::<fn(usize) -> ImagesMessage>,
            None::<fn(usize) -> ImagesMessage>,
            None,
            false,
            self.font_size,
        );

        let action_bar = row![
            button(text("Refresh")).on_press(ImagesMessage::LoadImages),
            Space::new().width(4),
            button(text("+ Pull")).on_press(ImagesMessage::ShowPullModal),
            Space::new().width(8),
            if let Some(i) = self.selected_index {
                if let Some(img) = self.images.get(i) {
                    let id = img.id.clone();
                    let btn: Element<'_, ImagesMessage, Theme, iced::Renderer> =
                        button(text("Remove"))
                            .on_press(ImagesMessage::RemoveImage(id))
                            .into();
                    btn
                } else {
                    Space::new().width(0).into()
                }
            } else {
                Space::new().width(0).into()
            },
        ]
        .align_y(Alignment::Center);

        container(
            column![
                text("Images").size(fs.size(20)),
                action_bar,
                Space::new().height(8),
                if let Some(ref err) = self.error_message {
                    let err_elem: Element<'_, ImagesMessage, Theme, iced::Renderer> =
                        text(err.clone())
                            .color(iced::Color::from_rgb(0.9, 0.2, 0.2))
                            .size(fs.size(12))
                            .into();
                    err_elem
                } else {
                    Space::new().height(0).into()
                },
                table,
            ]
            .spacing(4)
            .padding(Padding::new(16.0)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn view_pull_modal<'a>(&'a self) -> Element<'a, ImagesMessage, Theme, iced::Renderer> {
        let modal = pull_image_modal(
            &self.pull_image_name,
            &self.pull_image_tag,
            &self.pull_progress,
            self.font_size,
        );

        // Map PullImageMessage variants to ImagesMessage
        modal.map(|msg| match msg {
            PullImageMessage::ImageNameChanged(s) => ImagesMessage::PullImageNameChanged(s),
            PullImageMessage::TagChanged(s) => ImagesMessage::PullImageTagChanged(s),
            PullImageMessage::Pull => ImagesMessage::PullImage,
            PullImageMessage::Cancel => ImagesMessage::CloseModal,
        })
    }
}
