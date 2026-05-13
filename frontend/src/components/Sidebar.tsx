import darkPng from "../assets/icons/dark-icon.png";
import lightPng from "../assets/icons/light-icon.png";
import { useI18n } from "../i18n";

const NAV_ITEMS = [
  { id: "dashboard", icon: "📊" },
  { id: "containers", icon: "📦" },
  { id: "images", icon: "🖼️" },
  { id: "security", icon: "🛡️" },
  { id: "volumes", icon: "💾" },
  { id: "networks", icon: "🌐" },
  { id: "compose", icon: "📋" },
  { id: "settings", icon: "⚙️" },
] as const;

export const ABOUT_SCREEN = "about" as const;

export type Screen = (typeof NAV_ITEMS)[number]["id"] | typeof ABOUT_SCREEN;

interface SidebarProps {
  active: Screen;
  connected: boolean;
  darkMode: boolean;
  onNavigate: (screen: Screen) => void;
}

export function Sidebar({ active, connected, darkMode, onNavigate }: SidebarProps) {
  const { t } = useI18n();

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
          <div className="text-sm font-semibold text-white">{t.sidebar.productNamePrimary}</div>
          <div className="text-xs text-white/60">{t.sidebar.productNameSecondary}</div>
        </div>
      </div>

      <nav className="flex-1 py-2">
        {NAV_ITEMS.map((item) => (
          <button
            key={item.id}
            onClick={() => onNavigate(item.id)}
            className="w-full flex items-center gap-3 px-4 py-2.5 text-sm transition-colors text-left"
            style={{
              backgroundColor:
                active === item.id
                  ? "color-mix(in srgb, var(--color-sidebar-active) 20%, transparent)"
                  : "transparent",
              color: active === item.id ? "white" : "var(--color-sidebar-text)",
              borderLeft:
                active === item.id
                  ? "3px solid var(--color-sidebar-active)"
                  : "3px solid transparent",
            }}
          >
            <span className="text-base">{item.icon}</span>
            <span>{t.sidebar.screens[item.id]}</span>
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
            {t.sidebar.screens.about}
          </button>
        </div>
        <div className="flex items-center gap-2 text-xs">
          <span className={`w-2 h-2 rounded-full ${connected ? "bg-green-400" : "bg-red-400"}`} />
          <span style={{ color: "var(--color-sidebar-text)" }}>
            {connected ? t.sidebar.connected : t.sidebar.disconnected}
          </span>
        </div>
      </div>
    </aside>
  );
}
