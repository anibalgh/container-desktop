# Container Desktop — Frontend

React + TypeScript + Tailwind CSS application running inside a Tauri WebView.

## Scripts

```bash
npm run dev        # Vite dev server (port 5173, HMR enabled)
npm run build      # Production build → dist/
npx tsc --noEmit   # TypeScript type check only
```

## Architecture

```
src/
├── main.tsx         # ReactDOM entry point
├── App.tsx          # Layout, navigation routing, theme state
├── index.css        # Tailwind imports + CSS custom properties (light/dark theme)
├── lib/
│   ├── tauri.ts     # Typed wrappers for all 28 Tauri IPC commands + event listeners
│   └── types.ts     # TypeScript interfaces mirroring domain entities
├── components/
│   ├── Sidebar.tsx  # Navigation sidebar (7 primary screens + about link + connection indicator)
│   └── StatusBar.tsx # Bottom bar (screen title, Docker version, endpoint, app version)
└── screens/
    ├── About.tsx      # Project summary, license, tech stack, vibe coding note
    ├── Dashboard.tsx   # Docker daemon info + stat cards
    ├── Containers.tsx  # Table with start/stop/restart/remove actions
    ├── Images.tsx      # Table + pull modal with live progress stream
    ├── Volumes.tsx     # Table + create modal
    ├── Networks.tsx    # Table + create modal with driver selector
    ├── Compose.tsx     # File path input + up/down + live output viewer
    └── Settings.tsx    # Theme picker, endpoint config, font settings
```

## Backend Communication

All IPC goes through `@tauri-apps/api`:

- **Commands**: `invoke("command_name", { args })` — returns `Promise<T>`
- **Events**: `listen<T>("event-name", callback)` — streaming data (logs, pull progress, compose output)

See `src/lib/tauri.ts` for the complete IPC bridge.

The status bar resolves the displayed app version from Tauri at runtime and falls back to build-time frontend metadata, so the UI stays aligned with packaged releases.
