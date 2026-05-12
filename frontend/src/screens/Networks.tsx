import { useEffect, useState, useCallback } from "react";
import type { Network } from "../lib/types";
import { listNetworks, createNetwork, removeNetwork } from "../lib/tauri";

type SortDir = "asc" | "desc";
function useSort<T>(data: T[], dc: keyof T) {
  const [col, setCol] = useState<keyof T>(dc);
  const [dir, setDir] = useState<SortDir>("asc");
  const sorted = [...data].sort((a, b) => { const va = String(a[col] ?? ""), vb = String(b[col] ?? ""); return dir === "asc" ? va.localeCompare(vb) : vb.localeCompare(va); });
  function toggle(c: keyof T) { if (c === col) setDir(d => d === "asc" ? "desc" : "asc"); else { setCol(c); setDir("asc"); } }
  return { sorted, col, dir, toggle };
}
function STh({ col, cCol, dir, label, onClick }: { col: string; cCol: string; dir: SortDir; label: string; onClick: () => void }) {
  return <th onClick={onClick} className="px-4 py-2.5 text-left text-xs font-medium uppercase tracking-wider cursor-pointer select-none hover:opacity-80" style={{ color: "var(--color-text-muted)" }}>{label} {cCol === col ? (dir === "asc" ? "▲" : "▼") : ""}</th>;
}

export function NetworksScreen() {
  const [networks, setNetworks] = useState<Network[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showCreate, setShowCreate] = useState(false);
  const [newName, setNewName] = useState("");
  const [newDriver, setNewDriver] = useState("bridge");
  const [creating, setCreating] = useState(false);
  const { sorted, col, dir, toggle } = useSort(networks, "name");

  const load = useCallback(async (showLoading = true) => {
    if (showLoading) setLoading(true);
    try { setNetworks(await listNetworks()); } catch (e) { setError(String(e)); }
    finally { setLoading(false); }
  }, []);
  useEffect(() => {
    let cancelled = false;

    listNetworks()
      .then((data) => {
        if (!cancelled) setNetworks(data);
      })
      .catch((e) => {
        if (!cancelled) setError(String(e));
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });

    return () => {
      cancelled = true;
    };
  }, []);

  async function doCreate() {
    if (!newName.trim()) return; setCreating(true);
    try { await createNetwork(newName.trim(), newDriver || null); setShowCreate(false); setNewName(""); setNewDriver("bridge"); await load(); }
    catch (e) { setError(String(e)); } finally { setCreating(false); }
  }
  async function doRemove(id: string) { try { await removeNetwork(id); await load(); } catch (e) { setError(String(e)); } }

  if (loading) return <div className="flex items-center justify-center h-full"><div className="w-6 h-6 border-2 border-t-transparent rounded-full animate-spin" style={{ borderColor: "var(--color-accent)", borderTopColor: "transparent" }} /></div>;

  return (
    <div className="p-6 h-full flex flex-col">
      <div className="flex items-center justify-between mb-4">
        <h1 className="text-xl font-semibold" style={{ color: "var(--color-text)" }}>Networks</h1>
        <div className="flex items-center gap-3">
          <span className="text-sm" style={{ color: "var(--color-text-muted)" }}>{networks.length} network{networks.length !== 1 ? "s" : ""}</span>
          <button onClick={() => setShowCreate(true)} className="px-3 py-1.5 text-xs rounded-md text-white" style={{ backgroundColor: "var(--color-accent)" }}>Create</button>
          <button onClick={() => { void load(); }} className="px-3 py-1.5 text-xs rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>Refresh</button>
        </div>
      </div>
      {error && <div className="mb-3 px-3 py-2 text-sm rounded-md" style={{ backgroundColor: "rgba(239,68,68,0.1)", color: "var(--color-danger)" }}>{error}<button onClick={() => setError(null)} className="ml-2 underline">Dismiss</button></div>}
      <div className="flex-1 overflow-auto rounded-lg border" style={{ borderColor: "var(--color-border)" }}>
        <table className="w-full text-sm">
          <thead><tr style={{ backgroundColor: "var(--color-surface-secondary)" }}>
            <STh col="name" cCol={col as string} dir={dir} label="Name" onClick={() => toggle("name")} />
            <STh col="driver" cCol={col as string} dir={dir} label="Driver" onClick={() => toggle("driver")} />
            <STh col="scope" cCol={col as string} dir={dir} label="Scope" onClick={() => toggle("scope")} />
            <STh col="subnet" cCol={col as string} dir={dir} label="Subnet" onClick={() => toggle("subnet")} />
            <STh col="gateway" cCol={col as string} dir={dir} label="Gateway" onClick={() => toggle("gateway")} />
            <th className="px-4 py-2.5 text-left text-xs font-medium uppercase tracking-wider" style={{ color: "var(--color-text-muted)" }}>Actions</th>
          </tr></thead>
          <tbody>
            {sorted.length === 0 ? <tr><td colSpan={6} className="px-4 py-12 text-center" style={{ color: "var(--color-text-muted)" }}>No networks found.</td></tr> :
              sorted.map((n) => (
                <tr key={n.id} className="border-t" style={{ borderColor: "var(--color-border)" }}>
                  <td className="px-4 py-2.5"><span className="font-medium" style={{ color: "var(--color-text)" }}>{n.name}</span></td>
                  <td className="px-4 py-2.5"><span className="text-xs">{n.driver}</span></td>
                  <td className="px-4 py-2.5"><span className="text-xs">{n.scope}</span></td>
                  <td className="px-4 py-2.5"><span className="font-mono text-xs">{n.subnet || "—"}</span></td>
                  <td className="px-4 py-2.5"><span className="font-mono text-xs">{n.gateway || "—"}</span></td>
                  <td className="px-4 py-2.5"><button onClick={() => doRemove(n.id)} className="px-2 py-1 text-xs font-medium rounded border hover:opacity-80" style={{ borderColor: "var(--color-danger)", color: "var(--color-danger)" }}>Remove</button></td>
                </tr>
              ))}
          </tbody>
        </table>
      </div>
      {showCreate && (
        <div className="fixed inset-0 flex items-center justify-center z-50" style={{ backgroundColor: "rgba(0,0,0,0.5)" }}>
          <div className="rounded-lg p-6 max-w-sm w-full mx-4 shadow-xl" style={{ backgroundColor: "var(--color-surface)" }}>
            <h3 className="text-lg font-semibold mb-3" style={{ color: "var(--color-text)" }}>Create Network</h3>
            <input value={newName} onChange={(e) => setNewName(e.target.value)} placeholder="Network name" className="w-full px-3 py-2 text-sm rounded-md border mb-2" disabled={creating} style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface-secondary)", color: "var(--color-text)" }} />
            <select value={newDriver} onChange={(e) => setNewDriver(e.target.value)} className="w-full px-3 py-2 text-sm rounded-md border mb-4" disabled={creating} style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface-secondary)", color: "var(--color-text)" }}>
              <option value="bridge">bridge</option><option value="overlay">overlay</option><option value="host">host</option><option value="none">none</option>
            </select>
            <div className="flex justify-end gap-2">
              <button onClick={() => setShowCreate(false)} disabled={creating} className="px-4 py-2 text-sm rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>Cancel</button>
              <button onClick={doCreate} disabled={creating || !newName.trim()} className="px-4 py-2 text-sm rounded-md text-white disabled:opacity-50" style={{ backgroundColor: "var(--color-accent)" }}>{creating ? "Creating..." : "Create"}</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
