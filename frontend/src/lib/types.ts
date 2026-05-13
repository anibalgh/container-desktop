// TypeScript types matching domain entities from the Rust backend.
// These mirror the structures in crates/domain/src/entities/

export interface DockerInfo {
  server_version: string;
  containers_running: number;
  containers_paused: number;
  containers_stopped: number;
  images: number;
  os_type: string;
  architecture: string;
  endpoint: string;
}

export interface Container {
  id: string;
  name: string;
  image: string;
  status: string;
  state: ContainerState;
  ports: PortMapping[];
  mounts: Mount[];
  created: string;
  command: string;
}

export type ContainerState =
  | "Running"
  | "Exited"
  | "Paused"
  | "Restarting"
  | "Created"
  | "Removing"
  | "Dead";

export interface PortMapping {
  host_ip: string;
  host_port: string;
  container_port: string;
  protocol: string;
}

export interface Mount {
  source: string;
  destination: string;
  mount_type: string;
  read_only: boolean;
}

export interface Image {
  id: string;
  repo_name: string;
  tag: string;
  size: string;
  created: string;
  labels: string[];
}

export type SecurityTool = "Grype" | "Trivy" | "DockerScout";

export interface SecuritySettings {
  selected_tools: SecurityTool[];
}

export type VulnerabilitySeverity =
  | "Critical"
  | "High"
  | "Medium"
  | "Low"
  | "Negligible"
  | "Unknown";

export interface SeverityCount {
  severity: VulnerabilitySeverity;
  count: number;
}

export type SecurityScanState = "Idle" | "Running" | "Completed" | "Failed";

export interface SecurityInstallHint {
  title: string;
  description: string;
  commands: string[];
  note: string | null;
}

export interface SecurityToolStatus {
  tool: SecurityTool;
  available: boolean;
  selected: boolean;
  version: string | null;
  install_hint: SecurityInstallHint;
}

export interface SecurityReferenceLink {
  label: string;
  url: string;
}

export interface SecurityFinding {
  vulnerability_id: string;
  package_name: string;
  installed_version: string;
  severity: VulnerabilitySeverity;
  title: string | null;
  description: string | null;
  fixed_version: string | null;
  references: SecurityReferenceLink[];
  source_tool: SecurityTool;
}

export interface SecurityImageToolStatus {
  tool: SecurityTool;
  state: SecurityScanState;
  available: boolean;
  selected: boolean;
  findings_count: number;
  last_scanned_at: string | null;
  message: string | null;
}

export interface SecurityImageSummary {
  image_id: string;
  image_name: string;
  repo_name: string;
  tag: string;
  total_findings: number;
  severity_counts: SeverityCount[];
  tool_statuses: SecurityImageToolStatus[];
  last_scanned_at: string | null;
}

export interface SecurityOverview {
  tools: SecurityToolStatus[];
  total_images: number;
  scanned_images: number;
  images_with_findings: number;
  findings_by_severity: SeverityCount[];
  images: SecurityImageSummary[];
}

export interface SecurityToolReport {
  tool: SecurityTool;
  state: SecurityScanState;
  tool_version: string | null;
  generated_at: string | null;
  findings: SecurityFinding[];
  severity_counts: SeverityCount[];
  message: string | null;
}

export interface ImageSecurityReport {
  image_id: string;
  image_name: string;
  reports: SecurityToolReport[];
}

export interface SecurityScanProgressEvent {
  tool: SecurityTool;
  image_id: string;
  image_name: string;
  state: SecurityScanState;
  findings_count: number | null;
  message: string | null;
}

export interface Volume {
  name: string;
  driver: string;
  mountpoint: string;
  scope: string;
  created: string;
}

export interface Network {
  id: string;
  name: string;
  driver: string;
  scope: string;
  subnet: string | null;
  gateway: string | null;
  internal: boolean;
  containers_count: number;
  created: string;
}

export interface LogLine {
  stream: "Stdout" | "Stderr";
  content: string;
  timestamp: string | null;
}

export interface StreamStatusEvent {
  requestId: string;
  status: "started" | "completed" | "failed";
  error: string | null;
}

export interface LogStreamEvent {
  requestId: string;
  line: LogLine;
}

export interface TextStreamEvent {
  requestId: string;
  text: string;
}

export interface ProgressStreamEvent {
  requestId: string;
  message: string;
}

export interface ContainerStats {
  cpu_percent: number;
  memory_usage: string;
  memory_usage_bytes: number;
  memory_limit_bytes: number;
  network_rx: string;
  network_tx: string;
  block_read: string;
  block_write: string;
  pids: number;
}

export interface DockerEndpoint {
  host_url: string;
  tls_ca: string | null;
  tls_cert: string | null;
  tls_key: string | null;
  timeout_secs: number;
}

export interface AppSettings {
  theme_setting: ThemeSetting;
  language_setting: LanguageSetting;
  endpoint: DockerEndpoint;
  window_width: number;
  window_height: number;
  font_family: string;
  font_size: number;
  security: SecuritySettings;
}

export type Language = "en" | "es";

export type LanguageSetting =
  | "Auto"
  | { Manual: Language };

export type ThemeSetting =
  | "Auto"
  | { Manual: ThemeVariant };

export type ThemeVariant =
  | "Light" | "Dark" | "Dracula" | "Nord"
  | "SolarizedLight" | "SolarizedDark"
  | "GruvboxLight" | "GruvboxDark"
  | "CatppuccinLatte" | "CatppuccinFrappe"
  | "CatppuccinMacchiato" | "CatppuccinMocha"
  | "TokyoNight" | "TokyoNightStorm" | "TokyoNightLight"
  | "KanagawaWave" | "KanagawaDragon" | "KanagawaLotus"
  | "Moonfly" | "Nightfly" | "Oxocarbon" | "Ferra";
