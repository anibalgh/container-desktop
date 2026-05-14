# Changelog

## 1.0.2

- Hardened cross-platform Docker endpoint validation so unsupported local transports are rejected per operating system.
- Expanded the container terminal UI with Windows shell presets and shell-specific command handling.
- Improved cross-platform endpoint guidance, font handling, CI validation, and release packaging coverage for Linux, macOS, and Windows.
- Fixed the macOS bootstrap check by removing the Bash 4-only `mapfile` dependency from `scripts/load-project-context.sh`.
- Fixed Windows compilation in `crates/infrastructure` by gating the Unix socket Docker connection path behind platform-specific `#[cfg]`.
- Updated GitHub Actions jobs to explicit Node 24 LTS usage and moved release artifact uploads to a Node 24-compatible action version.
- Replaced the retired `macos-13` release runner label with `macos-15-intel` so the Intel macOS bundle job can be picked up by a hosted runner again.

## 1.0.1

- Added the native **Buscar... / Browse...** file picker in the Compose screen.
- Restricted the picker to Compose files with `.yml` and `.yaml` extensions.
- Auto-filled the selected Compose path so **Levantar / Up** is enabled as soon as a valid file is chosen.
- Bumped application release metadata from `1.0.0` to `1.0.1`.
