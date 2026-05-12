# Container Desktop

A cross-platform desktop application for managing Docker resources, built with **Tauri v2** + **React/TypeScript/Tailwind CSS**.

## Project skills bootstrap

At the start of **every session in this repository**, before planning, coding, reviewing, or answering questions, the active AI agent must load and internalize the reusable project skills stored under:

```text
.deepseek/skills/*/SKILL.md
```

This startup requirement applies **regardless of the model, tool, or AI agent implementation** being used.

### Canonical bootstrap command

Before doing anything else in this repository, run:

```bash
./scripts/load-project-context.sh
```

This script is the canonical bootstrap entry point. It enumerates every directory inside `.deepseek/skills/`, verifies each `SKILL.md` exists, and prints their contents so the active agent can read them immediately.

### Required startup behavior

1. Run `./scripts/load-project-context.sh`.
2. Read each `SKILL.md` file printed by the script.
3. Treat those files as active project guidance for the rest of the session.
4. Re-run the script if a task suggests skills may have been added or renamed during the session.

### Hard failure policy

If the bootstrap script fails, stop. Do **not** plan, edit files, review code, or answer repository-specific questions until the bootstrap succeeds or the failure is reported to the user.

### Skills currently expected in this repository

- `.deepseek/skills/docker/SKILL.md`
- `.deepseek/skills/docker-compose/SKILL.md`
- `.deepseek/skills/git/SKILL.md`
- `.deepseek/skills/github/SKILL.md`
- `.deepseek/skills/rust/SKILL.md`

If a direct user instruction conflicts with one of these skills, follow the user instruction for that task and otherwise keep the skills as the default project guidance.

## Architecture

This project uses **Clean Architecture** as a mandatory design rule. All new code, refactors, and bug fixes must preserve and reinforce that architecture instead of bypassing it with cross-layer shortcuts.

```
frontend/          ← React + TypeScript + Tailwind (SPA in WebView)
src-tauri/         ← Rust backend: Tauri commands → domain → infrastructure
crates/domain/     ← Entities, repository traits, domain errors
crates/infrastructure/ ← Docker API (bollard), config persistence
```

Dependency direction: `domain ← infrastructure ← src-tauri`

### Clean Architecture rules

- Keep business entities and repository traits in `crates/domain`.
- Keep Docker, persistence, and other external integrations in `crates/infrastructure`.
- Keep Tauri commands and application wiring in `src-tauri`.
- Depend only inward: outer layers may use inner layers, but inner layers must never depend on outer layers.
- Do not move domain logic into UI, Tauri command handlers, or infrastructure adapters.

| Layer | Tech | Purpose |
|-------|------|---------|
| Frontend | React, Tailwind CSS | Desktop UI rendered in WebView |
| IPC Bridge | Tauri commands | Type-safe Rust ↔ JS communication |
| Domain | Pure Rust | Entities, traits, no framework deps |
| Infrastructure | Bollard, directories | Docker API, config persistence |

## Quick Start

```bash
# Prerequisites: Docker, Node.js 24 LTS, Rust 1.80+
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
  pkg-config \
  libglib2.0-dev \
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

## Platform Support

Container Desktop targets **Linux**, **macOS**, and **Windows** via Tauri v2.

### Startup defaults

`DockerEndpoint::default()` selects the platform-appropriate local transport:

- **Linux / macOS**: `unix:///var/run/docker.sock`
- **Windows**: `npipe:////./pipe/docker_engine`

### Docker transport backends

`DockerClient::connect()` in `crates/infrastructure/src/docker/mod.rs` uses `#[cfg(target_os)]`:

| Transport | Supported on | Backend |
|-----------|-------------|---------|
| `unix://`  | Linux, macOS | `Docker::connect_with_local_defaults()` (bollard) |
| `tcp://`   | All | `Docker::connect_with_http()` (plain HTTP, no TLS yet) |
| `npipe://` | Windows only | `Docker::connect_with_named_pipe()` |

Non-Windows platforms that attempt `npipe://` get a clear error: "Named pipe connections are only supported on Windows".

### Font enumeration

`list_fonts()` in `src-tauri/src/commands/settings.rs` uses `#[cfg]` per platform:

- **Linux**: `fc-list :spacing=mono family` (fontconfig)
- **macOS**: `system_profiler SPFontsDataType` → parses "Family:" lines
- **Windows**: returns a curated list of 10 common monospace fonts (Cascadia Code, Consolas, Courier New, Fira Code, Hack, JetBrains Mono, Lucida Console, Monospace, Source Code Pro)

Fallback: if enumeration yields zero results, all platforms return at least `["Monospace"]`.

### Docker Compose

`ComposeClient` uses `docker compose` (subcommand, Docker 20.10+). This is the modern standard
across all platforms. The deprecated `docker-compose` standalone binary is not used; if needed,
change `make_compose_command()` in `crates/infrastructure/src/docker/compose.rs`.

### Config directory

`ConfigManager` uses the `directories` crate (`ProjectDirs`):

- **Linux**: `~/.config/container-desktop/settings.json`
- **macOS**: `~/Library/Application Support/com.container-desktop.ContainerDesktop/settings.json`
- **Windows**: `%APPDATA%\container-desktop\ContainerDesktop\settings.json`

### Known limitations

| Limitation | Platform | Detail |
|-----------|----------|--------|
| Font list limited | Windows, macOS | Windows returns a static list (no DirectWrite binding). macOS parses `system_profiler` output which lists all fonts, not just monospace. |
| No TLS support | All | `tcp://` connections are plain HTTP only. TLS (HTTPS with certs) is not yet implemented. |
| Named pipes | Linux, macOS | `npipe://` transport errors out — only valid on Windows. |
| `docker compose` required | All | The legacy `docker-compose` binary is not used. Ensure Docker 20.10+ is installed. |

### System requirements

- **Linux**: Docker Engine, WebKit2GTK 4.1, libsoup 3.0, libjavascriptcoregtk 4.1, GTK 3
- **macOS**: Docker Desktop, Xcode Command Line Tools
- **Windows**: Docker Desktop, Visual C++ Redistributable
