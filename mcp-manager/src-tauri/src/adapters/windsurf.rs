use super::{AdapterResult, ConfigAdapter, parse_servers_from_json, write_servers_to_json};
use crate::types::ServerEntry;
use std::path::PathBuf;

pub struct WindsurfAdapter {
    pub config_path_override: Option<PathBuf>,
}

impl WindsurfAdapter {
    pub fn new() -> Self {
        Self { config_path_override: None }
    }
}

impl ConfigAdapter for WindsurfAdapter {
    fn tool_id(&self) -> &str { "windsurf" }
    fn display_name(&self) -> &str { "Windsurf" }

    fn config_path(&self) -> PathBuf {
        if let Some(ref p) = self.config_path_override {
            return p.clone();
        }
        dirs::home_dir().unwrap().join(".codeium/windsurf/mcp_config.json")
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
        write_servers_to_json(&mut json, "mcpServers", servers, false)?;
        let output = serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(&path, output).map_err(|e| e.to_string())
    }
}
