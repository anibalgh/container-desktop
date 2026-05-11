use iced::widget::{column, container, scrollable, text};
use iced::{Element, Length, Padding, Theme};

/// A log entry with optional ANSI styling.
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub line: String,
    pub is_stderr: bool,
}

/// Creates a log viewer widget that displays a list of log lines.
pub fn log_viewer<'a, Message: Clone + 'a>(
    entries: &'a [LogEntry],
    _follow: bool,
) -> Element<'a, Message, Theme, iced::Renderer> {
    let log_lines = entries
        .iter()
        .map(|entry| {
            let line_text = if entry.is_stderr {
                text(&entry.line)
                    .size(12)
                    .color(iced::Color::from_rgb(0.9, 0.3, 0.3))
            } else {
                text(&entry.line).size(12)
            };

            container(line_text)
                .width(Length::Fill)
                .padding(Padding::new(1.0).left(8.0))
                .into()
        })
        .collect::<Vec<Element<'_, Message, Theme, iced::Renderer>>>();

    let scroll = scrollable(column(log_lines).spacing(0).width(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill);

    container(scroll)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |theme: &Theme| {
            let p = theme.extended_palette();
            container::Style {
                background: Some(iced::Background::Color(p.background.base.color)),
                ..Default::default()
            }
        })
        .into()
}

/// Parses ANSI-colored text into styled lines.
pub fn parse_ansi_lines(raw: &str) -> Vec<String> {
    // Simple line splitting; full ANSI parsing would strip escape codes.
    raw.lines().map(|l| l.to_string()).collect()
}
