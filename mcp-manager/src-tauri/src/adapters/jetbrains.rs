use super::{AdapterResult, ConfigAdapter, parse_servers_from_json, write_servers_to_json};
use crate::types::ServerEntry;
use std::path::PathBuf;

pub struct JetBrainsAdapter {
    pub product_name: String,
    pub product_version: String,
    pub config_path_override: Option<PathBuf>,
}

impl JetBrainsAdapter {
    pub fn new(product_name: &str, product_version: &str) -> Self {
        Self {
            product_name: product_name.to_string(),
            product_version: product_version.to_string(),
            config_path_override: None,
        }
    }

    fn jetbrains_base_dir() -> PathBuf {
        #[cfg(target_os = "windows")]
        { dirs::config_dir().unwrap().join("JetBrains") }
        #[cfg(target_os = "macos")]
        { dirs::home_dir().unwrap().join("Library/Application Support/JetBrains") }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        { dirs::home_dir().unwrap().join(".config/JetBrains") }
    }

    /// Scan for all JetBrains product directories that contain mcp.json
    pub fn detect_all() -> Vec<JetBrainsAdapter> {
        let base = Self::jetbrains_base_dir();
        let mut adapters = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&base) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let mcp_json = path.join("mcp.json");
                    if mcp_json.exists() {
                        let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
                        // Parse product name and version from dir name like "IntelliJIdea2024.1"
                        let (name, version) = Self::parse_product_dir(&dir_name);
                        adapters.push(JetBrainsAdapter {
                            product_name: name,
                            product_version: version,
                            config_path_override: Some(mcp_json),
                        });
                    }
                }
            }
        }
        adapters
    }

    fn parse_product_dir(dir_name: &str) -> (String, String) {
        // Try to split on the first digit sequence that looks like a year (20xx)
        if let Some(pos) = dir_name.find(|c: char| c.is_ascii_digit()) {
            let name = &dir_name[..pos];
            let version = &dir_name[pos..];
            (name.to_string(), version.to_string())
        } else {
            (dir_name.to_string(), String::new())
        }
    }
}

impl ConfigAdapter for JetBrainsAdapter {
    fn tool_id(&self) -> &str { "jetbrains" }

    fn display_name(&self) -> &str { "JetBrains" }

    fn config_path(&self) -> PathBuf {
        if let Some(ref p) = self.config_path_override {
            return p.clone();
        }
        let base = Self::jetbrains_base_dir();
        base.join(format!("{}{}", self.product_name, self.product_version))
            .join("mcp.json")
    }

    fn read_servers(&self) -> AdapterResult<Vec<ServerEntry>> {
        let path = self.config_path();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        parse_servers_from_json(&json, "servers")
    }

    fn write_servers(&self, servers: &[ServerEntry]) -> AdapterResult<()> {
        let path = self.config_path();
        let mut json = if path.exists() {
            let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
            serde_json::from_str(&content).map_err(|e| e.to_string())?
        } else {
            serde_json::json!({})
        };
        write_servers_to_json(&mut json, "servers", servers, false)?;
        let output = serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(&path, output).map_err(|e| e.to_string())
    }
}
