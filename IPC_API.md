# Container Desktop — Tauri IPC API Contract

All commands return `Result<T, String>` where `String` is the surfaced error message.
Streaming workflows emit Tauri events keyed by a client-provided `requestId`.

---

## Connection

| Command | Params | Returns |
|---------|--------|---------|
| `connect` | `endpoint: DockerEndpoint` | `DockerInfo` |
| `test_connection` | — | `DockerInfo` |
| `ping` | — | `bool` |
| `docker_cleanup_summary` | — | `DockerCleanupSummary` |
| `docker_system_prune` | — | `()` |

## Containers

| Command | Params | Returns |
|---------|--------|---------|
| `list_containers` | `all: bool` | `Vec<Container>` |
| `start_container` | `id: String` | `()` |
| `stop_container` | `id: String` | `()` |
| `restart_container` | `id: String` | `()` |
| `remove_container` | `id: String` | `()` |
| `container_logs` | `id: String, options: { tail?: number, follow: boolean, since?: number, until?: number, requestId: string }` | `()` + `container-log-line` / `container-log-status` |
| `inspect_container` | `id: String` | `String` (JSON) |
| `container_stats` | `id: String` | `ContainerStats` |
| `exec_create` | `id: String, cmd: string[], user?: string` | `String` (exec id) |
| `exec_start` | `execIdStr: String, requestId: string` | `()` + `exec-output` / `exec-status` |
| `exec_input` | `execIdStr: String, data: number[]` | `()` |
| `exec_disconnect` | `execIdStr: String` | `()` |
| `exec_resize` | `execIdStr: String, width: u16, height: u16` | `()` |

## Images

| Command | Params | Returns |
|---------|--------|---------|
| `list_images` | — | `Vec<Image>` |
| `pull_image` | `name: String, tag?: String, requestId: string` | `()` + `image-pull-progress` / `image-pull-status` |
| `remove_image` | `id: String` | `()` |
| `tag_image` | `id: String, repo: String, tag: String` | `()` |
| `inspect_image` | `id: String` | `String` (JSON) |

## Security

| Command | Params | Returns |
|---------|--------|---------|
| `security_overview` | — | `SecurityOverview` |
| `image_security_report` | `imageId: String` | `ImageSecurityReport` |
| `configure_security_tools` | `tools: SecurityTool[]` | `SecurityOverview` |
| `open_external_link` | `url: String` | `()` |

## Volumes

| Command | Params | Returns |
|---------|--------|---------|
| `list_volumes` | — | `Vec<Volume>` |
| `create_volume` | `name: String` | `Volume` |
| `remove_volume` | `name: String` | `()` |
| `inspect_volume` | `name: String` | `String` (JSON) |

## Networks

| Command | Params | Returns |
|---------|--------|---------|
| `list_networks` | — | `Vec<Network>` |
| `create_network` | `name: String, driver?: String` | `String` (id) |
| `remove_network` | `id: String` | `()` |
| `inspect_network` | `id: String` | `String` (JSON) |

## Compose

| Command | Params | Returns |
|---------|--------|---------|
| `compose_up` | `filePath: String, requestId: string` | `()` + `compose-output` / `compose-status` |
| `compose_down` | `filePath: String` | `()` |
| `compose_logs` | `filePath: String, requestId: string` | `()` + `compose-output` / `compose-status` |
| `compose_ps` | `filePath: String` | `Vec<String>` |

## Settings

| Command | Params | Returns |
|---------|--------|---------|
| `load_settings` | — | `AppSettings` |
| `save_settings` | `settings: AppSettings` | `()` |
| `list_fonts` | — | `Vec<String>` |

---

## Event Streams

Frontend subscribes via `listen<T>("event-name", callback)`.

| Event | Payload | Triggered by |
|-------|---------|-------------|
| `docker-connected` | `DockerInfo` | Startup connection test |
| `docker-error` | `String` | Startup connection failure |
| `container-log-line` | `LogStreamEvent` | `container_logs` line output |
| `container-log-status` | `StreamStatusEvent` | `container_logs` lifecycle |
| `image-pull-progress` | `ProgressStreamEvent` | `pull_image` progress |
| `image-pull-status` | `StreamStatusEvent` | `pull_image` lifecycle |
| `compose-output` | `LogStreamEvent` | `compose_up`, `compose_logs` line output |
| `compose-status` | `StreamStatusEvent` | `compose_up`, `compose_logs` lifecycle |
| `exec-output` | `TextStreamEvent` | `exec_start` output |
| `exec-status` | `StreamStatusEvent` | `exec_start` lifecycle |
| `security-scan-progress` | `SecurityScanProgress` | Background security scans |

---

Total: 42 IPC commands + 11 event streams
