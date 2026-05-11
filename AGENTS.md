# AGENTS.md

## Project Context

**Container Desktop** is a cross-platform desktop application for managing Docker resources (containers, images, volumes, networks, and compose stacks). Written 100% in Rust with a native GUI using the Iced framework, following Clean Architecture principles.

The application is inspired by Docker Desktop, Podman Desktop, and Rancher Desktop.

---

## Architecture

The project enforces Clean Architecture via a Cargo workspace with 3 internal crates + 1 binary. Dependency direction is enforced at compile time:

```
domain ← infrastructure ← ui ← main.rs
```

| Crate | Path | Purpose | Allowed Dependencies |
|---|---|---|---|
| `domain` | `crates/domain/` | Entities, repository traits, domain errors | None (zero deps beyond serde/async-trait) |
| `infrastructure` | `crates/infrastructure/` | Docker API (bollard), config persistence (directories) | `domain` only |
| `ui` | `crates/ui/` | Iced GUI, theme system, widgets, screens | `domain`, `infrastructure` |
| binary | `src/main.rs` | Entry point, wires everything | `domain`, `infrastructure`, `ui` |

### Key Rule

**Domain crate must never depend on infrastructure or UI crates.** If you need to add a new feature, define the trait in `domain/repository/` and the implementation in `infrastructure/docker/`.

---

## Build Commands

```bash
# Full workspace check (no binaries built)
cargo check

# Check specific crate only (faster)
cargo check -p domain
cargo check -p infrastructure
cargo check -p ui

# Build release binary
cargo build --release

# Run in development mode
cargo run

# Lint
cargo clippy

# Format
cargo fmt

# Generate docs
cargo doc --no-deps --open
```

---

## Project Structure

```
container-desktop/
├── Cargo.toml                     # Workspace root
├── src/main.rs                    # Binary entry point, window icon, title
├── crates/
│   ├── domain/src/
│   │   ├── lib.rs                 # Re-exports DomainError, DomainResult
│   │   ├── error.rs               # DomainError enum (8 variants)
│   │   ├── entities/              # Data structures (Image, Container, Volume, etc.)
│   │   └── repository/            # Async traits (ImageRepository, ContainerRepository, etc.)
│   ├── infrastructure/src/
│   │   ├── lib.rs                 # Re-exports ConfigManager, DockerClient, ComposeClient
│   │   ├── config/mod.rs          # ConfigManager: SettingsRepository impl via directories crate
│   │   └── docker/
│   │       ├── mod.rs             # DockerClient struct + connection logic
│   │       ├── connection.rs      # DockerConnectionRepository impl
│   │       ├── images.rs          # ImageRepository impl (bollard)
│   │       ├── containers.rs      # ContainerRepository impl (bollard)
│   │       ├── volumes.rs         # VolumeRepository impl (bollard)
│   │       ├── networks.rs        # NetworkRepository impl (bollard)
│   │       └── compose.rs         # ComposeRepository impl (docker-compose binary)
│   └── ui/src/
│       ├── lib.rs
│       ├── app.rs                 # ContainerDesktop struct, Message enum, update/view/theme
│       ├── theme.rs               # ThemeManager: ThemeSetting → iced::Theme, OnceLock caching
│       ├── widgets/
│       │   ├── mod.rs
│       │   ├── sidebar.rs         # Navigation sidebar (7 items + PNG header icon)
│       │   ├── status_bar.rs      # Docker connection status
│       │   ├── data_table.rs      # Reusable sortable/selectable table
│       │   ├── modals.rs          # Modal dialogs (pull image, create container)
│       │   ├── log_viewer.rs      # Log viewer (stdout/stderr coloring)
│       │   ├── terminal.rs        # PTY-based Docker exec terminal
│       │   └── icon.rs            # SVG icon system (embedded fallback, light/dark)
│       └── screens/
│           ├── mod.rs
│           ├── dashboard.rs       # Connection status + branded logo image
│           ├── containers.rs      # Full CRUD + logs (tail/since/until) + terminal (shell/root)
│           ├── images.rs          # List, pull, remove, tag
│           ├── volumes.rs         # List, create, remove
│           ├── networks.rs        # List, create, remove
│           ├── compose.rs         # Up, down, logs
│           └── settings.rs        # Theme picker, endpoint config
└── assets/icons/                  # PNG icons (dark/light variants)
    ├── dark/
    │   ├── container-desktop.png  # Dashboard logo (2048x2048)
    │   ├── icon8x8.png           # Window icon (2048x2048)
    │   └── icon16x16.png         # Sidebar header icon (2048x2048)
    └── light/
        ├── container-desktop.png
        ├── icon8x8.png
        └── icon16x16.png
```

---

## Code Conventions

### Documentation

Every public method, struct, and trait must have a `///` doc comment.

```rust
/// Lists all containers on the Docker daemon, optionally including stopped ones.
async fn list_containers(&self, all: bool) -> DomainResult<Vec<Container>>;
```

### Error Handling

Use `DomainResult<T>` (alias for `Result<T, DomainError>`) throughout the domain and infrastructure layers. The UI layer may use plain `Result<T, String>` for async tasks.

```rust
use domain::{DomainError, DomainResult};

pub(crate) async fn get_docker(&self) -> DomainResult<bollard::Docker> {
    let guard = self.docker.lock().await;
    guard.clone().ok_or_else(|| {
        DomainError::ConnectionFailed("Not connected to Docker daemon".to_string())
    })
}
```

### Imports

When using repository trait methods from the UI layer, import the trait explicitly:

```rust
use domain::repository::{ImageRepository, ContainerRepository, VolumeRepository, ...};
```

This is required because `Arc<DockerClient>` auto-derefs only when the trait is in scope.

### Async Trait Methods

All repository traits use `#[async_trait]` from the `async-trait` crate:

```rust
#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn list_images(&self) -> DomainResult<Vec<Image>>;
}
```

Implementations must also use `#[async_trait]`:

```rust
#[async_trait]
impl ImageRepository for DockerClient {
    async fn list_images(&self) -> DomainResult<Vec<Image>> { ... }
}
```

---

## Key Patterns

### Adding a new Docker resource

1. Create entity in `crates/domain/src/entities/`
2. Add repository trait in `crates/domain/src/repository/`
3. Implement trait in `crates/infrastructure/src/docker/`
4. Create screen module in `crates/ui/src/screens/`
5. Add screen state to `ContainerDesktop` in `crates/ui/src/app.rs`
6. Add message variant and routing

### Screen Message Routing

Screen-local messages (UI state changes only) are handled in the screen's own `update()`. Messages requiring Docker API calls are intercepted in `app.rs` via `handle_*()` methods:

```rust
// app.rs — Docker API messages intercepted here
ContainersMsg::LoadContainers => { /* API call */ }
ContainersMsg::ShowLogs(id) => { /* API call */ }

// All other messages forwarded to screen
other => self.containers.update(other).map(Message::Containers),
```

### Data Auto-Loading

Screens that display Docker data auto-load their content when navigated to via the sidebar. The `Navigate` handler dispatches the appropriate `Load*` message:

```rust
Message::Navigate(index) => {
    self.active_screen = index;
    match index {
        1 => self.handle_containers(ContainersMsg::LoadContainers),
        2 => self.handle_images(ImagesMsg::LoadImages),
        // ...
        _ => Task::none(),
    }
}
```

### Log Viewer Controls

Container logs support configurable line count with input validation (positive integers only), plus optional `since`/`until` timestamp filters. The `RefreshLogs` message reloads logs with the current parameters:

- `log_tail_lines`: String input, validated to only accept digits, must parse as u32 > 0
- `log_since`: Optional timestamp string
- `log_until`: Optional timestamp string
- `RefreshLogs` button or `ShowLogs` triggers API call with parsed parameters

### Terminal Workflow

Container terminal uses a two-step flow:
1. **Setup screen**: Shell picker (`sh | bash | zsh | ash | fish`), "Connect as root" toggler, "Connect" button
2. **Connected screen**: Command input with `docker exec <container> <shell> -c "<command>"` execution (with optional `-u root`)

### Bollard 0.21 API Notes

- **Option types** are in `bollard::query_parameters` (use `*Builder` pattern: `ListImagesOptionsBuilder::default().all(true).build()`)
- **Models** are in `bollard::config` (e.g., `ContainerCreateBody`, `HostConfig`, `ContainerSummary`)
- **Docker struct** constructors: `Docker::connect_with_local_defaults()`, `Docker::connect_with_http()`
- Required fields (like `ImageSummary.id`, `ImageSummary.repo_tags`) are **NOT** `Option<T>` — they are direct types
- `LogOutput` enum variants: `StdOut { message: Bytes }`, `StdErr`, `StdIn`, `Console`
- `StartExecResults::Attached` provides both `output: Pin<Box<dyn Stream>>` and `input: Pin<Box<dyn AsyncWrite>>`
- `ping()` returns `Result<String, Error>` (not `Result<(), Error>`)
- `remove_image()` returns `Vec<ImageDeleteResponseItem>` (not `()`)
- `container_logs()` streams `LogOutput`; consume with `futures::StreamExt::next()`

### Iced 0.14 UI Patterns

- Use `iced::application(boot_fn, update_fn, view_fn).theme(theme_fn).title(title_fn).run()`
- `boot_fn` returns `(State, Task<Message>)`
- `update_fn` takes `&mut State, Message` and returns `Task<Message>`
- `view_fn` takes `&State` and returns `Element<'_, Message, Theme, Renderer>`
- `container::Style` uses direct struct fields: `container::Style { background: Some(...), ..Default::default() }`
- Use explicit variable annotations for `.into()` calls on ambiguous types
- Use `Task::perform(async_fn, mapper_fn)` for async operations
- Use `Task::batch(vec![...])` for multiple concurrent tasks
- Use `Element::map(Message::Variant)` to wrap sub-messages
- Window title set via `.title(ContainerDesktop::title)` on the application builder
- Window icon set via `.window(window::Settings { icon, .. })` with `icon::from_file_data()`
- PNG images embedded at compile time with `include_bytes!`, loaded via `image::Handle::from_bytes()`
- No explicit tracing subscriber needed — Iced provides its own
- `iced::widget::toggler` for boolean controls; `iced::widget::pick_list` for dropdowns

### Theme System

```rust
// Resolve user setting to iced::Theme — cached via OnceLock on first call
let theme = ThemeManager::resolve(&settings.theme_setting);
// Auto: dark_light::detect() → Dark→TokyoNight, Light→CatppuccinLatte
// Default fallback: checks GTK_THEME and COLOR_SCHEME env vars for dark hints
// Manual: direct mapping from 23 ThemeVariant variants

// Check if dark mode
let is_dark = ThemeManager::is_dark(&theme);  // theme.mode() == Mode::Dark

// Detect OS dark mode for icon selection
let os_dark = ThemeManager::os_is_dark();
```

**Important**: All widget styles must set `style.text_color` from `theme.extended_palette().background.base.text` (or similar) to ensure visibility in dark mode. Never rely on the default button/text color.

---

## Configuration

Settings are persisted via the `directories` crate to platform-specific config directories:

| Platform | Config Path |
|---|---|
| Linux | `~/.config/container-desktop/ContainerDesktop/settings.json` |
| macOS | `~/Library/Application Support/com.container-desktop.ContainerDesktop/settings.json` |
| Windows | `C:\Users\<User>\AppData\Roaming\container-desktop\ContainerDesktop\config\settings.json` |

Settings schema: `AppSettings { theme_setting, endpoint, window_width, window_height }`

---

## Common Tasks

### Adding a new theme

Themes are defined in `domain::entities::ThemeVariant` enum (23 variants). To add a new one:

1. Add variant to `ThemeVariant` in `crates/domain/src/entities/settings.rs`
2. Add `display_name()` mapping
3. Add to `all()` static array
4. Add `is_dark()` mapping
5. Add `variant_to_theme()` mapping in `crates/ui/src/theme.rs`

### Adding a new screen

1. Create `crates/ui/src/screens/my_screen.rs` with state struct, message enum, update/view methods
2. Add to `crates/ui/src/screens/mod.rs`
3. Add state field to `ContainerDesktop` in `crates/ui/src/app.rs`
4. Add message variant `MyScreen(MyScreenMessage)` to main `Message` enum
5. Add `handle_my_screen()` method for Docker API interactions
6. Add `my_screen_view()` method for view routing
7. Add match arm in `view()` function

### Adding a new widget

1. Create `crates/ui/src/widgets/my_widget.rs`
2. Add to `crates/ui/src/widgets/mod.rs`
3. Keep widget functions pure: take state references, return `Element`
4. For interactive widgets, define a local `Message` type and let callers map it
5. Use explicit lifetime annotations (`'a`) for borrowed data
6. Always set `text_color` in widget styles using theme palette colors

### Adding a new icon

1. Add variant to `Icon` enum in `crates/ui/src/widgets/icon.rs`
2. Add filename mapping in `fn filename()`
3. Add SVG generation in `generate_icon_svg()` — use `{color}` for stroke and `{accent}` for fill
4. Place optional filesystem SVG at `assets/icons/{dark,light}/{name}.svg` (falls back to embedded)

### Testing Docker connection

```bash
# Ensure Docker is running
docker info

# Run the app
cargo run

# Check settings file
cat ~/.config/container-desktop/ContainerDesktop/settings.json
```

---

## Troubleshooting

### Build errors with bollard types

Bollard 0.21 uses generated types from bollard-stubs. Common pitfalls:
- Fields that were `Option<String>` in older bollard may be direct `String` in 0.21
- Enum fields (e.g., `ContainerSummaryStateEnum`, `PortSummaryTypeEnum`) need `format!("{:?}")` to convert to string
- `Bytes` type from the `bytes` crate needs `.to_vec()` to convert to `Vec<u8>`

### Lifetime errors in UI

- `Element<'a, Message, Theme, Renderer>` carries the lifetime of borrowed data
- Use `.clone()` on data that needs to outlive the function scope
- Use explicit variable type annotations when `.into()` is ambiguous
- Store computed data (like table rows) in screen state rather than computing in `view()`

### Docker connection fails

- On Linux: verify `docker.sock` permissions (`ls -la /var/run/docker.sock`)
- Add user to docker group: `sudo usermod -aG docker $USER`
- For remote connections: verify port is accessible and TLS certs are valid

### Dark mode text invisible

- Widget styles (`row_style`, `inactive_nav_style`, etc.) must explicitly set `style.text_color` using theme palette colors. The default `button::Style::default()` text color may be invisible on dark backgrounds.
- Data tables, sidebars, log viewers, and terminal views all use `palette.background.base.text` or `palette.primary.*.text` from `theme.extended_palette()`.

### Icon pixel data spam

- If the terminal floods with raw pixel values at startup, ensure `tracing_subscriber::fmt::init()` is **not** called in `main.rs`. Iced already installs a tracing subscriber; a second one at INFO level causes `iced_winit` to dump the full window icon buffer.
