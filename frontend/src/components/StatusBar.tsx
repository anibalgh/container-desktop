import { useI18n } from "../i18n";

interface StatusBarProps {
  title: string;
  dockerVersion?: string;
  endpoint?: string;
  connectionError?: string | null;
}

export function StatusBar({ title, dockerVersion, endpoint, connectionError }: StatusBarProps) {
  const { t } = useI18n();

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
        <span>{t.statusBar.version}</span>
      </div>
    </footer>
  );
}
