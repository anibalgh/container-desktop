import { useEffect, useState, useCallback } from "react";
import type { Image } from "../lib/types";
import { listImages, pullImage, removeImage, onImagePullProgress } from "../lib/tauri";

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

  const load = useCallback(async () => {
    setLoading(true);
    try {
      setImages(await listImages());
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { load(); }, [load]);

  async function doPull() {
    if (!pullName.trim()) return;
    setPulling(true);
    setPullProgress([]);
    const unlisten = await onImagePullProgress((msg) => {
      setPullProgress((prev) => [...prev, msg]);
    });
    try {
      await pullImage(pullName.trim(), pullTag || null);
      await load();
      setShowPull(false);
      setPullName("");
      setPullTag("latest");
    } catch (e) {
      setError(String(e));
    } finally {
      setPulling(false);
      unlisten();
    }
  }

  async function doRemove(id: string) {
    try {
      await removeImage(id);
      setConfirmRemove(null);
      await load();
    } catch (e) {
      setError(String(e));
    }
  }

  const shortId = (id: string) => id.startsWith("sha256:") ? id.substring(7, 19) : id.substring(0, 12);

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
          Images
        </h1>
        <div className="flex items-center gap-3">
          <span className="text-sm" style={{ color: "var(--color-text-muted)" }}>
            {images.length} image{images.length !== 1 ? "s" : ""}
          </span>
          <button
            onClick={() => setShowPull(true)}
            className="px-3 py-1.5 text-xs rounded-md text-white"
            style={{ backgroundColor: "var(--color-accent)" }}>
            Pull Image
          </button>
          <button
            onClick={load}
            className="px-3 py-1.5 text-xs rounded-md border"
            style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>
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
              <Th>Repository</Th>
              <Th>Tag</Th>
              <Th>Image ID</Th>
              <Th>Size</Th>
              <Th>Created</Th>
              <Th>Actions</Th>
            </tr>
          </thead>
          <tbody>
            {images.length === 0 ? (
              <tr>
                <td colSpan={6} className="px-4 py-12 text-center"
                  style={{ color: "var(--color-text-muted)" }}>
                  No images found. Pull one to get started.
                </td>
              </tr>
            ) : (
              images.map((img) => (
                <tr key={img.id} className="border-t"
                  style={{ borderColor: "var(--color-border)" }}>
                  <Td>
                    <span className="font-medium" style={{ color: "var(--color-text)" }}>
                      {img.repo_name || <span style={{ color: "var(--color-text-muted)" }}>&lt;none&gt;</span>}
                    </span>
                  </Td>
                  <Td>
                    <span className="px-1.5 py-0.5 rounded text-xs font-mono"
                      style={{ backgroundColor: "var(--color-surface-secondary)" }}>
                      {img.tag || "—"}
                    </span>
                  </Td>
                  <Td>
                    <span className="font-mono text-xs">{shortId(img.id)}</span>
                  </Td>
                  <Td><span className="text-xs">{img.size}</span></Td>
                  <Td><span className="text-xs">{img.created}</span></Td>
                  <Td>
                    <button
                      onClick={() => setConfirmRemove(img.id)}
                      className="px-2 py-1 text-[11px] font-medium rounded border transition-opacity hover:opacity-80"
                      style={{ borderColor: "var(--color-danger)", color: "var(--color-danger)" }}>
                      Remove
                    </button>
                  </Td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Pull modal */}
      {showPull && (
        <div className="fixed inset-0 flex items-center justify-center z-50"
          style={{ backgroundColor: "rgba(0,0,0,0.5)" }}>
          <div className="rounded-lg p-6 max-w-md w-full mx-4 shadow-xl max-h-[80vh] flex flex-col"
            style={{ backgroundColor: "var(--color-surface)", borderColor: "var(--color-border)" }}>
            <h3 className="text-lg font-semibold mb-4" style={{ color: "var(--color-text)" }}>
              Pull Image
            </h3>
            <div className="space-y-3 mb-4">
              <div>
                <label className="block text-xs font-medium mb-1" style={{ color: "var(--color-text-muted)" }}>
                  Image Name
                </label>
                <input
                  value={pullName}
                  onChange={(e) => setPullName(e.target.value)}
                  placeholder="nginx, alpine, ubuntu..."
                  className="w-full px-3 py-2 text-sm rounded-md border focus:outline-none"
                  style={{
                    borderColor: "var(--color-border)",
                    backgroundColor: "var(--color-surface-secondary)",
                    color: "var(--color-text)",
                  }}
                  disabled={pulling}
                />
              </div>
              <div>
                <label className="block text-xs font-medium mb-1" style={{ color: "var(--color-text-muted)" }}>
                  Tag
                </label>
                <input
                  value={pullTag}
                  onChange={(e) => setPullTag(e.target.value)}
                  placeholder="latest"
                  className="w-full px-3 py-2 text-sm rounded-md border focus:outline-none"
                  style={{
                    borderColor: "var(--color-border)",
                    backgroundColor: "var(--color-surface-secondary)",
                    color: "var(--color-text)",
                  }}
                  disabled={pulling}
                />
              </div>
            </div>

            {pullProgress.length > 0 && (
              <div className="flex-1 overflow-auto mb-4 p-3 rounded-md font-mono text-xs max-h-48"
                style={{ backgroundColor: "var(--color-surface-secondary)" }}>
                {pullProgress.map((line, i) => (
                  <div key={i} style={{ color: "var(--color-text-muted)" }}>{line}</div>
                ))}
              </div>
            )}

            <div className="flex justify-end gap-2">
              <button
                onClick={() => { setShowPull(false); setPullProgress([]); }}
                disabled={pulling}
                className="px-4 py-2 text-sm rounded-md border"
                style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>
                Cancel
              </button>
              <button
                onClick={doPull}
                disabled={pulling || !pullName.trim()}
                className="px-4 py-2 text-sm rounded-md text-white disabled:opacity-50"
                style={{ backgroundColor: "var(--color-accent)" }}>
                {pulling ? "Pulling..." : "Pull"}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Confirm remove modal */}
      {confirmRemove && (
        <div className="fixed inset-0 flex items-center justify-center z-50"
          style={{ backgroundColor: "rgba(0,0,0,0.5)" }}>
          <div className="rounded-lg p-6 max-w-sm w-full mx-4 shadow-xl"
            style={{ backgroundColor: "var(--color-surface)" }}>
            <h3 className="text-lg font-semibold mb-2" style={{ color: "var(--color-text)" }}>
              Remove Image
            </h3>
            <p className="text-sm mb-4" style={{ color: "var(--color-text-muted)" }}>
              Remove image{" "}
              <span className="font-medium" style={{ color: "var(--color-text)" }}>
                {images.find((i) => i.id === confirmRemove)?.repo_name || shortId(confirmRemove)}
              </span>?
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
