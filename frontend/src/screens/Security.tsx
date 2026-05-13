import { Fragment, useCallback, useEffect, useMemo, useState } from "react";
import type {
  ImageSecurityReport,
  SecurityFinding,
  SecurityInstallHint,
  SecurityOverview,
  SecurityTool,
  SecurityToolStatus,
  SeverityCount,
} from "../lib/types";
import {
  configureSecurityTools,
  imageSecurityReport,
  onSecurityScanProgress,
  securityOverview,
} from "../lib/tauri";
import { useI18n } from "../i18n";

const SEVERITY_COLORS: Record<string, string> = {
  Critical: "var(--color-danger)",
  High: "#f97316",
  Medium: "#eab308",
  Low: "var(--color-success)",
  Negligible: "#14b8a6",
  Unknown: "var(--color-text-muted)",
};

export function SecurityScreen() {
  const { t } = useI18n();
  const [overview, setOverview] = useState<SecurityOverview | null>(null);
  const [openImageId, setOpenImageId] = useState<string | null>(null);
  const [report, setReport] = useState<ImageSecurityReport | null>(null);
  const [loading, setLoading] = useState(true);
  const [loadingReport, setLoadingReport] = useState(false);
  const [savingSelection, setSavingSelection] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [installHint, setInstallHint] = useState<SecurityInstallHint | null>(null);

  const loadOverview = useCallback(async (showSpinner = false) => {
    if (showSpinner) setLoading(true);
    try {
      const data = await securityOverview();
      setOverview(data);
      setError(null);
      setOpenImageId((current) => (
        current && !data.images.some((image) => image.image_id === current) ? null : current
      ));
    } catch (loadError) {
      setError(String(loadError));
    } finally {
      setLoading(false);
    }
  }, []);

  const loadReport = useCallback(async (imageId: string) => {
    setLoadingReport(true);
    try {
      const data = await imageSecurityReport(imageId);
      setReport(data);
      setError(null);
    } catch (loadError) {
      setError(String(loadError));
      setReport(null);
    } finally {
      setLoadingReport(false);
    }
  }, []);

  useEffect(() => {
    queueMicrotask(() => {
      void loadOverview();
    });
  }, [loadOverview]);

  useEffect(() => {
    if (!openImageId) {
      return;
    }
    queueMicrotask(() => {
      void loadReport(openImageId);
    });
  }, [loadReport, openImageId]);

  useEffect(() => {
    let disposed = false;
    let unlisten: (() => void) | null = null;

    onSecurityScanProgress(() => {
      if (disposed) return;
      void loadOverview();
      if (openImageId) {
        void loadReport(openImageId);
      }
    }).then((listener) => {
      if (disposed) {
        listener();
      } else {
        unlisten = listener;
      }
    });

    return () => {
      disposed = true;
      unlisten?.();
    };
  }, [loadOverview, loadReport, openImageId]);

  const selectedTools = useMemo(
    () => overview?.tools.filter((tool) => tool.selected).map((tool) => tool.tool) ?? [],
    [overview],
  );

  async function persistSelection(nextTools: SecurityTool[]) {
    setSavingSelection(true);
    try {
      const data = await configureSecurityTools(nextTools);
      setOverview(data);
      setError(null);
      setOpenImageId((current) => (
        current && !data.images.some((image) => image.image_id === current) ? null : current
      ));
    } catch (saveError) {
      setError(String(saveError));
    } finally {
      setSavingSelection(false);
    }
  }

  async function toggleTool(toolStatus: SecurityToolStatus) {
    if (!overview) return;
    if (!toolStatus.available) {
      setInstallHint(toolStatus.install_hint);
      return;
    }

    const nextSelected = toolStatus.selected
      ? selectedTools.filter((tool) => tool !== toolStatus.tool)
      : [...selectedTools, toolStatus.tool];
    await persistSelection(nextSelected);
  }

  async function rescanSelected() {
    await persistSelection(selectedTools);
  }

  if (loading && !overview) {
    return (
      <div className="flex items-center justify-center h-full">
        <div
          className="w-6 h-6 border-2 border-t-transparent rounded-full animate-spin"
          style={{ borderColor: "var(--color-accent)", borderTopColor: "transparent" }}
        />
      </div>
    );
  }

  const openImage = overview?.images.find((image) => image.image_id === openImageId) ?? null;

  return (
    <div className="p-6 h-full flex flex-col gap-6">
      <div className="flex items-center justify-between gap-4">
        <div>
          <h1 className="text-xl font-semibold" style={{ color: "var(--color-text)" }}>
            {t.security.title}
          </h1>
          <p className="text-sm mt-1" style={{ color: "var(--color-text-muted)" }}>
            {t.security.subtitle}
          </p>
        </div>
        <div className="flex items-center gap-3">
          <button
            onClick={() => void loadOverview(true)}
            className="px-3 py-1.5 text-xs rounded-md border"
            style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}
          >
            {t.common.refresh}
          </button>
          <button
            onClick={() => void rescanSelected()}
            disabled={savingSelection || selectedTools.length === 0}
            className="px-3 py-1.5 text-xs rounded-md text-white disabled:opacity-50"
            style={{ backgroundColor: "var(--color-accent)" }}
          >
            {savingSelection ? t.security.rescanning : t.security.rescanSelected}
          </button>
        </div>
      </div>

      {error && (
        <div
          className="px-3 py-2 text-sm rounded-md"
          style={{ backgroundColor: "rgba(239,68,68,0.1)", color: "var(--color-danger)" }}
        >
          {error}
          <button onClick={() => setError(null)} className="ml-2 underline">
            {t.common.dismiss}
          </button>
        </div>
      )}

      {overview && (
        <>
          <section>
            <div className="flex items-center justify-between mb-3">
              <h2 className="text-sm font-semibold" style={{ color: "var(--color-text)" }}>
                {t.security.toolsTitle}
              </h2>
              <span className="text-xs" style={{ color: "var(--color-text-muted)" }}>
                {t.security.selectedCount(selectedTools.length)}
              </span>
            </div>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
              {overview.tools.map((tool) => (
                <button
                  key={tool.tool}
                  onClick={() => void toggleTool(tool)}
                  disabled={savingSelection}
                  className="rounded-lg border p-4 text-left transition-opacity disabled:opacity-60"
                  style={{
                    borderColor: tool.selected ? "var(--color-accent)" : "var(--color-border)",
                    backgroundColor: tool.selected
                      ? "color-mix(in srgb, var(--color-accent) 12%, transparent)"
                      : "var(--color-surface)",
                  }}
                >
                  <div className="flex items-start justify-between gap-3">
                    <div>
                      <div className="text-sm font-semibold" style={{ color: "var(--color-text)" }}>
                        {tool.tool === "DockerScout" ? "Docker Scout" : tool.tool}
                      </div>
                      <div className="text-xs mt-1" style={{ color: "var(--color-text-muted)" }}>
                        {tool.available
                          ? t.security.toolAvailable(tool.version ?? t.common.notAvailable)
                          : t.security.toolUnavailable}
                      </div>
                    </div>
                    <input
                      type="checkbox"
                      readOnly
                      checked={tool.selected}
                      className="mt-0.5"
                    />
                  </div>
                  {!tool.available && (
                    <p className="text-xs mt-3" style={{ color: "var(--color-accent)" }}>
                      {t.security.installHint}
                    </p>
                  )}
                </button>
              ))}
            </div>
          </section>

          <section className="space-y-6 min-h-0 flex-1">
              <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
                <SummaryCard label={t.security.summary.totalImages} value={overview.total_images} />
                <SummaryCard label={t.security.summary.scannedImages} value={overview.scanned_images} />
                <SummaryCard
                  label={t.security.summary.imagesWithFindings}
                  value={overview.images_with_findings}
                />
              </div>

              <div className="rounded-lg border p-4" style={{ borderColor: "var(--color-border)" }}>
                <h2 className="text-sm font-semibold mb-4" style={{ color: "var(--color-text)" }}>
                  {t.security.summary.severityChart}
                </h2>
                <div className="space-y-3">
                  {overview.findings_by_severity.map((bucket) => (
                    <SeverityBar key={bucket.severity} bucket={bucket} max={maxCount(overview.findings_by_severity)} />
                  ))}
                </div>
              </div>

              <div className="rounded-lg border overflow-hidden flex flex-col min-h-0" style={{ borderColor: "var(--color-border)" }}>
                <div className="px-4 py-3 border-b" style={{ borderColor: "var(--color-border)" }}>
                  <h2 className="text-sm font-semibold" style={{ color: "var(--color-text)" }}>
                    {t.security.imagesTitle}
                  </h2>
                </div>
                <div className="overflow-auto">
                  <table className="w-full text-sm">
                    <thead>
                      <tr style={{ backgroundColor: "var(--color-surface-secondary)" }}>
                        <th className="px-4 py-2.5 text-left text-xs uppercase tracking-wider" style={{ color: "var(--color-text-muted)" }}>
                          {t.security.columns.image}
                        </th>
                        <th className="px-4 py-2.5 text-left text-xs uppercase tracking-wider" style={{ color: "var(--color-text-muted)" }}>
                          {t.security.columns.findings}
                        </th>
                        <th className="px-4 py-2.5 text-left text-xs uppercase tracking-wider" style={{ color: "var(--color-text-muted)" }}>
                          {t.security.columns.tools}
                        </th>
                        <th className="px-4 py-2.5 text-left text-xs uppercase tracking-wider" style={{ color: "var(--color-text-muted)" }}>
                          {t.security.columns.lastScan}
                        </th>
                      </tr>
                    </thead>
                    <tbody>
                      {overview.images.length === 0 ? (
                        <tr>
                          <td colSpan={4} className="px-4 py-12 text-center" style={{ color: "var(--color-text-muted)" }}>
                            {t.security.emptyImages}
                          </td>
                        </tr>
                      ) : (
                        overview.images.map((image) => (
                          <tr
                            key={image.image_id}
                            onClick={() => setOpenImageId(image.image_id)}
                            className="border-t cursor-pointer hover:opacity-90"
                            style={{
                              borderColor: "var(--color-border)",
                              backgroundColor: openImageId === image.image_id
                                ? "color-mix(in srgb, var(--color-accent) 10%, transparent)"
                                : "transparent",
                            }}
                          >
                            <td className="px-4 py-3">
                              <div className="font-medium" style={{ color: "var(--color-text)" }}>
                                {image.image_name}
                              </div>
                              <div className="text-xs font-mono mt-1" style={{ color: "var(--color-text-muted)" }}>
                                {shortId(image.image_id)}
                              </div>
                            </td>
                            <td className="px-4 py-3">
                              <div className="flex flex-wrap gap-1">
                                {image.severity_counts.filter((bucket) => bucket.count > 0).length === 0 ? (
                                  <span className="text-xs" style={{ color: "var(--color-text-muted)" }}>
                                    {t.security.noFindings}
                                  </span>
                                ) : (
                                  image.severity_counts
                                    .filter((bucket) => bucket.count > 0)
                                    .map((bucket) => (
                                      <span
                                        key={bucket.severity}
                                        className="px-2 py-0.5 rounded text-xs font-medium"
                                        style={{
                                          backgroundColor: `${SEVERITY_COLORS[bucket.severity] ?? "var(--color-accent)"}20`,
                                          color: SEVERITY_COLORS[bucket.severity] ?? "var(--color-accent)",
                                        }}
                                      >
                                        {bucket.severity}: {bucket.count}
                                      </span>
                                    ))
                                )}
                              </div>
                            </td>
                            <td className="px-4 py-3">
                              <div className="flex flex-wrap gap-1">
                                {image.tool_statuses.map((status) => (
                                  <span
                                    key={`${image.image_id}-${status.tool}`}
                                    className="px-2 py-0.5 rounded text-xs"
                                    style={{
                                      backgroundColor: status.selected
                                        ? "color-mix(in srgb, var(--color-accent) 18%, transparent)"
                                        : "var(--color-surface-secondary)",
                                      color: status.available ? "var(--color-text)" : "var(--color-text-muted)",
                                    }}
                                  >
                                    {(status.tool === "DockerScout" ? "Scout" : status.tool)} · {t.security.toolState[status.state]}
                                  </span>
                                ))}
                              </div>
                            </td>
                            <td className="px-4 py-3 text-xs" style={{ color: "var(--color-text-muted)" }}>
                              {image.last_scanned_at ?? t.common.notAvailable}
                            </td>
                          </tr>
                        ))
                      )}
                    </tbody>
                  </table>
                </div>
              </div>
          </section>
        </>
      )}

      {openImage && (
        <div className="fixed inset-0 flex items-center justify-center z-50" style={{ backgroundColor: "rgba(0,0,0,0.5)" }}>
          <div
            className="rounded-lg w-full max-w-6xl max-h-[85vh] mx-4 shadow-xl flex flex-col"
            style={{ backgroundColor: "var(--color-surface)" }}
          >
            <div className="px-6 py-4 border-b flex items-center justify-between gap-4" style={{ borderColor: "var(--color-border)" }}>
              <div>
                <h2 className="text-lg font-semibold" style={{ color: "var(--color-text)" }}>
                  {t.security.detail.title(openImage.image_name)}
                </h2>
                <p className="text-xs mt-1 font-mono" style={{ color: "var(--color-text-muted)" }}>
                  {shortId(openImage.image_id)}
                </p>
              </div>
              <button
                onClick={() => {
                  setOpenImageId(null);
                  setReport(null);
                }}
                className="px-4 py-2 text-sm rounded-md border"
                style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}
              >
                {t.common.dismiss}
              </button>
            </div>
            <div className="flex-1 overflow-auto p-6">
              {loadingReport ? (
                <div className="flex items-center justify-center h-full min-h-48">
                  <div
                    className="w-6 h-6 border-2 border-t-transparent rounded-full animate-spin"
                    style={{ borderColor: "var(--color-accent)", borderTopColor: "transparent" }}
                  />
                </div>
              ) : !report || report.reports.length === 0 ? (
                <p className="text-sm" style={{ color: "var(--color-text-muted)" }}>
                  {t.security.detail.noReports}
                </p>
              ) : (
                <div className="space-y-4">
                  {report.reports.map((toolReport) => (
                    <div
                      key={`${report.image_id}-${toolReport.tool}`}
                      className="rounded-lg border p-4"
                      style={{ borderColor: "var(--color-border)" }}
                    >
                      <div className="flex items-center justify-between gap-3 mb-3">
                        <div>
                          <h3 className="text-sm font-semibold" style={{ color: "var(--color-text)" }}>
                            {toolReport.tool === "DockerScout" ? "Docker Scout" : toolReport.tool}
                          </h3>
                          <p className="text-xs mt-1" style={{ color: "var(--color-text-muted)" }}>
                            {t.security.toolState[toolReport.state]}
                            {toolReport.generated_at ? ` · ${toolReport.generated_at}` : ""}
                          </p>
                        </div>
                        <span
                          className="px-2 py-1 rounded text-xs font-medium"
                          style={{ backgroundColor: "var(--color-surface-secondary)", color: "var(--color-text)" }}
                        >
                          {t.security.detail.findings(toolReport.findings.length)}
                        </span>
                      </div>

                      {toolReport.message && (
                        <p className="text-xs mb-3" style={{ color: "var(--color-text-muted)" }}>
                          {toolReport.message}
                        </p>
                      )}

                      <div className="flex flex-wrap gap-2 mb-3">
                        {toolReport.severity_counts.filter((bucket) => bucket.count > 0).length === 0 ? (
                          <span className="text-xs" style={{ color: "var(--color-text-muted)" }}>
                            {t.security.noFindings}
                          </span>
                        ) : (
                          toolReport.severity_counts
                            .filter((bucket) => bucket.count > 0)
                            .map((bucket) => (
                              <span
                                key={`${toolReport.tool}-${bucket.severity}`}
                                className="px-2 py-0.5 rounded text-xs font-medium"
                                style={{
                                  backgroundColor: `${SEVERITY_COLORS[bucket.severity] ?? "var(--color-accent)"}20`,
                                  color: SEVERITY_COLORS[bucket.severity] ?? "var(--color-accent)",
                                }}
                              >
                                {bucket.severity}: {bucket.count}
                              </span>
                            ))
                        )}
                      </div>

                      {toolReport.findings.length > 0 && (
                        <div className="overflow-auto max-h-64 rounded-md border" style={{ borderColor: "var(--color-border)" }}>
                          <table className="w-full text-xs">
                            <thead>
                              <tr style={{ backgroundColor: "var(--color-surface-secondary)" }}>
                                <th className="px-3 py-2 text-left" style={{ color: "var(--color-text-muted)" }}>
                                  {t.security.detail.columns.vulnerability}
                                </th>
                                <th className="px-3 py-2 text-left" style={{ color: "var(--color-text-muted)" }}>
                                  {t.security.detail.columns.package}
                                </th>
                                <th className="px-3 py-2 text-left" style={{ color: "var(--color-text-muted)" }}>
                                  {t.security.detail.columns.version}
                                </th>
                                <th className="px-3 py-2 text-left" style={{ color: "var(--color-text-muted)" }}>
                                  {t.security.detail.columns.severity}
                                </th>
                              </tr>
                            </thead>
                            <tbody>
                              {sortFindings(toolReport.findings).map((finding) => (
                                <Fragment
                                  key={`${toolReport.tool}-${finding.vulnerability_id}-${finding.package_name}-${finding.installed_version}`}
                                >
                                  <tr className="border-t" style={{ borderColor: "var(--color-border)" }}>
                                    <td className="px-3 py-2 align-top" style={{ color: "var(--color-text)" }}>
                                      <div className="font-mono">{finding.vulnerability_id}</div>
                                      {finding.title && (
                                        <div className="mt-1" style={{ color: "var(--color-text-muted)" }}>
                                          {finding.title}
                                        </div>
                                      )}
                                    </td>
                                    <td className="px-3 py-2 align-top" style={{ color: "var(--color-text)" }}>
                                      <div>{finding.package_name}</div>
                                      {finding.fixed_version && (
                                        <div className="mt-1 font-mono" style={{ color: "var(--color-text-muted)" }}>
                                          Fix: {finding.fixed_version}
                                        </div>
                                      )}
                                    </td>
                                    <td className="px-3 py-2 align-top font-mono" style={{ color: "var(--color-text-muted)" }}>
                                      {finding.installed_version || t.common.notAvailable}
                                    </td>
                                    <td className="px-3 py-2 align-top">
                                      <span style={{ color: SEVERITY_COLORS[finding.severity] ?? "var(--color-text)" }}>
                                        {finding.severity}
                                      </span>
                                    </td>
                                  </tr>
                                  {finding.description && (
                                    <tr className="border-t-0" style={{ borderColor: "var(--color-border)" }}>
                                      <td colSpan={4} className="px-3 pb-3 pt-0">
                                        <div className="text-[11px]" style={{ color: "var(--color-text-muted)" }}>
                                          {finding.description}
                                        </div>
                                      </td>
                                    </tr>
                                  )}
                                </Fragment>
                              ))}
                            </tbody>
                          </table>
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        </div>
      )}

      {installHint && (
        <div className="fixed inset-0 flex items-center justify-center z-50" style={{ backgroundColor: "rgba(0,0,0,0.5)" }}>
          <div className="rounded-lg p-6 max-w-2xl w-full mx-4 shadow-xl" style={{ backgroundColor: "var(--color-surface)" }}>
            <h3 className="text-lg font-semibold mb-2" style={{ color: "var(--color-text)" }}>
              {installHint.title}
            </h3>
            <p className="text-sm mb-4" style={{ color: "var(--color-text-muted)" }}>
              {installHint.description}
            </p>
            <div className="space-y-2">
              {installHint.commands.map((command) => (
                <div
                  key={command}
                  className="rounded-md px-3 py-2 text-xs font-mono whitespace-pre-wrap break-all"
                  style={{ backgroundColor: "var(--color-surface-secondary)", color: "var(--color-text)" }}
                >
                  {command}
                </div>
              ))}
            </div>
            {installHint.note && (
              <p className="text-xs mt-4" style={{ color: "var(--color-text-muted)" }}>
                {installHint.note}
              </p>
            )}
            <div className="flex justify-end mt-6">
              <button
                onClick={() => setInstallHint(null)}
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

function SummaryCard({ label, value }: { label: string; value: number }) {
  return (
    <div className="rounded-lg border p-4" style={{ borderColor: "var(--color-border)" }}>
      <div className="text-xs uppercase tracking-wider" style={{ color: "var(--color-text-muted)" }}>
        {label}
      </div>
      <div className="text-2xl font-semibold mt-2" style={{ color: "var(--color-text)" }}>
        {value}
      </div>
    </div>
  );
}

function SeverityBar({ bucket, max }: { bucket: SeverityCount; max: number }) {
  const width = max > 0 ? `${Math.max((bucket.count / max) * 100, bucket.count > 0 ? 8 : 0)}%` : "0%";
  return (
    <div>
      <div className="flex items-center justify-between text-xs mb-1" style={{ color: "var(--color-text-muted)" }}>
        <span>{bucket.severity}</span>
        <span>{bucket.count}</span>
      </div>
      <div className="h-2 rounded-full" style={{ backgroundColor: "var(--color-surface-secondary)" }}>
        <div
          className="h-2 rounded-full transition-all"
          style={{
            width,
            backgroundColor: SEVERITY_COLORS[bucket.severity] ?? "var(--color-accent)",
          }}
        />
      </div>
    </div>
  );
}

function maxCount(buckets: SeverityCount[]): number {
  return buckets.reduce((max, bucket) => Math.max(max, bucket.count), 0);
}

function sortFindings(findings: SecurityFinding[]): SecurityFinding[] {
  return [...findings].sort((left, right) => (
    severityRank(right.severity) - severityRank(left.severity)
    || left.vulnerability_id.localeCompare(right.vulnerability_id)
    || left.package_name.localeCompare(right.package_name)
    || left.installed_version.localeCompare(right.installed_version)
  ));
}

function severityRank(severity: SecurityFinding["severity"]): number {
  switch (severity) {
    case "Critical":
      return 6;
    case "High":
      return 5;
    case "Medium":
      return 4;
    case "Low":
      return 3;
    case "Negligible":
      return 2;
    default:
      return 1;
  }
}

function shortId(id: string): string {
  return id.startsWith("sha256:") ? id.substring(7, 19) : id.substring(0, 12);
}
