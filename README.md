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

## AI Agent Bootstrap

Before any agent analyzes, answers questions about, plans changes for, or edits this repository, it must run:

```bash
./scripts/load-project-context.sh
```

That command is the canonical bootstrap entry point for project context. It enumerates `.deepseek/skills/`, verifies every `SKILL.md`, and prints the skill files so the active agent can load them before doing any repository work.

For automation or CI checks, use:

```bash
./scripts/load-project-context.sh --check
```

If the bootstrap fails, the agent should stop and surface the failure instead of continuing with partial context.

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
- Minimal **Acerca de** link in the sidebar footer for project and licensing details
- Modal dialogs for image pull and confirmation actions
- Live streaming output for image pulls, container logs, compose, and terminal sessions

### Cross-Platform

- **Linux**: Unix socket (`/var/run/docker.sock`)
- **macOS**: Unix socket  
- **Windows**: Named pipe + TCP

### Connection Options

- Local Docker daemon (auto-detect on startup)
- Local loopback Docker over TCP (`tcp://localhost`, `127.0.0.1`, `::1`)
- TLS-secured remote connections are not implemented yet

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

- **Node.js** 24 LTS and **npm**
- **Rust** 1.88+ ([rustup.rs](https://rustup.rs))
- **Docker Engine** running locally (or a remote daemon)
- **Docker Compose v2** via `docker compose` (required for Compose features)

### System Dependencies (Linux)

```bash
sudo apt install -y \
  pkg-config \
  libglib2.0-dev \
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
| **Settings** | Theme (Auto/Manual + 22 variants), Docker endpoint URL, remote connection help modal, Font Size (Normal/Large/Larger), Monospace Font (system detection) |
| **Acerca de** | Project summary, MIT license, tech stack, and vibe coding note with minimalist access from the sidebar footer |

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

### Remote Docker via SSH tunnel

Container Desktop only accepts direct `tcp://` endpoints for local loopback addresses. To use a Docker daemon from another machine on your LAN, expose the remote Docker socket on the **remote loopback** and tunnel it back to your machine over SSH.

1. Install `socat` on the remote machine:

   ```bash
   # Ubuntu / Debian
   sudo apt update && sudo apt install -y socat
   ```

2. On the remote machine, bridge loopback TCP to the Docker socket:

   ```bash
   socat TCP-LISTEN:2375,bind=127.0.0.1,fork UNIX-CONNECT:/var/run/docker.sock
   ```

3. On your local machine, create the SSH tunnel:

   ```bash
   ssh -N -L 23750:127.0.0.1:2375 usuario@192.168.0.135
   ```

4. In **Settings**, use this endpoint:

   ```text
   tcp://127.0.0.1:23750
   ```

The Settings screen includes the same instructions in a built-in help modal next to the Docker endpoint field.

---

## Planned Security Work

The next major functional area planned for Container Desktop is a dedicated **Security** screen focused on Docker image vulnerability visibility.

Planned scope:

1. Add a **Security** screen to the sidebar with a global image security summary and per-image drill-down.
2. Detect whether **Grype**, **Trivy**, and **Docker Scout** are installed on the host and show their availability in the UI.
3. Allow users to enable one or more installed scanners and run one background worker per selected tool, scanning images one at a time.
4. Persist scan results locally so previously completed analyses can be reopened later for each image.
5. Rebuild startup statistics from persisted scan data and current images, merging duplicate findings across tools so summary charts do not double-count vulnerabilities.
6. Show OS-specific installation guidance when the user selects a scanner that is not installed.

This work is currently tracked as backlog and is not implemented in `1.0.0`.

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
