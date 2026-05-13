import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  DockerInfo, DockerCleanupSummary, Container, Image, Volume, Network,
  ContainerStats, DockerEndpoint, AppSettings, LogStreamEvent,
  ProgressStreamEvent, StreamStatusEvent, TextStreamEvent,
  SecurityOverview, ImageSecurityReport, SecurityTool, SecurityScanProgressEvent,
} from "./types";

// ─── Connection ────────────────────────────────────────────
export async function connect(endpoint: DockerEndpoint): Promise<DockerInfo> {
  return invoke("connect", { endpoint });
}
export async function testConnection(): Promise<DockerInfo> {
  return invoke("test_connection");
}
export async function ping(): Promise<boolean> {
  return invoke("ping");
}
export async function dockerCleanupSummary(): Promise<DockerCleanupSummary> {
  return invoke("docker_cleanup_summary");
}
export async function dockerSystemPrune(): Promise<void> {
  return invoke("docker_system_prune");
}

// ─── Containers ────────────────────────────────────────────
export async function listContainers(all: boolean): Promise<Container[]> {
  return invoke("list_containers", { all });
}
export async function startContainer(id: string): Promise<void> {
  return invoke("start_container", { id });
}
export async function stopContainer(id: string): Promise<void> {
  return invoke("stop_container", { id });
}
export async function restartContainer(id: string): Promise<void> {
  return invoke("restart_container", { id });
}
export async function removeContainer(id: string): Promise<void> {
  return invoke("remove_container", { id });
}
export interface ContainerLogOptions {
  tail: number | null;
  follow: boolean;
  since: number | null;
  until: number | null;
  requestId: string;
}
export async function containerLogs(
  id: string,
  options: ContainerLogOptions,
): Promise<void> {
  return invoke("container_logs", { id, options });
}
export function onContainerLogLine(cb: (event: LogStreamEvent) => void): Promise<UnlistenFn> {
  return listen<LogStreamEvent>("container-log-line", (e) => cb(e.payload));
}
export function onContainerLogStatus(cb: (event: StreamStatusEvent) => void): Promise<UnlistenFn> {
  return listen<StreamStatusEvent>("container-log-status", (e) => cb(e.payload));
}
export async function inspectContainer(id: string): Promise<string> {
  return invoke("inspect_container", { id });
}
export async function containerStats(id: string): Promise<ContainerStats> {
  return invoke("container_stats", { id });
}
export async function execCreate(id: string, cmd: string[], user: string | null): Promise<string> {
  return invoke("exec_create", { id, cmd, user });
}
export async function execStart(execId: string, requestId: string): Promise<void> {
  return invoke("exec_start", { execIdStr: execId, requestId });
}
export async function execInput(execId: string, data: number[]): Promise<void> {
  return invoke("exec_input", { execIdStr: execId, data });
}
export async function execDisconnect(execId: string): Promise<void> {
  return invoke("exec_disconnect", { execIdStr: execId });
}
export async function execResize(execId: string, w: number, h: number): Promise<void> {
  return invoke("exec_resize", { execIdStr: execId, width: w, height: h });
}
export function onExecOutput(cb: (event: TextStreamEvent) => void): Promise<UnlistenFn> {
  return listen<TextStreamEvent>("exec-output", (e) => cb(e.payload));
}
export function onExecStatus(cb: (event: StreamStatusEvent) => void): Promise<UnlistenFn> {
  return listen<StreamStatusEvent>("exec-status", (e) => cb(e.payload));
}

// ─── Images ────────────────────────────────────────────────
export async function listImages(): Promise<Image[]> {
  return invoke("list_images");
}
export async function pullImage(name: string, tag: string | null, requestId: string): Promise<void> {
  return invoke("pull_image", { name, tag, requestId });
}
export function onImagePullProgress(cb: (event: ProgressStreamEvent) => void): Promise<UnlistenFn> {
  return listen<ProgressStreamEvent>("image-pull-progress", (e) => cb(e.payload));
}
export function onImagePullStatus(cb: (event: StreamStatusEvent) => void): Promise<UnlistenFn> {
  return listen<StreamStatusEvent>("image-pull-status", (e) => cb(e.payload));
}
export async function removeImage(id: string): Promise<void> {
  return invoke("remove_image", { id });
}
export async function tagImage(id: string, repo: string, tag: string): Promise<void> {
  return invoke("tag_image", { id, repo, tag });
}
export async function inspectImage(id: string): Promise<string> {
  return invoke("inspect_image", { id });
}

// ─── Security ────────────────────────────────────────────────
export async function securityOverview(): Promise<SecurityOverview> {
  return invoke("security_overview");
}
export async function imageSecurityReport(imageId: string): Promise<ImageSecurityReport> {
  return invoke("image_security_report", { imageId });
}
export async function configureSecurityTools(tools: SecurityTool[]): Promise<SecurityOverview> {
  return invoke("configure_security_tools", { tools });
}
export async function openExternalLink(url: string): Promise<void> {
  return invoke("open_external_link", { url });
}
export function onSecurityScanProgress(cb: (event: SecurityScanProgressEvent) => void): Promise<UnlistenFn> {
  return listen<SecurityScanProgressEvent>("security-scan-progress", (e) => cb(e.payload));
}

// ─── Volumes ───────────────────────────────────────────────
export async function listVolumes(): Promise<Volume[]> {
  return invoke("list_volumes");
}
export async function createVolume(name: string): Promise<Volume> {
  return invoke("create_volume", { name });
}
export async function removeVolume(name: string): Promise<void> {
  return invoke("remove_volume", { name });
}
export async function inspectVolume(name: string): Promise<string> {
  return invoke("inspect_volume", { name });
}

// ─── Networks ──────────────────────────────────────────────
export async function listNetworks(): Promise<Network[]> {
  return invoke("list_networks");
}
export async function createNetwork(name: string, driver: string | null): Promise<string> {
  return invoke("create_network", { name, driver });
}
export async function removeNetwork(id: string): Promise<void> {
  return invoke("remove_network", { id });
}
export async function inspectNetwork(id: string): Promise<string> {
  return invoke("inspect_network", { id });
}

// ─── Compose ───────────────────────────────────────────────
export async function composeUp(filePath: string, requestId: string): Promise<void> {
  return invoke("compose_up", { filePath, requestId });
}
export async function composeDown(filePath: string): Promise<void> {
  return invoke("compose_down", { filePath });
}
export async function composeLogs(filePath: string, requestId: string): Promise<void> {
  return invoke("compose_logs", { filePath, requestId });
}
export function onComposeOutput(cb: (event: LogStreamEvent) => void): Promise<UnlistenFn> {
  return listen<LogStreamEvent>("compose-output", (e) => cb(e.payload));
}
export function onComposeStatus(cb: (event: StreamStatusEvent) => void): Promise<UnlistenFn> {
  return listen<StreamStatusEvent>("compose-status", (e) => cb(e.payload));
}
export async function composePs(filePath: string): Promise<string[]> {
  return invoke("compose_ps", { filePath });
}

// ─── Settings ──────────────────────────────────────────────
export async function loadSettings(): Promise<AppSettings> {
  return invoke("load_settings");
}
export async function saveSettings(settings: AppSettings): Promise<void> {
  return invoke("save_settings", { settings });
}
export async function listFonts(): Promise<string[]> {
  return invoke("list_fonts");
}

// ─── Events ────────────────────────────────────────────────
export function onDockerConnected(cb: (info: DockerInfo) => void): Promise<UnlistenFn> {
  return listen<DockerInfo>("docker-connected", (e) => cb(e.payload));
}
export function onDockerError(cb: (error: string) => void): Promise<UnlistenFn> {
  return listen<string>("docker-error", (e) => cb(e.payload));
}
