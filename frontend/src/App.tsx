import { useState, useEffect } from "react";
import { Sidebar, type Screen } from "./components/Sidebar";
import { StatusBar } from "./components/StatusBar";
import { Dashboard } from "./screens/Dashboard";
import { ContainersScreen } from "./screens/Containers";
import { ImagesScreen } from "./screens/Images";
import { VolumesScreen } from "./screens/Volumes";
import { NetworksScreen } from "./screens/Networks";
import { ComposeScreen } from "./screens/Compose";
import { SettingsScreen } from "./screens/Settings";
import type { DockerInfo } from "./lib/types";
import { loadSettings } from "./lib/tauri";

export default function App() {
  const [activeScreen, setActiveScreen] = useState<Screen>("Dashboard");
  const [dockerInfo, setDockerInfo] = useState<DockerInfo | null>(null);
  const [darkMode, setDarkMode] = useState(() => {
    // Default to OS preference, can be overridden by settings
    return window.matchMedia("(prefers-color-scheme: dark)").matches;
  });

  useEffect(() => {
    // Load theme preference from settings
    loadSettings()
      .then((settings) => {
        if (typeof settings.theme_setting === "object" && "Manual" in settings.theme_setting) {
          const variant = settings.theme_setting.Manual;
          // Determine if variant is dark
          const darkVariants = [
            "Dark", "Dracula", "Nord", "SolarizedDark",
            "GruvboxDark", "CatppuccinFrappe", "CatppuccinMacchiato",
            "CatppuccinMocha", "TokyoNight", "TokyoNightStorm",
            "KanagawaWave", "KanagawaDragon", "Moonfly",
            "Nightfly", "Oxocarbon", "Ferra",
          ];
          setDarkMode(darkVariants.includes(variant));
        }
      })
      .catch(() => {
        // Use default OS preference
      });

    // Listen for OS theme changes when in Auto mode
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    const handler = (e: MediaQueryListEvent) => {
      // Only auto-update if we haven't loaded a manual setting yet
      loadSettings().then((settings) => {
        if (settings.theme_setting === "Auto") {
          setDarkMode(e.matches);
        }
      }).catch(() => {});
    };
    mediaQuery.addEventListener("change", handler);
    return () => mediaQuery.removeEventListener("change", handler);
  }, []);

  // Apply dark class to root element
  useEffect(() => {
    document.documentElement.classList.toggle("dark", darkMode);
  }, [darkMode]);

  const connected = dockerInfo !== null;

  function renderScreen() {
    switch (activeScreen) {
      case "Dashboard":
        return (
          <Dashboard
            connected={connected}
            onConnectionChange={(info) => setDockerInfo(info)}
          />
        );
      case "Containers":
        return <ContainersScreen />;
      case "Images":
        return <ImagesScreen />;
      case "Volumes":
        return <VolumesScreen />;
      case "Networks":
        return <NetworksScreen />;
      case "Compose":
        return <ComposeScreen />;
      case "Settings":
        return <SettingsScreen />;
      default:
        return (
          <Dashboard
            connected={connected}
            onConnectionChange={(info) => setDockerInfo(info)}
          />
        );
    }
  }

  return (
    <div className="flex h-screen overflow-hidden">
      <Sidebar
        active={activeScreen}
        connected={connected}
        onNavigate={setActiveScreen}
      />
      <div className="flex-1 flex flex-col min-w-0">
        <main className="flex-1 overflow-auto">
          {renderScreen()}
        </main>
        <StatusBar
          title={activeScreen}
          dockerVersion={dockerInfo?.server_version}
          endpoint={dockerInfo?.endpoint}
        />
      </div>
    </div>
  );
}
