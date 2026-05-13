import { useEffect, useState } from "react";
import type { DockerInfo, SecurityOverview, SeverityCount } from "../lib/types";
import {
  testConnection,
  onDockerConnected,
  onDockerError,
  onSecurityScanProgress,
  securityOverview,
} from "../lib/tauri";
import { useI18n } from "../i18n";

interface DashboardProps {
  connected: boolean;
  onConnectionChange: (info: DockerInfo | null, error?: string) => void;
}

export function Dashboard({ connected, onConnectionChange }: DashboardProps) {
  const { t } = useI18n();
  const [info, setInfo] = useState<DockerInfo | null>(null);
  const [securitySummary, setSecuritySummary] = useState<SecurityOverview | null>(null);
  const [loading, setLoading] = useState(true);
  const [connectionError, setConnectionError] = useState<string | null>(null);
  const [securityError, setSecurityError] = useState<string | null>(null);

  useEffect(() => {
    // Listen for initial connection event
    const unlisten1 = onDockerConnected((data) => {
      setInfo(data);
      setConnectionError(null);
      onConnectionChange(data);
      setLoading(false);
    });

    const unlisten2 = onDockerError((err) => {
      setInfo(null);
      setConnectionError(err);
      onConnectionChange(null, err);
      setLoading(false);
    });

    // Also try explicit connection test
    testConnection()
      .then((data) => {
        setInfo(data);
        setConnectionError(null);
        onConnectionChange(data);
        setLoading(false);
      })
      .catch((err) => {
        const message = String(err);
        setInfo(null);
        setConnectionError(message);
        onConnectionChange(null, message);
        setLoading(false);
      });

    return () => {
      unlisten1.then((f) => f());
      unlisten2.then((f) => f());
    };
  }, [onConnectionChange]);

  useEffect(() => {
    let cancelled = false;
    let unlisten: (() => void) | null = null;

    async function loadSecuritySummary() {
      if (!connected) {
        if (!cancelled) {
          setSecuritySummary(null);
          setSecurityError(null);
        }
        return;
      }

      try {
        const summary = await securityOverview();
        if (!cancelled) {
          setSecuritySummary(summary);
          setSecurityError(null);
        }
      } catch (error) {
        if (!cancelled) {
          setSecurityError(String(error));
        }
      }
    }

    void loadSecuritySummary();

    onSecurityScanProgress(() => {
      void loadSecuritySummary();
    }).then((listener) => {
      if (cancelled) {
        listener();
      } else {
        unlisten = listener;
      }
    });

    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, [connected]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="flex flex-col items-center gap-4">
          <div className="w-8 h-8 border-2 border-t-transparent rounded-full animate-spin"
            style={{ borderColor: "var(--color-accent)", borderTopColor: "transparent" }} />
          <span style={{ color: "var(--color-text-muted)" }}>
            {t.dashboard.connecting}
          </span>
        </div>
      </div>
    );
  }

  if (!info && !connected) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-center">
          <div className="text-6xl mb-4">🐳</div>
          <h1 className="text-2xl font-semibold mb-2"
            style={{ color: "var(--color-text)" }}>
            {t.dashboard.productName}
          </h1>
          <p style={{ color: "var(--color-text-muted)" }}>
            {t.dashboard.daemonUnreachable}
          </p>
          <p className="text-sm mt-1" style={{ color: "var(--color-text-muted)" }}>
            {t.dashboard.daemonHelp}
          </p>
          {connectionError && (
            <p className="text-sm mt-3 font-mono break-all" style={{ color: "var(--color-danger)" }}>
              {connectionError}
            </p>
          )}
        </div>
      </div>
    );
  }

  return (
    <div className="p-8">
      <h1 className="text-2xl font-semibold mb-6"
        style={{ color: "var(--color-text)" }}>
        {t.dashboard.title}
      </h1>

      {/* Stats grid */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <StatCard
          label={t.dashboard.stats.containersRunning}
          value={info?.containers_running ?? 0}
          color="var(--color-success)"
        />
        <StatCard
          label={t.dashboard.stats.containersStopped}
          value={(info?.containers_stopped ?? 0) + (info?.containers_paused ?? 0)}
          color="var(--color-warning)"
        />
        <StatCard
          label={t.dashboard.stats.images}
          value={info?.images ?? 0}
          color="var(--color-accent)"
        />
        <StatCard
          label={t.dashboard.stats.architecture}
          value={info?.architecture ?? t.common.notAvailable}
          color="var(--color-text-muted)"
          isString
        />
      </div>

      <div className="rounded-lg border p-4 mb-8" style={{ backgroundColor: "var(--color-surface-secondary)" }}>
        <div
          className="flex items-center justify-between gap-4 mb-4"
        >
          <h2 className="text-lg font-medium" style={{ color: "var(--color-text)" }}>
            {t.dashboard.security.title}
          </h2>
          <span className="text-xs" style={{ color: "var(--color-text-muted)" }}>
            {t.dashboard.security.scannedImages(securitySummary?.scanned_images ?? 0)}
          </span>
        </div>
        {securityError ? (
          <p className="text-sm" style={{ color: "var(--color-danger)" }}>
            {securityError}
          </p>
        ) : !securitySummary ? (
          <p className="text-sm" style={{ color: "var(--color-text-muted)" }}>
            {t.dashboard.security.noResults}
          </p>
        ) : (
          <div className="space-y-4">
            <div className="rounded-lg border overflow-hidden" style={{ borderColor: "var(--color-border)" }}>
              <table className="w-full text-sm">
                <tbody>
                  <SummaryRow label={t.security.summary.totalImages} value={securitySummary.total_images} />
                  <SummaryRow label={t.dashboard.security.scannedImagesLabel} value={securitySummary.scanned_images} />
                  <SummaryRow label={t.security.summary.imagesWithFindings} value={securitySummary.images_with_findings} />
                </tbody>
              </table>
            </div>
            <div className="rounded-lg border overflow-hidden" style={{ borderColor: "var(--color-border)" }}>
              <table className="w-full text-sm">
                <thead>
                  <tr style={{ backgroundColor: "var(--color-surface)" }}>
                    <th className="px-4 py-2 text-left text-xs uppercase tracking-wide" style={{ color: "var(--color-text-muted)" }}>
                      {t.dashboard.security.vulnerabilitiesBySeverity}
                    </th>
                    <th className="px-4 py-2 text-right text-xs uppercase tracking-wide" style={{ color: "var(--color-text-muted)" }}>
                      {t.dashboard.security.countLabel}
                    </th>
                  </tr>
                </thead>
                <tbody>
                  {securitySummary.findings_by_severity.map((bucket) => (
                    <SeverityTableRow key={bucket.severity} bucket={bucket} />
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}
      </div>

      {/* System info */}
      <div className="rounded-lg border p-6"
        style={{ backgroundColor: "var(--color-surface-secondary)" }}>
        <h2 className="text-lg font-medium mb-4" style={{ color: "var(--color-text)" }}>
          {t.dashboard.systemInformation}
        </h2>
        <div className="grid grid-cols-2 gap-3 text-sm">
          <InfoRow label={t.dashboard.dockerVersion} value={info?.server_version} />
          <InfoRow label={t.dashboard.os} value={info?.os_type} />
          <InfoRow label={t.dashboard.architecture} value={info?.architecture} />
          <InfoRow label={t.dashboard.endpoint} value={info?.endpoint} mono />
        </div>
      </div>

    </div>
  );
}

function SummaryRow({
  label,
  value,
}: {
  label: string;
  value: number;
}) {
  return (
    <tr className="border-t first:border-t-0" style={{ borderColor: "var(--color-border)" }}>
      <td className="px-4 py-3" style={{ color: "var(--color-text)" }}>{label}</td>
      <td className="px-4 py-3 text-right font-semibold" style={{ color: "var(--color-text)" }}>{value}</td>
    </tr>
  );
}

function SeverityTableRow({ bucket }: { bucket: SeverityCount }) {
  return (
    <tr className="border-t" style={{ borderColor: "var(--color-border)" }}>
      <td className="px-4 py-3">
        <span
          className="inline-flex items-center rounded-full px-2.5 py-1 text-xs font-semibold"
          style={{
            color: severityColor(bucket.severity),
            backgroundColor: `${severityColor(bucket.severity)}1A`,
          }}
        >
          {bucket.severity}
        </span>
      </td>
      <td className="px-4 py-3 text-right font-semibold" style={{ color: severityColor(bucket.severity) }}>
        {bucket.count}
      </td>
    </tr>
  );
}

function severityColor(severity: SeverityCount["severity"]): string {
  switch (severity) {
    case "Critical":
      return "var(--color-danger)";
    case "High":
      return "#f97316";
    case "Medium":
      return "#eab308";
    case "Low":
      return "var(--color-success)";
    case "Negligible":
      return "#14b8a6";
    default:
      return "var(--color-text-muted)";
  }
}

function StatCard({
  label,
  value,
  color,
  isString,
}: {
  label: string;
  value: string | number;
  color: string;
  isString?: boolean;
}) {
  return (
    <div
      className="rounded-lg border p-4"
      style={{ backgroundColor: "var(--color-surface-secondary)" }}
    >
      <div className="text-xs uppercase tracking-wide mb-1"
        style={{ color: "var(--color-text-muted)" }}>
        {label}
      </div>
      <div
        className={`font-semibold ${isString ? "text-base" : "text-3xl"}`}
        style={{ color }}
      >
        {value}
      </div>
    </div>
  );
}

function InfoRow({
  label,
  value,
  mono,
}: {
  label: string;
  value?: string;
  mono?: boolean;
}) {
  return (
    <div>
      <span style={{ color: "var(--color-text-muted)" }}>{label}</span>
      <span
        className={`ml-2 ${mono ? "font-mono text-xs" : ""}`}
        style={{ color: "var(--color-text)" }}
      >
        {value ?? <DashboardFallback />}
      </span>
    </div>
  );
}

function DashboardFallback() {
  const { t } = useI18n();
  return <>{t.common.notAvailable}</>;
}
