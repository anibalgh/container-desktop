import { useEffect, useState, useCallback } from "react";
import type { Container } from "../lib/types";
import { listContainers, startContainer, stopContainer, restartContainer, removeContainer } from "../lib/tauri";

export function ContainersScreen() {
  const [containers, setContainers] = useState<Container[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [confirmRemove, setConfirmRemove] = useState<string | null>(null);
  const [actionLoading, setActionLoading] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await listContainers(true);
      setContainers(data);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { load(); }, [load]);

  async function doAction(id: string, name: string, action: string) {
    setActionLoading(id);
    try {
      if (action === "start") await startContainer(id);
      else if (action === "stop") await stopContainer(id);
      else if (action === "restart") await restartContainer(id);
      await load();
    } catch (e) {
      setError(`${name}: ${e}`);
    } finally {
      setActionLoading(null);
    }
  }

  async function doRemove(id: string) {
    setActionLoading(id);
    try {
      await removeContainer(id);
      setConfirmRemove(null);
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setActionLoading(null);
    }
  }

  const stateColor = (state: string) => {
    switch (state) {
      case "Running": return "var(--color-success)";
      case "Exited": return "var(--color-danger)";
      case "Paused": return "var(--color-warning)";
      default: return "var(--color-text-muted)";
    }
  };

  const shortId = (id: string) => id.substring(0, 12);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="w-6 h-6 border-2 border-t-transparent rounded-full animate-spin"
          style={{ borderColor: "var(--color-accent)", borderTopColor: "transparent" }} />
      </div>
    );
  }

  return (
    <div className="p-6 h-full flex flex-col">
      <div className="flex items-center justify-between mb-4">
        <h1 className="text-xl font-semibold" style={{ color: "var(--color-text)" }}>
          Containers
        </h1>
        <div className="flex items-center gap-3">
          <span className="text-sm" style={{ color: "var(--color-text-muted)" }}>
            {containers.length} container{containers.length !== 1 ? "s" : ""}
          </span>
          <button
            onClick={load}
            className="px-3 py-1.5 text-xs rounded-md border transition-colors hover:opacity-80"
            style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}
          >
            Refresh
          </button>
        </div>
      </div>

      {error && (
        <div className="mb-3 px-3 py-2 text-sm rounded-md"
          style={{ backgroundColor: "rgba(239,68,68,0.1)", color: "var(--color-danger)" }}>
          {error}
          <button onClick={() => setError(null)} className="ml-2 underline">Dismiss</button>
        </div>
      )}

      <div className="flex-1 overflow-auto rounded-lg border"
        style={{ borderColor: "var(--color-border)" }}>
        <table className="w-full text-sm">
          <thead>
            <tr style={{ backgroundColor: "var(--color-surface-secondary)" }}>
              <Th>Name</Th>
              <Th>Image</Th>
              <Th>State</Th>
              <Th>Ports</Th>
              <Th>Created</Th>
              <Th>Actions</Th>
            </tr>
          </thead>
          <tbody>
            {containers.length === 0 ? (
              <tr>
                <td colSpan={6} className="px-4 py-12 text-center"
                  style={{ color: "var(--color-text-muted)" }}>
                  No containers found.
                </td>
              </tr>
            ) : (
              containers.map((c) => (
                <tr key={c.id} className="border-t hover:opacity-80"
                  style={{ borderColor: "var(--color-border)" }}>
                  <Td>
                    <div className="font-medium" style={{ color: "var(--color-text)" }}>{c.name}</div>
                    <div className="text-xs font-mono" style={{ color: "var(--color-text-muted)" }}>
                      {shortId(c.id)}
                    </div>
                  </Td>
                  <Td>
                    <span className="font-mono text-xs">{c.image}</span>
                  </Td>
                  <Td>
                    <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium"
                      style={{
                        backgroundColor: `${stateColor(c.state)}20`,
                        color: stateColor(c.state),
                      }}>
                      <span className="w-1.5 h-1.5 rounded-full"
                        style={{ backgroundColor: stateColor(c.state) }} />
                      {c.state}
                    </span>
                    <div className="text-xs mt-0.5" style={{ color: "var(--color-text-muted)" }}>
                      {c.status}
                    </div>
                  </Td>
                  <Td>
                    <div className="font-mono text-xs">
                      {c.ports.length > 0
                        ? c.ports.map((p, i) => (
                            <div key={i}>{p.host_ip}:{p.host_port}→{p.container_port}/{p.protocol}</div>
                          ))
                        : <span style={{ color: "var(--color-text-muted)" }}>—</span>}
                    </div>
                  </Td>
                  <Td>
                    <span className="text-xs">{c.created}</span>
                  </Td>
                  <Td>
                    <div className="flex items-center gap-1">
                      {c.state === "Exited" || c.state === "Created" ? (
                        <ActionBtn
                          label="Start"
                          color="var(--color-success)"
                          loading={actionLoading === c.id}
                          onClick={() => doAction(c.id, c.name, "start")}
                        />
                      ) : null}
                      {c.state === "Running" ? (
                        <>
                          <ActionBtn
                            label="Stop"
                            color="var(--color-warning)"
                            loading={actionLoading === c.id}
                            onClick={() => doAction(c.id, c.name, "stop")}
                          />
                          <ActionBtn
                            label="Restart"
                            color="var(--color-accent)"
                            loading={actionLoading === c.id}
                            onClick={() => doAction(c.id, c.name, "restart")}
                          />
                        </>
                      ) : null}
                      {c.state !== "Removing" && (
                        <ActionBtn
                          label="Remove"
                          color="var(--color-danger)"
                          loading={false}
                          onClick={() => setConfirmRemove(c.id)}
                        />
                      )}
                    </div>
                  </Td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Confirm remove modal */}
      {confirmRemove && (
        <div className="fixed inset-0 flex items-center justify-center z-50"
          style={{ backgroundColor: "rgba(0,0,0,0.5)" }}>
          <div className="rounded-lg p-6 max-w-sm w-full mx-4 shadow-xl"
            style={{ backgroundColor: "var(--color-surface)", borderColor: "var(--color-border)" }}>
            <h3 className="text-lg font-semibold mb-2" style={{ color: "var(--color-text)" }}>
              Remove Container
            </h3>
            <p className="text-sm mb-4" style={{ color: "var(--color-text-muted)" }}>
              Are you sure you want to remove container{" "}
              <span className="font-medium" style={{ color: "var(--color-text)" }}>
                {containers.find((c) => c.id === confirmRemove)?.name ?? shortId(confirmRemove)}
              </span>?
              This action cannot be undone.
            </p>
            <div className="flex justify-end gap-2">
              <button
                onClick={() => setConfirmRemove(null)}
                className="px-4 py-2 text-sm rounded-md border"
                style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>
                Cancel
              </button>
              <button
                onClick={() => doRemove(confirmRemove)}
                className="px-4 py-2 text-sm rounded-md text-white"
                style={{ backgroundColor: "var(--color-danger)" }}>
                Remove
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function Th({ children }: { children: React.ReactNode }) {
  return (
    <th className="px-4 py-2.5 text-left text-xs font-medium uppercase tracking-wider"
      style={{ color: "var(--color-text-muted)" }}>
      {children}
    </th>
  );
}

function Td({ children }: { children: React.ReactNode }) {
  return (
    <td className="px-4 py-2.5" style={{ color: "var(--color-text)" }}>
      {children}
    </td>
  );
}

function ActionBtn({
  label,
  color,
  loading,
  onClick,
}: {
  label: string;
  color: string;
  loading: boolean;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      disabled={loading}
      className="px-2 py-1 text-[11px] font-medium rounded border transition-opacity hover:opacity-80 disabled:opacity-50"
      style={{ borderColor: color, color }}
    >
      {loading ? "..." : label}
    </button>
  );
}
