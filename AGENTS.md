# Container Desktop

A cross-platform desktop application for managing Docker resources, built with **Tauri v2** + **React/TypeScript/Tailwind CSS**.

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
| Frontend | React, Tailwind CSS | Desktop UI rendered in WebView |
| IPC Bridge | Tauri commands | Type-safe Rust ↔ JS communication |
| Domain | Pure Rust | Entities, traits, no framework deps |
| Infrastructure | Bollard, directories | Docker API, config persistence |

## Quick Start

```bash
# Prerequisites: Docker, Node.js 20+, Rust 1.80+
# System deps (Linux): libwebkit2gtk-4.1-dev, libsoup-3.0-dev, libjavascriptcoregtk-4.1-dev

# Install frontend deps
cd frontend && npm install

# Run in development mode
cd .. && cargo tauri dev

# Build release
cargo tauri build
```

## Build Commands

```bash
# Check all crates
cargo check

# Check specific crate
cargo check -p domain
cargo check -p infrastructure
cargo check -p container-desktop-app

# Frontend
cd frontend && npm run dev       # Dev server
cd frontend && npm run build     # Production build
cd frontend && npx tsc --noEmit # Type check

# Lint
cargo clippy
cargo fmt
```

## Project Structure

```
container-desktop/
├── Cargo.toml                 # Virtual workspace
├── AGENTS.md
├── frontend/                  # React + TypeScript + Tailwind
│   ├── package.json
│   ├── vite.config.ts
│   ├── index.html
│   └── src/
│       ├── main.tsx           # React entry point
│       ├── App.tsx            # Layout, navigation, theme
│       ├── index.css          # Tailwind + CSS variables
│       ├── lib/
│       │   ├── tauri.ts       # Tauri IPC bridge (all commands)
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
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json
│   ├── icons/
│   └── src/
│       ├── main.rs            # Tauri entry point
│       ├── lib.rs             # App state, setup, command registration
│       └── commands/
│           ├── mod.rs
│           ├── connection.rs
│           ├── containers.rs
│           ├── compose.rs
│           ├── images.rs
│           ├── networks.rs
│           ├── settings.rs
│           └── volumes.rs
├── crates/
│   ├── domain/src/
│   │   ├── lib.rs
│   │   ├── error.rs
│   │   ├── entities/          # Image, Container, Volume, etc.
│   │   └── repository/        # Async traits
│   └── infrastructure/src/
│       ├── lib.rs
│       ├── config/mod.rs      # ConfigManager (settings persistence)
│       └── docker/
│           ├── mod.rs         # DockerClient
│           ├── connection.rs
│           ├── images.rs
│           ├── containers.rs
│           ├── volumes.rs
│           ├── networks.rs
│           └── compose.rs
└── assets/                    # Legacy icons
```

## Key Patterns

### Adding a new Docker resource

1. Create entity in `crates/domain/src/entities/`
2. Add repository trait in `crates/domain/src/repository/`
3. Implement trait in `crates/infrastructure/src/docker/`
4. Add Tauri command in `src-tauri/src/commands/`
5. Register command in `src-tauri/src/lib.rs`
6. Add TypeScript binding in `frontend/src/lib/tauri.ts`
7. Create screen in `frontend/src/screens/`
8. Add screen to `frontend/src/App.tsx`

### Frontend ↔ Backend Communication

- All IPC goes through `@tauri-apps/api` (invoke + event listeners)
- `src-tauri/src/commands/*.rs`: `#[tauri::command]` functions
- `frontend/src/lib/tauri.ts`: typed TypeScript wrappers
- Streaming commands (logs, pull progress) use Tauri events
- Settings loaded via `load_settings` / `save_settings` commands

### Theme System

- CSS custom properties in `index.css` define light/dark palettes
- `.dark` class on `<html>` toggles dark mode
- Theme preference persisted via `AppSettings.theme_setting`
- Auto mode follows OS `prefers-color-scheme`

## Troubleshooting

### Docker connection fails

- On Linux: verify `docker.sock` permissions (`ls -la /var/run/docker.sock`)
- Add user to docker group: `sudo usermod -aG docker $USER`
- For remote connections: verify port is accessible and TLS certs are valid

### Build errors with missing system libraries

```bash
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev
```
