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
