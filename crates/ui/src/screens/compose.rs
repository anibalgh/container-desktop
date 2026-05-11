use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Padding, Theme};

use crate::typography::FontScale;
use crate::widgets::data_table::{data_table, Column, DataTableConfig};
use crate::widgets::log_viewer::{log_viewer, LogEntry};

pub struct ComposeScreen {
    pub font_size: u16,
    pub stacks: Vec<ComposeStackInfo>,
    pub selected_index: Option<usize>,
    pub compose_file: String,
    pub log_entries: Vec<LogEntry>,
    pub showing_logs: bool,
    pub is_loading: bool,
}

#[derive(Debug, Clone)]
pub struct ComposeStackInfo {
    pub name: String,
    pub file: String,
    pub status: String,
    pub services: Vec<String>,
}

impl Default for ComposeScreen {
    fn default() -> Self {
        Self {
            font_size: 14,
            stacks: Vec::new(),
            selected_index: None,
            compose_file: String::new(),
            log_entries: Vec::new(),
            showing_logs: false,
            is_loading: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ComposeMessage {
    LoadStacks,
    FilePathChanged(String),
    ComposeUp,
    ComposeDown(String),
    ShowLogs(String),
    CloseLogs,
    LogsLoaded(Vec<LogEntry>),
    Error(String),
    Noop,
}

impl ComposeScreen {
    pub fn update(&mut self, message: ComposeMessage) -> iced::Task<ComposeMessage> {
        match message {
            ComposeMessage::LoadStacks => {
                self.is_loading = true;
                iced::Task::perform(
                    async move { Ok::<Vec<ComposeStackInfo>, String>(Vec::new()) },
                    |r| match r {
                        Ok(s) => {
                            // handled in a future
                            let _ = s;
                            ComposeMessage::Noop
                        }
                        Err(e) => ComposeMessage::Error(e),
                    },
                )
            }
            ComposeMessage::FilePathChanged(p) => {
                self.compose_file = p;
                iced::Task::none()
            }
            ComposeMessage::ComposeUp => iced::Task::none(),
            ComposeMessage::ComposeDown(_) => iced::Task::none(),
            ComposeMessage::ShowLogs(_) => {
                self.showing_logs = true;
                self.log_entries = vec![LogEntry {
                    line: "Loading logs...".into(),
                    is_stderr: false,
                }];
                iced::Task::none()
            }
            ComposeMessage::CloseLogs => {
                self.showing_logs = false;
                iced::Task::none()
            }
            ComposeMessage::LogsLoaded(entries) => {
                self.log_entries = entries;
                iced::Task::none()
            }
            ComposeMessage::Error(_) => iced::Task::none(),
            ComposeMessage::Noop => iced::Task::none(),
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, ComposeMessage, Theme, iced::Renderer> {
        let fs = FontScale::new(self.font_size);
        if self.showing_logs {
            return container(
                column![
                    row![
                        text("Compose Logs").size(fs.size(20)),
                        Space::new().width(Length::Fill),
                        button(text("Close")).on_press(ComposeMessage::CloseLogs),
                    ]
                    .align_y(Alignment::Center),
                    Space::new().height(8),
                    log_viewer(&self.log_entries, true, self.font_size),
                ]
                .spacing(4)
                .padding(Padding::new(16.0)),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        }

        let compose_bar = row![
            text("Compose file:").size(fs.size(12)),
            text_input("/path/to/docker-compose.yml", &self.compose_file)
                .on_input(ComposeMessage::FilePathChanged)
                .padding(6)
                .size(fs.size(12))
                .width(300),
            Space::new().width(8),
            button(text("Up")).on_press(ComposeMessage::ComposeUp),
            Space::new().width(4),
            button(text("Down")).on_press(ComposeMessage::ComposeDown(self.compose_file.clone())),
            Space::new().width(4),
            button(text("Logs")).on_press(ComposeMessage::ShowLogs(self.compose_file.clone())),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        let table_config = DataTableConfig {
            columns: vec![
                Column {
                    header: "STACK".into(),
                    width: 200.0,
                },
                Column {
                    header: "FILE".into(),
                    width: 300.0,
                },
                Column {
                    header: "STATUS".into(),
                    width: 120.0,
                },
                Column {
                    header: "SERVICES".into(),
                    width: 200.0,
                },
            ],
            row_height: 28.0,
        };

        let rows: Vec<Vec<String>> = self
            .stacks
            .iter()
            .map(|s| {
                vec![
                    s.name.clone(),
                    s.file.clone(),
                    s.status.clone(),
                    s.services.join(", "),
                ]
            })
            .collect();

        let table = data_table(
            table_config,
            rows,
            None,
            |_| ComposeMessage::Noop,
            None::<fn(usize) -> ComposeMessage>,
            None::<fn(usize) -> ComposeMessage>,
            None,
            false,
            self.font_size,
        );

        container(
            column![
                text("Docker Compose").size(fs.size(20)),
                compose_bar,
                Space::new().height(12),
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
