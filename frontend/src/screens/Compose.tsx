import { useState } from "react";
import { composeUp, composeDown, onComposeOutput } from "../lib/tauri";
import type { LogLine } from "../lib/types";

export function ComposeScreen() {
  const [filePath, setFilePath] = useState("");
  const [output, setOutput] = useState<LogLine[]>([]);
  const [running, setRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function doUp() {
    if (!filePath.trim()) return;
    setRunning(true);
    setOutput([]);
    setError(null);
    const unlisten = await onComposeOutput((line) => {
      setOutput((prev) => [...prev, line]);
    });
    try {
      await composeUp(filePath.trim());
    } catch (e) {
      setError(String(e));
    } finally {
      setRunning(false);
      unlisten();
    }
  }

  async function doDown() {
    if (!filePath.trim()) return;
    setRunning(true);
    setError(null);
    try {
      await composeDown(filePath.trim());
      setOutput((prev) => [...prev, { stream: "Stdout", content: "Compose down completed.", timestamp: null }]);
    } catch (e) {
      setError(String(e));
    } finally {
      setRunning(false);
    }
  }

  return (
    <div className="p-6 h-full flex flex-col">
      <h1 className="text-xl font-semibold mb-4" style={{ color: "var(--color-text)" }}>Docker Compose</h1>

      <div className="flex items-end gap-3 mb-4">
        <div className="flex-1">
          <label className="block text-xs font-medium mb-1" style={{ color: "var(--color-text-muted)" }}>
            Compose File Path
          </label>
          <input
            value={filePath}
            onChange={(e) => setFilePath(e.target.value)}
            placeholder="/path/to/docker-compose.yml"
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
          onClick={doUp}
          disabled={running || !filePath.trim()}
          className="px-4 py-2 text-sm rounded-md text-white disabled:opacity-50"
          style={{ backgroundColor: "var(--color-success)" }}>
          {running ? "..." : "Up"}
        </button>
        <button
          onClick={doDown}
          disabled={running || !filePath.trim()}
          className="px-4 py-2 text-sm rounded-md text-white disabled:opacity-50"
          style={{ backgroundColor: "var(--color-danger)" }}>
          Down
        </button>
      </div>

      {error && (
        <div className="mb-3 px-3 py-2 text-sm rounded-md"
          style={{ backgroundColor: "rgba(239,68,68,0.1)", color: "var(--color-danger)" }}>
          {error} <button onClick={() => setError(null)} className="ml-2 underline">Dismiss</button>
        </div>
      )}

      <div className="flex-1 overflow-auto rounded-lg border p-4 font-mono text-xs"
        style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface-secondary)" }}>
        {output.length === 0 ? (
          <div style={{ color: "var(--color-text-muted)" }}>
            Enter a compose file path and click Up to start.
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
