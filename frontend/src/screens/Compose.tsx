import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { composeUp, composeDown, onComposeOutput, onComposeStatus } from "../lib/tauri";
import type { LogLine } from "../lib/types";
import { useI18n } from "../i18n";

const MAX_COMPOSE_LINES = 500;

export function ComposeScreen() {
  const { t } = useI18n();
  const [filePath, setFilePath] = useState("");
  const [output, setOutput] = useState<LogLine[]>([]);
  const [running, setRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function doUp() {
    if (!filePath.trim()) return;
    const requestId = crypto.randomUUID();
    setRunning(true);
    setOutput([]);
    setError(null);
    let resolveStatus!: () => void;
    let rejectStatus!: (error: Error) => void;
    const statusPromise = new Promise<void>((resolve, reject) => {
      resolveStatus = resolve;
      rejectStatus = reject;
    });
    const [unlistenOutput, unlistenStatus] = await Promise.all([
      onComposeOutput((event) => {
        if (event.requestId !== requestId) return;
        setOutput((prev) => [...prev, event.line].slice(-MAX_COMPOSE_LINES));
      }),
      onComposeStatus((event) => {
        if (event.requestId !== requestId) return;
        if (event.status === "completed") resolveStatus();
        if (event.status === "failed") rejectStatus(new Error(event.error ?? "Compose failed"));
      }),
    ]);
    try {
      await composeUp(filePath.trim(), requestId);
      await statusPromise;
    } catch (e) {
      setError(String(e));
    } finally {
      setRunning(false);
      unlistenOutput();
      unlistenStatus();
    }
  }

  async function doDown() {
    if (!filePath.trim()) return;
    setRunning(true);
    setError(null);
    try {
      await composeDown(filePath.trim());
      setOutput((prev) => [...prev, { stream: "Stdout", content: t.compose.downCompleted, timestamp: null }]);
    } catch (e) {
      setError(String(e));
    } finally {
      setRunning(false);
    }
  }

  async function chooseComposeFile() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "Compose",
          extensions: ["yml", "yaml"],
        },
      ],
    });

    if (typeof selected === "string") {
      setFilePath(selected);
      setError(null);
    }
  }

  return (
    <div className="p-6 h-full flex flex-col">
      <h1 className="text-xl font-semibold mb-4" style={{ color: "var(--color-text)" }}>{t.compose.title}</h1>

      <div className="flex items-end gap-3 mb-4">
        <div className="flex-1">
          <label className="block text-xs font-medium mb-1" style={{ color: "var(--color-text-muted)" }}>
            {t.compose.filePathLabel}
          </label>
          <input
            value={filePath}
            onChange={(e) => setFilePath(e.target.value)}
            placeholder={t.compose.filePathPlaceholder}
            className="w-full px-3 py-2 text-sm rounded-md border font-mono"
            disabled={running}
            style={{
              borderColor: "var(--color-border)",
              backgroundColor: "var(--color-surface-secondary)",
              color: "var(--color-text)",
            }}
          />
        </div>
        <button
          onClick={() => void chooseComposeFile()}
          disabled={running}
          className="px-4 py-2 text-sm rounded-md border disabled:opacity-50"
          style={{
            borderColor: "var(--color-border)",
            backgroundColor: "var(--color-surface-secondary)",
            color: "var(--color-text)",
          }}
        >
          {t.compose.browse}
        </button>
        <button
          onClick={doUp}
          disabled={running || !filePath.trim()}
          className="px-4 py-2 text-sm rounded-md text-white disabled:opacity-50"
          style={{ backgroundColor: "var(--color-success)" }}>
          {running ? "..." : t.compose.up}
        </button>
        <button
          onClick={doDown}
          disabled={running || !filePath.trim()}
          className="px-4 py-2 text-sm rounded-md text-white disabled:opacity-50"
          style={{ backgroundColor: "var(--color-danger)" }}>
          {t.compose.down}
        </button>
      </div>

      {error && (
        <div className="mb-3 px-3 py-2 text-sm rounded-md"
          style={{ backgroundColor: "rgba(239,68,68,0.1)", color: "var(--color-danger)" }}>
          {error} <button onClick={() => setError(null)} className="ml-2 underline">{t.common.dismiss}</button>
        </div>
      )}

      <div className="flex-1 overflow-auto rounded-lg border p-4 font-mono text-xs"
        style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface-secondary)" }}>
        {output.length === 0 ? (
          <div style={{ color: "var(--color-text-muted)" }}>
            {t.compose.emptyState}
          </div>
        ) : (
          output.map((line, i) => (
            <div key={i} style={{
              color: line.stream === "Stderr" ? "var(--color-danger)" : "var(--color-text)",
            }}>
              {line.content}
            </div>
          ))
        )}
      </div>
    </div>
  );
}
