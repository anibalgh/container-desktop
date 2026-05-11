use iced::widget::{button, column, container, pick_list, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Padding, Theme};

use crate::widgets::data_table::{data_table, Column, DataTableConfig};
use crate::widgets::log_viewer::{log_viewer, LogEntry};
use domain::entities::{Container, ContainerState};

/// State for the containers screen.
pub struct ContainersScreen {
    pub containers: Vec<Container>,
    pub selected_index: Option<usize>,
    pub selected_container: Option<Container>,
    pub log_entries: Vec<LogEntry>,
    pub log_tail_lines: String,
    pub log_since: String,
    pub log_until: String,
    pub showing_logs: bool,
    pub showing_terminal: bool,
    pub terminal_connected: bool,
    pub terminal_output: String,
    pub terminal_input: String,
    pub terminal_shell: String,
    pub terminal_root: bool,
    pub terminal_connecting: bool,
    pub error_message: Option<String>,
    pub is_loading: bool,
}

impl Default for ContainersScreen {
    fn default() -> Self {
        Self {
            containers: Vec::new(),
            selected_index: None,
            selected_container: None,
            log_entries: Vec::new(),
            log_tail_lines: String::from("200"),
            log_since: String::new(),
            log_until: String::new(),
            showing_logs: false,
            showing_terminal: false,
            terminal_connected: false,
            terminal_output: String::new(),
            terminal_input: String::new(),
            terminal_shell: String::from("sh"),
            terminal_root: false,
            terminal_connecting: false,
            error_message: None,
            is_loading: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContainersMessage {
    LoadContainers,
    ContainersLoaded(Vec<Container>),
    SelectContainer(usize),
    StartContainer(String),
    StopContainer(String),
    RestartContainer(String),
    RemoveContainer(String),
    ShowLogs(String),
    ShowTerminal(String),
    ConnectTerminal,
    TerminalConnected(String),
    TerminalRootToggled(bool),
    RefreshLogs,
    LogTailLinesChanged(String),
    LogSinceChanged(String),
    LogUntilChanged(String),
    LogsLoaded(Vec<LogEntry>),
    TerminalOutput(String),
    TerminalInput(String),
    TerminalShellChanged(String),
    SendTerminalInput,
    CommandResult(String),
    CloseLogs,
    CloseTerminal,
    CreateContainer,
    Error(String),
    Noop,
}

impl ContainersScreen {
    pub fn update(&mut self, message: ContainersMessage) -> iced::Task<ContainersMessage> {
        match message {
            ContainersMessage::SelectContainer(i) => {
                self.selected_index = Some(i);
                self.selected_container = self.containers.get(i).cloned();
                self.showing_logs = false;
                self.showing_terminal = false;
                self.terminal_connected = false;
                iced::Task::none()
            }
            ContainersMessage::StartContainer(id) => {
                let id_clone = id.clone();
                iced::Task::perform(
                    async move {
                        // Would call docker_client.start_container(&id).await
                        Ok::<_, String>(format!("Started {id_clone}"))
                    },
                    |r| match r {
                        Ok(_) => ContainersMessage::LoadContainers,
                        Err(e) => ContainersMessage::Error(e),
                    },
                )
            }
            ContainersMessage::StopContainer(id) => {
                let id_clone = id.clone();
                iced::Task::perform(async move { Ok::<_, String>(id_clone) }, |r| match r {
                    Ok(_) => ContainersMessage::LoadContainers,
                    Err(e) => ContainersMessage::Error(e),
                })
            }
            ContainersMessage::RestartContainer(id) => {
                let id_clone = id.clone();
                iced::Task::perform(async move { Ok::<_, String>(id_clone) }, |r| match r {
                    Ok(_) => ContainersMessage::LoadContainers,
                    Err(e) => ContainersMessage::Error(e),
                })
            }
            ContainersMessage::RemoveContainer(id) => {
                let id_clone = id.clone();
                iced::Task::perform(async move { Ok::<_, String>(id_clone) }, |r| match r {
                    Ok(_) => ContainersMessage::LoadContainers,
                    Err(e) => ContainersMessage::Error(e),
                })
            }
            ContainersMessage::LoadContainers => {
                self.is_loading = true;
                iced::Task::perform(
                    async move { Ok::<Vec<Container>, String>(Vec::new()) },
                    |r| match r {
                        Ok(c) => ContainersMessage::ContainersLoaded(c),
                        Err(e) => ContainersMessage::Error(e),
                    },
                )
            }
            ContainersMessage::ContainersLoaded(containers) => {
                self.containers = containers;
                self.is_loading = false;
                iced::Task::none()
            }
            ContainersMessage::ShowLogs(_id) => {
                self.showing_logs = true;
                self.showing_terminal = false;
                self.terminal_connected = false;
                self.log_entries = Vec::new();
                iced::Task::none()
            }
            ContainersMessage::ShowTerminal(_id) => {
                self.showing_terminal = true;
                self.showing_logs = false;
                self.terminal_connected = false;
                self.terminal_connecting = false;
                self.terminal_output = String::new();
                self.terminal_input.clear();
                iced::Task::none()
            }
            ContainersMessage::ConnectTerminal => {
                self.terminal_connecting = true;
                self.terminal_output =
                    format!("Connecting to container with {}...\n", self.terminal_shell);
                iced::Task::none()
            }
            ContainersMessage::TerminalConnected(msg) => {
                self.terminal_connected = true;
                self.terminal_connecting = false;
                self.terminal_output.push_str(&msg);
                self.terminal_output.push_str("$ ");
                iced::Task::none()
            }
            ContainersMessage::CloseLogs => {
                self.showing_logs = false;
                iced::Task::none()
            }
            ContainersMessage::CloseTerminal => {
                self.showing_terminal = false;
                self.terminal_connected = false;
                self.terminal_connecting = false;
                iced::Task::none()
            }
            ContainersMessage::RefreshLogs => iced::Task::none(),
            ContainersMessage::LogsLoaded(entries) => {
                self.log_entries = entries;
                iced::Task::none()
            }
            ContainersMessage::LogTailLinesChanged(value) => {
                if value.is_empty() || value.chars().all(|c| c.is_ascii_digit()) {
                    self.log_tail_lines = value;
                }
                iced::Task::none()
            }
            ContainersMessage::LogSinceChanged(value) => {
                self.log_since = value;
                iced::Task::none()
            }
            ContainersMessage::LogUntilChanged(value) => {
                self.log_until = value;
                iced::Task::none()
            }
            ContainersMessage::TerminalOutput(_) => iced::Task::none(),
            ContainersMessage::TerminalInput(input) => {
                self.terminal_input = input;
                iced::Task::none()
            }
            ContainersMessage::TerminalShellChanged(shell) => {
                self.terminal_shell = shell;
                iced::Task::none()
            }
            ContainersMessage::TerminalRootToggled(root) => {
                self.terminal_root = root;
                iced::Task::none()
            }
            ContainersMessage::SendTerminalInput => {
                if !self.terminal_input.is_empty() {
                    self.terminal_output
                        .push_str(&format!("{}\n", self.terminal_input));
                    self.terminal_input.clear();
                }
                iced::Task::none()
            }
            ContainersMessage::CommandResult(output) => {
                self.terminal_output.push_str(&output);
                self.terminal_output.push_str("$ ");
                iced::Task::none()
            }
            _ => iced::Task::none(),
        }
    }

    pub fn view<'a>(
        &'a self,
        _docker_client: Option<&'a std::sync::Arc<infrastructure::DockerClient>>,
    ) -> Element<'a, ContainersMessage, Theme, iced::Renderer> {
        if self.showing_logs {
            return self.view_logs();
        }
        if self.showing_terminal && !self.terminal_connected {
            return self.view_terminal_setup();
        }
        if self.showing_terminal && self.terminal_connected {
            return self.view_terminal();
        }

        let table_config = DataTableConfig {
            columns: vec![
                Column {
                    header: "NAME".into(),
                    width: 180.0,
                },
                Column {
                    header: "IMAGE".into(),
                    width: 200.0,
                },
                Column {
                    header: "STATUS".into(),
                    width: 100.0,
                },
                Column {
                    header: "PORTS".into(),
                    width: 180.0,
                },
                Column {
                    header: "CREATED".into(),
                    width: 140.0,
                },
            ],
            row_height: 28.0,
        };

        let rows: Vec<Vec<String>> = self
            .containers
            .iter()
            .map(|c| {
                vec![
                    c.name.clone(),
                    c.image.clone(),
                    format!("{:?}", c.state),
                    c.ports
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    c.created.clone(),
                ]
            })
            .collect();

        let table = data_table(
            table_config,
            rows,
            self.selected_index,
            ContainersMessage::SelectContainer,
        );

        let action_bar = self.action_bar();

        container(
            column![
                text("Containers").size(20),
                action_bar,
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

    fn action_bar<'a>(&'a self) -> Element<'a, ContainersMessage, Theme, iced::Renderer> {
        let _has_selection = self.selected_container.is_some();

        let buttons: Vec<Element<'_, ContainersMessage, Theme, iced::Renderer>> = vec![
            button(text("Refresh"))
                .on_press(ContainersMessage::LoadContainers)
                .into(),
            Space::new().width(4).into(),
            button(text("+ Create"))
                .on_press(ContainersMessage::CreateContainer)
                .into(),
            Space::new().width(8).into(),
        ];

        let action_buttons: Vec<Element<'_, ContainersMessage, Theme, iced::Renderer>> =
            if let Some(c) = &self.selected_container {
                let id = c.id.clone();
                let running = matches!(c.state, ContainerState::Running);
                let mut actions = vec![Space::new().width(8).into()];

                if running {
                    actions.push(
                        button(text("Stop"))
                            .on_press(ContainersMessage::StopContainer(id.clone()))
                            .into(),
                    );
                    actions.push(Space::new().width(4).into());
                    actions.push(
                        button(text("Restart"))
                            .on_press(ContainersMessage::RestartContainer(id.clone()))
                            .into(),
                    );
                } else {
                    actions.push(
                        button(text("Start"))
                            .on_press(ContainersMessage::StartContainer(id.clone()))
                            .into(),
                    );
                }
                actions.push(Space::new().width(4).into());
                actions.push(
                    button(text("Remove"))
                        .on_press(ContainersMessage::RemoveContainer(id.clone()))
                        .into(),
                );
                actions.push(Space::new().width(4).into());
                actions.push(
                    button(text("Logs"))
                        .on_press(ContainersMessage::ShowLogs(id.clone()))
                        .into(),
                );
                actions.push(Space::new().width(4).into());
                actions.push(
                    button(text("Terminal"))
                        .on_press(ContainersMessage::ShowTerminal(id.clone()))
                        .into(),
                );

                actions
            } else {
                Vec::new()
            };

        row(buttons
            .into_iter()
            .chain(action_buttons)
            .collect::<Vec<_>>())
        .align_y(Alignment::Center)
        .into()
    }

    fn view_logs<'a>(&'a self) -> Element<'a, ContainersMessage, Theme, iced::Renderer> {
        let reload_btn = if self.is_loading {
            button(text("Loading..."))
        } else {
            button(text("Reload")).on_press(ContainersMessage::RefreshLogs)
        };

        let tail_valid = !self.log_tail_lines.is_empty()
            && self
                .log_tail_lines
                .parse::<u32>()
                .map(|n| n > 0)
                .unwrap_or(false);

        let controls = row![
            text("Lines:").size(12),
            text_input("200", &self.log_tail_lines)
                .on_input(ContainersMessage::LogTailLinesChanged)
                .padding(4)
                .size(12)
                .width(80),
            Space::new().width(12),
            text("Since:").size(12),
            text_input("timestamp or empty", &self.log_since)
                .on_input(ContainersMessage::LogSinceChanged)
                .padding(4)
                .size(12)
                .width(160),
            Space::new().width(12),
            text("Until:").size(12),
            text_input("timestamp or empty", &self.log_until)
                .on_input(ContainersMessage::LogUntilChanged)
                .padding(4)
                .size(12)
                .width(160),
            Space::new().width(Length::Fill),
            reload_btn,
            Space::new().width(4),
            button(text("Close")).on_press(ContainersMessage::CloseLogs),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        let mut col = column![
            text("Container Logs").size(20),
            controls,
            Space::new().height(8),
        ]
        .spacing(4);

        if !tail_valid {
            col = col.push(
                text("Lines must be a positive integer greater than 0")
                    .size(11)
                    .color(iced::Color::from_rgb(0.9, 0.3, 0.3)),
            );
            col = col.push(Space::new().height(4));
        }

        col = col.push(log_viewer(&self.log_entries, true));

        container(col.padding(Padding::new(16.0)))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_terminal_setup<'a>(&'a self) -> Element<'a, ContainersMessage, Theme, iced::Renderer> {
        use iced::widget::toggler;

        let shells = vec!["sh", "bash", "zsh", "ash", "fish"];
        let shell_picker = pick_list(
            shells.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            Some(self.terminal_shell.clone()),
            ContainersMessage::TerminalShellChanged,
        );

        let connect_btn = if self.terminal_connecting {
            button(text("Connecting..."))
        } else {
            button(text("Connect")).on_press(ContainersMessage::ConnectTerminal)
        };

        let mut content = column![
            text("Terminal").size(20),
            Space::new().height(12),
            text("Select the shell type for the container:").size(14),
            Space::new().height(8),
            row![text("Shell:").size(12), shell_picker,]
                .spacing(8)
                .align_y(Alignment::Center),
            Space::new().height(8),
            toggler(self.terminal_root)
                .label("Connect as root (-u root)")
                .on_toggle(ContainersMessage::TerminalRootToggled),
            Space::new().height(12),
            connect_btn,
        ]
        .spacing(4)
        .padding(Padding::new(16.0));

        if !self.terminal_output.is_empty() {
            use iced::widget::scrollable;
            content = content.push(Space::new().height(8));
            content = content.push(
                container(
                    scrollable(
                        text(&self.terminal_output)
                            .size(12)
                            .font(iced::Font::MONOSPACE),
                    )
                    .height(Length::Fill)
                    .width(Length::Fill),
                )
                .height(Length::Fill)
                .width(Length::Fill)
                .style(move |theme: &Theme| {
                    let p = theme.extended_palette();
                    container::Style {
                        background: Some(iced::Background::Color(p.background.base.color)),
                        ..Default::default()
                    }
                })
                .padding(8),
            );
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_terminal<'a>(&'a self) -> Element<'a, ContainersMessage, Theme, iced::Renderer> {
        use iced::widget::{scrollable, text_input};

        container(
            column![
                row![
                    text("Terminal").size(20),
                    Space::new().width(Length::Fill),
                    button(text("Disconnect")).on_press(ContainersMessage::CloseTerminal),
                ]
                .align_y(Alignment::Center),
                Space::new().height(8),
                container(
                    scrollable(
                        text(&self.terminal_output)
                            .size(12)
                            .font(iced::Font::MONOSPACE),
                    )
                    .height(Length::Fill)
                    .width(Length::Fill),
                )
                .height(Length::Fill)
                .width(Length::Fill)
                .style(move |theme: &Theme| {
                    let p = theme.extended_palette();
                    container::Style {
                        background: Some(iced::Background::Color(p.background.base.color)),
                        ..Default::default()
                    }
                })
                .padding(8),
                Space::new().height(4),
                text_input("Type command...", &self.terminal_input)
                    .on_input(ContainersMessage::TerminalInput)
                    .on_submit(ContainersMessage::SendTerminalInput)
                    .padding(6)
                    .size(12)
                    .font(iced::Font::MONOSPACE),
            ]
            .spacing(4)
            .padding(Padding::new(16.0)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
