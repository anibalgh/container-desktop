use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Padding, Theme};

use crate::typography::FontScale;
use crate::widgets::data_table::{data_table, Column, DataTableConfig};
use domain::entities::Network;

pub struct NetworksScreen {
    pub font_size: u16,
    pub networks: Vec<Network>,
    pub selected_index: Option<usize>,
    pub show_create: bool,
    pub new_network_name: String,
    pub new_network_driver: String,
    pub is_loading: bool,
}

impl Default for NetworksScreen {
    fn default() -> Self {
        Self {
            font_size: 14,
            networks: Vec::new(),
            selected_index: None,
            show_create: false,
            new_network_name: String::new(),
            new_network_driver: String::from("bridge"),
            is_loading: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum NetworksMessage {
    LoadNetworks,
    NetworksLoaded(Vec<Network>),
    SelectNetwork(usize),
    ShowCreate,
    NetworkNameChanged(String),
    DriverChanged(String),
    CreateNetwork,
    RemoveNetwork(String),
    CloseCreate,
    Error(String),
    Noop,
}

impl NetworksScreen {
    pub fn update(&mut self, message: NetworksMessage) -> iced::Task<NetworksMessage> {
        match message {
            NetworksMessage::LoadNetworks => {
                self.is_loading = true;
                iced::Task::perform(async move { Ok::<Vec<Network>, String>(Vec::new()) }, |r| {
                    match r {
                        Ok(n) => NetworksMessage::NetworksLoaded(n),
                        Err(e) => NetworksMessage::Error(e),
                    }
                })
            }
            NetworksMessage::NetworksLoaded(networks) => {
                self.networks = networks;
                self.is_loading = false;
                iced::Task::none()
            }
            NetworksMessage::SelectNetwork(i) => {
                self.selected_index = Some(i);
                iced::Task::none()
            }
            NetworksMessage::ShowCreate => {
                self.show_create = true;
                iced::Task::none()
            }
            NetworksMessage::NetworkNameChanged(n) => {
                self.new_network_name = n;
                iced::Task::none()
            }
            NetworksMessage::DriverChanged(d) => {
                self.new_network_driver = d;
                iced::Task::none()
            }
            NetworksMessage::CreateNetwork => {
                self.show_create = false;
                iced::Task::none()
            }
            NetworksMessage::RemoveNetwork(_) => iced::Task::none(),
            NetworksMessage::CloseCreate => {
                self.show_create = false;
                iced::Task::none()
            }
            NetworksMessage::Error(_) => iced::Task::none(),
            NetworksMessage::Noop => iced::Task::none(),
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, NetworksMessage, Theme, iced::Renderer> {
        let fs = FontScale::new(self.font_size);
        let table_config = DataTableConfig {
            columns: vec![
                Column {
                    header: "NAME".into(),
                    width: 180.0,
                },
                Column {
                    header: "ID".into(),
                    width: 140.0,
                },
                Column {
                    header: "DRIVER".into(),
                    width: 120.0,
                },
                Column {
                    header: "SCOPE".into(),
                    width: 100.0,
                },
                Column {
                    header: "SUBNET".into(),
                    width: 160.0,
                },
                Column {
                    header: "INTERNAL".into(),
                    width: 80.0,
                },
            ],
            row_height: 28.0,
        };

        let rows: Vec<Vec<String>> = self
            .networks
            .iter()
            .map(|n| {
                vec![
                    n.name.clone(),
                    n.id.chars().take(12).collect(),
                    n.driver.clone(),
                    n.scope.clone(),
                    n.subnet.clone().unwrap_or_default(),
                    if n.internal {
                        "Yes".into()
                    } else {
                        "No".into()
                    },
                ]
            })
            .collect();

        let table = data_table(
            table_config,
            rows,
            self.selected_index,
            NetworksMessage::SelectNetwork,
            None::<fn(usize) -> NetworksMessage>,
            None::<fn(usize) -> NetworksMessage>,
            None,
            false,
            self.font_size,
        );

        let mut create_section: Vec<Element<'_, NetworksMessage, Theme, iced::Renderer>> =
            Vec::new();

        let action_row = row![
            button(text("Refresh")).on_press(NetworksMessage::LoadNetworks),
            Space::new().width(4),
            button(text("+ Create")).on_press(NetworksMessage::ShowCreate),
        ]
        .align_y(Alignment::Center);

        if self.show_create {
            create_section.push(
                row![
                    text("Name:").size(fs.size(12)),
                    text_input("network-name", &self.new_network_name)
                        .on_input(NetworksMessage::NetworkNameChanged)
                        .padding(4)
                        .size(fs.size(12))
                        .width(120),
                    Space::new().width(8),
                    text("Driver:").size(fs.size(12)),
                    text_input("bridge", &self.new_network_driver)
                        .on_input(NetworksMessage::DriverChanged)
                        .padding(4)
                        .size(fs.size(12))
                        .width(100),
                    Space::new().width(8),
                    button(text("Create")).on_press(NetworksMessage::CreateNetwork),
                    button(text("Cancel")).on_press(NetworksMessage::CloseCreate),
                ]
                .spacing(4)
                .align_y(Alignment::Center)
                .into(),
            );
        }

        let mut content = column![text("Networks").size(fs.size(20)), action_row,].spacing(4);

        for section in create_section {
            content = content.push(section);
        }

        content = content.push(Space::new().height(8));
        content = content.push(table);

        container(content.padding(Padding::new(16.0)))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
