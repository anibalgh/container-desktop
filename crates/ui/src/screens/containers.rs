use iced::widget::{button, column, container, pick_list, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Padding, Theme};

use crate::typography::FontScale;
use crate::widgets::data_table::{data_table, Column, DataTableConfig};
use crate::widgets::log_viewer::{log_viewer, LogEntry};
use domain::entities::{Container, ContainerState};

// Static date/time picker value arrays (must outlive borrowed references in views).
static YEARS: &[u16] = &[
    2020, 2021, 2022, 2023, 2024, 2025, 2026, 2027, 2028, 2029, 2030,
];
static MONTHS: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
static DAYS: &[u8] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31,
];
static HOURS: &[u8] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
];
static MINUTES: &[u8] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
];

/// State for the containers screen.
pub struct ContainersScreen {
    pub font_size: u16,
    pub containers: Vec<Container>,
    pub selected_index: Option<usize>,
    pub selected_container: Option<Container>,
    pub log_entries: Vec<LogEntry>,
    pub log_tail_lines: String,
    pub log_since: String,
    pub log_until: String,
    // Date/time picker fields for "Since" filter
    /// Sort state for the container table.
    pub sort_column: Option<usize>,
    pub sort_ascending: bool,
    /// Column widths (updated via resize handles).
    pub column_widths: Vec<f32>,
    pub log_since_year: Option<u16>,
    pub log_since_month: Option<u8>,
    pub log_since_day: Option<u8>,
    pub log_since_hour: Option<u8>,
    pub log_since_minute: Option<u8>,
    // Date/time picker fields for "Until" filter
    pub log_until_year: Option<u16>,
    pub log_until_month: Option<u8>,
    pub log_until_day: Option<u8>,
    pub log_until_hour: Option<u8>,
    pub log_until_minute: Option<u8>,
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
            font_size: 14,
            containers: Vec::new(),
            selected_index: None,
            selected_container: None,
            log_entries: Vec::new(),
            log_tail_lines: String::from("200"),
            log_since: String::new(),
            log_until: String::new(),
            sort_column: None,
            sort_ascending: true,
            column_widths: vec![180.0, 200.0, 100.0, 180.0, 140.0],
            log_since_year: None,
            log_since_month: None,
            log_since_day: None,
            log_since_hour: None,
            log_since_minute: None,
            log_until_year: None,
            log_until_month: None,
            log_until_day: None,
            log_until_hour: None,
            log_until_minute: None,
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
    LogSinceYearChanged(u16),
    LogSinceMonthChanged(u8),
    LogSinceDayChanged(u8),
    LogSinceHourChanged(u8),
    LogSinceMinuteChanged(u8),
    LogUntilYearChanged(u16),
    LogUntilMonthChanged(u8),
    LogUntilDayChanged(u8),
    LogUntilHourChanged(u8),
    LogUntilMinuteChanged(u8),
    LogsLoaded(Vec<LogEntry>),
    TerminalOutput(String),
    TerminalInput(String),
    TerminalShellChanged(String),
    SendTerminalInput,
    CommandResult(String),
    CloseLogs,
    CloseTerminal,
    CreateContainer,
    SortColumn(usize),
    ResizeColumn(usize),
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
            ContainersMessage::LogSinceYearChanged(y) => {
                self.log_since_year = Some(y);
                self.rebuild_log_since();
                iced::Task::none()
            }
            ContainersMessage::LogSinceMonthChanged(m) => {
                self.log_since_month = Some(m);
                self.rebuild_log_since();
                iced::Task::none()
            }
            ContainersMessage::LogSinceDayChanged(d) => {
                self.log_since_day = Some(d);
                self.rebuild_log_since();
                iced::Task::none()
            }
            ContainersMessage::LogSinceHourChanged(h) => {
                self.log_since_hour = Some(h);
                self.rebuild_log_since();
                iced::Task::none()
            }
            ContainersMessage::LogSinceMinuteChanged(m) => {
                self.log_since_minute = Some(m);
                self.rebuild_log_since();
                iced::Task::none()
            }
            ContainersMessage::LogUntilYearChanged(y) => {
                self.log_until_year = Some(y);
                self.rebuild_log_until();
                iced::Task::none()
            }
            ContainersMessage::LogUntilMonthChanged(m) => {
                self.log_until_month = Some(m);
                self.rebuild_log_until();
                iced::Task::none()
            }
            ContainersMessage::LogUntilDayChanged(d) => {
                self.log_until_day = Some(d);
                self.rebuild_log_until();
                iced::Task::none()
            }
            ContainersMessage::LogUntilHourChanged(h) => {
                self.log_until_hour = Some(h);
                self.rebuild_log_until();
                iced::Task::none()
            }
            ContainersMessage::LogUntilMinuteChanged(m) => {
                self.log_until_minute = Some(m);
                self.rebuild_log_until();
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
            ContainersMessage::SortColumn(col) => {
                if self.sort_column == Some(col) {
                    self.sort_ascending = !self.sort_ascending;
                } else {
                    self.sort_column = Some(col);
                    self.sort_ascending = true;
                }
                self.sort_containers();
                iced::Task::none()
            }
            ContainersMessage::ResizeColumn(col) => {
                if col < self.column_widths.len() {
                    // Cycle: 80 → 150 → 250 → 80
                    self.column_widths[col] = match self.column_widths[col] as i32 {
                        w if w < 120 => 150.0,
                        w if w < 200 => 250.0,
                        _ => 80.0,
                    };
                }
                iced::Task::none()
            }
            _ => iced::Task::none(),
        }
    }

    fn sort_containers(&mut self) {
        if let Some(col) = self.sort_column {
            let asc = self.sort_ascending;
            self.containers.sort_by(|a, b| {
                let a_str: String = match col {
                    0 => a.name.clone(),
                    1 => a.image.clone(),
                    2 => format!("{:?}", a.state),
                    3 => a
                        .ports
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    4 => a.created.clone(),
                    _ => return std::cmp::Ordering::Equal,
                };
                let b_str: String = match col {
                    0 => b.name.clone(),
                    1 => b.image.clone(),
                    2 => format!("{:?}", b.state),
                    3 => b
                        .ports
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    4 => b.created.clone(),
                    _ => return std::cmp::Ordering::Equal,
                };
                if asc {
                    a_str.cmp(&b_str)
                } else {
                    b_str.cmp(&a_str)
                }
            });
        }
    }

    fn rebuild_log_since(&mut self) {
        self.log_since = build_timestamp(
            self.log_since_year,
            self.log_since_month,
            self.log_since_day,
            self.log_since_hour,
            self.log_since_minute,
        );
    }

    fn rebuild_log_until(&mut self) {
        self.log_until = build_timestamp(
            self.log_until_year,
            self.log_until_month,
            self.log_until_day,
            self.log_until_hour,
            self.log_until_minute,
        );
    }

    pub fn view<'a>(
        &'a self,
        _docker_client: Option<&'a std::sync::Arc<infrastructure::DockerClient>>,
    ) -> Element<'a, ContainersMessage, Theme, iced::Renderer> {
        let fs = FontScale::new(self.font_size);
        if self.showing_logs {
            return self.view_logs();
        }
        if self.showing_terminal && !self.terminal_connected {
            return self.view_terminal_setup();
        }
        if self.showing_terminal && self.terminal_connected {
            return self.view_terminal();
        }

        // Build table config with current (possibly resized) column widths
        let widths = &self.column_widths;
        let table_config = DataTableConfig {
            columns: vec![
                Column {
                    header: "NAME".into(),
                    width: *widths.first().unwrap_or(&180.0),
                },
                Column {
                    header: "IMAGE".into(),
                    width: *widths.get(1).unwrap_or(&200.0),
                },
                Column {
                    header: "STATUS".into(),
                    width: *widths.get(2).unwrap_or(&100.0),
                },
                Column {
                    header: "PORTS".into(),
                    width: *widths.get(3).unwrap_or(&180.0),
                },
                Column {
                    header: "CREATED".into(),
                    width: *widths.get(4).unwrap_or(&140.0),
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
            Some(ContainersMessage::SortColumn),
            Some(ContainersMessage::ResizeColumn),
            self.sort_column,
            self.sort_ascending,
            self.font_size,
        );

        let action_bar = self.action_bar();

        container(
            column![
                text("Containers").size(fs.size(20)),
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
        let _fs = FontScale::new(self.font_size);
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
        let fs = FontScale::new(self.font_size);
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

        let since_label: Element<'_, ContainersMessage, Theme, iced::Renderer> =
            text("Since:").size(fs.size(12)).into();
        let since_year = datetime_pick(
            "Y",
            YEARS,
            self.log_since_year,
            ContainersMessage::LogSinceYearChanged,
            self.font_size,
        );
        let since_month = datetime_pick(
            "M",
            MONTHS,
            self.log_since_month,
            ContainersMessage::LogSinceMonthChanged,
            self.font_size,
        );
        let since_day = datetime_pick(
            "D",
            DAYS,
            self.log_since_day,
            ContainersMessage::LogSinceDayChanged,
            self.font_size,
        );
        let since_hour = datetime_pick(
            "h",
            HOURS,
            self.log_since_hour,
            ContainersMessage::LogSinceHourChanged,
            self.font_size,
        );
        let since_min = datetime_pick(
            "m",
            MINUTES,
            self.log_since_minute,
            ContainersMessage::LogSinceMinuteChanged,
            self.font_size,
        );

        let until_label: Element<'_, ContainersMessage, Theme, iced::Renderer> =
            text("Until:").size(fs.size(12)).into();
        let until_year = datetime_pick(
            "Y",
            YEARS,
            self.log_until_year,
            ContainersMessage::LogUntilYearChanged,
            self.font_size,
        );
        let until_month = datetime_pick(
            "M",
            MONTHS,
            self.log_until_month,
            ContainersMessage::LogUntilMonthChanged,
            self.font_size,
        );
        let until_day = datetime_pick(
            "D",
            DAYS,
            self.log_until_day,
            ContainersMessage::LogUntilDayChanged,
            self.font_size,
        );
        let until_hour = datetime_pick(
            "h",
            HOURS,
            self.log_until_hour,
            ContainersMessage::LogUntilHourChanged,
            self.font_size,
        );
        let until_min = datetime_pick(
            "m",
            MINUTES,
            self.log_until_minute,
            ContainersMessage::LogUntilMinuteChanged,
            self.font_size,
        );

        let controls = row![
            text("Lines:").size(fs.size(12)),
            text_input("200", &self.log_tail_lines)
                .on_input(ContainersMessage::LogTailLinesChanged)
                .padding(4)
                .size(fs.size(12))
                .width(80),
            Space::new().width(16),
            since_label,
            since_year,
            since_month,
            since_day,
            since_hour,
            since_min,
            Space::new().width(16),
            until_label,
            until_year,
            until_month,
            until_day,
            until_hour,
            until_min,
            Space::new().width(Length::Fill),
            reload_btn,
            Space::new().width(4),
            button(text("Close")).on_press(ContainersMessage::CloseLogs),
        ]
        .spacing(2)
        .align_y(Alignment::Center);

        let mut col = column![
            text("Container Logs").size(fs.size(20)),
            controls,
            Space::new().height(8),
        ]
        .spacing(4);

        if !tail_valid {
            col = col.push(
                text("Lines must be a positive integer greater than 0")
                    .size(fs.size(11))
                    .color(iced::Color::from_rgb(0.9, 0.3, 0.3)),
            );
            col = col.push(Space::new().height(4));
        }

        col = col.push(log_viewer(&self.log_entries, true, self.font_size));

        container(col.padding(Padding::new(16.0)))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_terminal_setup<'a>(&'a self) -> Element<'a, ContainersMessage, Theme, iced::Renderer> {
        let fs = FontScale::new(self.font_size);
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
            text("Terminal").size(fs.size(20)),
            Space::new().height(12),
            text("Select the shell type for the container:").size(fs.size(14)),
            Space::new().height(8),
            row![text("Shell:").size(fs.size(12)), shell_picker,]
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
                            .size(fs.size(12))
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
        let fs = FontScale::new(self.font_size);
        use iced::widget::{scrollable, text_input};

        container(
            column![
                row![
                    text("Terminal").size(fs.size(20)),
                    Space::new().width(Length::Fill),
                    button(text("Disconnect")).on_press(ContainersMessage::CloseTerminal),
                ]
                .align_y(Alignment::Center),
                Space::new().height(8),
                container(
                    scrollable(
                        text(&self.terminal_output)
                            .size(fs.size(12))
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
                    .size(fs.size(12))
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

/// Builds an RFC 3339 timestamp string from optional date/time components.
///
/// Returns an empty string if year is `None`. Missing components default to 1
/// (for month/day) or 0 (for hour/minute/second).
fn build_timestamp(
    year: Option<u16>,
    month: Option<u8>,
    day: Option<u8>,
    hour: Option<u8>,
    minute: Option<u8>,
) -> String {
    let y = match year {
        Some(y) => y,
        None => return String::new(),
    };
    let mo = month.unwrap_or(1).clamp(1, 12);
    let d = day.unwrap_or(1).clamp(1, 31);
    let h = hour.unwrap_or(0).min(23);
    let m = minute.unwrap_or(0).min(59);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:00Z")
}

/// Builds a pick_list of values with labels, returning the selected value.
fn datetime_pick<'a, T, Message>(
    label: &'a str,
    values: &'a [T],
    selected: Option<T>,
    on_select: impl Fn(T) -> Message + 'a,
    font_size: u16,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    T: Clone + std::fmt::Display + 'a,
    Message: Clone + 'a,
{
    let fs = FontScale::new(font_size);
    let label_text = text(label).size(fs.size(10));
    let pick = pick_list(
        values.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
        selected.map(|v| v.to_string()),
        move |s| {
            let val = values
                .iter()
                .find(|v| v.to_string() == s)
                .cloned()
                .unwrap_or_else(|| values[0].clone());
            on_select(val)
        },
    );
    row![label_text, pick]
        .spacing(2)
        .align_y(Alignment::Center)
        .into()
}
