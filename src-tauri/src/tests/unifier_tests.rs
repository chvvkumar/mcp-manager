#[cfg(test)]
mod tests {
    use crate::types::*;
    use crate::unifier::build_unified_servers;

    fn make_stdio_entry(name: &str, command: &str, args: Vec<&str>) -> ServerEntry {
        ServerEntry {
            name: name.to_string(),
            transport: Transport::Stdio,
            command: Some(command.to_string()),
            args: Some(args.into_iter().map(String::from).collect()),
            env: None, url: None, headers: None, disabled: None,
            extra_fields: serde_json::Map::new(),
        }
    }

    fn make_http_entry(name: &str, url: &str) -> ServerEntry {
        ServerEntry {
            name: name.to_string(),
            transport: Transport::Http,
            command: None, args: None, env: None,
            url: Some(url.to_string()),
            headers: None, disabled: None,
            extra_fields: serde_json::Map::new(),
        }
    }

    #[test]
    fn test_dedup_same_stdio_server_across_tools() {
        let tool_servers = vec![
            ("claude-code".to_string(), vec![
                make_stdio_entry("docker-mcp", "docker", vec!["mcp", "gateway", "run"]),
            ]),
            ("cline".to_string(), vec![
                make_stdio_entry("MCP_DOCKER", "docker", vec!["mcp", "gateway", "run"]),
            ]),
        ];
        let unified = build_unified_servers(&tool_servers, &None);
        assert_eq!(unified.len(), 1);
        assert_eq!(unified[0].bindings.len(), 2);
    }

    #[test]
    fn test_dedup_same_http_server_across_tools() {
        let tool_servers = vec![
            ("claude-code".to_string(), vec![
                make_http_entry("context7", "https://mcp.context7.com/mcp"),
            ]),
            ("vscode".to_string(), vec![
                make_http_entry("ctx7", "https://mcp.context7.com/mcp/"),
            ]),
        ];
        let unified = build_unified_servers(&tool_servers, &None);
        assert_eq!(unified.len(), 1);
        assert_eq!(unified[0].bindings.len(), 2);
    }

    #[test]
    fn test_different_servers_not_deduped() {
        let tool_servers = vec![
            ("claude-code".to_string(), vec![
                make_stdio_entry("docker-mcp", "docker", vec!["mcp", "gateway", "run"]),
                make_http_entry("context7", "https://mcp.context7.com/mcp"),
            ]),
        ];
        let unified = build_unified_servers(&tool_servers, &None);
        assert_eq!(unified.len(), 2);
    }
}
