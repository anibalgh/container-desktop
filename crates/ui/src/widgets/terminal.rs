use iced::widget::{column, container, scrollable, text, text_input};
use iced::{Element, Length, Padding, Theme};
use std::sync::{Arc, Mutex as StdMutex};

/// State for the interactive terminal widget.
pub struct TerminalState {
    pub output: String,
    pub input_buffer: String,
    pub is_running: bool,
}

impl Default for TerminalState {
    fn default() -> Self {
        Self {
            output: String::new(),
            input_buffer: String::new(),
            is_running: false,
        }
    }
}

/// Messages for the terminal widget.
#[derive(Debug, Clone)]
pub enum TerminalMessage {
    InputChanged(String),
    SendInput,
    OutputReceived(String),
    TerminalClosed,
}

/// Creates an interactive terminal widget.
pub fn terminal<'a>(
    state: &'a TerminalState,
) -> Element<'a, TerminalMessage, Theme, iced::Renderer> {
    let output_text = text(&state.output).size(12).font(iced::Font::MONOSPACE);

    let output_view = container(
        scrollable(
            container(output_text)
                .width(Length::Fill)
                .padding(Padding::new(4.0)),
        )
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |theme: &Theme| {
        let p = theme.extended_palette();
        container::Style {
            background: Some(iced::Background::Color(p.background.base.color)),
            text_color: Some(p.background.base.text),
            ..Default::default()
        }
    });

    let input = text_input("", &state.input_buffer)
        .on_input(TerminalMessage::InputChanged)
        .on_submit(TerminalMessage::SendInput)
        .padding(6)
        .size(12)
        .font(iced::Font::MONOSPACE);

    let input_row = container(input)
        .width(Length::Fill)
        .padding(Padding::new(4.0))
        .style(move |theme: &Theme| {
            let p = theme.extended_palette();
            container::Style {
                background: Some(iced::Background::Color(p.background.strong.color)),
                ..Default::default()
            }
        });

    let terminal_content = column![
        output_view,
        container(
            text("┤").size(10)
        ).padding(Padding::new(2.0).left(8.0)),
        input_row,
    ]
    .spacing(0)
    .width(Length::Fill)
    .height(Length::Fill);

    container(terminal_content)
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

/// Spawns a terminal session using `docker exec -it <container> <cmd>`.
/// Returns a pair of (stdin_writer, stdout_reader_future).
pub async fn spawn_docker_exec(
    container_id: String,
    cmd: Option<String>,
) -> Result<
    (
        Box<dyn tokio::io::AsyncWrite + Unpin + Send>,
        tokio::task::JoinHandle<()>,
        tokio::sync::mpsc::UnboundedReceiver<String>,
    ),
    String,
> {
    let shell = cmd.unwrap_or_else(|| "/bin/sh".to_string());

    // For interactive terminal, we use `docker exec -it` via PTY
    // This is a simplified approach using tokio::process
    use portable_pty::{native_pty_system, PtySize, CommandBuilder};

    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Failed to open PTY: {e}"))?;

    let mut cmd = CommandBuilder::new("docker");
    cmd.args(["exec", "-it", &container_id, &shell]);

    let mut child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| format!("Failed to spawn docker exec: {e}"))?;

    let _child = child; // keep child alive for PTY lifetime

    let mut master = pair.master;
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    let mut reader = master
        .try_clone_reader()
        .map_err(|e| format!("Failed to clone PTY reader: {e}"))?;

    let reader_handle = tokio::spawn(async move {
        use std::io::Read;
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let text = String::from_utf8_lossy(&buf[..n]).to_string();
                    if tx.send(text).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    let writer = Box::new(PTYWriter {
        pty: Arc::new(StdMutex::new(Some(master))),
    });

    Ok((writer, reader_handle, rx))
}

/// Writer that sends data to a PTY.
struct PTYWriter {
    pty: Arc<StdMutex<Option<portable_pty::MasterPty>>>,
}

impl tokio::io::AsyncWrite for PTYWriter {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        if let Some(master) = self.pty.lock().unwrap().as_mut() {
            match master.write(buf) {
                Ok(n) => std::task::Poll::Ready(Ok(n)),
                Err(e) => std::task::Poll::Ready(Err(e)),
            }
        } else {
            std::task::Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "PTY closed",
            )))
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
}
