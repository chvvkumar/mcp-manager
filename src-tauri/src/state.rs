use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    pub display_names: HashMap<String, String>,
    pub unlinks: HashMap<String, Vec<String>>,
    pub preferences: Preferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    pub auto_scan_on_launch: bool,
    pub backup_retention_days: i64,
}

impl Default for Preferences {
    fn default() -> Self {
        Self { auto_scan_on_launch: true, backup_retention_days: 30 }
    }
}

impl AppState {
    pub fn state_path() -> PathBuf {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".mcp-manager").join("state.json")
    }

    pub fn load() -> Self {
        let path = Self::state_path();
        if path.exists() {
            std::fs::read_to_string(&path).ok().and_then(|c| serde_json::from_str(&c).ok()).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::state_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let output = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(&path, output).map_err(|e| e.to_string())
    }
}
