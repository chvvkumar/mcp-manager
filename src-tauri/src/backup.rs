use crate::types::BackupEntry;
use chrono::Utc;
use std::path::{Path, PathBuf};

pub struct BackupManager {
    backup_dir: PathBuf,
}

impl BackupManager {
    pub fn new(backup_dir: PathBuf) -> Self {
        Self { backup_dir }
    }

    pub fn default_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".mcp-manager")
            .join("backups")
    }

    pub fn backup(&self, tool_id: &str, config_path: &Path) -> Result<PathBuf, String> {
        if !config_path.exists() {
            return Err(format!("Config file does not exist: {:?}", config_path));
        }
        let tool_backup_dir = self.backup_dir.join(tool_id);
        std::fs::create_dir_all(&tool_backup_dir).map_err(|e| e.to_string())?;
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S%.3f").to_string();
        let ext = config_path.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_else(|| "json".to_string());
        let backup_filename = format!("{}_{}.{}", tool_id, timestamp, ext);
        let backup_path = tool_backup_dir.join(backup_filename);
        std::fs::copy(config_path, &backup_path).map_err(|e| e.to_string())?;
        Ok(backup_path)
    }

    pub fn restore(&self, backup_path: &Path, config_path: &Path) -> Result<(), String> {
        if !backup_path.exists() {
            return Err(format!("Backup file does not exist: {:?}", backup_path));
        }
        std::fs::copy(backup_path, config_path).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn list_backups(&self, tool_id: &str) -> Result<Vec<BackupEntry>, String> {
        let tool_backup_dir = self.backup_dir.join(tool_id);
        if !tool_backup_dir.exists() {
            return Ok(vec![]);
        }
        let mut entries = Vec::new();
        for entry in std::fs::read_dir(&tool_backup_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            entries.push(BackupEntry {
                tool_id: tool_id.to_string(),
                timestamp: entry.file_name().to_string_lossy().to_string(),
                file_path: entry.path().to_string_lossy().to_string(),
                size_bytes: metadata.len(),
            });
        }
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(entries)
    }

    pub fn prune(&self, max_age_days: i64) -> Result<usize, String> {
        let cutoff = Utc::now() - chrono::Duration::days(max_age_days);
        let mut removed = 0;
        if !self.backup_dir.exists() { return Ok(0); }
        for tool_dir in std::fs::read_dir(&self.backup_dir).map_err(|e| e.to_string())? {
            let tool_dir = tool_dir.map_err(|e| e.to_string())?;
            if !tool_dir.file_type().map_err(|e| e.to_string())?.is_dir() { continue; }
            for file in std::fs::read_dir(tool_dir.path()).map_err(|e| e.to_string())? {
                let file = file.map_err(|e| e.to_string())?;
                let metadata = file.metadata().map_err(|e| e.to_string())?;
                if let Ok(modified) = metadata.modified() {
                    let modified_dt: chrono::DateTime<Utc> = modified.into();
                    if modified_dt < cutoff {
                        std::fs::remove_file(file.path()).map_err(|e| e.to_string())?;
                        removed += 1;
                    }
                }
            }
        }
        Ok(removed)
    }
}
