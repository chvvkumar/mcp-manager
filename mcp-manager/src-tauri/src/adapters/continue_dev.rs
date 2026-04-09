use super::{AdapterResult, ConfigAdapter, parse_servers_from_json, write_servers_to_json};
use crate::types::ServerEntry;
use std::path::PathBuf;

pub struct ContinueDevAdapter {
    pub config_path_override: Option<PathBuf>,
}

impl ContinueDevAdapter {
    pub fn new() -> Self {
        Self { config_path_override: None }
    }

    fn yaml_path(&self) -> PathBuf {
        dirs::home_dir().unwrap().join(".continue/config.yaml")
    }

    fn json_path(&self) -> PathBuf {
        dirs::home_dir().unwrap().join(".continue/config.json")
    }
}

impl ConfigAdapter for ContinueDevAdapter {
    fn tool_id(&self) -> &str { "continue-dev" }
    fn display_name(&self) -> &str { "Continue" }

    fn config_path(&self) -> PathBuf {
        if let Some(ref p) = self.config_path_override {
            return p.clone();
        }
        let yaml = self.yaml_path();
        if yaml.exists() {
            return yaml;
        }
        let json = self.json_path();
        if json.exists() {
            return json;
        }
        // Default to yaml (preferred)
        yaml
    }

    fn read_servers(&self) -> AdapterResult<Vec<ServerEntry>> {
        let path = self.config_path();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;

        let is_yaml = path.extension().and_then(|e| e.to_str()) == Some("yaml")
            || path.extension().and_then(|e| e.to_str()) == Some("yml");

        let json: serde_json::Value = if is_yaml {
            serde_yaml::from_str(&content).map_err(|e| e.to_string())?
        } else {
            serde_json::from_str(&content).map_err(|e| e.to_string())?
        };

        parse_servers_from_json(&json, "mcpServers")
    }

    fn write_servers(&self, servers: &[ServerEntry]) -> AdapterResult<()> {
        let path = self.config_path();

        let is_yaml = path.extension().and_then(|e| e.to_str()) == Some("yaml")
            || path.extension().and_then(|e| e.to_str()) == Some("yml");

        let mut json = if path.exists() {
            let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
            if is_yaml {
                serde_yaml::from_str(&content).map_err(|e| e.to_string())?
            } else {
                serde_json::from_str(&content).map_err(|e| e.to_string())?
            }
        } else {
            serde_json::json!({})
        };

        write_servers_to_json(&mut json, "mcpServers", servers, false)?;

        let output = if is_yaml {
            serde_yaml::to_string(&json).map_err(|e| e.to_string())?
        } else {
            serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?
        };

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(&path, output).map_err(|e| e.to_string())
    }
}
