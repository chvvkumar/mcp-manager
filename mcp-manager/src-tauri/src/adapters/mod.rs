use crate::types::ServerEntry;
use std::path::PathBuf;

pub mod claude_desktop;
pub mod claude_code;
pub mod vscode;
pub mod cline;
pub mod roo_code;
pub mod cursor;
pub mod windsurf;
pub mod jetbrains;
pub mod visual_studio;
pub mod copilot_cli;
pub mod amazon_q;
pub mod zed;
pub mod continue_dev;
pub mod gemini_cli;

pub type AdapterResult<T> = Result<T, String>;

pub trait ConfigAdapter: Send + Sync {
    fn tool_id(&self) -> &str;
    fn display_name(&self) -> &str;
    fn detect(&self) -> bool {
        self.config_path().exists()
    }
    fn config_path(&self) -> PathBuf;
    fn read_servers(&self) -> AdapterResult<Vec<ServerEntry>>;
    fn write_servers(&self, servers: &[ServerEntry]) -> AdapterResult<()>;
    fn supports_native_disable(&self) -> bool {
        false
    }
}

use serde_json::Value;
use std::collections::HashMap;

/// Parse a JSON object of MCP servers from a top-level key
pub fn parse_servers_from_json(
    json: &Value,
    key: &str,
) -> AdapterResult<Vec<ServerEntry>> {
    let servers_obj = json
        .get(key)
        .and_then(|v| v.as_object())
        .unwrap_or(&serde_json::Map::new())
        .clone();

    let mut entries = Vec::new();
    for (name, value) in &servers_obj {
        let transport = if value.get("url").is_some() {
            crate::types::Transport::Http
        } else {
            crate::types::Transport::Stdio
        };

        let command = value.get("command").and_then(|v| v.as_str()).map(String::from);
        let args = value.get("args").and_then(|v| {
            v.as_array().map(|arr| {
                arr.iter().filter_map(|a| a.as_str().map(String::from)).collect()
            })
        });
        let env = value.get("env").and_then(|v| {
            v.as_object().map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect::<HashMap<String, String>>()
            })
        });
        let url = value.get("url").and_then(|v| v.as_str()).map(String::from);
        let headers = value.get("headers").and_then(|v| {
            v.as_object().map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect::<HashMap<String, String>>()
            })
        });
        let disabled = value.get("disabled").and_then(|v| v.as_bool());

        let known_keys = ["command", "args", "env", "url", "headers", "disabled", "type"];
        let extra_fields: serde_json::Map<String, serde_json::Value> = value
            .as_object()
            .map(|obj| {
                obj.iter()
                    .filter(|(k, _)| !known_keys.contains(&k.as_str()))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            })
            .unwrap_or_default();

        entries.push(ServerEntry {
            name: name.clone(),
            transport,
            command,
            args,
            env,
            url,
            headers,
            disabled,
            extra_fields,
        });
    }
    Ok(entries)
}

/// Serialize server entries back into a JSON object under the given key
pub fn write_servers_to_json(
    existing_json: &mut Value,
    key: &str,
    servers: &[ServerEntry],
    include_type_field: bool,
) -> AdapterResult<()> {
    let mut servers_map = serde_json::Map::new();
    for server in servers {
        let mut obj = serde_json::Map::new();

        if include_type_field {
            match server.transport {
                crate::types::Transport::Stdio => { obj.insert("type".to_string(), Value::String("stdio".to_string())); }
                crate::types::Transport::Http => { obj.insert("type".to_string(), Value::String("http".to_string())); }
            }
        }
        if let Some(ref cmd) = server.command {
            obj.insert("command".to_string(), Value::String(cmd.clone()));
        }
        if let Some(ref args) = server.args {
            obj.insert("args".to_string(), Value::Array(args.iter().map(|a| Value::String(a.clone())).collect()));
        }
        if let Some(ref env) = server.env {
            obj.insert("env".to_string(), Value::Object(env.iter().map(|(k, v)| (k.clone(), Value::String(v.clone()))).collect()));
        }
        if let Some(ref url) = server.url {
            obj.insert("url".to_string(), Value::String(url.clone()));
        }
        if let Some(ref headers) = server.headers {
            obj.insert("headers".to_string(), Value::Object(headers.iter().map(|(k, v)| (k.clone(), Value::String(v.clone()))).collect()));
        }
        if let Some(disabled) = server.disabled {
            obj.insert("disabled".to_string(), Value::Bool(disabled));
        }
        for (k, v) in &server.extra_fields {
            obj.insert(k.clone(), v.clone());
        }
        servers_map.insert(server.name.clone(), Value::Object(obj));
    }
    existing_json[key] = Value::Object(servers_map);
    Ok(())
}
