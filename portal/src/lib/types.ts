export interface ServiceInfo {
  id: string;
  name: string;
  state: string;
  ready: boolean;
}

export interface ServiceDefinition {
  id: string;
  name: string;
  ready: boolean;
  binary_path: string;
  args: string[];
  env: Record<string, string>;
  auto_restart: boolean;
  restart_limit: number | null;
  current_version: string | null;
  port?: number;
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

export interface ConfigField {
  key: string;
  value: string;
  field_type: string;
  description: string;
}

export interface ServiceConfig {
  has_config: boolean;
  has_template: boolean;
  fields: ConfigField[];
}

export interface LogEntry {
  timestamp: string;
  level: string;
  message: string;
}

export interface ServiceStats {
  service_id: string;
  cpu_usage: number;
  memory_mb: number;
  pid: number | null;
}
