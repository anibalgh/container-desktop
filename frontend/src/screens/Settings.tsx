import { useEffect, useState, type ReactNode } from "react";
import type {
  AppSettings,
  LanguageSetting,
  ThemeSetting,
  ThemeVariant,
} from "../lib/types";
import { loadSettings, saveSettings, listFonts } from "../lib/tauri";
import { useI18n } from "../i18n";

const THEME_VARIANTS: ThemeVariant[] = [
  "Light", "Dark", "Dracula", "Nord",
  "SolarizedLight", "SolarizedDark", "GruvboxLight", "GruvboxDark",
  "CatppuccinLatte", "CatppuccinFrappe", "CatppuccinMacchiato", "CatppuccinMocha",
  "TokyoNight", "TokyoNightStorm", "TokyoNightLight",
  "KanagawaWave", "KanagawaDragon", "KanagawaLotus",
  "Moonfly", "Nightfly", "Oxocarbon", "Ferra",
];

function applyFont(size: number, family: string | undefined) {
  document.documentElement.style.fontSize = `${size}px`;
  if (family) {
    document.documentElement.style.setProperty("--font-mono", `"${family}", monospace`);
  } else {
    document.documentElement.style.removeProperty("--font-mono");
  }
}

interface SettingsProps {
  onThemeSettingChange: (themeSetting: ThemeSetting) => void;
  onLanguageSettingChange: (languageSetting: LanguageSetting) => void;
}

export function SettingsScreen({
  onThemeSettingChange,
  onLanguageSettingChange,
}: SettingsProps) {
  const { language, t } = useI18n();
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [saved, setSaved] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [fonts, setFonts] = useState<string[]>(["Monospace"]);
  const [showRemoteHelp, setShowRemoteHelp] = useState(false);

  useEffect(() => {
    loadSettings()
      .then(setSettings)
      .catch((loadError) => setError(String(loadError)))
      .finally(() => setLoading(false));

    listFonts()
      .then(setFonts)
      .catch(() => setFonts(["Monospace", "Fira Code", "JetBrains Mono", "Cascadia Code", "Consolas"]));
  }, []);

  async function doSave() {
    if (!settings) return;

    setError(null);

    try {
      await saveSettings(settings);
      applyFont(settings.font_size, settings.font_family || undefined);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (saveError) {
      setError(String(saveError));
    }
  }

  function updateThemeSetting(themeSetting: ThemeSetting) {
    if (!settings) return;

    setSettings({
      ...settings,
      theme_setting: themeSetting,
    });
    onThemeSettingChange(themeSetting);
  }

  function updateLanguageSetting(languageSetting: LanguageSetting) {
    if (!settings) return;

    setSettings({
      ...settings,
      language_setting: languageSetting,
    });
    onLanguageSettingChange(languageSetting);
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div
          className="w-6 h-6 border-2 border-t-transparent rounded-full animate-spin"
          style={{ borderColor: "var(--color-accent)", borderTopColor: "transparent" }}
        />
      </div>
    );
  }

  if (!settings) {
    return (
      <div className="p-8">
        <p style={{ color: "var(--color-text-muted)" }}>{t.settings.failedToLoad}</p>
        {error && (
          <p className="mt-3 font-mono text-sm break-all" style={{ color: "var(--color-danger)" }}>
            {error}
          </p>
        )}
      </div>
    );
  }

  const themeIsAuto = settings.theme_setting === "Auto";
  const themeManual = typeof settings.theme_setting === "object"
    ? settings.theme_setting.Manual
    : "Dark";
  const languageIsAuto = settings.language_setting === "Auto";
  const manualLanguage = typeof settings.language_setting === "object"
    ? settings.language_setting.Manual
    : "en";
  const currentLanguageLabel = language === "es"
    ? t.settings.language.spanish
    : t.settings.language.english;

  return (
    <div className="p-6 max-w-2xl">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-xl font-semibold" style={{ color: "var(--color-text)" }}>
          {t.settings.title}
        </h1>
        <button
          onClick={doSave}
          className="px-4 py-2 text-sm rounded-md text-white"
          style={{ backgroundColor: saved ? "var(--color-success)" : "var(--color-accent)" }}
        >
          {saved ? t.common.saved : t.common.save}
        </button>
      </div>
      {error && (
        <div
          className="mb-4 px-3 py-2 text-sm rounded-md"
          style={{ backgroundColor: "rgba(239,68,68,0.1)", color: "var(--color-danger)" }}
        >
          {error}
          <button onClick={() => setError(null)} className="ml-2 underline">
            {t.common.dismiss}
          </button>
        </div>
      )}

      <Section title={t.settings.sections.language}>
        <div className="flex items-center gap-3 mb-3">
          <OptionButton
            active={languageIsAuto}
            label={t.settings.language.auto}
            onClick={() => updateLanguageSetting("Auto")}
          />
          <OptionButton
            active={!languageIsAuto && manualLanguage === "es"}
            label={t.settings.language.spanish}
            onClick={() => updateLanguageSetting({ Manual: "es" })}
          />
          <OptionButton
            active={!languageIsAuto && manualLanguage === "en"}
            label={t.settings.language.english}
            onClick={() => updateLanguageSetting({ Manual: "en" })}
          />
        </div>
        {languageIsAuto && (
          <p className="text-xs" style={{ color: "var(--color-text-muted)" }}>
            {t.settings.language.currentAuto(currentLanguageLabel)}
          </p>
        )}
      </Section>

      <Section title={t.settings.sections.theme}>
        <div className="flex items-center gap-3 mb-3">
          <OptionButton
            active={themeIsAuto}
            label={t.settings.theme.auto}
            onClick={() => updateThemeSetting("Auto")}
          />
          <OptionButton
            active={!themeIsAuto}
            label={t.settings.theme.manual}
            onClick={() => updateThemeSetting({ Manual: themeManual })}
          />
        </div>
        {!themeIsAuto && (
          <select
            value={themeManual}
            onChange={(event) => updateThemeSetting({ Manual: event.target.value as ThemeVariant })}
            className="px-3 py-2 text-sm rounded-md border"
            style={{
              borderColor: "var(--color-border)",
              backgroundColor: "var(--color-surface-secondary)",
              color: "var(--color-text)",
            }}
          >
            {THEME_VARIANTS.map((variant) => (
              <option key={variant} value={variant}>
                {variant}
              </option>
            ))}
          </select>
        )}
      </Section>

      <Section title={t.settings.sections.dockerEndpoint}>
        <input
          value={settings.endpoint.host_url}
          onChange={(event) => setSettings({
            ...settings,
            endpoint: { ...settings.endpoint, host_url: event.target.value },
          })}
          className="w-full px-3 py-2 text-sm rounded-md border font-mono"
          style={{
            borderColor: "var(--color-border)",
            backgroundColor: "var(--color-surface-secondary)",
            color: "var(--color-text)",
          }}
        />
        <p className="text-xs mt-1" style={{ color: "var(--color-text-muted)" }}>
          {t.settings.dockerEndpointHint}
        </p>
        <button
          onClick={() => setShowRemoteHelp(true)}
          className="mt-2 text-xs underline"
          style={{ color: "var(--color-accent)" }}
        >
          {t.settings.dockerEndpointRemoteHelpLink}
        </button>
      </Section>

      <Section title={t.settings.sections.fontSize}>
        <div className="flex items-center gap-2">
          {[
            { label: t.settings.fontSizes.normal, size: 14 },
            { label: t.settings.fontSizes.large, size: 18 },
            { label: t.settings.fontSizes.larger, size: 22 },
          ].map((fontSize) => (
            <button
              key={fontSize.label}
              onClick={() => setSettings({ ...settings, font_size: fontSize.size })}
              className="px-4 py-2 text-sm rounded-md border"
              style={{
                borderColor: "var(--color-border)",
                backgroundColor: settings.font_size === fontSize.size ? "var(--color-accent)" : "transparent",
                color: settings.font_size === fontSize.size ? "white" : "var(--color-text)",
              }}
            >
              {fontSize.label}
            </button>
          ))}
        </div>
      </Section>

      <Section title={t.settings.sections.monospaceFont}>
        <select
          value={settings.font_family}
          onChange={(event) => setSettings({ ...settings, font_family: event.target.value })}
          className="w-full px-3 py-2 text-sm rounded-md border"
          style={{
            borderColor: "var(--color-border)",
            backgroundColor: "var(--color-surface-secondary)",
            color: "var(--color-text)",
          }}
        >
          <option value="">{t.settings.systemDefault}</option>
          {fonts.map((font) => (
            <option key={font} value={font}>
              {font}
            </option>
          ))}
        </select>
      </Section>

      {showRemoteHelp && (
        <div
          className="fixed inset-0 flex items-center justify-center z-50"
          style={{ backgroundColor: "rgba(0,0,0,0.5)" }}
        >
          <div
            className="rounded-lg p-6 max-w-2xl w-full mx-4 shadow-xl"
            style={{ backgroundColor: "var(--color-surface)" }}
          >
            <h3 className="text-lg font-semibold mb-2" style={{ color: "var(--color-text)" }}>
              {t.settings.dockerEndpointRemoteHelp.title}
            </h3>
            <p className="text-sm mb-4" style={{ color: "var(--color-text-muted)" }}>
              {t.settings.dockerEndpointRemoteHelp.intro}
            </p>

            <HelpBlock
              title={t.settings.dockerEndpointRemoteHelp.installLabel}
              lines={t.settings.dockerEndpointRemoteHelp.installCommands}
            />
            <HelpBlock
              title={t.settings.dockerEndpointRemoteHelp.remoteCommandLabel}
              lines={[t.settings.dockerEndpointRemoteHelp.remoteCommand]}
            />
            <HelpBlock
              title={t.settings.dockerEndpointRemoteHelp.localCommandLabel}
              lines={[t.settings.dockerEndpointRemoteHelp.localCommand]}
            />
            <HelpBlock
              title={t.settings.dockerEndpointRemoteHelp.configStringLabel}
              lines={[t.settings.dockerEndpointRemoteHelp.configString]}
            />

            <div className="flex justify-end gap-2 mt-6">
              <button
                onClick={() => setShowRemoteHelp(false)}
                className="px-4 py-2 text-sm rounded-md border"
                style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}
              >
                {t.common.dismiss}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <div className="mb-6">
      <h2 className="text-sm font-semibold mb-3" style={{ color: "var(--color-text)" }}>
        {title}
      </h2>
      {children}
    </div>
  );
}

function OptionButton({
  active,
  label,
  onClick,
}: {
  active: boolean;
  label: string;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className="px-3 py-1.5 text-sm rounded-md border"
      style={{
        borderColor: "var(--color-border)",
        backgroundColor: active ? "var(--color-accent)" : "transparent",
        color: active ? "white" : "var(--color-text)",
      }}
    >
      {label}
    </button>
  );
}

function HelpBlock({ title, lines }: { title: string; lines: string[] }) {
  return (
    <div className="mb-4">
      <p className="text-sm font-medium mb-2" style={{ color: "var(--color-text)" }}>
        {title}
      </p>
      <div
        className="rounded-md px-3 py-2 text-xs font-mono whitespace-pre-wrap break-all"
        style={{
          backgroundColor: "var(--color-surface-secondary)",
          color: "var(--color-text)",
        }}
      >
        {lines.join("\n")}
      </div>
    </div>
  );
}
