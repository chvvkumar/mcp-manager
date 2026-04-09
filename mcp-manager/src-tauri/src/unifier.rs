use crate::types::*;
use std::collections::HashMap;

fn normalize_command(cmd: &str) -> String {
    std::path::Path::new(cmd)
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| cmd.to_string())
}

fn normalize_url(url: &str) -> String {
    url.trim_end_matches('/').to_lowercase()
}

fn server_fingerprint(entry: &ServerEntry) -> String {
    match entry.transport {
        Transport::Stdio => {
            let cmd = normalize_command(entry.command.as_deref().unwrap_or(""));
            let mut args = entry.args.clone().unwrap_or_default();
            args.sort();
            format!("stdio:{}:{}", cmd, args.join(","))
        }
        Transport::Http => {
            let url = normalize_url(entry.url.as_deref().unwrap_or(""));
            format!("http:{}", url)
        }
    }
}

pub type UnlinkOverrides = Option<HashMap<String, Vec<String>>>;

pub fn build_unified_servers(
    tool_servers: &[(String, Vec<ServerEntry>)],
    _unlink_overrides: &UnlinkOverrides,
) -> Vec<UnifiedServer> {
    let mut groups: HashMap<String, Vec<(String, ServerEntry)>> = HashMap::new();

    for (tool_id, servers) in tool_servers {
        for server in servers {
            let fp = server_fingerprint(server);
            groups
                .entry(fp)
                .or_default()
                .push((tool_id.clone(), server.clone()));
        }
    }

    let mut unified: Vec<UnifiedServer> = groups
        .into_iter()
        .map(|(_fp, entries)| {
            let first = &entries[0].1;
            let display_name = first.name.clone();

            let bindings: Vec<ToolBinding> = entries
                .iter()
                .map(|(tool_id, entry)| ToolBinding {
                    tool_id: tool_id.clone(),
                    server_name: entry.name.clone(),
                    status: if entry.disabled == Some(true) {
                        BindingStatus::Disabled
                    } else {
                        BindingStatus::Enabled
                    },
                    overrides: None,
                })
                .collect();

            UnifiedServer {
                id: uuid::Uuid::new_v4().to_string(),
                display_name,
                transport: first.transport.clone(),
                command: first.command.clone(),
                args: first.args.clone(),
                env: first.env.clone(),
                url: first.url.clone(),
                headers: first.headers.clone(),
                bindings,
            }
        })
        .collect();

    unified.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));
    unified
}
