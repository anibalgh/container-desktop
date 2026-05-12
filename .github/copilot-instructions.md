# Copilot Instructions

Before any repository-specific analysis, answer, plan, review, or code change in this repository:

1. Run `./scripts/load-project-context.sh`.
2. Read every `SKILL.md` file printed by that command.
3. Treat `AGENTS.md` and those skill files as active project instructions for the rest of the session.
4. If the script fails, stop and report the failure instead of proceeding with partial context.
5. Re-run the script whenever `.deepseek/skills/` may have changed during the session.

This bootstrap step is mandatory for every session, regardless of model, tool, or agent implementation.

## Build, test, and lint commands

Use the workspace root unless noted otherwise.

```bash
# Rust workspace
cargo test
cargo test -p domain theme_variant
cargo test -p infrastructure validate_container_name
cargo test -p container-desktop-app validate_endpoint_url
cargo clippy --all-targets --all-features
cargo fmt --all -- --check

# Frontend
cd frontend && npm run build
cd frontend && npm run lint
cd frontend && npx tsc --noEmit

# Local app build/run
cargo tauri dev
cargo tauri build
```

There is no frontend test script in `frontend/package.json`; automated test coverage currently lives in the Rust crates.

## High-level architecture

- This repo uses Clean Architecture with a strict flow: React screens in `frontend` call typed wrappers in `frontend/src/lib/tauri.ts`, which invoke Tauri commands in `src-tauri/src/commands`, which depend on traits and entities in `crates/domain`, while `crates/infrastructure` provides the concrete Docker/config implementations.
- `src-tauri/src/lib.rs` is the composition root. It loads persisted settings through `ConfigManager`, creates the shared `DockerClient`, registers all commands, and emits the initial `docker-connected` or `docker-error` event after startup.
- `frontend/src/App.tsx` bootstraps the UI by loading settings before rendering, applying font and theme state to `<html>`, resolving language, and routing between screens.
- `frontend/src/lib/types.ts` mirrors the Rust domain contracts for the frontend. Keep Rust and TypeScript data shapes aligned rather than introducing frontend-only variants.
- Long-running Docker operations do not return large synchronous payloads; they stream through Tauri events such as `container-log-line`, `exec-output`, `image-pull-progress`, `compose-output`, `docker-connected`, and `docker-error`.

## Key conventions

- Keep Tauri commands thin. Input validation happens in `src-tauri/src/commands`, but command handlers should mainly validate and delegate to repository implementations in `crates/infrastructure`.
- Preserve serialized Rust enum shapes in TypeScript. For example, `ThemeSetting` and `LanguageSetting` are `"Auto"` or `{ Manual: ... }`, not flattened frontend-specific forms.
- Settings are the app bootstrap mechanism. `ConfigManager::load_settings()` creates a default `settings.json` on first run, and the frontend immediately applies font family, font size, theme, and language from those settings.
- Saving settings is not cosmetic: endpoint changes are validated and applied to the live Docker client before persistence succeeds.
- Use `frontend/src/i18n` for UI text. Existing screens and shared components read text through `useI18n()` instead of inline literals.
- Compose support uses `docker compose` via `ComposeClient`, and compose file paths are validated and canonicalized before execution.
- Input validation is intentionally layered: Tauri commands guard Docker IDs and endpoint URLs, while infrastructure adds stricter resource-level checks such as compose path validation and Docker container-name validation.
- Plain `tcp://` Docker endpoints are only allowed for loopback/local hosts while TLS support remains unimplemented.
- When adding a new Docker resource or capability, update every layer consistently: domain entity/trait, infrastructure implementation, Tauri command and registration, frontend `lib/tauri.ts` wrapper, mirrored frontend types if needed, and the consuming screen/navigation flow.
