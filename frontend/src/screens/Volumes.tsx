import { useEffect, useState, useCallback } from "react";
import type { Volume } from "../lib/types";
import { listVolumes, createVolume, removeVolume } from "../lib/tauri";

export function VolumesScreen() {
  const [volumes, setVolumes] = useState<Volume[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showCreate, setShowCreate] = useState(false);
  const [newName, setNewName] = useState("");
  const [creating, setCreating] = useState(false);

  const load = useCallback(async () => {
    setLoading(true);
    try { setVolumes(await listVolumes()); } catch (e) { setError(String(e)); }
    finally { setLoading(false); }
  }, []);
  useEffect(() => { load(); }, [load]);

  async function doCreate() {
    if (!newName.trim()) return;
    setCreating(true);
    try {
      await createVolume(newName.trim());
      setShowCreate(false);
      setNewName("");
      await load();
    } catch (e) { setError(String(e)); }
    finally { setCreating(false); }
  }

  async function doRemove(name: string) {
    try { await removeVolume(name); await load(); }
    catch (e) { setError(String(e)); }
  }

  if (loading) {
    return <div className="flex items-center justify-center h-full">
      <div className="w-6 h-6 border-2 border-t-transparent rounded-full animate-spin"
        style={{ borderColor: "var(--color-accent)", borderTopColor: "transparent" }} />
    </div>;
  }

  return (
    <div className="p-6 h-full flex flex-col">
      <div className="flex items-center justify-between mb-4">
        <h1 className="text-xl font-semibold" style={{ color: "var(--color-text)" }}>Volumes</h1>
        <div className="flex items-center gap-3">
          <span className="text-sm" style={{ color: "var(--color-text-muted)" }}>{volumes.length} volume{volumes.length !== 1 ? "s" : ""}</span>
          <button onClick={() => setShowCreate(true)} className="px-3 py-1.5 text-xs rounded-md text-white"
            style={{ backgroundColor: "var(--color-accent)" }}>Create</button>
          <button onClick={load} className="px-3 py-1.5 text-xs rounded-md border"
            style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>Refresh</button>
        </div>
      </div>
      {error && <div className="mb-3 px-3 py-2 text-sm rounded-md"
        style={{ backgroundColor: "rgba(239,68,68,0.1)", color: "var(--color-danger)" }}>
        {error} <button onClick={() => setError(null)} className="ml-2 underline">Dismiss</button>
      </div>}
      <div className="flex-1 overflow-auto rounded-lg border" style={{ borderColor: "var(--color-border)" }}>
        <table className="w-full text-sm">
          <thead><tr style={{ backgroundColor: "var(--color-surface-secondary)" }}>
            <Th>Name</Th><Th>Driver</Th><Th>Mountpoint</Th><Th>Created</Th><Th>Actions</Th>
          </tr></thead>
          <tbody>
            {volumes.length === 0 ? (
              <tr><td colSpan={5} className="px-4 py-12 text-center" style={{ color: "var(--color-text-muted)" }}>No volumes found.</td></tr>
            ) : volumes.map((v) => (
              <tr key={v.name} className="border-t" style={{ borderColor: "var(--color-border)" }}>
                <Td><span className="font-medium" style={{ color: "var(--color-text)" }}>{v.name}</span></Td>
                <Td><span className="text-xs">{v.driver}</span></Td>
                <Td><span className="font-mono text-xs">{v.mountpoint}</span></Td>
                <Td><span className="text-xs">{v.created}</span></Td>
                <Td>
                  <button onClick={() => doRemove(v.name)}
                    className="px-2 py-1 text-[11px] font-medium rounded border hover:opacity-80"
                    style={{ borderColor: "var(--color-danger)", color: "var(--color-danger)" }}>Remove</button>
                </Td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {showCreate && (
        <div className="fixed inset-0 flex items-center justify-center z-50" style={{ backgroundColor: "rgba(0,0,0,0.5)" }}>
          <div className="rounded-lg p-6 max-w-sm w-full mx-4 shadow-xl" style={{ backgroundColor: "var(--color-surface)" }}>
            <h3 className="text-lg font-semibold mb-3" style={{ color: "var(--color-text)" }}>Create Volume</h3>
            <input value={newName} onChange={(e) => setNewName(e.target.value)} placeholder="Volume name"
              className="w-full px-3 py-2 text-sm rounded-md border mb-4" disabled={creating}
              style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface-secondary)", color: "var(--color-text)" }} />
            <div className="flex justify-end gap-2">
              <button onClick={() => setShowCreate(false)} disabled={creating}
                className="px-4 py-2 text-sm rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>Cancel</button>
              <button onClick={doCreate} disabled={creating || !newName.trim()}
                className="px-4 py-2 text-sm rounded-md text-white disabled:opacity-50" style={{ backgroundColor: "var(--color-accent)" }}>
                {creating ? "Creating..." : "Create"}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function Th({ children }: { children: React.ReactNode }) {
  return <th className="px-4 py-2.5 text-left text-xs font-medium uppercase tracking-wider"
    style={{ color: "var(--color-text-muted)" }}>{children}</th>;
}
function Td({ children }: { children: React.ReactNode }) {
  return <td className="px-4 py-2.5" style={{ color: "var(--color-text)" }}>{children}</td>;
}
