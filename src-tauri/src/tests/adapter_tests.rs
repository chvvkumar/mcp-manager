#[cfg(test)]
mod tests {
    use crate::types::*;
    use crate::adapters::ConfigAdapter;
    use std::io::Write;

    #[test]
    fn test_server_entry_stdio_serialization() {
        let entry = ServerEntry {
            name: "docker-mcp".to_string(),
            transport: Transport::Stdio,
            command: Some("docker".to_string()),
            args: Some(vec!["mcp".to_string(), "gateway".to_string(), "run".to_string()]),
            env: None,
            url: None,
            headers: None,
            disabled: None,
            extra_fields: serde_json::Map::new(),
        };
        let json = serde_json::to_value(&entry).unwrap();
        assert_eq!(json["name"], "docker-mcp");
        assert_eq!(json["transport"], "stdio");
    }

    #[test]
    fn test_server_entry_http_serialization() {
        let entry = ServerEntry {
            name: "context7".to_string(),
            transport: Transport::Http,
            command: None,
            args: None,
            env: None,
            url: Some("https://mcp.context7.com/mcp".to_string()),
            headers: Some(std::collections::HashMap::from([
                ("CONTEXT7_API_KEY".to_string(), "ctx7sk-abc123".to_string()),
            ])),
            disabled: None,
            extra_fields: serde_json::Map::new(),
        };
        let json = serde_json::to_value(&entry).unwrap();
        assert_eq!(json["transport"], "http");
        assert!(json["url"].as_str().unwrap().contains("context7"));
    }

    #[test]
    fn test_claude_desktop_read_servers() {
        let dir = tempfile::tempdir().unwrap();
        let config_path = dir.path().join("claude_desktop_config.json");
        let mut file = std::fs::File::create(&config_path).unwrap();
        write!(file, r#"{{
            "mcpServers": {{
                "docker-mcp": {{
                    "command": "docker",
                    "args": ["mcp", "gateway", "run"]
                }},
                "remote-api": {{
                    "type": "http",
                    "url": "https://example.com/mcp",
                    "headers": {{"Authorization": "Bearer token123"}}
                }}
            }}
        }}"#).unwrap();

        let adapter = crate::adapters::claude_desktop::ClaudeDesktopAdapter {
            config_path_override: Some(config_path),
        };
        let servers = adapter.read_servers().unwrap();
        assert_eq!(servers.len(), 2);

        let stdio = servers.iter().find(|s| s.name == "docker-mcp").unwrap();
        assert_eq!(stdio.transport, crate::types::Transport::Stdio);
        assert_eq!(stdio.command.as_deref(), Some("docker"));

        let http = servers.iter().find(|s| s.name == "remote-api").unwrap();
        assert_eq!(http.transport, crate::types::Transport::Http);
        assert!(http.url.as_ref().unwrap().contains("example.com"));
    }

    #[test]
    fn test_cline_read_with_disabled_flag() {
        let dir = tempfile::tempdir().unwrap();
        let config_path = dir.path().join("cline_mcp_settings.json");
        let mut file = std::fs::File::create(&config_path).unwrap();
        write!(file, r#"{{
            "mcpServers": {{
                "docker-mcp": {{
                    "disabled": true,
                    "timeout": 60,
                    "type": "stdio",
                    "command": "docker",
                    "args": ["mcp", "gateway", "run"]
                }}
            }}
        }}"#).unwrap();

        let adapter = crate::adapters::cline::ClineAdapter {
            config_path_override: Some(config_path),
        };
        let servers = adapter.read_servers().unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].disabled, Some(true));
        assert!(adapter.supports_native_disable());
    }

    #[test]
    fn test_claude_code_preserves_other_fields() {
        let dir = tempfile::tempdir().unwrap();
        let config_path = dir.path().join(".claude.json");
        let mut file = std::fs::File::create(&config_path).unwrap();
        write!(file, r#"{{
            "installMethod": "native",
            "userID": "abc123",
            "mcpServers": {{
                "docker-mcp": {{
                    "type": "stdio",
                    "command": "docker",
                    "args": ["mcp", "gateway", "run"]
                }}
            }},
            "skillUsage": {{}}
        }}"#).unwrap();

        let adapter = crate::adapters::claude_code::ClaudeCodeAdapter {
            config_path_override: Some(config_path.clone()),
        };

        let servers = adapter.read_servers().unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].name, "docker-mcp");

        let mut new_servers = servers.clone();
        new_servers.push(crate::types::ServerEntry {
            name: "context7".to_string(),
            transport: crate::types::Transport::Http,
            command: None,
            args: None,
            env: None,
            url: Some("https://mcp.context7.com/mcp".to_string()),
            headers: None,
            disabled: None,
            extra_fields: serde_json::Map::new(),
        });
        adapter.write_servers(&new_servers).unwrap();

        let content = std::fs::read_to_string(&config_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["installMethod"], "native");
        assert_eq!(json["userID"], "abc123");
        assert!(json["skillUsage"].is_object());
        assert_eq!(json["mcpServers"].as_object().unwrap().len(), 2);
    }

    #[test]
    fn test_zed_adapter_reads_context_servers() {
        let dir = tempfile::tempdir().unwrap();
        let config_path = dir.path().join("settings.json");
        let mut file = std::fs::File::create(&config_path).unwrap();
        write!(file, r#"{{
            "theme": "One Dark",
            "context_servers": {{
                "my-server": {{
                    "command": "npx",
                    "args": ["-y", "some-package"],
                    "env": {{}}
                }}
            }},
            "vim_mode": true
        }}"#).unwrap();

        let adapter = crate::adapters::zed::ZedAdapter {
            config_path_override: Some(config_path),
        };
        let servers = adapter.read_servers().unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].name, "my-server");
    }
}
