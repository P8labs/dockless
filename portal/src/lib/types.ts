export interface ServiceInfo {
  id: string;
  name: string;
  state: string;
}

export interface ServiceDefinition {
  id: string;
  name: string;
  binary_path: string;
  args: string[];
  env: Record<string, string>;
  auto_restart: boolean;
  restart_limit: number | null;
  current_version: string | null;
}

export interface ServiceView extends ServiceDefinition {
  state: string;
}

export interface SystemStats {
  cpu_usage: number;
  memory_used: number;
  memory_total: number;
  disk_used: number;
  disk_total: number;
  uptime: number;
}

export interface HealthInfo {
  name: string;
  status: string;
  node_id: string;
  stats: SystemStats;
}

export interface ArtifactInfo {
  service: string;
  current_version: string | null;
  available_versions: string[];
}

export interface ApiResponse {
  status: boolean;
  message: string;
}
