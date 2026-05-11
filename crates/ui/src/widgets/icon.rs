use iced::widget::svg;

/// Icon identifiers used throughout the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    Dashboard,
    Containers,
    Images,
    Volumes,
    Networks,
    Compose,
    Settings,
    Play,
    Stop,
    Restart,
    Remove,
    Terminal,
    Logs,
    Pull,
    Plus,
    Docker,
}

impl Icon {
    /// Returns the SVG content for this icon matching the given theme mode.
    pub fn svg(&self, dark_mode: bool) -> svg::Handle {
        let name = self.filename();
        let mode_str = if dark_mode { "dark" } else { "light" };
        let path = format!("assets/icons/{mode_str}/{name}.svg");

        // Try to load from filesystem first, fall back to embedded.
        match svg::Handle::from_path(&path) {
            handle => handle,
        }
    }

    fn filename(&self) -> &'static str {
        match self {
            Icon::Dashboard => "dashboard",
            Icon::Containers => "container",
            Icon::Images => "image",
            Icon::Volumes => "volume",
            Icon::Networks => "network",
            Icon::Compose => "compose",
            Icon::Settings => "settings",
            Icon::Play => "play",
            Icon::Stop => "stop",
            Icon::Restart => "restart",
            Icon::Remove => "remove",
            Icon::Terminal => "terminal",
            Icon::Logs => "logs",
            Icon::Pull => "pull",
            Icon::Plus => "plus",
            Icon::Docker => "docker",
        }
    }
}

/// Creates an SVG icon widget.
pub fn icon<'a>(icon: Icon, dark_mode: bool, size: f32) -> iced::widget::Svg<'a, iced::Theme> {
    svg::Svg::new(svg::Handle::from_memory(include_icon_bytes(
        icon, dark_mode,
    )))
    .width(size)
    .height(size)
}

fn include_icon_bytes(icon: Icon, dark_mode: bool) -> Vec<u8> {
    let _mode = if dark_mode { "dark" } else { "light" };
    let name = icon.filename();

    // Fallback: generate simple SVG shapes for each icon type
    generate_icon_svg(name, dark_mode).into_bytes()
}

fn generate_icon_svg(name: &str, dark: bool) -> String {
    let color = if dark { "#e0e0e0" } else { "#333333" };
    let accent = if dark { "#7aa2f7" } else { "#3366cc" };

    match name {
        "dashboard" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><rect x="3" y="3" width="7" height="9" rx="1"/><rect x="14" y="3" width="7" height="5" rx="1"/><rect x="14" y="12" width="7" height="9" rx="1"/><rect x="3" y="16" width="7" height="5" rx="1"/></svg>"#
        ),
        "container" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><rect x="2" y="4" width="20" height="16" rx="2"/><rect x="6" y="4" width="12" height="2"/><line x1="2" y1="12" x2="22" y2="12"/></svg>"#
        ),
        "image" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="8.5" cy="8.5" r="1.5" fill="{accent}"/><polyline points="21 15 16 10 5 21"/></svg>"#
        ),
        "volume" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><rect x="2" y="6" width="20" height="12" rx="2"/><circle cx="12" cy="12" r="2"/><path d="M6 6V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v2"/></svg>"#
        ),
        "network" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><rect x="2" y="2" width="20" height="8" rx="2"/><rect x="6" y="14" width="12" height="6" rx="2"/><path d="M12 10v4"/><circle cx="7" cy="17" r="1" fill="{accent}"/><circle cx="17" cy="17" r="1" fill="{accent}"/><line x1="8" y1="5" x2="16" y2="5"/></svg>"#
        ),
        "compose" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>"#
        ),
        "settings" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>"#
        ),
        "play" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{accent}" stroke-width="2"><polygon points="5 3 19 12 5 21 5 3" fill="{accent}"/></svg>"#
        ),
        "stop" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><rect x="4" y="4" width="16" height="16" rx="2"/></svg>"#
        ),
        "restart" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"/></svg>"#
        ),
        "remove" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>"#
        ),
        "terminal" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><polyline points="4 17 10 11 4 5"/><line x1="12" y1="19" x2="20" y2="19"/></svg>"#
        ),
        "logs" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>"#
        ),
        "pull" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>"#
        ),
        "plus" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>"#
        ),
        "docker" => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{accent}" stroke-width="1.5"><rect x="1" y="5" width="22" height="16" rx="2" stroke="{accent}"/><circle cx="6" cy="13" r="1.5" fill="{accent}"/><circle cx="10" cy="13" r="1.5" fill="{accent}"/><circle cx="14" cy="13" r="1.5" fill="{accent}"/><circle cx="18" cy="13" r="1.5" fill="{accent}"/><circle cx="6" cy="9" r="1.5" fill="{accent}"/><circle cx="10" cy="9" r="1.5" fill="{accent}"/></svg>"#
        ),
        _ => format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10" fill="none" stroke="{color}" stroke-width="2"/><circle cx="12" cy="12" r="3" fill="{color}"/></svg>"#
        ),
    }
}
