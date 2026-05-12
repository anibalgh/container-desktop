import { useEffect, useState, useCallback } from "react";
import type { Image } from "../lib/types";
import { listImages, pullImage, removeImage, onImagePullProgress } from "../lib/tauri";

type SortDir = "asc" | "desc";

function useSort<T>(data: T[], defaultCol: keyof T) {
  const [col, setCol] = useState<keyof T>(defaultCol);
  const [dir, setDir] = useState<SortDir>("asc");
  const sorted = [...data].sort((a, b) => {
    const va = String(a[col] ?? ""), vb = String(b[col] ?? "");
    return dir === "asc" ? va.localeCompare(vb) : vb.localeCompare(va);
  });
  function toggle(c: keyof T) {
    if (c === col) setDir((d) => (d === "asc" ? "desc" : "asc"));
    else { setCol(c); setDir("asc"); }
  }
  return { sorted, col, dir, toggle };
}

function SortTh({ col, currentCol, dir, label, onClick }: {
  col: string; currentCol: string; dir: SortDir; label: string; onClick: () => void;
}) {
  return (
    <th onClick={onClick} className="px-4 py-2.5 text-left text-xs font-medium uppercase tracking-wider cursor-pointer select-none hover:opacity-80"
      style={{ color: "var(--color-text-muted)" }}>
      {label} {currentCol === col ? (dir === "asc" ? "▲" : "▼") : ""}
    </th>
  );
}

export function ImagesScreen() {
  const [images, setImages] = useState<Image[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showPull, setShowPull] = useState(false);
  const [pullName, setPullName] = useState("");
  const [pullTag, setPullTag] = useState("latest");
  const [pullProgress, setPullProgress] = useState<string[]>([]);
  const [pulling, setPulling] = useState(false);
  const [confirmRemove, setConfirmRemove] = useState<string | null>(null);

  const { sorted, col, dir, toggle } = useSort(images, "repo_name");

  const load = useCallback(async () => {
    setLoading(true);
    try { setImages(await listImages()); } catch (e) { setError(String(e)); }
    finally { setLoading(false); }
  }, []);
  useEffect(() => { load(); }, [load]);

  async function doPull() {
    if (!pullName.trim()) return;
    setPulling(true); setPullProgress([]);
    const unlisten = await onImagePullProgress((msg) => setPullProgress((prev) => [...prev, msg]));
    try { await pullImage(pullName.trim(), pullTag || null); await load(); setShowPull(false); setPullName(""); setPullTag("latest"); }
    catch (e) { setError(String(e)); }
    finally { setPulling(false); unlisten(); }
  }

  async function doRemove(id: string) {
    try { await removeImage(id); setConfirmRemove(null); await load(); }
    catch (e) { setError(String(e)); }
  }

  const shortId = (id: string) => id.startsWith("sha256:") ? id.substring(7, 19) : id.substring(0, 12);

  if (loading) return <div className="flex items-center justify-center h-full"><div className="w-6 h-6 border-2 border-t-transparent rounded-full animate-spin" style={{ borderColor: "var(--color-accent)", borderTopColor: "transparent" }} /></div>;

  return (
    <div className="p-6 h-full flex flex-col">
      <div className="flex items-center justify-between mb-4">
        <h1 className="text-xl font-semibold" style={{ color: "var(--color-text)" }}>Images</h1>
        <div className="flex items-center gap-3">
          <span className="text-sm" style={{ color: "var(--color-text-muted)" }}>{images.length} image{images.length !== 1 ? "s" : ""}</span>
          <button onClick={() => setShowPull(true)} className="px-3 py-1.5 text-xs rounded-md text-white" style={{ backgroundColor: "var(--color-accent)" }}>Pull</button>
          <button onClick={load} className="px-3 py-1.5 text-xs rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>Refresh</button>
        </div>
      </div>
      {error && <div className="mb-3 px-3 py-2 text-sm rounded-md" style={{ backgroundColor: "rgba(239,68,68,0.1)", color: "var(--color-danger)" }}>{error}<button onClick={() => setError(null)} className="ml-2 underline">Dismiss</button></div>}
      <div className="flex-1 overflow-auto rounded-lg border" style={{ borderColor: "var(--color-border)" }}>
        <table className="w-full text-sm">
          <thead><tr style={{ backgroundColor: "var(--color-surface-secondary)" }}>
            <SortTh col="repo_name" currentCol={col as string} dir={dir} label="Repository" onClick={() => toggle("repo_name")} />
            <SortTh col="tag" currentCol={col as string} dir={dir} label="Tag" onClick={() => toggle("tag")} />
            <SortTh col="id" currentCol={col as string} dir={dir} label="Image ID" onClick={() => toggle("id")} />
            <SortTh col="size" currentCol={col as string} dir={dir} label="Size" onClick={() => toggle("size")} />
            <SortTh col="created" currentCol={col as string} dir={dir} label="Created" onClick={() => toggle("created")} />
            <th className="px-4 py-2.5 text-left text-xs font-medium uppercase tracking-wider" style={{ color: "var(--color-text-muted)" }}>Actions</th>
          </tr></thead>
          <tbody>
            {sorted.length === 0 ? <tr><td colSpan={6} className="px-4 py-12 text-center" style={{ color: "var(--color-text-muted)" }}>No images found.</td></tr> :
              sorted.map((img) => (
                <tr key={img.id} className="border-t" style={{ borderColor: "var(--color-border)" }}>
                  <td className="px-4 py-2.5"><span className="font-medium" style={{ color: "var(--color-text)" }}>{img.repo_name || <span style={{ color: "var(--color-text-muted)" }}>&lt;none&gt;</span>}</span></td>
                  <td className="px-4 py-2.5"><span className="px-1.5 py-0.5 rounded text-xs font-mono" style={{ backgroundColor: "var(--color-surface-secondary)" }}>{img.tag || "—"}</span></td>
                  <td className="px-4 py-2.5"><span className="font-mono text-xs">{shortId(img.id)}</span></td>
                  <td className="px-4 py-2.5"><span className="text-xs">{img.size}</span></td>
                  <td className="px-4 py-2.5"><span className="text-xs">{img.created}</span></td>
                  <td className="px-4 py-2.5"><button onClick={() => setConfirmRemove(img.id)} className="px-2 py-1 text-xs font-medium rounded border hover:opacity-80" style={{ borderColor: "var(--color-danger)", color: "var(--color-danger)" }}>Remove</button></td>
                </tr>
              ))}
          </tbody>
        </table>
      </div>

      {showPull && (
        <div className="fixed inset-0 flex items-center justify-center z-50" style={{ backgroundColor: "rgba(0,0,0,0.5)" }}>
          <div className="rounded-lg p-6 max-w-md w-full mx-4 shadow-xl max-h-[80vh] flex flex-col" style={{ backgroundColor: "var(--color-surface)" }}>
            <h3 className="text-lg font-semibold mb-4" style={{ color: "var(--color-text)" }}>Pull Image</h3>
            <div className="space-y-3 mb-4">
              <div>
                <label className="block text-xs font-medium mb-1" style={{ color: "var(--color-text-muted)" }}>Image Name</label>
                <input value={pullName} onChange={(e) => setPullName(e.target.value)} placeholder="nginx, alpine..." className="w-full px-3 py-2 text-sm rounded-md border" disabled={pulling} style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface-secondary)", color: "var(--color-text)" }} />
              </div>
              <div>
                <label className="block text-xs font-medium mb-1" style={{ color: "var(--color-text-muted)" }}>Tag</label>
                <input value={pullTag} onChange={(e) => setPullTag(e.target.value)} placeholder="latest" className="w-full px-3 py-2 text-sm rounded-md border" disabled={pulling} style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface-secondary)", color: "var(--color-text)" }} />
              </div>
            </div>
            {pullProgress.length > 0 && (
              <div className="flex-1 overflow-auto mb-4 p-3 rounded-md font-mono text-xs max-h-48" style={{ backgroundColor: "var(--color-surface-secondary)" }}>
                {pullProgress.map((line, i) => <div key={i} style={{ color: "var(--color-text-muted)" }}>{line}</div>)}
              </div>
            )}
            <div className="flex justify-end gap-2">
              <button onClick={() => { setShowPull(false); setPullProgress([]); }} disabled={pulling} className="px-4 py-2 text-sm rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>Cancel</button>
              <button onClick={doPull} disabled={pulling || !pullName.trim()} className="px-4 py-2 text-sm rounded-md text-white disabled:opacity-50" style={{ backgroundColor: "var(--color-accent)" }}>{pulling ? "Pulling..." : "Pull"}</button>
            </div>
          </div>
        </div>
      )}

      {confirmRemove && (
        <div className="fixed inset-0 flex items-center justify-center z-50" style={{ backgroundColor: "rgba(0,0,0,0.5)" }}>
          <div className="rounded-lg p-6 max-w-sm w-full mx-4 shadow-xl" style={{ backgroundColor: "var(--color-surface)" }}>
            <h3 className="text-lg font-semibold mb-2" style={{ color: "var(--color-text)" }}>Remove Image</h3>
            <p className="text-sm mb-4" style={{ color: "var(--color-text-muted)" }}>Remove <span className="font-medium" style={{ color: "var(--color-text)" }}>{images.find((i) => i.id === confirmRemove)?.repo_name || shortId(confirmRemove)}</span>?</p>
            <div className="flex justify-end gap-2">
              <button onClick={() => setConfirmRemove(null)} className="px-4 py-2 text-sm rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>Cancel</button>
              <button onClick={() => doRemove(confirmRemove)} className="px-4 py-2 text-sm rounded-md text-white" style={{ backgroundColor: "var(--color-danger)" }}>Remove</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
