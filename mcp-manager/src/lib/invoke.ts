import { invoke } from "@tauri-apps/api/core";
import type { ScanResult, ServerEntry, BackupEntry, Preferences } from "../types";

export async function scanConfigs(): Promise<ScanResult> {
  return invoke<ScanResult>("scan_configs");
}

export async function updateServerBinding(
  serverId: string,
  toolId: string,
  action: "enable" | "disable" | "add" | "remove",
  serverConfig?: ServerEntry
): Promise<void> {
  return invoke("update_server_binding", {
    serverId, toolId, action,
    serverConfig: serverConfig ?? null,
  });
}

export async function listBackups(toolId: string): Promise<BackupEntry[]> {
  return invoke<BackupEntry[]>("list_backups", { toolId });
}

export async function restoreBackup(toolId: string, backupPath: string): Promise<void> {
  return invoke("restore_backup", { toolId, backupPath });
}

export async function getPreferences(): Promise<Preferences> {
  return invoke<Preferences>("get_preferences");
}

export async function savePreferences(preferences: Preferences): Promise<void> {
  return invoke("save_preferences", { preferences });
}
