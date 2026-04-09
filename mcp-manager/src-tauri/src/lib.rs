mod types;
mod adapters;
mod registry;
mod unifier;
mod backup;
mod state;
mod commands;

use commands::AppData;
use backup::BackupManager;
use state::AppState;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::load();
    let backup_manager = BackupManager::new(BackupManager::default_dir());
    let _ = backup_manager.prune(app_state.preferences.backup_retention_days);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppData {
            state: Mutex::new(app_state),
            backup_manager,
        })
        .invoke_handler(tauri::generate_handler![
            commands::scan_configs,
            commands::update_server_binding,
            commands::list_backups,
            commands::restore_backup,
            commands::get_preferences,
            commands::save_preferences,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests;
