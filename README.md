# Container Desktop

[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/tauri-2.x-blue.svg)](https://tauri.app)
[![React](https://img.shields.io/badge/react-19.x-61dafb.svg)](https://react.dev)
[![Tailwind](https://img.shields.io/badge/tailwind-4.x-38bdf8.svg)](https://tailwindcss.com)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A cross-platform desktop application for managing Docker containers, images, volumes, networks, and compose stacks — built with **Tauri v2** + **React / TypeScript / Tailwind CSS**.

> Inspired by Docker Desktop, Podman Desktop, and Rancher Desktop.

![Screenshot](./assets/screenshot.png)

---

## Features

### Docker Resource Management

| Resource | Actions |
|---|---|
| **Containers** | List, start, stop, restart, remove. Log viewer with tail/since/until filters. Interactive terminal (sh, bash, zsh, ash, dash) with root option and single-command mode. Resource stats (CPU, memory, network, block I/O, PIDs). |
| **Images** | List, pull (with live progress), remove, tag, inspect |
| **Volumes** | List, create, remove, inspect |
| **Networks** | List, create (bridge/overlay/host/none), remove, inspect |
| **Docker Compose** | Up, down, streaming log viewer |

### User Interface

- Modern web-based UI rendered in a native WebView
- **22 distinct themes** with unique color palettes: Catppuccin, Tokyo Night, Dracula, Nord, Gruvbox, Solarized, Kanagawa, Moonfly, Nightfly, Oxocarbon, Ferra
- Auto mode follows OS `prefers-color-scheme`; manual override with instant preview
- **Sortable data tables** — click any column header to sort ascending/descending
- **Font size presets**: Normal / Large / Larger (proportional scaling across all UI)
- **Monospace font selector** — detects installed system fonts via `fc-list`
- Navigation sidebar with PNG icon (adapts to theme) and connection status indicator
- Modal dialogs for image pull and confirmation actions
- Live streaming output for image pulls, container logs, compose, and terminal sessions

### Cross-Platform

- **Linux**: Unix socket (`/var/run/docker.sock`)
- **macOS**: Unix socket  
- **Windows**: Named pipe + TCP

### Connection Options

- Local Docker daemon (auto-detect on startup)
- Remote Docker over TCP
- TLS-secured connections (certificate-based)

---

## Architecture

```
frontend/          ← React + TypeScript + Tailwind (SPA in WebView)
src-tauri/         ← Rust backend: Tauri commands → domain → infrastructure
crates/domain/     ← Entities, repository traits, domain errors
crates/infrastructure/ ← Docker API (bollard), config persistence
```

Dependency direction: `domain ← infrastructure ← src-tauri`

| Layer | Tech | Purpose |
|-------|------|---------|
| Frontend | React 19, Tailwind CSS 4 | Desktop UI rendered in WebView |
| IPC Bridge | Tauri v2 commands (32 commands) | Type-safe Rust ↔ JS communication |
| Domain | Pure Rust | Entities, traits, no framework deps |
| Infrastructure | Bollard 0.21, directories | Docker API, config persistence |

---

## Prerequisites

- **Node.js** 20+ and **npm**
- **Rust** 1.88+ ([rustup.rs](https://rustup.rs))
- **Docker Engine** running locally (or a remote daemon)
- **docker-compose** binary (optional, for Compose features)

### System Dependencies (Linux)

```bash
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev
```

**macOS / Windows**: No additional system dependencies required.

---

## Quick Start

### 1. Install frontend dependencies

```bash
cd frontend
npm install
```

### 2. Run in development mode

```bash
cd ..           # back to project root
cargo tauri dev
```

Starts the Vite dev server (hot-reload on port 5173) and opens the Tauri window.

### 3. Build for production

```bash
cargo tauri build
```

Output: `src-tauri/target/release/bundle/`.

---

## Development

```bash
# Backend
cargo check
cargo check -p domain
cargo check -p container-desktop-app
cargo clippy
cargo fmt

# Frontend
cd frontend
npm run dev        # Dev server with HMR (port 5173)
npm run build      # Production build
npx tsc --noEmit   # Type check only
```

---

## Usage Guide

### Screens

| Screen | Description |
|---|---|
| **Dashboard** | Connection status, Docker daemon info (version, OS, container/image counts, architecture) |
| **Containers** | Sortable table with state badges. Select a container to access: **Logs** (tail N lines, since/until datetime filters, follow mode), **Terminal** (shell picker, root checkbox, interactive vs single-command), **Stats** (CPU%, memory, network RX/TX, block I/O, PIDs) |
| **Images** | Sortable table. Pull from registry with live progress stream. Remove with confirmation. |
| **Volumes** | Sortable table. Create / remove. |
| **Networks** | Sortable table. Create with driver selector (bridge/overlay/host/none). Remove. |
| **Docker Compose** | Compose file path input, up/down buttons, live output stream |
| **Settings** | Theme (Auto/Manual + 22 variants), Docker endpoint URL, Font Size (Normal/Large/Larger), Monospace Font (system detection) |

### Theme Selection

1. Navigate to **Settings**
2. Toggle **Auto** (follows OS) or **Manual**
3. If Manual, pick a theme — changes apply instantly
4. Click **Save** to persist across restarts

Available themes: Light, Dark, Dracula, Nord, Solarized Light/Dark, Gruvbox Light/Dark, Catppuccin (Latte/Frappe/Macchiato/Mocha), Tokyo Night (Standard/Storm/Light), Kanagawa (Wave/Dragon/Lotus), Moonfly, Nightfly, Oxocarbon, Ferra.

### Font Size

Three proportional presets that scale the entire UI:

| Preset | Base px | Effect |
|---|---|---|
| **Normal** | 14 | Default |
| **Large** | 18 | ~30% larger |
| **Larger** | 22 | ~60% larger |

All text sizes use `rem` units — changing the root size scales everything proportionally while preserving layout aesthetics.

### Monospace Font

The dropdown lists all monospace fonts detected on your system via `fc-list`. The selection applies to code blocks, logs, terminal output, and table data. Falls back to system monospace if none selected.

---

## Configuration

Settings are persisted to:

| Platform | Path |
|---|---|
| Linux | `~/.config/container-desktop/ContainerDesktop/settings.json` |
| macOS | `~/Library/Application Support/com.container-desktop.ContainerDesktop/settings.json` |
| Windows | `C:\Users\<User>\AppData\Roaming\container-desktop\ContainerDesktop\config\settings.json` |

### Example `settings.json`

```json
{
  "theme_setting": { "Manual": "TokyoNight" },
  "endpoint": {
    "host_url": "unix:///var/run/docker.sock",
    "tls_ca": null,
    "tls_cert": null,
    "tls_key": null,
    "timeout_secs": 30
  },
  "window_width": 1280,
  "window_height": 800,
  "font_family": "JetBrains Mono",
  "font_size": 14
}
```

---

## Project Structure

```
container-desktop/
├── Cargo.toml                 # Virtual workspace
├── frontend/                  # React + TypeScript + Tailwind
│   ├── package.json
│   ├── vite.config.ts
│   ├── index.html
│   └── src/
│       ├── main.tsx           # React entry point
│       ├── App.tsx            # Layout, navigation, theme, font
│       ├── index.css          # Tailwind + 22 theme palettes
│       ├── lib/
│       │   ├── tauri.ts       # Tauri IPC bridge (32 commands + events)
│       │   └── types.ts       # TypeScript interfaces
│       ├── components/
│       │   ├── Sidebar.tsx    # Navigation sidebar + PNG icon
│       │   └── StatusBar.tsx  # Bottom status bar
│       └── screens/
│           ├── Dashboard.tsx
│           ├── Containers.tsx # Table + Logs/Terminal/Stats tabs
│           ├── Images.tsx
│           ├── Volumes.tsx
│           ├── Networks.tsx
│           ├── Compose.tsx
│           └── Settings.tsx
├── src-tauri/                 # Tauri application
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/default.json
│   └── src/
│       ├── main.rs
│       ├── lib.rs             # App state, setup, command registration
│       └── commands/          # Tauri IPC handlers
│           ├── connection.rs
│           ├── containers.rs  # + exec_create, exec_start, exec_input, exec_resize
│           ├── images.rs
│           ├── volumes.rs
│           ├── networks.rs
│           ├── compose.rs
│           └── settings.rs    # + list_fonts
└── crates/
    ├── domain/src/            # Entities + repository traits
    └── infrastructure/src/    # Docker API (bollard) + config persistence
```

---

## Technology Stack

| Technology | Purpose |
|---|---|
| [Tauri v2](https://tauri.app) | Desktop framework |
| [React 19](https://react.dev) | UI library |
| [TypeScript](https://www.typescriptlang.org) | Type-safe frontend |
| [Tailwind CSS 4](https://tailwindcss.com) | Utility-first CSS |
| [Vite](https://vite.dev) | Frontend build tool |
| [Bollard 0.21](https://github.com/fussybeaver/bollard) | Docker Engine API |
| [Tokio](https://tokio.rs) | Async runtime |
| [Serde](https://serde.rs) | Serialization |

---

## License

MIT — See [LICENSE](LICENSE) for details.
