import { useEffect, useMemo, useState, useCallback } from "react";
import type { Container, Image } from "../lib/types";
import {
  listContainers,
  listImages,
  pullImage,
  removeImage,
  onImagePullProgress,
  onImagePullStatus,
} from "../lib/tauri";
import { useI18n } from "../i18n";

type SortDir = "asc" | "desc";
type ImageRow = Image & { usedByCount: number };
type SortCol = "repo_name" | "tag" | "id" | "usedByCount" | "size" | "created";

const MAX_PULL_PROGRESS_LINES = 400;

function SortTh({ col, currentCol, dir, label, onClick }: {
  col: SortCol;
  currentCol: SortCol;
  dir: SortDir;
  label: string;
  onClick: () => void;
}) {
  return (
    <th onClick={onClick} className="px-4 py-2.5 text-left text-xs font-medium uppercase tracking-wider cursor-pointer select-none hover:opacity-80"
      style={{ color: "var(--color-text-muted)" }}>
      {label} {currentCol === col ? (dir === "asc" ? "▲" : "▼") : ""}
    </th>
  );
}

export function ImagesScreen() {
  const { t } = useI18n();
  const [images, setImages] = useState<Image[]>([]);
  const [containers, setContainers] = useState<Container[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showPull, setShowPull] = useState(false);
  const [pullName, setPullName] = useState("");
  const [pullTag, setPullTag] = useState("latest");
  const [pullProgress, setPullProgress] = useState<string[]>([]);
  const [pulling, setPulling] = useState(false);
  const [confirmRemoveIds, setConfirmRemoveIds] = useState<string[] | null>(null);
  const [removing, setRemoving] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectionMode, setSelectionMode] = useState(false);
  const [selectedIds, setSelectedIds] = useState<string[]>([]);
  const [sortCol, setSortCol] = useState<SortCol>("repo_name");
  const [sortDir, setSortDir] = useState<SortDir>("asc");

  const load = useCallback(async (showLoading = true) => {
    if (showLoading) setLoading(true);
    setError(null);
    try {
      const [nextImages, nextContainers] = await Promise.all([
        listImages(),
        listContainers(true),
      ]);
      setImages(nextImages);
      setContainers(nextContainers);
      setSelectedIds((current) => current.filter((id) => nextImages.some((image) => image.id === id)));
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    queueMicrotask(() => {
      void load();
    });
  }, [load]);

  const rows = useMemo<ImageRow[]>(() => (
    images.map((image) => ({
      ...image,
      usedByCount: containers.filter((container) => containerUsesImage(container, image)).length,
    }))
  ), [containers, images]);

  const filteredRows = useMemo(() => {
    const query = searchQuery.trim().toLowerCase();
    const filtered = rows.filter((row) => {
      if (!query) return true;
      return [row.repo_name, row.tag, row.id].some((value) => value.toLowerCase().includes(query));
    });

    return [...filtered].sort((left, right) => compareRows(left, right, sortCol, sortDir));
  }, [rows, searchQuery, sortCol, sortDir]);

  const allVisibleSelected = filteredRows.length > 0 && filteredRows.every((row) => selectedIds.includes(row.id));

  function toggleSort(nextCol: SortCol) {
    if (sortCol === nextCol) {
      setSortDir((current) => (current === "asc" ? "desc" : "asc"));
      return;
    }
    setSortCol(nextCol);
    setSortDir("asc");
  }

  function toggleSelectionMode() {
    if (selectionMode) {
      setSelectedIds([]);
    }
    setSelectionMode(!selectionMode);
  }

  function toggleSelected(id: string) {
    setSelectedIds((current) => (
      current.includes(id) ? current.filter((item) => item !== id) : [...current, id]
    ));
  }

  function toggleSelectAllVisible() {
    const visibleIds = filteredRows.map((row) => row.id);
    setSelectedIds((current) => {
      if (visibleIds.every((id) => current.includes(id))) {
        return current.filter((id) => !visibleIds.includes(id));
      }
      return Array.from(new Set([...current, ...visibleIds]));
    });
  }

  async function doPull() {
    if (!pullName.trim()) return;
    const requestId = crypto.randomUUID();
    setPulling(true);
    setPullProgress([]);
    let resolveStatus!: () => void;
    let rejectStatus!: (error: Error) => void;
    const statusPromise = new Promise<void>((resolve, reject) => {
      resolveStatus = resolve;
      rejectStatus = reject;
    });
    const [unlistenProgress, unlistenStatus] = await Promise.all([
      onImagePullProgress((event) => {
        if (event.requestId !== requestId) return;
        setPullProgress((prev) => [...prev, event.message].slice(-MAX_PULL_PROGRESS_LINES));
      }),
      onImagePullStatus((event) => {
        if (event.requestId !== requestId) return;
        if (event.status === "completed") resolveStatus();
        if (event.status === "failed") rejectStatus(new Error(event.error ?? "Image pull failed"));
      }),
    ]);
    try {
      await pullImage(pullName.trim(), pullTag || null, requestId);
      await statusPromise;
      await load(false);
      setShowPull(false);
      setPullName("");
      setPullTag("latest");
    } catch (e) {
      setError(String(e));
    } finally {
      setPulling(false);
      unlistenProgress();
      unlistenStatus();
    }
  }

  async function doRemove(ids: string[]) {
    if (ids.length === 0) return;
    setRemoving(true);
    try {
      for (const id of ids) {
        await removeImage(id);
      }
      setConfirmRemoveIds(null);
      setSelectedIds((current) => current.filter((id) => !ids.includes(id)));
      await load(false);
    } catch (e) {
      setError(String(e));
    } finally {
      setRemoving(false);
    }
  }

  const shortId = (id: string) => id.startsWith("sha256:") ? id.substring(7, 19) : id.substring(0, 12);
  const confirmSingleName = confirmRemoveIds?.length === 1
    ? rows.find((row) => row.id === confirmRemoveIds[0])?.repo_name || shortId(confirmRemoveIds[0])
    : null;

  if (loading) return <div className="flex items-center justify-center h-full"><div className="w-6 h-6 border-2 border-t-transparent rounded-full animate-spin" style={{ borderColor: "var(--color-accent)", borderTopColor: "transparent" }} /></div>;

  return (
    <div className="p-6 h-full flex flex-col">
      <div className="flex items-center justify-between mb-4 gap-3">
        <h1 className="text-xl font-semibold" style={{ color: "var(--color-text)" }}>{t.images.title}</h1>
        <div className="flex items-center gap-3 flex-wrap justify-end">
          <span className="text-sm" style={{ color: "var(--color-text-muted)" }}>{t.images.count(images.length)}</span>
          {selectionMode && <span className="text-xs" style={{ color: "var(--color-text-muted)" }}>{t.images.selectedCount(selectedIds.length)}</span>}
          {selectionMode && (
            <>
              <button onClick={toggleSelectAllVisible} className="px-3 py-1.5 text-xs rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>
                {t.images.selectAllFiltered}
              </button>
              <button onClick={() => setSelectedIds([])} disabled={selectedIds.length === 0} className="px-3 py-1.5 text-xs rounded-md border disabled:opacity-50" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>
                {t.images.clearSelection}
              </button>
              <button onClick={() => setConfirmRemoveIds(selectedIds)} disabled={selectedIds.length === 0} className="px-3 py-1.5 text-xs rounded-md text-white disabled:opacity-50" style={{ backgroundColor: "var(--color-danger)" }}>
                {t.images.removeSelected}
              </button>
            </>
          )}
          <button onClick={() => setShowPull(true)} className="px-3 py-1.5 text-xs rounded-md text-white" style={{ backgroundColor: "var(--color-accent)" }}>{t.images.pull}</button>
          <button onClick={() => { void load(); }} className="px-3 py-1.5 text-xs rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>{t.common.refresh}</button>
        </div>
      </div>
      <div className="mb-3 flex items-center gap-3 flex-wrap">
        <button onClick={toggleSelectionMode} className="px-3 py-1.5 text-xs rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>
          {selectionMode ? t.images.exitSelectionMode : t.images.selectionMode}
        </button>
        <label className="text-sm" style={{ color: "var(--color-text)" }}>
          {t.images.filterLabel}
        </label>
        <input
          value={searchQuery}
          onChange={(event) => setSearchQuery(event.target.value)}
          placeholder={t.images.searchPlaceholder}
          className="w-full max-w-md px-3 py-2 text-sm rounded-md border"
          style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface-secondary)", color: "var(--color-text)" }}
        />
      </div>
      {error && <div className="mb-3 px-3 py-2 text-sm rounded-md" style={{ backgroundColor: "rgba(239,68,68,0.1)", color: "var(--color-danger)" }}>{error}<button onClick={() => setError(null)} className="ml-2 underline">{t.common.dismiss}</button></div>}
      <div className="flex-1 overflow-auto rounded-lg border" style={{ borderColor: "var(--color-border)" }}>
        <table className="w-full text-sm">
          <thead><tr style={{ backgroundColor: "var(--color-surface-secondary)" }}>
            {selectionMode && (
              <th className="px-4 py-2.5 text-left text-xs font-medium uppercase tracking-wider" style={{ color: "var(--color-text-muted)" }}>
                <input type="checkbox" checked={allVisibleSelected} onChange={toggleSelectAllVisible} />
              </th>
            )}
            <SortTh col="repo_name" currentCol={sortCol} dir={sortDir} label={t.images.columns.repository} onClick={() => toggleSort("repo_name")} />
            <SortTh col="tag" currentCol={sortCol} dir={sortDir} label={t.images.columns.tag} onClick={() => toggleSort("tag")} />
            <SortTh col="id" currentCol={sortCol} dir={sortDir} label={t.images.columns.imageId} onClick={() => toggleSort("id")} />
            <SortTh col="usedByCount" currentCol={sortCol} dir={sortDir} label={t.images.columns.usage} onClick={() => toggleSort("usedByCount")} />
            <SortTh col="size" currentCol={sortCol} dir={sortDir} label={t.images.columns.size} onClick={() => toggleSort("size")} />
            <SortTh col="created" currentCol={sortCol} dir={sortDir} label={t.images.columns.created} onClick={() => toggleSort("created")} />
            <th className="px-4 py-2.5 text-left text-xs font-medium uppercase tracking-wider" style={{ color: "var(--color-text-muted)" }}>{t.images.columns.actions}</th>
          </tr></thead>
          <tbody>
            {filteredRows.length === 0 ? <tr><td colSpan={selectionMode ? 8 : 7} className="px-4 py-12 text-center" style={{ color: "var(--color-text-muted)" }}>{t.images.empty}</td></tr> :
              filteredRows.map((img) => (
                <tr key={img.id} className="border-t" style={{ borderColor: "var(--color-border)" }}>
                  {selectionMode && (
                    <td className="px-4 py-2.5">
                      <input type="checkbox" checked={selectedIds.includes(img.id)} onChange={() => toggleSelected(img.id)} />
                    </td>
                  )}
                  <td className="px-4 py-2.5"><span className="font-medium" style={{ color: "var(--color-text)" }}>{img.repo_name || <span style={{ color: "var(--color-text-muted)" }}>{t.images.none}</span>}</span></td>
                  <td className="px-4 py-2.5"><span className="px-1.5 py-0.5 rounded text-xs font-mono" style={{ backgroundColor: "var(--color-surface-secondary)" }}>{img.tag || t.common.notAvailable}</span></td>
                  <td className="px-4 py-2.5"><span className="font-mono text-xs">{shortId(img.id)}</span></td>
                  <td className="px-4 py-2.5">
                    <span className="text-xs" style={{ color: img.usedByCount > 0 ? "var(--color-success)" : "var(--color-text-muted)" }}>
                      {img.usedByCount > 0 ? t.images.inUse(img.usedByCount) : t.images.unused}
                    </span>
                  </td>
                  <td className="px-4 py-2.5"><span className="text-xs">{img.size}</span></td>
                  <td className="px-4 py-2.5"><span className="text-xs">{img.created}</span></td>
                  <td className="px-4 py-2.5"><button onClick={() => setConfirmRemoveIds([img.id])} className="px-2 py-1 text-xs font-medium rounded border hover:opacity-80" style={{ borderColor: "var(--color-danger)", color: "var(--color-danger)" }}>{t.common.remove}</button></td>
                </tr>
              ))}
          </tbody>
        </table>
      </div>

      {showPull && (
        <div className="fixed inset-0 flex items-center justify-center z-50" style={{ backgroundColor: "rgba(0,0,0,0.5)" }}>
          <div className="rounded-lg p-6 max-w-md w-full mx-4 shadow-xl max-h-[80vh] flex flex-col" style={{ backgroundColor: "var(--color-surface)" }}>
            <h3 className="text-lg font-semibold mb-4" style={{ color: "var(--color-text)" }}>{t.images.pullDialog.title}</h3>
            <div className="space-y-3 mb-4">
              <div>
                <label className="block text-xs font-medium mb-1" style={{ color: "var(--color-text-muted)" }}>{t.images.pullDialog.imageName}</label>
                <input value={pullName} onChange={(e) => setPullName(e.target.value)} placeholder={t.images.pullDialog.imageNamePlaceholder} className="w-full px-3 py-2 text-sm rounded-md border" disabled={pulling} style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface-secondary)", color: "var(--color-text)" }} />
              </div>
              <div>
                <label className="block text-xs font-medium mb-1" style={{ color: "var(--color-text-muted)" }}>{t.images.pullDialog.tag}</label>
                <input value={pullTag} onChange={(e) => setPullTag(e.target.value)} placeholder={t.images.pullDialog.tagPlaceholder} className="w-full px-3 py-2 text-sm rounded-md border" disabled={pulling} style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface-secondary)", color: "var(--color-text)" }} />
              </div>
            </div>
            {pullProgress.length > 0 && (
              <div className="flex-1 overflow-auto mb-4 p-3 rounded-md font-mono text-xs max-h-48" style={{ backgroundColor: "var(--color-surface-secondary)" }}>
                {pullProgress.map((line, i) => <div key={i} style={{ color: "var(--color-text-muted)" }}>{line}</div>)}
              </div>
            )}
            <div className="flex justify-end gap-2">
              <button onClick={() => { setShowPull(false); setPullProgress([]); }} disabled={pulling} className="px-4 py-2 text-sm rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>{t.common.cancel}</button>
              <button onClick={doPull} disabled={pulling || !pullName.trim()} className="px-4 py-2 text-sm rounded-md text-white disabled:opacity-50" style={{ backgroundColor: "var(--color-accent)" }}>{pulling ? t.images.pullDialog.pulling : t.images.pullDialog.pull}</button>
            </div>
          </div>
        </div>
      )}

      {confirmRemoveIds && (
        <div className="fixed inset-0 flex items-center justify-center z-50" style={{ backgroundColor: "rgba(0,0,0,0.5)" }}>
          <div className="rounded-lg p-6 max-w-sm w-full mx-4 shadow-xl" style={{ backgroundColor: "var(--color-surface)" }}>
            <h3 className="text-lg font-semibold mb-2" style={{ color: "var(--color-text)" }}>
              {confirmRemoveIds.length === 1 ? t.images.confirmRemove.title : t.images.confirmRemove.bulkTitle}
            </h3>
            <p className="text-sm mb-4" style={{ color: "var(--color-text-muted)" }}>
              {confirmRemoveIds.length === 1 && confirmSingleName
                ? t.images.confirmRemove.message(confirmSingleName)
                : t.images.confirmRemove.bulkMessage(confirmRemoveIds.length)}
            </p>
            <div className="flex justify-end gap-2">
              <button onClick={() => setConfirmRemoveIds(null)} className="px-4 py-2 text-sm rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>{t.common.cancel}</button>
              <button onClick={() => void doRemove(confirmRemoveIds)} disabled={removing} className="px-4 py-2 text-sm rounded-md text-white disabled:opacity-50" style={{ backgroundColor: "var(--color-danger)" }}>{removing ? t.common.loading : t.common.remove}</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function compareRows(left: ImageRow, right: ImageRow, sortCol: SortCol, sortDir: SortDir): number {
  const direction = sortDir === "asc" ? 1 : -1;
  const value = (() => {
    switch (sortCol) {
      case "usedByCount":
        return left.usedByCount - right.usedByCount;
      default:
        return String(left[sortCol] ?? "").localeCompare(String(right[sortCol] ?? ""));
    }
  })();

  if (value !== 0) {
    return value * direction;
  }

  return left.repo_name.localeCompare(right.repo_name) * direction;
}

function containerUsesImage(container: Container, image: Image): boolean {
  const imageRef = container.image;
  const repoTag = image.repo_name && image.tag ? `${image.repo_name}:${image.tag}` : null;
  const short = image.id.startsWith("sha256:") ? image.id.substring(7, 19) : image.id.substring(0, 12);

  return imageRef === image.id
    || imageRef === short
    || imageRef === image.repo_name
    || (repoTag !== null && imageRef === repoTag);
}
