import { createContext, useContext, useMemo, type ReactNode } from "react";
import type { Language, LanguageSetting } from "../lib/types";
import { messages, type Messages } from "./messages";

interface I18nContextValue {
  language: Language;
  t: Messages;
}

const I18nContext = createContext<I18nContextValue | null>(null);

// Auto language follows the browser/WebView locale list and only falls back to
// English when none of the supported locales match.
export function resolveSystemLanguage(preferredLanguages: readonly string[] = navigator.languages): Language {
  for (const candidate of preferredLanguages) {
    const normalized = candidate.toLowerCase();
    if (normalized.startsWith("es")) return "es";
    if (normalized.startsWith("en")) return "en";
  }

  const fallback = navigator.language.toLowerCase();
  if (fallback.startsWith("es")) return "es";
  return "en";
}

export function resolveLanguage(setting: LanguageSetting, preferredLanguages?: readonly string[]): Language {
  if (typeof setting === "object" && "Manual" in setting) {
    return setting.Manual;
  }

  return resolveSystemLanguage(preferredLanguages);
}

export function I18nProvider({ language, children }: { language: Language; children: ReactNode }) {
  const value = useMemo<I18nContextValue>(() => ({
    language,
    t: messages[language],
  }), [language]);

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n(): I18nContextValue {
  const context = useContext(I18nContext);
  if (!context) {
    throw new Error("useI18n must be used within I18nProvider");
  }

  return context;
}
