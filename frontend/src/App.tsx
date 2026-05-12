import { useState, useEffect, useCallback } from "react";
import { Sidebar, type Screen } from "./components/Sidebar";
import { StatusBar } from "./components/StatusBar";
import { Dashboard } from "./screens/Dashboard";
import { ContainersScreen } from "./screens/Containers";
import { ImagesScreen } from "./screens/Images";
import { VolumesScreen } from "./screens/Volumes";
import { NetworksScreen } from "./screens/Networks";
import { ComposeScreen } from "./screens/Compose";
import { SettingsScreen } from "./screens/Settings";
import { AboutScreen } from "./screens/About";
import type { DockerInfo, ThemeVariant } from "./lib/types";
import { loadSettings } from "./lib/tauri";

const DARK_VARIANTS: ThemeVariant[] = [
  "Dark", "Dracula", "Nord", "SolarizedDark", "GruvboxDark",
  "CatppuccinFrappe", "CatppuccinMacchiato", "CatppuccinMocha",
  "TokyoNight", "TokyoNightStorm", "KanagawaWave", "KanagawaDragon",
  "Moonfly", "Nightfly", "Oxocarbon", "Ferra",
];

function isDark(variant: ThemeVariant): boolean {
  return DARK_VARIANTS.includes(variant);
}

export default function App() {
  const [activeScreen, setActiveScreen] = useState<Screen>("Dashboard");
  const [dockerInfo, setDockerInfo] = useState<DockerInfo | null>(null);
  const [themeVariant, setThemeVariant] = useState<ThemeVariant>(() =>
    window.matchMedia("(prefers-color-scheme: dark)").matches ? "Dark" : "Light",
  );

  useEffect(() => {
    loadSettings().then((s) => {
      if (typeof s.theme_setting === "object" && "Manual" in s.theme_setting) {
        setThemeVariant(s.theme_setting.Manual);
      }
      // Apply font size globally
      document.documentElement.style.fontSize = `${s.font_size}px`;
      if (s.font_family) {
        document.documentElement.style.setProperty("--font-mono", `"${s.font_family}", monospace`);
      }
    }).catch(() => {});

    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const handler = (e: MediaQueryListEvent) => {
      loadSettings().then((s) => {
        if (s.theme_setting === "Auto") setThemeVariant(e.matches ? "Dark" : "Light");
      }).catch(() => {});
    };
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, []);

  useEffect(() => {
    const dark = isDark(themeVariant);
    document.documentElement.setAttribute("data-theme", themeVariant);
    document.documentElement.classList.toggle("dark", dark);
    document.documentElement.style.colorScheme = dark ? "dark" : "light";
  }, [themeVariant]);

  const connected = dockerInfo !== null;

  const handleConnectionChange = useCallback((info: DockerInfo | null) => {
    setDockerInfo(info);
  }, []);

  function handleThemeChange(variant: ThemeVariant) {
    setThemeVariant(variant);
  }

  function renderScreen() {
    switch (activeScreen) {
      case "Dashboard": return <Dashboard connected={connected} onConnectionChange={handleConnectionChange} />;
      case "Containers": return <ContainersScreen />;
      case "Images": return <ImagesScreen />;
      case "Volumes": return <VolumesScreen />;
      case "Networks": return <NetworksScreen />;
      case "Compose": return <ComposeScreen />;
      case "Settings": return <SettingsScreen onThemeChange={handleThemeChange} />;
      case "Acerca de": return <AboutScreen />;
      default: return <Dashboard connected={connected} onConnectionChange={handleConnectionChange} />;
    }
  }

  return (
    <div className="flex h-screen overflow-hidden">
      <Sidebar active={activeScreen} connected={connected} darkMode={isDark(themeVariant)} onNavigate={setActiveScreen} />
      <div className="flex-1 flex flex-col min-w-0">
        <main className="flex-1 overflow-auto">{renderScreen()}</main>
        <StatusBar title={activeScreen} dockerVersion={dockerInfo?.server_version} endpoint={dockerInfo?.endpoint} />
      </div>
    </div>
  );
}
