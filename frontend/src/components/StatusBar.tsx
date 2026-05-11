interface StatusBarProps {
  title: string;
  dockerVersion?: string;
  endpoint?: string;
}

export function StatusBar({ title, dockerVersion, endpoint }: StatusBarProps) {
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
      </div>
      <div className="flex items-center gap-4">
        {endpoint && (
          <span className="font-mono text-[11px]">{endpoint}</span>
        )}
        <span>Container Desktop v0.1.0</span>
      </div>
    </footer>
  );
}
