use super::{AdapterResult, ConfigAdapter, parse_servers_from_json, write_servers_to_json};
use crate::types::ServerEntry;
use std::path::PathBuf;

pub struct RooCodeAdapter {
    pub config_path_override: Option<PathBuf>,
}

impl RooCodeAdapter {
    pub fn new() -> Self {
        Self { config_path_override: None }
    }
}

impl ConfigAdapter for RooCodeAdapter {
    fn tool_id(&self) -> &str { "roo-code" }
    fn display_name(&self) -> &str { "Roo Code" }

    fn config_path(&self) -> PathBuf {
        if let Some(ref p) = self.config_path_override {
            return p.clone();
        }
        #[cfg(target_os = "windows")]
        { dirs::config_dir().unwrap().join("Code/User/globalStorage/rooveterinaryinc.roo-cline/settings/cline_mcp_settings.json") }
        #[cfg(target_os = "macos")]
        { dirs::home_dir().unwrap().join("Library/Application Support/Code/User/globalStorage/rooveterinaryinc.roo-cline/settings/cline_mcp_settings.json") }
        #[cfg(target_os = "linux")]
        { dirs::home_dir().unwrap().join(".config/Code/User/globalStorage/rooveterinaryinc.roo-cline/settings/cline_mcp_settings.json") }
    }

    fn read_servers(&self) -> AdapterResult<Vec<ServerEntry>> {
        let path = self.config_path();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        parse_servers_from_json(&json, "mcpServers")
    }

    fn write_servers(&self, servers: &[ServerEntry]) -> AdapterResult<()> {
        let path = self.config_path();
        let mut json = if path.exists() {
            let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
            serde_json::from_str(&content).map_err(|e| e.to_string())?
        } else {
            serde_json::json!({})
        };
        write_servers_to_json(&mut json, "mcpServers", servers, true)?;
        let output = serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(&path, output).map_err(|e| e.to_string())
    }

    fn supports_native_disable(&self) -> bool { true }
}
