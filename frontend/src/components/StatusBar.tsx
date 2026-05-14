import { getVersion } from "@tauri-apps/api/app";
import { useEffect, useState } from "react";
import { useI18n } from "../i18n";

interface StatusBarProps {
  title: string;
  dockerVersion?: string;
  endpoint?: string;
  connectionError?: string | null;
}

export function StatusBar({ title, dockerVersion, endpoint, connectionError }: StatusBarProps) {
  const { t } = useI18n();
  const [appVersion, setAppVersion] = useState(__APP_VERSION__);

  useEffect(() => {
    let cancelled = false;

    async function loadVersion() {
      try {
        const version = await getVersion();
        if (!cancelled) {
          setAppVersion(version);
        }
      } catch {
        if (!cancelled) {
          setAppVersion(__APP_VERSION__);
        }
      }
    }

    void loadVersion();

    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <footer
      className="flex items-center justify-between px-4 py-1 text-xs shrink-0 border-t"
      style={{
        backgroundColor: "var(--color-status-bg)",
        color: "var(--color-text-muted)",
      }}
    >
      <div className="flex items-center gap-4">
        <span className="font-medium" style={{ color: "var(--color-text)" }}>
          {title}
        </span>
        {dockerVersion && <span>Docker {dockerVersion}</span>}
        {!dockerVersion && connectionError && (
          <span style={{ color: "var(--color-danger)" }}>{connectionError}</span>
        )}
      </div>
      <div className="flex items-center gap-4">
        {endpoint && (
          <span className="font-mono text-xs">{endpoint}</span>
        )}
        <span>{t.statusBar.version(appVersion)}</span>
      </div>
    </footer>
  );
}
