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

## Testing

```bash
# Run all tests (71 tests across 3 crates)
cargo test

# Run tests for a specific crate
cargo test -p domain
cargo test -p infrastructure
cargo test -p container-desktop-app

# Run tests with output
cargo test -- --nocapture

# Run a subset by name
cargo test -- validate_container_name
cargo test -p domain -- theme_variant
```

| Crate | Tests | Focus |
|-------|-------|-------|
| `domain` | 34 | Entities (serialization, Display, defaults), `DomainError` formatting, `ThemeVariant` (is_dark, all variants) |
| `infrastructure` | 23 | Input validators (`validate_compose_path`, `validate_container_name`), `ConfigManager` (save/load settings, font sync) |
| `src-tauri` | 14 | Input validators (`validate_docker_id`, `validate_endpoint_url`) |

### Test isolation

- `ConfigManager::with_path()` (test-only constructor) accepts a custom path so tests don't touch the real user config directory.
- Compose path tests create temporary `.yml` files in `/tmp` and clean them up.
- Each `ConfigManager` test uses a uniquely-named JSON file to avoid parallel-test collisions.

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

## Security

### Input validation layers

All user-supplied input from the frontend is validated at two levels:

| Layer | Location | Validators |
|-------|----------|------------|
| **Tauri commands** | `src-tauri/src/commands/` | `validate_docker_id()` — length ≤1024, non-empty, no null bytes. `validate_endpoint_url()` — scheme allowlist (unix://, tcp://, npipe://), length ≤4096. Exec args sanitized. |
| **Infrastructure** | `crates/infrastructure/src/docker/` | `validate_compose_path()` — no `..` traversal, .yml/.yaml only, max 10 MB, canonicalized. `validate_container_name()` — Docker naming rules, max 255 chars. |

### Content Security Policy

The WebView CSP is restricted to same-origin resources:

```
default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline';
img-src 'self' data:; connect-src 'self' ipc: http://ipc.localhost;
font-src 'self' data:;
```

Configured in `src-tauri/tauri.conf.json` under `app.security.csp`.

### Defense in depth

- **Path traversal**: compose file paths reject `..`, canonicalize before use, and validate extensions.
- **DoS prevention**: all IDs and URLs have maximum length limits; compose files capped at 10 MB; exec input capped at 64 KB.
- **Null byte injection**: all string inputs are checked for embedded `\0`.
- **Scheme restriction**: Docker endpoint URLs must use an allowed transport scheme.
- **Container names**: enforced against Docker's `[a-zA-Z0-9_.-]+` pattern.
