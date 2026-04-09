import { useAppContext } from "../context/AppContext";
import { useServers } from "../hooks/useServers";
import { ToggleCell } from "./ToggleCell";
import type { UnifiedServer, DetectedTool } from "../types";

interface ServerRowProps {
  server: UnifiedServer;
  detectedTools: DetectedTool[];
}

export function ServerRow({ server, detectedTools }: ServerRowProps) {
  const { setSelectedServerId } = useAppContext();
  const { toggleBinding, toggleGlobal } = useServers();
  const allEnabled = server.bindings.every((b) => b.status === "enabled");

  return (
    <tr className="border-b border-gray-700 hover:bg-gray-800/50">
      <td className="px-3 py-2">
        <button onClick={() => toggleGlobal(server.id, !allEnabled)}
          className={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors ${allEnabled ? "bg-green-500" : "bg-gray-600"}`}>
          <span className={`inline-block h-3.5 w-3.5 transform rounded-full bg-white transition-transform ${allEnabled ? "translate-x-4.5" : "translate-x-0.5"}`} />
        </button>
      </td>
      <td className="px-3 py-2">
        <button onClick={() => setSelectedServerId(server.id)} className="flex items-center gap-2 text-left hover:text-blue-400">
          <span className="text-gray-400" title={server.transport}>{server.transport === "stdio" ? ">" : "~"}</span>
          <span className="text-sm font-medium text-gray-200">{server.display_name}</span>
        </button>
      </td>
      {detectedTools.map((tool) => {
        const binding = server.bindings.find((b) => b.tool_id === tool.tool_id);
        return (
          <ToggleCell key={tool.tool_id}
            status={binding ? binding.status : "not_present"}
            onToggle={() => {
              if (binding) {
                toggleBinding(binding.server_name, binding.tool_id, binding.status, {
                  name: binding.server_name, transport: server.transport,
                  command: server.command, args: server.args, env: server.env,
                  url: server.url, headers: server.headers,
                });
              }
            }}
            onAdd={() => setSelectedServerId(server.id)}
          />
        );
      })}
      <td className="px-3 py-2">
        <button onClick={() => setSelectedServerId(server.id)} className="text-sm text-gray-400 hover:text-white">Edit</button>
        <button onClick={(e) => { e.stopPropagation(); window.dispatchEvent(new CustomEvent("delete-server", { detail: server })); }}
          className="text-sm text-red-400 hover:text-red-300 ml-2">Del</button>
      </td>
    </tr>
  );
}
