use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Padding, Theme};

use crate::widgets::data_table::{data_table, Column, DataTableConfig};
use domain::entities::Volume;

pub struct VolumesScreen {
    pub volumes: Vec<Volume>,
    pub selected_index: Option<usize>,
    pub show_create: bool,
    pub new_volume_name: String,
    pub is_loading: bool,
}

impl Default for VolumesScreen {
    fn default() -> Self {
        Self {
            volumes: Vec::new(),
            selected_index: None,
            show_create: false,
            new_volume_name: String::new(),
            is_loading: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum VolumesMessage {
    LoadVolumes,
    VolumesLoaded(Vec<Volume>),
    SelectVolume(usize),
    ShowCreate,
    VolumeNameChanged(String),
    CreateVolume,
    RemoveVolume(String),
    CloseCreate,
    Error(String),
    Noop,
}

impl VolumesScreen {
    pub fn update(&mut self, message: VolumesMessage) -> iced::Task<VolumesMessage> {
        match message {
            VolumesMessage::LoadVolumes => {
                self.is_loading = true;
                iced::Task::perform(async move { Ok::<Vec<Volume>, String>(Vec::new()) }, |r| {
                    match r {
                        Ok(v) => VolumesMessage::VolumesLoaded(v),
                        Err(e) => VolumesMessage::Error(e),
                    }
                })
            }
            VolumesMessage::VolumesLoaded(volumes) => {
                self.volumes = volumes;
                self.is_loading = false;
                iced::Task::none()
            }
            VolumesMessage::SelectVolume(i) => {
                self.selected_index = Some(i);
                iced::Task::none()
            }
            VolumesMessage::ShowCreate => {
                self.show_create = true;
                iced::Task::none()
            }
            VolumesMessage::VolumeNameChanged(n) => {
                self.new_volume_name = n;
                iced::Task::none()
            }
            VolumesMessage::CreateVolume => {
                self.show_create = false;
                self.new_volume_name.clear();
                iced::Task::none()
            }
            VolumesMessage::RemoveVolume(_) => iced::Task::none(),
            VolumesMessage::CloseCreate => {
                self.show_create = false;
                iced::Task::none()
            }
            VolumesMessage::Error(_) => iced::Task::none(),
            VolumesMessage::Noop => iced::Task::none(),
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, VolumesMessage, Theme, iced::Renderer> {
        let table_config = DataTableConfig {
            columns: vec![
                Column {
                    header: "NAME".into(),
                    width: 200.0,
                },
                Column {
                    header: "DRIVER".into(),
                    width: 120.0,
                },
                Column {
                    header: "MOUNTPOINT".into(),
                    width: 300.0,
                },
                Column {
                    header: "SCOPE".into(),
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
            .volumes
            .iter()
            .map(|v| {
                vec![
                    v.name.clone(),
                    v.driver.clone(),
                    v.mountpoint.clone(),
                    v.scope.clone(),
                    v.created.clone(),
                ]
            })
            .collect();

        let table = data_table(
            table_config,
            rows,
            self.selected_index,
            VolumesMessage::SelectVolume,
        );

        let action_row = row![
            button(text("Refresh")).on_press(VolumesMessage::LoadVolumes),
            Space::new().width(4),
            button(text("+ Create")).on_press(VolumesMessage::ShowCreate),
        ]
        .align_y(Alignment::Center);

        if self.show_create {
            let create_bar = row![
                text("Name:").size(12),
                text_input("volume-name", &self.new_volume_name)
                    .on_input(VolumesMessage::VolumeNameChanged)
                    .padding(4)
                    .size(12),
                button(text("Create")).on_press(VolumesMessage::CreateVolume),
                button(text("Cancel")).on_press(VolumesMessage::CloseCreate),
            ]
            .spacing(4)
            .align_y(Alignment::Center);

            container(
                column![
                    text("Volumes").size(20),
                    action_row,
                    Space::new().height(8),
                    create_bar,
                    Space::new().height(8),
                    table,
                ]
                .spacing(4)
                .padding(Padding::new(16.0)),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            container(
                column![
                    text("Volumes").size(20),
                    action_row,
                    Space::new().height(8),
                    table,
                ]
                .spacing(4)
                .padding(Padding::new(16.0)),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
    }
}
