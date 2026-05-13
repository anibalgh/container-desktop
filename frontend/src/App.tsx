import { useState, useEffect, useCallback } from "react";
import { Sidebar, type Screen } from "./components/Sidebar";
import { StatusBar } from "./components/StatusBar";
import { Dashboard } from "./screens/Dashboard";
import { ContainersScreen } from "./screens/Containers";
import { ImagesScreen } from "./screens/Images";
import { SecurityScreen } from "./screens/Security";
import { VolumesScreen } from "./screens/Volumes";
import { NetworksScreen } from "./screens/Networks";
import { ComposeScreen } from "./screens/Compose";
import { SettingsScreen } from "./screens/Settings";
import { AboutScreen } from "./screens/About";
import type {
  AppSettings,
  DockerInfo,
  Language,
  LanguageSetting,
  ThemeSetting,
  ThemeVariant,
} from "./lib/types";
import { loadSettings } from "./lib/tauri";
import { I18nProvider, resolveLanguage, useI18n } from "./i18n";

const DARK_VARIANTS: ThemeVariant[] = [
  "Dark", "Dracula", "Nord", "SolarizedDark", "GruvboxDark",
  "CatppuccinFrappe", "CatppuccinMacchiato", "CatppuccinMocha",
  "TokyoNight", "TokyoNightStorm", "KanagawaWave", "KanagawaDragon",
  "Moonfly", "Nightfly", "Oxocarbon", "Ferra",
];

interface BootstrapState {
  settings: AppSettings;
  themeVariant: ThemeVariant;
  language: Language;
}

function isDark(variant: ThemeVariant): boolean {
  return DARK_VARIANTS.includes(variant);
}

function resolveThemeVariant(themeSetting: ThemeSetting): ThemeVariant {
  if (typeof themeSetting === "object" && "Manual" in themeSetting) {
    return themeSetting.Manual;
  }

  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "Dark" : "Light";
}

function applyFont(size: number, family: string | undefined) {
  document.documentElement.style.fontSize = `${size}px`;
  if (family) {
    document.documentElement.style.setProperty("--font-mono", `"${family}", monospace`);
  } else {
    document.documentElement.style.removeProperty("--font-mono");
  }
}

export default function App() {
  const [bootstrap, setBootstrap] = useState<BootstrapState | null>(null);
  const [settingsError, setSettingsError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;

    async function bootstrapApp() {
      try {
        const settings = await loadSettings();
        applyFont(settings.font_size, settings.font_family || undefined);

        if (cancelled) return;

        setBootstrap({
          settings,
          themeVariant: resolveThemeVariant(settings.theme_setting),
          language: resolveLanguage(settings.language_setting, navigator.languages),
        });
        setSettingsError(null);
      } catch (error) {
        if (!cancelled) {
          setSettingsError(String(error));
        }
      }
    }

    void bootstrapApp();

    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (!bootstrap) return;

    const dark = isDark(bootstrap.themeVariant);
    document.documentElement.setAttribute("data-theme", bootstrap.themeVariant);
    document.documentElement.classList.toggle("dark", dark);
    document.documentElement.style.colorScheme = dark ? "dark" : "light";
  }, [bootstrap]);

  useEffect(() => {
    const themeSetting = bootstrap?.settings.theme_setting;
    if (!themeSetting) return;

    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const handler = (event: MediaQueryListEvent) => {
      setBootstrap((current) => {
        if (!current || current.settings.theme_setting !== "Auto") {
          return current;
        }

        return {
          ...current,
          themeVariant: event.matches ? "Dark" : "Light",
        };
      });
    };

    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, [bootstrap?.settings.theme_setting]);

  const handleThemeSettingChange = useCallback((themeSetting: ThemeSetting) => {
    setBootstrap((current) => {
      if (!current) return current;

      return {
        ...current,
        settings: {
          ...current.settings,
          theme_setting: themeSetting,
        },
        themeVariant: resolveThemeVariant(themeSetting),
      };
    });
  }, []);

  const handleLanguageSettingChange = useCallback((languageSetting: LanguageSetting) => {
    setBootstrap((current) => {
      if (!current) return current;

      return {
        ...current,
        settings: {
          ...current.settings,
          language_setting: languageSetting,
        },
        language: resolveLanguage(languageSetting, navigator.languages),
      };
    });
  }, []);

  if (settingsError && !bootstrap) {
    return (
      <div className="flex items-center justify-center h-screen p-6">
        <div className="max-w-md text-center text-sm" style={{ color: "var(--color-danger)" }}>
          {settingsError}
        </div>
      </div>
    );
  }

  if (!bootstrap) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div
          className="w-8 h-8 border-2 border-t-transparent rounded-full animate-spin"
          style={{ borderColor: "var(--color-accent)", borderTopColor: "transparent" }}
        />
      </div>
    );
  }

  return (
    <I18nProvider language={bootstrap.language}>
      <AppShell
        themeVariant={bootstrap.themeVariant}
        onThemeSettingChange={handleThemeSettingChange}
        onLanguageSettingChange={handleLanguageSettingChange}
      />
    </I18nProvider>
  );
}

function AppShell({
  themeVariant,
  onThemeSettingChange,
  onLanguageSettingChange,
}: {
  themeVariant: ThemeVariant;
  onThemeSettingChange: (themeSetting: ThemeSetting) => void;
  onLanguageSettingChange: (languageSetting: LanguageSetting) => void;
}) {
  const { t } = useI18n();
  const [activeScreen, setActiveScreen] = useState<Screen>("dashboard");
  const [dockerInfo, setDockerInfo] = useState<DockerInfo | null>(null);
  const [dockerError, setDockerError] = useState<string | null>(null);

  const connected = dockerInfo !== null;

  const handleConnectionChange = useCallback((info: DockerInfo | null, error?: string) => {
    setDockerInfo(info);
    setDockerError(info ? null : (error ?? null));
  }, []);

  function renderScreen() {
    switch (activeScreen) {
      case "dashboard":
        return <Dashboard connected={connected} onConnectionChange={handleConnectionChange} />;
      case "containers":
        return <ContainersScreen />;
      case "images":
        return <ImagesScreen />;
      case "security":
        return <SecurityScreen />;
      case "volumes":
        return <VolumesScreen />;
      case "networks":
        return <NetworksScreen />;
      case "compose":
        return <ComposeScreen />;
      case "settings":
        return (
          <SettingsScreen
            onThemeSettingChange={onThemeSettingChange}
            onLanguageSettingChange={onLanguageSettingChange}
          />
        );
      case "about":
        return <AboutScreen />;
      default:
        return <Dashboard connected={connected} onConnectionChange={handleConnectionChange} />;
    }
  }

  return (
    <div className="flex h-screen overflow-hidden">
      <Sidebar
        active={activeScreen}
        connected={connected}
        darkMode={isDark(themeVariant)}
        onNavigate={setActiveScreen}
      />
      <div className="flex-1 flex flex-col min-w-0">
        <main className="flex-1 overflow-auto">{renderScreen()}</main>
        <StatusBar
          title={t.sidebar.screens[activeScreen]}
          dockerVersion={dockerInfo?.server_version}
          endpoint={dockerInfo?.endpoint}
          connectionError={dockerError}
        />
      </div>
    </div>
  );
}
