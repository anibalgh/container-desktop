# Container Desktop

[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org)
[![Iced](https://img.shields.io/badge/iced-0.14-blue.svg)](https://iced.rs)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A cross-platform desktop application for managing Docker containers, images, volumes, networks, and compose stacks — built entirely in Rust with a modern native GUI.

> Inspired by Docker Desktop, Podman Desktop, and Rancher Desktop.

<p align="center">
  <img src="assets/screenshot.png" alt="Container Desktop Screenshot" width="800">
</p>

---

## Features

### Docker Resource Management

| Resource | Actions |
|---|---|
| **Containers** | List, create, start, stop, restart, remove, view logs, interactive terminal |
| **Images** | List, pull, remove, tag, inspect |
| **Volumes** | List, create, remove, inspect |
| **Networks** | List, create, remove, inspect |
| **Docker Compose** | Up, down, view logs for compose stacks |

### User Interface

- Modern native GUI built with [Iced](https://iced.rs) (Elm-like architecture)
- **23 built-in themes**: Catppuccin, Tokyo Night, Gruvbox, Dracula, Nord, Solarized, and more
- Automatic light/dark mode detection (follows OS preference)
- Manual theme override persisted to user config
- SVG iconography with light/dark color variants
- Responsive sidebar navigation with 7 screens
- Sortable, selectable data tables
- Modal dialogs for image pull, container creation, and confirmations
- Streaming log viewer with ANSI color support
- Interactive terminal (PTY-based) for container exec sessions

### Cross-Platform

- **Linux**: Unix socket (`/var/run/docker.sock`)
- **macOS**: Unix socket
- **Windows**: Named pipe + TCP

### Connection Options

- Local Docker daemon (auto-detect)
- Remote Docker over TCP
- TLS-secured connections (certificate-based)
- Auto-connect on startup with fallback

---

## Architecture

The project follows **Clean Architecture** principles with strictly enforced dependency direction at compile time:

```
┌──────────────────────────────────────────────┐
│                 main.rs                       │
│           (entry point, wiring)               │
├──────────────────────────────────────────────┤
│  ui crate          →  infrastructure  →  domain │
│  (iced GUI)            (bollard+config)  (entities+traits) │
│  Depends on both       Implements traits   No deps       │
└──────────────────────────────────────────────┘
```

### Crate Dependency Graph

```
domain        — Entities, repository traits, error types
  ↑
infrastructure — Docker API (bollard), config persistence
  ↑
ui             — Iced GUI, theme system, widgets, screens
  ↑
main.rs        — Application entry point
```

| Crate | Purpose | Key Dependencies |
|---|---|---|
| `domain` | Core business logic, entities, repository traits | `serde`, `thiserror`, `async-trait` |
| `infrastructure` | Docker API client, config management | `bollard`, `directories`, `tokio` |
| `ui` | Desktop GUI, theme engine, widgets, screens | `iced`, `dark-light`, `portable-pty` |

---

## Prerequisites

- **Rust** 1.88 or later ([rustup.rs](https://rustup.rs))
- **Docker Engine** running locally (or a remote Docker daemon)
- **docker-compose** binary (optional, for Compose features)
- On Linux: `libxkbcommon-dev`, `libgtk-3-dev` (Iced dependencies)

### Install System Dependencies

**Ubuntu / Debian:**
```bash
sudo apt install build-essential pkg-config libxkbcommon-dev libgtk-3-dev
```

**Fedora:**
```bash
sudo dnf install gcc pkg-config libxkbcommon-devel gtk3-devel
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
- Install [Rust](https://rustup.rs) via `rustup-init.exe`
- No additional system dependencies required

---

## Quick Start

### 1. Clone the repository

```bash
git clone https://github.com/your-username/container-desktop.git
cd container-desktop
```

### 2. Build

```bash
cargo build --release
```

### 3. Run

```bash
cargo run --release
```

On first launch, the application will:
1. Create the config directory at `~/.config/container-desktop/`
2. Auto-detect and connect to the local Docker daemon
3. Apply your OS theme preference (auto light/dark mode)
4. Open the Dashboard screen

### Development Mode

```bash
cargo run
```

For faster iteration, check compilation without building:

```bash
cargo check                # Check entire workspace
cargo check -p domain      # Check domain crate only
cargo check -p ui          # Check UI crate only
```

---

## Usage Guide

### Screens

| # | Screen | Description |
|---|---|---|
| 0 | **Dashboard** | Connection status, Docker daemon info summary |
| 1 | **Containers** | List all containers, start/stop/restart/remove, view logs, open terminal |
| 2 | **Images** | List images, pull from registry, remove, tag |
| 3 | **Volumes** | List volumes, create, remove |
| 4 | **Networks** | List networks, create with driver selection, remove |
| 5 | **Docker Compose** | Specify compose file path, up/down, view streaming logs |
| 6 | **Settings** | Theme picker (23 themes), endpoint configuration, connection test |

### Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl+1` to `Ctrl+7` | Switch to screens 0-6 |
| `F5` | Refresh current screen data |

### Theme Selection

1. Navigate to **Settings** (sidebar)
2. Choose **Auto** to follow your OS light/dark preference
3. Choose **Manual** and select from 23 built-in themes
4. Click **Save Settings**

Available themes: Light, Dark, Dracula, Nord, Solarized Light/Dark, Gruvbox Light/Dark, Catppuccin (Latte/Frappe/Macchiato/Mocha), Tokyo Night (Standard/Storm/Light), Kanagawa (Wave/Dragon/Lotus), Moonfly, Nightfly, Oxocarbon, Ferra.

---

## Configuration

Settings are persisted to:

| Platform | Path |
|---|---|
| Linux | `~/.config/container-desktop/settings.json` |
| macOS | `~/Library/Application Support/com.container-desktop.ContainerDesktop/settings.json` |
| Windows | `C:\Users\<User>\AppData\Roaming\container-desktop\ContainerDesktop\config\settings.json` |

### Example `settings.json`

```json
{
  "theme_setting": {
    "Manual": "TokyoNight"
  },
  "endpoint": {
    "host_url": "unix:///var/run/docker.sock",
    "tls_ca": null,
    "tls_cert": null,
    "tls_key": null,
    "timeout_secs": 30
  },
  "window_width": 1280,
  "window_height": 800
}
```

### Remote Docker Connection

To connect to a remote Docker daemon over TCP:

1. Navigate to **Settings**
2. Set the host URL: `tcp://192.168.1.10:2376`
3. Optionally configure TLS certificates
4. Click **Test Connection**
5. Click **Save Settings**
6. Restart the application

> Warning: Unencrypted TCP connections to remote Docker daemons are insecure. Use TLS for production environments.

---

## Project Structure

```
container-desktop/
├── Cargo.toml                  # Workspace root
├── src/
│   └── main.rs                 # Application entry point
├── crates/
│   ├── domain/                 # Core domain layer
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── error.rs        # DomainError enum
│   │       ├── entities/       # Data structures
│   │       │   ├── container.rs, image.rs, volume.rs
│   │       │   ├── network.rs, compose.rs
│   │       │   ├── endpoint.rs, settings.rs
│   │       │   └── mod.rs
│   │       └── repository/     # Trait definitions
│   │           ├── container.rs, image.rs, volume.rs
│   │           ├── network.rs, compose.rs
│   │           ├── connection.rs, settings.rs
│   │           └── mod.rs
│   ├── infrastructure/         # External implementations
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config/
│   │       │   └── mod.rs      # ConfigManager (settings persistence)
│   │       └── docker/
│   │           ├── mod.rs      # DockerClient, connection logic
│   │           ├── connection.rs, compose.rs
│   │           ├── containers.rs, images.rs
│   │           ├── volumes.rs, networks.rs
│   └── ui/                     # Iced GUI layer
│       └── src/
│           ├── lib.rs
│           ├── app.rs          # Main app, message routing
│           ├── theme.rs        # ThemeManager (23 themes + auto-detect)
│           ├── widgets/
│           │   ├── mod.rs
│           │   ├── sidebar.rs  # Navigation sidebar
│           │   ├── status_bar.rs
│           │   ├── data_table.rs
│           │   ├── modals.rs   # Dialog modals
│           │   ├── log_viewer.rs
│           │   ├── terminal.rs # PTY terminal widget
│           │   └── icon.rs     # SVG icon system
│           └── screens/
│               ├── mod.rs
│               ├── dashboard.rs, containers.rs
│               ├── images.rs, volumes.rs
│               ├── networks.rs, compose.rs
│               └── settings.rs
└── assets/
    └── icons/
        ├── light/              # Light-theme SVG icons
        └── dark/               # Dark-theme SVG icons
```

---

## Technology Stack

| Technology | Version | Purpose |
|---|---|---|
| [Rust](https://www.rust-lang.org) | 1.88 | Language |
| [Iced](https://iced.rs) | 0.14 | Native GUI framework |
| [Bollard](https://github.com/fussybeaver/bollard) | 0.21 | Docker Engine API client |
| [Tokio](https://tokio.rs) | 1.x | Async runtime |
| [Directories](https://github.com/soc/directories-rs) | 6.0 | Platform config paths |
| [Serde](https://serde.rs) | 1.x | Serialization |
| [Dark Light](https://github.com/frewsxcv/rust-dark-light) | 1.1 | OS theme detection |
| [Portable PTY](https://github.com/wez/wezterm) | 0.8 | Pseudoterminal (PTY) |

---

## License

MIT — See [LICENSE](LICENSE) for details.

---

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Ensure the project builds: `cargo check`
5. Submit a pull request

### Development Guidelines

- All public methods must include documentation comments (`///`)
- Follow Clean Architecture: domain has no external dependencies
- Import repository traits when calling Docker API methods from the UI layer
- Use `cargo check -p <crate>` for targeted compilation checks
- Format code with `cargo fmt` before committing
- Run `cargo clippy` for lint checks
