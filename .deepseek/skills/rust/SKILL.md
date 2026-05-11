---
name: rust
description: Rust development with Cargo, clippy, rustfmt, testing, and workspace conventions. Project-specific knowledge for Container Desktop: bollard 0.21 Docker API, Iced 0.14 GUI framework, and Clean Architecture (domain ← infrastructure ← ui ← main.rs). Use when writing or modifying Rust code, running cargo commands, fixing type errors, working with async traits, or navigating the workspace crate graph.
---

# Rust — Container Desktop Project

## Workspace Structure

```
container-desktop/
├── Cargo.toml              # Workspace root with 3 crates + binary
├── src/main.rs             # Binary entry point
└── crates/
    ├── domain/             # Entities, repository traits, DomainError (zero deps beyond serde/async-trait)
    ├── infrastructure/     # Docker API (bollard), config persistence (depends on domain only)
    └── ui/                 # Iced GUI, theme, widgets, screens (depends on domain + infrastructure)
```

## Build Commands

```bash
cargo check                    # Full workspace check
cargo check -p domain          # Check specific crate (faster)
cargo build --release          # Release binary
cargo run                      # Dev mode
cargo clippy                   # Lint (must pass)
cargo fmt                      # Format (must pass)
cargo doc --no-deps --open     # Generate docs
```

## Clean Architecture Rules

**Dependency direction is enforced at compile time:** `domain ← infrastructure ← ui ← main.rs`

- **domain crate**: Must never depend on infrastructure or UI crates. Define traits in `domain/repository/`, impls in `infrastructure/docker/`.
- **infrastructure crate**: May only depend on `domain`.
- **ui crate**: May depend on `domain` and `infrastructure`.

## Bollard 0.21 API Notes

- **Option types**: In `bollard::query_parameters`, use `*Builder` pattern: `ListImagesOptionsBuilder::default().all(true).build()`
- **Models**: In `bollard::config` (e.g., `ContainerCreateBody`, `HostConfig`, `ContainerSummary`)
- **Docker constructors**: `Docker::connect_with_local_defaults()`, `Docker::connect_with_http()`
- **Required fields**: `ImageSummary.id`, `ImageSummary.repo_tags` are **NOT** `Option<T>` — they are direct types
- **LogOutput enum**: `StdOut { message: Bytes }`, `StdErr`, `StdIn`, `Console`
- **StartExecResults::Attached**: Provides `output: Pin<Box<dyn Stream>>` and `input: Pin<Box<dyn AsyncWrite>>`
- **Return types**: `ping()` returns `Result<String, Error>`, `remove_image()` returns `Vec<ImageDeleteResponseItem>`, `container_logs()` streams `LogOutput` (consume with `futures::StreamExt::next()`)
- **Enum fields**: `ContainerSummaryStateEnum`, `PortSummaryTypeEnum` need `format!("{:?}")` to convert to string
- **Bytes**: `Bytes` type from the `bytes` crate needs `.to_vec()` to convert to `Vec<u8>`

## Iced 0.14 UI Patterns

- **Application builder**: `iced::application(boot_fn, update_fn, view_fn).theme(theme_fn).title(title_fn).run()`
- **boot_fn**: Returns `(State, Task<Message>)`
- **update_fn**: Takes `&mut State, Message`, returns `Task<Message>`
- **view_fn**: Takes `&State`, returns `Element<'_, Message, Theme, Renderer>`
- **Async**: Use `Task::perform(async_fn, mapper_fn)` for async ops; `Task::batch(vec![...])` for concurrent tasks
- **Wrapping**: Use `Element::map(Message::Variant)` to wrap sub-messages
- **Window**: Title via `.title()`, icon via `.window(window::Settings { icon, .. })` with `icon::from_file_data()`
- **PNG images**: Embedded with `include_bytes!`, loaded via `image::Handle::from_bytes()`
- **Widgets**: `iced::widget::toggler` for booleans, `iced::widget::pick_list` for dropdowns
- **Styles**: `container::Style` uses direct struct fields: `container::Style { background: Some(...), ..Default::default() }`
- **Lifetimes**: `Element<'a, Message, Theme, Renderer>` carries borrowed data lifetime — use `.clone()` or store precomputed data in state
- **No explicit tracing subscriber**: Iced provides its own; do NOT call `tracing_subscriber::fmt::init()` in main.rs

## Theme System

```rust
let theme = ThemeManager::resolve(&settings.theme_setting);
// Auto: dark_light::detect() → Dark→TokyoNight, Light→CatppuccinLatte
// Default fallback: checks GTK_THEME and COLOR_SCHEME env vars
// Manual: direct mapping from 23 ThemeVariant variants

let is_dark = ThemeManager::is_dark(&theme);
let os_dark = ThemeManager::os_is_dark();
```

**Critical**: All widget styles must set `style.text_color` from `theme.extended_palette().background.base.text` to ensure visibility in dark mode. Never rely on default button/text color.

## Async Trait Patterns

All repository traits use `#[async_trait]` from the `async-trait` crate:

```rust
#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn list_images(&self) -> DomainResult<Vec<Image>>;
}
```

Implementations must also use `#[async_trait]`.

## Error Handling

Use `DomainResult<T>` (alias `Result<T, DomainError>`) throughout domain and infrastructure layers:

```rust
use domain::{DomainError, DomainResult};

pub(crate) async fn get_docker(&self) -> DomainResult<bollard::Docker> {
    let guard = self.docker.lock().await;
    guard.clone().ok_or_else(|| {
        DomainError::ConnectionFailed("Not connected to Docker daemon".to_string())
    })
}
```

UI layer may use `Result<T, String>` for async tasks.

## Import Conventions

When using repository trait methods from the UI layer, import the trait explicitly:

```rust
use domain::repository::{ImageRepository, ContainerRepository, VolumeRepository, NetworkRepository, ComposeRepository};
```

Required because `Arc<DockerClient>` auto-derefs only when the trait is in scope.

## Documentation

Every public method, struct, and trait must have a `///` doc comment.

## Adding a New Docker Resource

1. Create entity in `crates/domain/src/entities/`
2. Add repository trait in `crates/domain/src/repository/`
3. Implement trait in `crates/infrastructure/src/docker/`
4. Create screen module in `crates/ui/src/screens/`
5. Add screen state to `ContainerDesktop` in `crates/ui/src/app.rs`
6. Add message variant and routing

## Screen Message Routing

Screen-local messages handled in the screen's own `update()`. Docker API messages intercepted in `app.rs` via `handle_*()` methods:

```rust
ContainersMsg::LoadContainers => { /* API call */ }
other => self.containers.update(other).map(Message::Containers),
```
