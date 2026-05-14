import { useEffect, useState, useCallback, useRef } from "react";
import type { Container, ContainerStats } from "../lib/types";
import {
  listContainers, startContainer, stopContainer, restartContainer, removeContainer,
  containerLogs, onContainerLogLine, onContainerLogStatus, containerStats,
  execCreate, execStart, execInput, execDisconnect, onExecOutput, onExecStatus,
} from "../lib/tauri";
import { useI18n } from "../i18n";

type SortDir = "asc" | "desc";
type TabId = "logs" | "terminal" | "stats";
const MAX_LOG_LINES = 600;
const MAX_TERMINAL_LINES = 600;
const TERMINAL_SHELLS = [
  "sh",
  "bash",
  "zsh",
  "ash",
  "dash",
  "pwsh",
  "powershell.exe",
  "cmd.exe",
] as const;

function appendCappedText(current: string, addition: string, maxLines: number) {
  const combined = `${current}${addition}`;
  const lines = combined.split("\n");
  if (lines.length <= maxLines) return combined;
  return lines.slice(lines.length - maxLines).join("\n");
}

function shellPrompt(shell: string, isRoot: boolean) {
  if (isRoot) return "#";
  const normalized = shell.trim().toLowerCase();
  if (normalized === "cmd" || normalized === "cmd.exe") return ">";
  if (normalized === "powershell" || normalized === "powershell.exe" || normalized === "pwsh" || normalized === "pwsh.exe") {
    return "PS>";
  }
  return "$";
}

function buildExecCommand(shell: string, mode: "interactive" | "command", rawCommand: string) {
  if (mode === "interactive") {
    return [shell];
  }

  const command = rawCommand.trim();
  if (!command) {
    return [shell];
  }

  const normalized = shell.trim().toLowerCase();
  if (normalized === "cmd" || normalized === "cmd.exe") {
    return [shell, "/C", command];
  }
  if (normalized === "powershell" || normalized === "powershell.exe" || normalized === "pwsh" || normalized === "pwsh.exe") {
    return [shell, "-Command", command];
  }
  return [shell, "-c", command];
}

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

export function ContainersScreen() {
  const { t } = useI18n();
  const [containers, setContainers] = useState<Container[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selected, setSelected] = useState<string | null>(null);
  const [confirmRemove, setConfirmRemove] = useState<string | null>(null);
  const [actionLoading, setActionLoading] = useState<string | null>(null);

  // Tabs
  const [tab, setTab] = useState<TabId>("logs");

  // Logs state
  const [logLines, setLogLines] = useState("");
  const [logTail, setLogTail] = useState("100");
  const [logSince, setLogSince] = useState("");
  const [logUntil, setLogUntil] = useState("");
  const [logFollow, setLogFollow] = useState(false);
  const [logLoading, setLogLoading] = useState(false);
  const logUnlisten = useRef<(() => void) | null>(null);
  const logStatusUnlisten = useRef<(() => void) | null>(null);

  // Terminal state
  const [termShell, setTermShell] = useState("sh");
  const [termRoot, setTermRoot] = useState(false);
  const [termMode, setTermMode] = useState<"interactive" | "command">("interactive");
  const [termCmd, setTermCmd] = useState("");
  const [termOutput, setTermOutput] = useState("");
  const [termInput, setTermInput] = useState("");
  const [termConnected, setTermConnected] = useState(false);
  const [termConnecting, setTermConnecting] = useState(false);
  const [termExecId, setTermExecId] = useState<string | null>(null);
  const [termSessionRoot, setTermSessionRoot] = useState(false);
  const [termSessionShell, setTermSessionShell] = useState("sh");
  const [termSessionMode, setTermSessionMode] = useState<"interactive" | "command" | null>(null);
  const [termCanCopy, setTermCanCopy] = useState(false);
  const termUnlisten = useRef<(() => void) | null>(null);
  const termStatusUnlisten = useRef<(() => void) | null>(null);

  // Stats state
  const [stats, setStats] = useState<ContainerStats | null>(null);
  const [statsLoading, setStatsLoading] = useState(false);

  const { sorted, col, dir, toggle } = useSort(containers, "name");

  const load = useCallback(async () => {
    setLoading(true); setError(null);
    try { setContainers(await listContainers(true)); }
    catch (e) { setError(String(e)); }
    finally { setLoading(false); }
  }, []);
  useEffect(() => { load(); }, [load]);

  // Cleanup listeners
  useEffect(() => {
    return () => {
      logUnlisten.current?.();
      logStatusUnlisten.current?.();
      termUnlisten.current?.();
      termStatusUnlisten.current?.();
    };
  }, []);

  useEffect(() => {
    setTermConnected(false);
    setTermConnecting(false);
    setTermExecId(null);
    setTermSessionRoot(false);
    setTermSessionShell("sh");
    setTermSessionMode(null);
    setTermOutput("");
    setTermInput("");
    setTermCanCopy(false);
    termUnlisten.current?.();
    termUnlisten.current = null;
    termStatusUnlisten.current?.();
    termStatusUnlisten.current = null;
  }, [selected]);

  async function doAction(id: string, name: string, action: string) {
    setActionLoading(id);
    try {
      if (action === "start") await startContainer(id);
      else if (action === "stop") await stopContainer(id);
      else if (action === "restart") await restartContainer(id);
      await load();
    } catch (e) { setError(`${name}: ${e}`); }
    finally { setActionLoading(null); }
  }

  async function doRemove(id: string) {
    setActionLoading(id);
    try { await removeContainer(id); setConfirmRemove(null); await load(); }
    catch (e) { setError(String(e)); }
    finally { setActionLoading(null); }
  }

  // ── Logs ──
  async function loadLogs() {
    if (!selected) return;
    logUnlisten.current?.();
    logStatusUnlisten.current?.();
    setLogLines(""); setLogLoading(true);
    const requestId = crypto.randomUUID();
    const tailNum = parseInt(logTail) || 100;
    const sinceTs = logSince ? Math.floor(new Date(logSince).getTime() / 1000) : null;
    const untilTs = logUntil ? Math.floor(new Date(logUntil).getTime() / 1000) : null;
    const unlisten = await onContainerLogLine((event) => {
      if (event.requestId !== requestId) return;
      setLogLines((prev) => appendCappedText(prev, `[${event.line.stream}] ${event.line.content}\n`, MAX_LOG_LINES));
    });
    const statusUnlisten = await onContainerLogStatus((event) => {
      if (event.requestId !== requestId) return;
      if (event.status === "failed") setError(event.error ?? "Container logs failed");
      if (event.status !== "started") setLogLoading(false);
    });
    logUnlisten.current = unlisten;
    logStatusUnlisten.current = statusUnlisten;
    try {
      await containerLogs(selected, {
        tail: tailNum,
        follow: logFollow,
        since: sinceTs,
        until: untilTs,
        requestId,
      });
    } catch (e) { setError(String(e)); }
    finally { if (logFollow) setLogLoading(false); }
  }

  // ── Terminal ──
  async function connectTerminal() {
    if (!selected || termConnecting || termConnected) return;
    termUnlisten.current?.();
    termStatusUnlisten.current?.();
    setTermOutput(""); setTermConnected(false); setTermConnecting(true); setTermCanCopy(false);
    const requestId = crypto.randomUUID();
    const execCommand = buildExecCommand(termShell, termMode, termCmd);
    const cmd: string[] = [];
    if (termRoot) cmd.push("-u", "root");
    cmd.push(selected);
    cmd.push(...execCommand);
    // Use docker exec via command for simplicity
    setTermOutput((prev) => appendCappedText(prev, `${shellPrompt(termShell, termRoot)} docker exec ${cmd.join(" ")}\n`, MAX_TERMINAL_LINES));

    const unlisten = await onExecOutput((event) => {
      if (event.requestId !== requestId) return;
      if (event.text.trim().length > 0) {
        setTermCanCopy(true);
      }
      setTermOutput((prev) => appendCappedText(prev, event.text, MAX_TERMINAL_LINES));
    });
    const statusUnlisten = await onExecStatus((event) => {
      if (event.requestId !== requestId) return;
      if (event.status === "started") {
        setTermConnecting(false);
        setTermConnected(true);
        return;
      }
      if (event.status === "failed") {
        setTermConnecting(false);
        setTermConnected(false);
        setTermExecId(null);
        setTermSessionShell("sh");
        setTermSessionMode(null);
        setTermOutput((prev) => appendCappedText(prev, `\nError: ${event.error ?? "Exec failed"}\n`, MAX_TERMINAL_LINES));
      }
      if (event.status === "completed") {
        setTermConnecting(false);
        setTermConnected(false);
        setTermExecId(null);
        setTermSessionShell("sh");
        setTermSessionMode(null);
      }
    });
    termUnlisten.current = unlisten;
    termStatusUnlisten.current = statusUnlisten;

    try {
      const execId = await execCreate(selected, execCommand, termRoot ? "root" : null);
      setTermExecId(execId);
      setTermSessionRoot(termRoot);
      setTermSessionShell(termShell);
      setTermSessionMode(termMode);
      await execStart(execId, requestId);
    } catch (e) {
      setTermConnecting(false);
      setTermExecId(null);
      setTermSessionShell("sh");
      setTermSessionMode(null);
      setTermCanCopy(false);
      setTermOutput((prev) => appendCappedText(prev, `Error: ${e}\n`, MAX_TERMINAL_LINES));
    }
  }

  async function disconnectTerminal() {
    const execId = termExecId;
    termUnlisten.current?.();
    termUnlisten.current = null;
    termStatusUnlisten.current?.();
    termStatusUnlisten.current = null;
    setTermConnected(false);
    setTermConnecting(false);
    setTermExecId(null);
    setTermSessionShell("sh");
    setTermSessionMode(null);
    setTermCanCopy(termOutput.trim().length > 0);
    if (execId) {
      try { await execDisconnect(execId); }
      catch (e) { setError(String(e)); }
    }
  }

  async function sendTermInput() {
    if (!termExecId || !termInput.trim()) return;
    const data = new TextEncoder().encode(termInput + "\n");
    setTermOutput((prev) => appendCappedText(prev, `${shellPrompt(termSessionShell, termSessionRoot)} ${termInput}\n`, MAX_TERMINAL_LINES));
    setTermInput("");
    try { await execInput(termExecId, Array.from(data)); }
    catch (e) { setError(String(e)); }
  }

  async function copyTerminalOutput() {
    if (!termCanCopy || !termOutput.trim()) return;
    try {
      await navigator.clipboard.writeText(termOutput);
    } catch (e) {
      setError(String(e));
    }
  }

  // ── Stats ──
  async function loadStats() {
    if (!selected) return;
    setStatsLoading(true);
    try { setStats(await containerStats(selected)); }
    catch (e) { setError(String(e)); }
    finally { setStatsLoading(false); }
  }

  const stateColor = (state: string) => {
    switch (state) { case "Running": return "var(--color-success)"; case "Exited": return "var(--color-danger)"; case "Paused": return "var(--color-warning)"; default: return "var(--color-text-muted)"; }
  };
  const shortId = (id: string) => id.substring(0, 12);
  const terminalLocked = termConnecting || termConnected;
  const terminalPrompt = shellPrompt(termSessionShell, termSessionRoot);

  if (loading) return <div className="flex items-center justify-center h-full"><div className="w-6 h-6 border-2 border-t-transparent rounded-full animate-spin" style={{ borderColor: "var(--color-accent)", borderTopColor: "transparent" }} /></div>;

  const selContainer = containers.find((c) => c.id === selected);

  return (
    <div className="p-3 h-full flex flex-col gap-3">
      {/* Header */}
      <div className="flex items-center justify-between shrink-0">
        <h1 className="text-lg font-semibold" style={{ color: "var(--color-text)" }}>{t.containers.title}</h1>
        <div className="flex items-center gap-2">
          <span className="text-xs" style={{ color: "var(--color-text-muted)" }}>{t.containers.count(containers.length)}</span>
          <button onClick={load} className="px-2 py-1 text-xs rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>{t.common.refresh}</button>
        </div>
      </div>
      {error && <div className="px-3 py-1.5 text-xs rounded-md shrink-0" style={{ backgroundColor: "rgba(239,68,68,0.1)", color: "var(--color-danger)" }}>{error} <button onClick={() => setError(null)} className="ml-2 underline">{t.common.dismiss}</button></div>}

      {/* Table */}
      <div className="flex-1 min-h-0 flex flex-col">
        <div className="flex-1 min-h-[200px] overflow-auto rounded-lg border" style={{ borderColor: "var(--color-border)" }}>
          <table className="w-full text-sm">
            <thead><tr style={{ backgroundColor: "var(--color-surface-secondary)" }}>
              <SortTh col="name" currentCol={col as string} dir={dir} label={t.containers.columns.name} onClick={() => toggle("name")} />
              <SortTh col="image" currentCol={col as string} dir={dir} label={t.containers.columns.image} onClick={() => toggle("image")} />
              <SortTh col="state" currentCol={col as string} dir={dir} label={t.containers.columns.state} onClick={() => toggle("state")} />
              <th className="px-4 py-2.5 text-left text-xs font-medium uppercase tracking-wider" style={{ color: "var(--color-text-muted)" }}>{t.containers.columns.ports}</th>
              <SortTh col="created" currentCol={col as string} dir={dir} label={t.containers.columns.created} onClick={() => toggle("created")} />
              <th className="px-4 py-2.5 text-left text-xs font-medium uppercase tracking-wider" style={{ color: "var(--color-text-muted)" }}>{t.containers.columns.actions}</th>
            </tr></thead>
            <tbody>
              {sorted.length === 0 ? <tr><td colSpan={6} className="px-4 py-12 text-center" style={{ color: "var(--color-text-muted)" }}>{t.containers.empty}</td></tr> :
                sorted.map((c) => (
                  <tr key={c.id} onClick={() => setSelected(c.id === selected ? null : c.id)}
                    className="border-t cursor-pointer hover:opacity-80"
                    style={{ borderColor: "var(--color-border)", backgroundColor: selected === c.id ? "color-mix(in srgb, var(--color-accent) 10%, transparent)" : "transparent" }}>
                    <td className="px-4 py-2"><div className="font-medium" style={{ color: "var(--color-text)" }}>{c.name}</div><div className="text-xs font-mono" style={{ color: "var(--color-text-muted)" }}>{shortId(c.id)}</div></td>
                    <td className="px-4 py-2"><span className="font-mono text-xs">{c.image}</span></td>
                    <td className="px-4 py-2"><span className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded-full text-xs font-medium" style={{ backgroundColor: `${stateColor(c.state)}20`, color: stateColor(c.state) }}><span className="w-1.5 h-1.5 rounded-full" style={{ backgroundColor: stateColor(c.state) }} />{t.containers.states[c.state]}</span><div className="text-xs mt-0.5" style={{ color: "var(--color-text-muted)" }}>{c.status}</div></td>
                    <td className="px-4 py-2"><div className="font-mono text-xs">{c.ports.length > 0 ? c.ports.map((p, i) => <div key={i}>{p.host_ip}:{p.host_port}→{p.container_port}</div>) : <span style={{ color: "var(--color-text-muted)" }}>{t.common.notAvailable}</span>}</div></td>
                    <td className="px-4 py-2"><span className="text-xs">{c.created}</span></td>
                    <td className="px-4 py-2"><div className="flex items-center gap-1" onClick={(e) => e.stopPropagation()}>
                      {(c.state === "Exited" || c.state === "Created") && <Btn label={t.containers.actions.start} color="var(--color-success)" loading={actionLoading === c.id} onClick={() => doAction(c.id, c.name, "start")} />}
                      {c.state === "Running" && <><Btn label={t.containers.actions.stop} color="var(--color-warning)" loading={actionLoading === c.id} onClick={() => doAction(c.id, c.name, "stop")} /><Btn label={t.containers.actions.restart} color="var(--color-accent)" loading={actionLoading === c.id} onClick={() => doAction(c.id, c.name, "restart")} /></>}
                      {c.state !== "Removing" && <Btn label={t.containers.actions.remove} color="var(--color-danger)" loading={false} onClick={() => setConfirmRemove(c.id)} />}
                    </div></td>
                  </tr>
                ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Detail panel */}
      {selected && (
        <div className="shrink-0 rounded-lg border" style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface-secondary)", maxHeight: "40vh" }}>
          {/* Tabs */}
          <div className="flex items-center border-b px-2" style={{ borderColor: "var(--color-border)" }}>
            {(["logs", "terminal", "stats"] as TabId[]).map((t) => (
              <button key={t} onClick={() => setTab(t)}
                className="px-3 py-2 text-xs font-medium capitalize border-b-2 transition-colors"
                style={{ borderColor: tab === t ? "var(--color-accent)" : "transparent", color: tab === t ? "var(--color-accent)" : "var(--color-text-muted)" }}>
                {selContainer?.name ?? shortId(selected ?? "")} — {tabLabel(t)}
              </button>
            ))}
          </div>

          {/* Logs tab */}
          {tab === "logs" && (
            <div className="p-2 flex flex-col" style={{ maxHeight: "calc(40vh - 40px)" }}>
              <div className="flex items-center gap-2 mb-2 flex-wrap">
                <input value={logTail} onChange={(e) => setLogTail(e.target.value.replace(/\D/g, ""))} placeholder={t.containers.logs.tailPlaceholder} className="w-16 px-2 py-1 text-xs rounded border" style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface)", color: "var(--color-text)" }} />
                <input type="datetime-local" value={logSince} onChange={(e) => setLogSince(e.target.value)} className="px-2 py-1 text-xs rounded border" style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface)", color: "var(--color-text)" }} />
                <input type="datetime-local" value={logUntil} onChange={(e) => setLogUntil(e.target.value)} className="px-2 py-1 text-xs rounded border" style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface)", color: "var(--color-text)" }} />
                <label className="flex items-center gap-1 text-xs" style={{ color: "var(--color-text-muted)" }}><input type="checkbox" checked={logFollow} onChange={(e) => setLogFollow(e.target.checked)} /> {t.containers.logs.follow}</label>
                <button onClick={loadLogs} disabled={logLoading} className="px-2 py-1 text-xs rounded text-white" style={{ backgroundColor: "var(--color-accent)" }}>{logLoading ? t.common.loading : t.common.load}</button>
              </div>
              <div className="flex-1 overflow-auto font-mono text-xs rounded p-2 whitespace-pre-wrap" style={{ backgroundColor: "#0d1117", color: "#c9d1d9", minHeight: "60px" }}>
                {logLines || <span style={{ color: "#8b949e" }}>{t.containers.logs.empty}</span>}
              </div>
            </div>
          )}

          {/* Terminal tab */}
          {tab === "terminal" && (
            <div className="p-2 flex flex-col" style={{ maxHeight: "calc(40vh - 40px)" }}>
              <div className="flex items-center gap-2 mb-2 flex-wrap">
                <select disabled={terminalLocked} value={termShell} onChange={(e) => setTermShell(e.target.value)} className="px-2 py-1 text-xs rounded border disabled:opacity-60" style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface)", color: "var(--color-text)" }}>
                  {TERMINAL_SHELLS.map((s) => <option key={s} value={s}>{s}</option>)}
                </select>
                <label className="flex items-center gap-1 text-xs" style={{ color: "var(--color-text-muted)" }}><input disabled={terminalLocked} type="checkbox" checked={termRoot} onChange={(e) => setTermRoot(e.target.checked)} /> {t.containers.terminal.root}</label>
                <label className="flex items-center gap-1 text-xs" style={{ color: "var(--color-text-muted)" }}><input disabled={terminalLocked} type="radio" name="mode" checked={termMode === "interactive"} onChange={() => setTermMode("interactive")} /> {t.containers.terminal.interactive}</label>
                <label className="flex items-center gap-1 text-xs" style={{ color: "var(--color-text-muted)" }}><input disabled={terminalLocked} type="radio" name="mode" checked={termMode === "command"} onChange={() => setTermMode("command")} /> {t.containers.terminal.command}</label>
                {termMode === "command" && <input disabled={terminalLocked} value={termCmd} onChange={(e) => setTermCmd(e.target.value)} placeholder={t.containers.terminal.commandPlaceholder} className="px-2 py-1 text-xs rounded border flex-1 disabled:opacity-60" style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface)", color: "var(--color-text)" }} />}
                {termCanCopy && <button onClick={() => { void copyTerminalOutput(); }} className="px-2 py-1 text-xs rounded border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>{t.containers.terminal.copy}</button>}
                <button onClick={() => { void (termConnected ? disconnectTerminal() : connectTerminal()); }} disabled={termConnecting} className="px-2 py-1 text-xs rounded text-white disabled:opacity-60" style={{ backgroundColor: termConnected ? "var(--color-danger)" : "var(--color-accent)" }}>{termConnected ? t.common.disconnect : termConnecting ? t.common.loading : t.common.connect}</button>
              </div>
              <div className="flex-1 overflow-auto font-mono text-xs rounded p-2 whitespace-pre-wrap" style={{ backgroundColor: "#0d1117", color: "#c9d1d9", minHeight: "60px" }}>
                {termOutput || <span style={{ color: "#8b949e" }}>{t.containers.terminal.empty}</span>}
              </div>
              {termConnected && termSessionMode === "interactive" && (
                <div className="flex items-center gap-2 mt-1">
                  <span style={{ color: "var(--color-accent)" }}>{terminalPrompt}</span>
                  <input value={termInput} onChange={(e) => setTermInput(e.target.value)}
                    onKeyDown={(e) => { if (e.key === "Enter") sendTermInput(); }}
                    placeholder={t.containers.terminal.inputPlaceholder}
                    className="flex-1 px-2 py-1 text-xs rounded border bg-transparent"
                    style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }} />
                </div>
              )}
            </div>
          )}

          {/* Stats tab */}
          {tab === "stats" && (
            <div className="p-3" style={{ maxHeight: "calc(40vh - 40px)", overflow: "auto" }}>
              <div className="flex items-center gap-2 mb-3">
                <button onClick={loadStats} disabled={statsLoading} className="px-2 py-1 text-xs rounded text-white" style={{ backgroundColor: "var(--color-accent)" }}>{statsLoading ? t.common.loading : t.containers.stats.refresh}</button>
              </div>
              {stats && (
                <div className="grid grid-cols-2 md:grid-cols-4 gap-2">
                  <StatCard label={t.containers.stats.cpu} value={`${stats.cpu_percent}%`} color="var(--color-accent)" />
                  <StatCard label={t.containers.stats.memory} value={stats.memory_usage} color="var(--color-success)" />
                  <StatCard label={t.containers.stats.netRx} value={stats.network_rx} color="var(--color-warning)" />
                  <StatCard label={t.containers.stats.netTx} value={stats.network_tx} color="var(--color-warning)" />
                  <StatCard label={t.containers.stats.blockRead} value={stats.block_read} color="var(--color-text-muted)" />
                  <StatCard label={t.containers.stats.blockWrite} value={stats.block_write} color="var(--color-text-muted)" />
                  <StatCard label={t.containers.stats.pids} value={String(stats.pids)} color="var(--color-text-muted)" />
                </div>
              )}
              {!stats && !statsLoading && <p className="text-xs" style={{ color: "var(--color-text-muted)" }}>{t.containers.stats.empty}</p>}
            </div>
          )}
        </div>
      )}

      {/* Confirm remove modal */}
      {confirmRemove && (
        <div className="fixed inset-0 flex items-center justify-center z-50" style={{ backgroundColor: "rgba(0,0,0,0.5)" }}>
          <div className="rounded-lg p-6 max-w-sm w-full mx-4 shadow-xl" style={{ backgroundColor: "var(--color-surface)" }}>
            <h3 className="text-lg font-semibold mb-2" style={{ color: "var(--color-text)" }}>{t.containers.confirmRemove.title}</h3>
            <p className="text-sm mb-4" style={{ color: "var(--color-text-muted)" }}>{t.containers.confirmRemove.message(containers.find((c) => c.id === confirmRemove)?.name ?? shortId(confirmRemove))}</p>
            <div className="flex justify-end gap-2">
              <button onClick={() => setConfirmRemove(null)} className="px-4 py-2 text-sm rounded-md border" style={{ borderColor: "var(--color-border)", color: "var(--color-text)" }}>{t.common.cancel}</button>
              <button onClick={() => doRemove(confirmRemove)} className="px-4 py-2 text-sm rounded-md text-white" style={{ backgroundColor: "var(--color-danger)" }}>{t.common.remove}</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );

  function tabLabel(tabId: TabId): string {
    switch (tabId) {
      case "logs":
        return t.containers.tabs.logs;
      case "terminal":
        return t.containers.tabs.terminal;
      case "stats":
        return t.containers.tabs.stats;
      default:
        return tabId;
    }
  }
}

function Btn({ label, color, loading, onClick }: { label: string; color: string; loading: boolean; onClick: () => void }) {
  return <button onClick={onClick} disabled={loading} className="px-1.5 py-0.5 text-xs font-medium rounded border disabled:opacity-50" style={{ borderColor: color, color }}>{loading ? "..." : label}</button>;
}

function StatCard({ label, value, color }: { label: string; value: string; color: string }) {
  return <div className="rounded border p-2" style={{ borderColor: "var(--color-border)", backgroundColor: "var(--color-surface)" }}>
    <div className="text-xs uppercase tracking-wide" style={{ color: "var(--color-text-muted)" }}>{label}</div>
    <div className="font-semibold text-sm" style={{ color }}>{value}</div>
  </div>;
}
