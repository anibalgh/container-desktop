# Container Desktop

[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/tauri-2.x-blue.svg)](https://tauri.app)
[![React](https://img.shields.io/badge/react-19.x-61dafb.svg)](https://react.dev)
[![Tailwind](https://img.shields.io/badge/tailwind-4.x-38bdf8.svg)](https://tailwindcss.com)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A cross-platform desktop application for managing Docker containers, images, volumes, networks, and compose stacks — built with **Tauri v2** + **React / TypeScript / Tailwind CSS**.

> Inspired by Docker Desktop, Podman Desktop, and Rancher Desktop.

---

## Features

### Docker Resource Management

| Resource | Actions |
|---|---|
| **Containers** | List, start, stop, restart, remove, view logs, interactive terminal |
| **Images** | List, pull (with live progress), remove, tag, inspect |
| **Volumes** | List, create, remove, inspect |
| **Networks** | List, create (bridge/overlay/host), remove, inspect |
| **Docker Compose** | Up, down, streaming log viewer |

### User Interface

- Modern web-based UI rendered in a native WebView
- Dark/light theme with CSS custom properties
- Auto mode follows OS `prefers-color-scheme`
- 22 manual theme variants persisted to user config
- Sidebar navigation with connection status indicator
- Responsive data tables for all Docker resources
- Modal dialogs for pull, create, and confirmation actions
- Live streaming output for image pulls, container logs, and compose

### Cross-Platform

- **Linux**: Unix socket (`/var/run/docker.sock`)
- **macOS**: Unix socket
- **Windows**: Named pipe + TCP

### Connection Options

- Local Docker daemon (auto-detect)
- Remote Docker over TCP
- TLS-secured connections (certificate-based)
- Auto-connect on startup with event-based status

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
| IPC Bridge | Tauri v2 commands | Type-safe Rust ↔ JS communication |
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

This starts the Vite dev server (hot-reload) and opens the Tauri window.

### 3. Build for production

```bash
cargo tauri build
```

The output bundle will be in `src-tauri/target/release/bundle/`.

---

## Development

### Backend (Rust)

```bash
cargo check                        # Check entire workspace
cargo check -p domain              # Check domain crate only
cargo check -p container-desktop-app  # Check Tauri app only
cargo clippy                       # Lint
cargo fmt                          # Format
```

### Frontend (TypeScript)

```bash
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
| **Dashboard** | Connection status, Docker daemon info (version, OS, container/image counts) |
| **Containers** | List all containers with state badges, start/stop/restart/remove, confirmation dialogs |
| **Images** | List images, pull from registry with live progress, remove |
| **Volumes** | List volumes, create, remove |
| **Networks** | List networks, create (with driver selection: bridge/overlay/host/none), remove |
| **Docker Compose** | Compose file path input, up/down buttons, live output stream |
| **Settings** | Theme mode (Auto/Manual + 22 variants), Docker endpoint URL, font family/size |

### Theme Selection

1. Navigate to **Settings**
2. Toggle **Auto** (follows OS) or **Manual**
3. If Manual, pick a theme from the dropdown (Dark, Dracula, Nord, Catppuccin, Tokyo Night, etc.)
4. Click **Save**

Available themes: Light, Dark, Dracula, Nord, Solarized Light/Dark, Gruvbox Light/Dark, Catppuccin (Latte/Frappe/Macchiato/Mocha), Tokyo Night (Standard/Storm/Light), Kanagawa (Wave/Dragon/Lotus), Moonfly, Nightfly, Oxocarbon, Ferra.

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
  "font_family": "Monospace",
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
│       ├── App.tsx            # Layout, navigation, theme
│       ├── index.css          # Tailwind + CSS variables
│       ├── lib/
│       │   ├── tauri.ts       # Tauri IPC bridge (28 commands)
│       │   └── types.ts       # TypeScript interfaces
│       ├── components/
│       │   ├── Sidebar.tsx    # Navigation sidebar
│       │   └── StatusBar.tsx  # Bottom status bar
│       └── screens/
│           ├── Dashboard.tsx
│           ├── Containers.tsx
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
│           ├── containers.rs
│           ├── images.rs
│           ├── volumes.rs
│           ├── networks.rs
│           ├── compose.rs
│           └── settings.rs
└── crates/
    ├── domain/src/            # Entities + repository traits
    └── infrastructure/src/    # Docker API (bollard) + config persistence
```

---

## Technology Stack

| Technology | Purpose |
|---|---|
| [Tauri v2](https://tauri.app) | Desktop framework (Rust backend + WebView frontend) |
| [React 19](https://react.dev) | UI library |
| [TypeScript](https://www.typescriptlang.org) | Type-safe frontend |
| [Tailwind CSS 4](https://tailwindcss.com) | Utility-first CSS framework |
| [Vite](https://vite.dev) | Frontend build tool |
| [Bollard](https://github.com/fussybeaver/bollard) | Docker Engine API client |
| [Tokio](https://tokio.rs) | Async runtime |
| [Serde](https://serde.rs) | Serialization |

---

## License

MIT — See [LICENSE](LICENSE) for details.
