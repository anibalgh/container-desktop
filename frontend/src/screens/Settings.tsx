import { useEffect, useState } from "react";
import type { AppSettings, ThemeVariant } from "../lib/types";
import { loadSettings, saveSettings } from "../lib/tauri";

const THEME_VARIANTS: ThemeVariant[] = [
  "Light", "Dark", "Dracula", "Nord",
  "SolarizedLight", "SolarizedDark", "GruvboxLight", "GruvboxDark",
  "CatppuccinLatte", "CatppuccinFrappe", "CatppuccinMacchiato", "CatppuccinMocha",
  "TokyoNight", "TokyoNightStorm", "TokyoNightLight",
  "KanagawaWave", "KanagawaDragon", "KanagawaLotus",
  "Moonfly", "Nightfly", "Oxocarbon", "Ferra",
];

export function SettingsScreen() {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [saved, setSaved] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadSettings()
      .then(setSettings)
      .catch((e) => setError(String(e)))
      .finally(() => setLoading(false));
  }, []);

  async function doSave() {
    if (!settings) return;
    setError(null);
    try {
      await saveSettings(settings);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (e) {
      setError(String(e));
    }
  }

  function updateThemeMode(mode: "Auto" | "Manual") {
    if (!settings) return;
    setSettings({
      ...settings,
      theme_setting: mode === "Auto" ? "Auto" : { Manual: "Dark" },
    });
  }

  function updateThemeVariant(variant: ThemeVariant) {
    if (!settings) return;
    setSettings({
      ...settings,
      theme_setting: { Manual: variant },
    });
  }

  if (loading) {
    return <div className="flex items-center justify-center h-full">
      <div className="w-6 h-6 border-2 border-t-transparent rounded-full animate-spin"
        style={{ borderColor: "var(--color-accent)", borderTopColor: "transparent" }} />
    </div>;
  }

  if (!settings) {
    return <div className="p-8"><p style={{ color: "var(--color-text-muted)" }}>Failed to load settings.</p></div>;
  }

  const themeIsAuto = settings.theme_setting === "Auto";
  const themeManual = typeof settings.theme_setting === "object" ? settings.theme_setting.Manual : "Dark";

  return (
    <div className="p-6 max-w-2xl">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-xl font-semibold" style={{ color: "var(--color-text)" }}>Settings</h1>
        <button
          onClick={doSave}
          className="px-4 py-2 text-sm rounded-md text-white"
          style={{ backgroundColor: saved ? "var(--color-success)" : "var(--color-accent)" }}>
          {saved ? "Saved ✓" : "Save"}
        </button>
      </div>

      {error && <div className="mb-4 px-3 py-2 text-sm rounded-md"
        style={{ backgroundColor: "rgba(239,68,68,0.1)", color: "var(--color-danger)" }}>
        {error} <button onClick={() => setError(null)} className="ml-2 underline">Dismiss</button>
      </div>}

      {/* Theme */}
      <Section title="Theme">
        <div className="flex items-center gap-3 mb-3">
          <button
            onClick={() => updateThemeMode("Auto")}
            className={`px-3 py-1.5 text-sm rounded-md border ${themeIsAuto ? "text-white" : ""}`}
            style={{
              borderColor: "var(--color-border)",
              backgroundColor: themeIsAuto ? "var(--color-accent)" : "transparent",
              color: themeIsAuto ? "white" : "var(--color-text)",
            }}>
            Auto (OS)
          </button>
          <button
            onClick={() => updateThemeMode("Manual")}
            className={`px-3 py-1.5 text-sm rounded-md border ${!themeIsAuto ? "text-white" : ""}`}
            style={{
              borderColor: "var(--color-border)",
              backgroundColor: !themeIsAuto ? "var(--color-accent)" : "transparent",
              color: !themeIsAuto ? "white" : "var(--color-text)",
            }}>
            Manual
          </button>
        </div>
        {!themeIsAuto && (
          <select
            value={themeManual}
            onChange={(e) => updateThemeVariant(e.target.value as ThemeVariant)}
            className="px-3 py-2 text-sm rounded-md border"
            style={{
              borderColor: "var(--color-border)",
              backgroundColor: "var(--color-surface-secondary)",
              color: "var(--color-text)",
            }}>
            {THEME_VARIANTS.map((v) => (
              <option key={v} value={v}>{v}</option>
            ))}
          </select>
        )}
      </Section>

      {/* Docker Endpoint */}
      <Section title="Docker Endpoint">
        <input
          value={settings.endpoint.host_url}
          onChange={(e) => setSettings({
            ...settings,
            endpoint: { ...settings.endpoint, host_url: e.target.value },
          })}
          className="w-full px-3 py-2 text-sm rounded-md border font-mono"
          style={{
            borderColor: "var(--color-border)",
            backgroundColor: "var(--color-surface-secondary)",
            color: "var(--color-text)",
          }}
        />
        <p className="text-xs mt-1" style={{ color: "var(--color-text-muted)" }}>
          Examples: unix:///var/run/docker.sock, tcp://192.168.1.10:2375
        </p>
      </Section>

      {/* Font */}
      <Section title="Font">
        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="block text-xs font-medium mb-1" style={{ color: "var(--color-text-muted)" }}>
              Font Family
            </label>
            <input
              value={settings.font_family}
              onChange={(e) => setSettings({ ...settings, font_family: e.target.value })}
              className="w-full px-3 py-2 text-sm rounded-md border"
              style={{
                borderColor: "var(--color-border)",
                backgroundColor: "var(--color-surface-secondary)",
                color: "var(--color-text)",
              }}
            />
          </div>
          <div>
            <label className="block text-xs font-medium mb-1" style={{ color: "var(--color-text-muted)" }}>
              Font Size
            </label>
            <input
              type="number"
              value={settings.font_size}
              onChange={(e) => setSettings({ ...settings, font_size: parseInt(e.target.value) || 14 })}
              min={10} max={24}
              className="w-full px-3 py-2 text-sm rounded-md border"
              style={{
                borderColor: "var(--color-border)",
                backgroundColor: "var(--color-surface-secondary)",
                color: "var(--color-text)",
              }}
            />
          </div>
        </div>
      </Section>
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="mb-6">
      <h2 className="text-sm font-semibold mb-3" style={{ color: "var(--color-text)" }}>{title}</h2>
      {children}
    </div>
  );
}
