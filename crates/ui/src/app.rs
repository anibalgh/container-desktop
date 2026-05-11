use iced::widget::{column, container, row};
use iced::{Element, Length, Task, Theme};

use crate::screens::{
    compose::ComposeMessage as ComposeMsg, compose::ComposeScreen,
    containers::ContainersMessage as ContainersMsg, containers::ContainersScreen,
    dashboard::DashboardScreen, images::ImagesMessage as ImagesMsg, images::ImagesScreen,
    networks::NetworksMessage as NetworksMsg, networks::NetworksScreen,
    settings::SettingsMessage as SettingsMsg, settings::SettingsScreen,
    volumes::VolumesMessage as VolumesMsg, volumes::VolumesScreen,
};
use crate::theme::ThemeManager;
use crate::widgets::{sidebar, status_bar};
use domain::entities::{AppSettings, DockerInfo, ThemeSetting};
use domain::repository::{
    ComposeRepository, ContainerRepository, DockerConnectionRepository, ImageRepository,
    NetworkRepository, SettingsRepository, VolumeRepository,
};
use infrastructure::{ComposeClient, ConfigManager, DockerClient};
use std::sync::Arc;

pub struct ContainerDesktop {
    pub active_screen: usize,
    pub settings: AppSettings,
    pub docker_client: Arc<DockerClient>,
    config_manager: Arc<ConfigManager>,
    pub docker_info: Option<DockerInfo>,
    pub dashboard: DashboardScreen,
    pub images: ImagesScreen,
    pub containers: ContainersScreen,
    pub volumes: VolumesScreen,
    pub networks: NetworksScreen,
    pub compose: ComposeScreen,
    pub settings_screen: SettingsScreen,
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(usize),
    DockerConnected(Result<DockerInfo, String>),
    Containers(ContainersMsg),
    Images(ImagesMsg),
    Volumes(VolumesMsg),
    Networks(NetworksMsg),
    Compose(ComposeMsg),
    Settings(SettingsMsg),
    ThemeChanged(ThemeSetting),
    Refresh,
    Noop,
}

impl ContainerDesktop {
    pub fn boot() -> (Self, Task<Message>) {
        let config_manager = Arc::new(ConfigManager::new().expect("config"));
        let settings = AppSettings::default();
        let docker_client = Arc::new(DockerClient::new(settings.endpoint.clone()));

        let load_task = {
            let cm = config_manager.clone();
            Task::perform(
                async move { cm.load_settings().await.ok() },
                |maybe| match maybe {
                    Some(s) => Message::ThemeChanged(s.theme_setting),
                    None => Message::Noop,
                },
            )
        };

        let connect_task = {
            let client = docker_client.clone();
            Task::perform(
                async move {
                    client.connect().await.map_err(|e| e.to_string())?;
                    client.test_connection().await.map_err(|e| e.to_string())
                },
                Message::DockerConnected,
            )
        };

        let app = Self {
            active_screen: 0,
            settings,
            docker_client,
            config_manager,
            docker_info: None,
            dashboard: DashboardScreen::default(),
            images: ImagesScreen::default(),
            containers: ContainersScreen::default(),
            volumes: VolumesScreen::default(),
            networks: NetworksScreen::default(),
            compose: ComposeScreen::default(),
            settings_screen: SettingsScreen::default(),
        };

        (app, Task::batch(vec![load_task, connect_task]))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(index) => {
                self.active_screen = index;
                match index {
                    1 => self.handle_containers(ContainersMsg::LoadContainers),
                    2 => self.handle_images(ImagesMsg::LoadImages),
                    3 => self.handle_volumes(VolumesMsg::LoadVolumes),
                    4 => self.handle_networks(NetworksMsg::LoadNetworks),
                    _ => Task::none(),
                }
            }
            Message::DockerConnected(result) => {
                match result {
                    Ok(info) => self.docker_info = Some(info),
                    Err(e) => tracing::warn!("Docker connection failed: {e}"),
                }
                Task::none()
            }
            Message::ThemeChanged(new_setting) => {
                self.settings.theme_setting = new_setting.clone();
                self.settings_screen.theme_setting = new_setting;
                let settings = self.settings.clone();
                let cm = self.config_manager.clone();
                Task::perform(
                    async move { cm.save_settings(&settings).await.map_err(|e| e.to_string()) },
                    |_| Message::Noop,
                )
            }
            Message::Refresh => Task::none(),
            Message::Images(img_msg) => self.handle_images(img_msg),
            Message::Containers(ct_msg) => self.handle_containers(ct_msg),
            Message::Volumes(vol_msg) => self.handle_volumes(vol_msg),
            Message::Networks(net_msg) => self.handle_networks(net_msg),
            Message::Compose(comp_msg) => self.handle_compose(comp_msg),
            Message::Settings(set_msg) => self.handle_settings(set_msg),
            Message::Noop => Task::none(),
        }
    }

    fn handle_images(&mut self, msg: ImagesMsg) -> Task<Message> {
        match msg {
            ImagesMsg::LoadImages => {
                let client = self.docker_client.clone();
                Task::perform(
                    async move { client.list_images().await.map_err(|e| e.to_string()) },
                    |r| {
                        Message::Images(match r {
                            Ok(imgs) => ImagesMsg::ImagesLoaded(imgs),
                            Err(e) => ImagesMsg::Error(e),
                        })
                    },
                )
            }
            ImagesMsg::PullImage => {
                let name = self.images.pull_image_name.clone();
                let tag = if self.images.pull_image_tag.is_empty() {
                    None
                } else {
                    Some(self.images.pull_image_tag.clone())
                };
                let client = self.docker_client.clone();
                Task::perform(
                    async move {
                        let _ = client.pull_image(&name, tag.as_deref()).await;
                        Ok::<_, String>(())
                    },
                    |r| match r {
                        Ok(()) => Message::Images(ImagesMsg::LoadImages),
                        Err(e) => Message::Images(ImagesMsg::Error(e)),
                    },
                )
            }
            ImagesMsg::RemoveImage(id) => {
                let id_c = id.clone();
                let client = self.docker_client.clone();
                Task::perform(
                    async move { client.remove_image(&id_c).await.map_err(|e| e.to_string()) },
                    |r| {
                        Message::Images(match r {
                            Ok(()) => ImagesMsg::LoadImages,
                            Err(e) => ImagesMsg::Error(e),
                        })
                    },
                )
            }
            other => self.images.update(other).map(Message::Images),
        }
    }

    fn handle_containers(&mut self, msg: ContainersMsg) -> Task<Message> {
        match msg {
            ContainersMsg::LoadContainers => {
                let client = self.docker_client.clone();
                Task::perform(
                    async move {
                        client
                            .list_containers(true)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    |r| {
                        Message::Containers(match r {
                            Ok(ctrs) => ContainersMsg::ContainersLoaded(ctrs),
                            Err(e) => ContainersMsg::Error(e),
                        })
                    },
                )
            }
            ContainersMsg::StartContainer(id) => {
                let id_c = id.clone();
                let client = self.docker_client.clone();
                Task::perform(
                    async move {
                        client
                            .start_container(&id_c)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    |r| {
                        Message::Containers(match r {
                            Ok(()) => ContainersMsg::LoadContainers,
                            Err(e) => ContainersMsg::Error(e),
                        })
                    },
                )
            }
            ContainersMsg::StopContainer(id) => {
                let id_c = id.clone();
                let client = self.docker_client.clone();
                Task::perform(
                    async move {
                        client
                            .stop_container(&id_c)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    |r| {
                        Message::Containers(match r {
                            Ok(()) => ContainersMsg::LoadContainers,
                            Err(e) => ContainersMsg::Error(e),
                        })
                    },
                )
            }
            ContainersMsg::RestartContainer(id) => {
                let id_c = id.clone();
                let client = self.docker_client.clone();
                Task::perform(
                    async move {
                        client
                            .restart_container(&id_c)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    |r| {
                        Message::Containers(match r {
                            Ok(()) => ContainersMsg::LoadContainers,
                            Err(e) => ContainersMsg::Error(e),
                        })
                    },
                )
            }
            ContainersMsg::RemoveContainer(id) => {
                let id_c = id.clone();
                let client = self.docker_client.clone();
                Task::perform(
                    async move {
                        client
                            .remove_container(&id_c)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    |r| {
                        Message::Containers(match r {
                            Ok(()) => ContainersMsg::LoadContainers,
                            Err(e) => ContainersMsg::Error(e),
                        })
                    },
                )
            }
            ContainersMsg::ShowLogs(id) => {
                let id_c = id.clone();
                let client = self.docker_client.clone();
                let tail = self
                    .containers
                    .log_tail_lines
                    .parse::<u32>()
                    .ok()
                    .filter(|n| *n > 0);
                self.containers.showing_logs = true;
                self.containers.showing_terminal = false;
                self.containers.log_entries = Vec::new();
                self.containers.is_loading = true;
                Task::perform(
                    async move {
                        let stream = client
                            .container_logs(&id_c, tail, false)
                            .await
                            .map_err(|e| e.to_string())?;
                        use futures::StreamExt;
                        let mut entries = Vec::new();
                        futures::pin_mut!(stream);
                        while let Some(result) = stream.next().await {
                            match result {
                                Ok(log_line) => {
                                    entries.push(crate::widgets::log_viewer::LogEntry {
                                        line: log_line.content,
                                        is_stderr: matches!(
                                            log_line.stream,
                                            domain::entities::LogStream::Stderr
                                        ),
                                    });
                                }
                                Err(_) => {}
                            }
                        }
                        Ok::<_, String>(entries)
                    },
                    |r| {
                        Message::Containers(match r {
                            Ok(entries) => ContainersMsg::LogsLoaded(entries),
                            Err(e) => ContainersMsg::Error(e),
                        })
                    },
                )
            }
            ContainersMsg::RefreshLogs => {
                let container_id = self
                    .containers
                    .selected_container
                    .as_ref()
                    .map(|c| c.id.clone());
                let tail = self
                    .containers
                    .log_tail_lines
                    .parse::<u32>()
                    .ok()
                    .filter(|n| *n > 0);
                let client = self.docker_client.clone();
                self.containers.is_loading = true;
                match container_id {
                    Some(id) => Task::perform(
                        async move {
                            let stream = client
                                .container_logs(&id, tail, false)
                                .await
                                .map_err(|e| e.to_string())?;
                            use futures::StreamExt;
                            let mut entries = Vec::new();
                            futures::pin_mut!(stream);
                            while let Some(result) = stream.next().await {
                                match result {
                                    Ok(log_line) => {
                                        entries.push(crate::widgets::log_viewer::LogEntry {
                                            line: log_line.content,
                                            is_stderr: matches!(
                                                log_line.stream,
                                                domain::entities::LogStream::Stderr
                                            ),
                                        });
                                    }
                                    Err(_) => {}
                                }
                            }
                            Ok::<_, String>(entries)
                        },
                        |r| {
                            Message::Containers(match r {
                                Ok(entries) => ContainersMsg::LogsLoaded(entries),
                                Err(e) => ContainersMsg::Error(e),
                            })
                        },
                    ),
                    None => Task::none(),
                }
            }
            ContainersMsg::ShowTerminal(_id) => {
                self.containers.showing_terminal = true;
                self.containers.showing_logs = false;
                self.containers.terminal_connected = false;
                self.containers.terminal_connecting = false;
                self.containers.terminal_output.clear();
                self.containers.terminal_input.clear();
                Task::none()
            }
            ContainersMsg::ConnectTerminal => {
                let container_id = self
                    .containers
                    .selected_container
                    .as_ref()
                    .map(|c| c.id.clone())
                    .unwrap_or_default();
                let shell = self.containers.terminal_shell.clone();
                let root = self.containers.terminal_root;
                self.containers.terminal_connecting = true;
                self.containers
                    .terminal_output
                    .push_str(&format!("Connecting with {}...\n", shell));
                Task::perform(
                    async move {
                        let mut args = vec!["exec"];
                        if root {
                            args.push("-u");
                            args.push("root");
                        }
                        args.push(&container_id);
                        args.push(&shell);
                        args.push("-c");
                        args.push("echo connected");
                        let output = tokio::process::Command::new("docker")
                            .args(&args)
                            .output()
                            .await
                            .map_err(|e| format!("Failed to connect: {e}"))?;
                        if output.status.success() {
                            let mode = if root { " (root)" } else { "" };
                            Ok::<_, String>(format!(
                                "Connected to container {}{} with {}\n",
                                &container_id[..container_id.len().min(12)],
                                mode,
                                shell
                            ))
                        } else {
                            Err(format!(
                                "Connection failed: {}",
                                String::from_utf8_lossy(&output.stderr)
                            ))
                        }
                    },
                    |r| {
                        Message::Containers(match r {
                            Ok(msg) => ContainersMsg::TerminalConnected(msg),
                            Err(e) => ContainersMsg::TerminalConnected(format!("Error: {e}\n")),
                        })
                    },
                )
            }
            ContainersMsg::SendTerminalInput => {
                let cmd = self.containers.terminal_input.clone();
                if cmd.is_empty() {
                    return Task::none();
                }
                let container_id = self
                    .containers
                    .selected_container
                    .as_ref()
                    .map(|c| c.id.clone())
                    .unwrap_or_default();
                let shell = self.containers.terminal_shell.clone();
                let root = self.containers.terminal_root;
                self.containers
                    .terminal_output
                    .push_str(&format!("{}\n", cmd));
                self.containers.terminal_input.clear();
                Task::perform(
                    async move {
                        let mut args = vec!["exec"];
                        if root {
                            args.push("-u");
                            args.push("root");
                        }
                        args.push(&container_id);
                        args.push(&shell);
                        args.push("-c");
                        args.push(&cmd);
                        let output = tokio::process::Command::new("docker")
                            .args(&args)
                            .output()
                            .await
                            .map_err(|e| format!("Failed to execute: {e}"))?;
                        let mut result = String::new();
                        if !output.stdout.is_empty() {
                            result.push_str(&String::from_utf8_lossy(&output.stdout));
                        }
                        if !output.stderr.is_empty() {
                            result.push_str(&String::from_utf8_lossy(&output.stderr));
                        }
                        Ok::<_, String>(result)
                    },
                    |r| {
                        Message::Containers(match r {
                            Ok(out) => ContainersMsg::CommandResult(out),
                            Err(e) => ContainersMsg::CommandResult(format!("Error: {e}\n")),
                        })
                    },
                )
            }
            other => self.containers.update(other).map(Message::Containers),
        }
    }

    fn handle_volumes(&mut self, msg: VolumesMsg) -> Task<Message> {
        match msg {
            VolumesMsg::LoadVolumes => {
                let client = self.docker_client.clone();
                Task::perform(
                    async move { client.list_volumes().await.map_err(|e| e.to_string()) },
                    |r| {
                        Message::Volumes(match r {
                            Ok(v) => VolumesMsg::VolumesLoaded(v),
                            Err(e) => VolumesMsg::Error(e),
                        })
                    },
                )
            }
            VolumesMsg::CreateVolume => {
                let name = self.volumes.new_volume_name.clone();
                let client = self.docker_client.clone();
                Task::perform(
                    async move { client.create_volume(&name).await.map_err(|e| e.to_string()) },
                    |r| {
                        Message::Volumes(match r {
                            Ok(_) => VolumesMsg::LoadVolumes,
                            Err(e) => VolumesMsg::Error(e),
                        })
                    },
                )
            }
            VolumesMsg::RemoveVolume(name) => {
                let name_c = name.clone();
                let client = self.docker_client.clone();
                Task::perform(
                    async move {
                        client
                            .remove_volume(&name_c)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    |r| {
                        Message::Volumes(match r {
                            Ok(()) => VolumesMsg::LoadVolumes,
                            Err(e) => VolumesMsg::Error(e),
                        })
                    },
                )
            }
            other => self.volumes.update(other).map(Message::Volumes),
        }
    }

    fn handle_networks(&mut self, msg: NetworksMsg) -> Task<Message> {
        match msg {
            NetworksMsg::LoadNetworks => {
                let client = self.docker_client.clone();
                Task::perform(
                    async move { client.list_networks().await.map_err(|e| e.to_string()) },
                    |r| {
                        Message::Networks(match r {
                            Ok(n) => NetworksMsg::NetworksLoaded(n),
                            Err(e) => NetworksMsg::Error(e),
                        })
                    },
                )
            }
            NetworksMsg::CreateNetwork => {
                let name = self.networks.new_network_name.clone();
                let driver = self.networks.new_network_driver.clone();
                let driver_opt = if driver.is_empty() {
                    None
                } else {
                    Some(driver)
                };
                let client = self.docker_client.clone();
                Task::perform(
                    async move {
                        client
                            .create_network(&name, driver_opt.as_deref())
                            .await
                            .map_err(|e| e.to_string())
                    },
                    |r| {
                        Message::Networks(match r {
                            Ok(_) => NetworksMsg::LoadNetworks,
                            Err(e) => NetworksMsg::Error(e),
                        })
                    },
                )
            }
            NetworksMsg::RemoveNetwork(id) => {
                let id_c = id.clone();
                let client = self.docker_client.clone();
                Task::perform(
                    async move {
                        client
                            .remove_network(&id_c)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    |r| {
                        Message::Networks(match r {
                            Ok(()) => NetworksMsg::LoadNetworks,
                            Err(e) => NetworksMsg::Error(e),
                        })
                    },
                )
            }
            other => self.networks.update(other).map(Message::Networks),
        }
    }

    fn handle_compose(&mut self, msg: ComposeMsg) -> Task<Message> {
        match msg {
            ComposeMsg::ComposeUp => {
                let file = self.compose.compose_file.clone();
                let client = ComposeClient::new();
                Task::perform(
                    async move {
                        let _ = client.compose_up(&file).await.map_err(|e| e.to_string());
                        Ok::<_, String>(())
                    },
                    |r| {
                        Message::Compose(match r {
                            Ok(()) => ComposeMsg::Noop,
                            Err(e) => ComposeMsg::Error(e),
                        })
                    },
                )
            }
            ComposeMsg::ComposeDown(file) => {
                let file_c = file.clone();
                let client = ComposeClient::new();
                Task::perform(
                    async move {
                        client
                            .compose_down(&file_c)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    |r| {
                        Message::Compose(match r {
                            Ok(()) => ComposeMsg::Noop,
                            Err(e) => ComposeMsg::Error(e),
                        })
                    },
                )
            }
            other => self.compose.update(other).map(Message::Compose),
        }
    }

    fn handle_settings(&mut self, msg: SettingsMsg) -> Task<Message> {
        match msg {
            SettingsMsg::ThemeModeChanged(_) | SettingsMsg::ThemeVariantChanged(_) => {
                let _ = self.settings_screen.update(msg);
                self.settings.theme_setting = self.settings_screen.theme_setting.clone();
                let settings = self.settings.clone();
                let cm = self.config_manager.clone();
                Task::perform(
                    async move { cm.save_settings(&settings).await.map_err(|e| e.to_string()) },
                    |_| Message::Noop,
                )
            }
            SettingsMsg::Save => {
                let mut endpoint = self.settings.endpoint.clone();
                endpoint.host_url = self.settings_screen.endpoint_url.clone();
                self.settings.endpoint = endpoint;
                self.settings_screen.saved = true;
                self.settings.theme_setting = self.settings_screen.theme_setting.clone();
                let settings = self.settings.clone();
                let cm = self.config_manager.clone();
                Task::perform(
                    async move { cm.save_settings(&settings).await.map_err(|e| e.to_string()) },
                    |_| Message::Noop,
                )
            }
            other => self.settings_screen.update(other).map(Message::Settings),
        }
    }

    pub fn view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        let dark = ThemeManager::is_dark(&self.theme());

        let content: Element<'_, Message, Theme, iced::Renderer> = match self.active_screen {
            0 => self
                .dashboard
                .view::<Message>(self.docker_info.is_some(), dark),
            1 => self.containers_view(),
            2 => self.images_view(),
            3 => self.volumes_view(),
            4 => self.networks_view(),
            5 => self.compose_view(),
            6 => self.settings_view(),
            _ => self
                .dashboard
                .view::<Message>(self.docker_info.is_some(), dark),
        };

        let sidebar_widget = sidebar::sidebar(
            self.active_screen,
            dark,
            Message::Navigate,
            self.docker_info.is_some(),
        );

        let screen_title = match self.active_screen {
            0 => "Dashboard",
            1 => "Containers",
            2 => "Images",
            3 => "Volumes",
            4 => "Networks",
            5 => "Docker Compose",
            6 => "Settings",
            _ => "Dashboard",
        };

        let status = status_bar::status_bar(&self.docker_info, screen_title);

        column![
            row![
                sidebar_widget,
                container(content).width(Length::Fill).height(Length::Fill),
            ]
            .height(Length::Fill),
            status,
        ]
        .into()
    }

    fn containers_view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        self.containers
            .view(Some(&self.docker_client))
            .map(Message::Containers)
    }
    fn images_view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        self.images.view().map(Message::Images)
    }
    fn volumes_view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        self.volumes.view().map(Message::Volumes)
    }
    fn networks_view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        self.networks.view().map(Message::Networks)
    }
    fn compose_view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        self.compose.view().map(Message::Compose)
    }
    fn settings_view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        self.settings_screen.view().map(Message::Settings)
    }

    pub fn theme(&self) -> Theme {
        ThemeManager::resolve(&self.settings.theme_setting)
    }

    pub fn title(&self) -> String {
        format!(
            "Container Desktop{}",
            self.docker_info
                .as_ref()
                .map(|i| format!(" - Docker {}", i.server_version))
                .unwrap_or_default()
        )
    }
}
