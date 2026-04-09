export type Transport = "stdio" | "http";
export type BindingStatus = "enabled" | "disabled";

export interface ServerEntry {
  name: string;
  transport: Transport;
  command?: string;
  args?: string[];
  env?: Record<string, string>;
  url?: string;
  headers?: Record<string, string>;
  disabled?: boolean;
}

export interface ToolBinding {
  tool_id: string;
  server_name: string;
  status: BindingStatus;
  overrides?: {
    env?: Record<string, string>;
    headers?: Record<string, string>;
    args?: string[];
  };
}

export interface UnifiedServer {
  id: string;
  display_name: string;
  transport: Transport;
  command?: string;
  args?: string[];
  env?: Record<string, string>;
  url?: string;
  headers?: Record<string, string>;
  bindings: ToolBinding[];
}

export interface DetectedTool {
  tool_id: string;
  display_name: string;
  config_path: string;
  server_count: number;
}

export interface ScanResult {
  tools: DetectedTool[];
  servers: UnifiedServer[];
}

export interface BackupEntry {
  tool_id: string;
  timestamp: string;
  file_path: string;
  size_bytes: number;
}

export interface Preferences {
  auto_scan_on_launch: boolean;
  backup_retention_days: number;
}
