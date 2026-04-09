use crate::backup::BackupManager;
use crate::registry::{build_adapter_registry, detect_installed_tools};
use crate::state::AppState;
use crate::types::*;
use crate::unifier::build_unified_servers;
use std::sync::Mutex;
use tauri::State;

pub struct AppData {
    pub state: Mutex<AppState>,
    pub backup_manager: BackupManager,
}

#[tauri::command]
pub fn scan_configs() -> Result<ScanResult, String> {
    let adapters = detect_installed_tools();
    let mut tool_servers: Vec<(String, Vec<ServerEntry>)> = Vec::new();
    let mut tools: Vec<DetectedTool> = Vec::new();

    for adapter in &adapters {
        let servers = adapter.read_servers().unwrap_or_default();
        tools.push(DetectedTool {
            tool_id: adapter.tool_id().to_string(),
            display_name: adapter.display_name().to_string(),
            config_path: adapter.config_path().to_string_lossy().to_string(),
            server_count: servers.len(),
        });
        tool_servers.push((adapter.tool_id().to_string(), servers));
    }

    let servers = build_unified_servers(&tool_servers, &None);
    Ok(ScanResult { tools, servers })
}

#[tauri::command]
pub fn update_server_binding(
    server_id: String,
    tool_id: String,
    action: String,
    server_config: Option<ServerEntry>,
    app_data: State<AppData>,
) -> Result<(), String> {
    let adapters = build_adapter_registry();
    let adapter = adapters.iter().find(|a| a.tool_id() == tool_id)
        .ok_or_else(|| format!("Unknown tool: {}", tool_id))?;

    let config_path = adapter.config_path();
    if config_path.exists() {
        app_data.backup_manager.backup(&tool_id, &config_path)?;
    }

    let mut servers = adapter.read_servers()?;

    match action.as_str() {
        "enable" => {
            if adapter.supports_native_disable() {
                if let Some(s) = servers.iter_mut().find(|s| s.name == server_id) {
                    s.disabled = Some(false);
                }
            } else {
                if let Some(config) = server_config {
                    if !servers.iter().any(|s| s.name == config.name) {
                        servers.push(config);
                    }
                }
            }
        }
        "disable" => {
            if adapter.supports_native_disable() {
                if let Some(s) = servers.iter_mut().find(|s| s.name == server_id) {
                    s.disabled = Some(true);
                }
            } else {
                servers.retain(|s| s.name != server_id);
            }
        }
        "add" => {
            if let Some(config) = server_config {
                servers.push(config);
            } else {
                return Err("server_config required for add action".to_string());
            }
        }
        "remove" => {
            servers.retain(|s| s.name != server_id);
        }
        _ => return Err(format!("Unknown action: {}", action)),
    }

    adapter.write_servers(&servers)?;

    let verify = adapter.read_servers();
    if verify.is_err() {
        let backups = app_data.backup_manager.list_backups(&tool_id)?;
        if let Some(latest) = backups.first() {
            app_data.backup_manager.restore(
                &std::path::PathBuf::from(&latest.file_path),
                &config_path,
            )?;
            return Err("Write verification failed — config restored from backup".to_string());
        }
    }

    Ok(())
}

#[tauri::command]
pub fn list_backups(tool_id: String, app_data: State<AppData>) -> Result<Vec<BackupEntry>, String> {
    app_data.backup_manager.list_backups(&tool_id)
}

#[tauri::command]
pub fn restore_backup(
    tool_id: String,
    backup_path: String,
    app_data: State<AppData>,
) -> Result<(), String> {
    let adapters = build_adapter_registry();
    let adapter = adapters.iter().find(|a| a.tool_id() == tool_id)
        .ok_or_else(|| format!("Unknown tool: {}", tool_id))?;
    let config_path = adapter.config_path();
    if config_path.exists() {
        app_data.backup_manager.backup(&tool_id, &config_path)?;
    }
    app_data.backup_manager.restore(&std::path::PathBuf::from(backup_path), &config_path)
}

#[tauri::command]
pub fn get_preferences(app_data: State<AppData>) -> Result<crate::state::Preferences, String> {
    let state = app_data.state.lock().map_err(|e| e.to_string())?;
    Ok(state.preferences.clone())
}

#[tauri::command]
pub fn save_preferences(
    preferences: crate::state::Preferences,
    app_data: State<AppData>,
) -> Result<(), String> {
    let mut state = app_data.state.lock().map_err(|e| e.to_string())?;
    state.preferences = preferences;
    state.save()
}
