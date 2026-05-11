# Container Desktop — Tauri IPC API Contract

All commands return `Result<T, String>` where `String` is the error message.
Streaming commands emit Tauri events.

---

## Connection

| Command | Params | Returns |
|---------|--------|---------|
| `connect` | `endpoint: DockerEndpoint` | `DockerInfo` |
| `test_connection` | — | `DockerInfo` |
| `ping` | — | `bool` |

## Containers

| Command | Params | Returns |
|---------|--------|---------|
| `list_containers` | `all: bool` | `Vec<Container>` |
| `create_container` | `config: ContainerConfig` | `String` (id) |
| `start_container` | `id: String` | `()` |
| `stop_container` | `id: String` | `()` |
| `restart_container` | `id: String` | `()` |
| `remove_container` | `id: String` | `()` |
| `container_logs` | `id: String, tail: u32?, follow: bool` | **event stream**: `container-log-line` → `LogLine` |
| `inspect_container` | `id: String` | `String` (JSON) |
| `container_stats` | `id: String` | `ContainerStats` |

## Images

| Command | Params | Returns |
|---------|--------|---------|
| `list_images` | — | `Vec<Image>` |
| `pull_image` | `name: String, tag: String?` | **event stream**: `image-pull-progress` → `String` |
| `remove_image` | `id: String` | `()` |
| `tag_image` | `id: String, repo: String, tag: String` | `()` |
| `inspect_image` | `id: String` | `String` (JSON) |

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
| `create_network` | `name: String, driver: String?` | `String` (id) |
| `remove_network` | `id: String` | `()` |
| `inspect_network` | `id: String` | `String` (JSON) |

## Compose

| Command | Params | Returns |
|---------|--------|---------|
| `list_stacks` | — | `Vec<ComposeStack>` |
| `compose_up` | `file_path: String` | **event stream**: `compose-output` → `LogLine` |
| `compose_down` | `file_path: String` | `()` |
| `compose_logs` | `file_path: String` | **event stream**: `compose-output` → `LogLine` |
| `compose_ps` | `file_path: String` | `Vec<String>` |

## Settings

| Command | Params | Returns |
|---------|--------|---------|
| `load_settings` | — | `AppSettings` |
| `save_settings` | `settings: AppSettings` | `()` |

---

## Event Streams

Frontend subscribes via `listen<T>("event-name", callback)`.

| Event | Payload | Triggered by |
|-------|---------|-------------|
| `container-log-line` | `LogLine` | `container_logs` streaming response |
| `image-pull-progress` | `String` | `pull_image` streaming response |
| `compose-output` | `LogLine` | `compose_up`, `compose_logs` |
