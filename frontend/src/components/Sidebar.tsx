import darkPng from "../assets/icons/dark-icon.png";
import lightPng from "../assets/icons/light-icon.png";

const NAV_ITEMS = [
  { label: "Dashboard", icon: "📊" },
  { label: "Containers", icon: "📦" },
  { label: "Images", icon: "🖼️" },
  { label: "Volumes", icon: "💾" },
  { label: "Networks", icon: "🌐" },
  { label: "Compose", icon: "📋" },
  { label: "Settings", icon: "⚙️" },
] as const;

export const ABOUT_SCREEN = "Acerca de" as const;

export type Screen = (typeof NAV_ITEMS)[number]["label"] | typeof ABOUT_SCREEN;

interface SidebarProps {
  active: Screen;
  connected: boolean;
  darkMode: boolean;
  onNavigate: (screen: Screen) => void;
}

export function Sidebar({ active, connected, darkMode, onNavigate }: SidebarProps) {
  return (
    <aside
      className="flex flex-col w-56 shrink-0 select-none"
      style={{
        backgroundColor: "var(--color-sidebar)",
        color: "var(--color-sidebar-text)",
      }}
    >
      <div className="flex items-center gap-3 px-4 py-4 border-b border-white/10">
        <img src={darkMode ? darkPng : lightPng} alt="" className="w-8 h-8 rounded-lg" />
        <div>
          <div className="text-sm font-semibold text-white">Container</div>
          <div className="text-xs text-white/60">Desktop</div>
        </div>
      </div>

      <nav className="flex-1 py-2">
        {NAV_ITEMS.map((item) => (
          <button
            key={item.label}
            onClick={() => onNavigate(item.label)}
            className="w-full flex items-center gap-3 px-4 py-2.5 text-sm transition-colors text-left"
            style={{
              backgroundColor:
                active === item.label
                  ? "color-mix(in srgb, var(--color-sidebar-active) 20%, transparent)"
                  : "transparent",
              color: active === item.label ? "white" : "var(--color-sidebar-text)",
              borderLeft:
                active === item.label
                  ? "3px solid var(--color-sidebar-active)"
                  : "3px solid transparent",
            }}
          >
            <span className="text-base">{item.icon}</span>
            <span>{item.label}</span>
          </button>
        ))}
      </nav>

      <div className="px-4 py-3 border-t border-white/10">
        <div className="flex justify-end mb-3">
          <button
            onClick={() => onNavigate(ABOUT_SCREEN)}
            className="text-[11px] leading-none transition-opacity hover:opacity-100"
            style={{
              color: active === ABOUT_SCREEN ? "white" : "rgba(255, 255, 255, 0.65)",
              textDecoration: active === ABOUT_SCREEN ? "underline" : "none",
              textUnderlineOffset: "0.2em",
            }}
          >
            {ABOUT_SCREEN}
          </button>
        </div>
        <div className="flex items-center gap-2 text-xs">
          <span className={`w-2 h-2 rounded-full ${connected ? "bg-green-400" : "bg-red-400"}`} />
          <span style={{ color: "var(--color-sidebar-text)" }}>
            {connected ? "Connected" : "Disconnected"}
          </span>
        </div>
      </div>
    </aside>
  );
}
