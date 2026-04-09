import { useAppContext } from "../context/AppContext";
import { useServers } from "../hooks/useServers";
import { ServerForm } from "./ServerForm";
import type { ServerEntry } from "../types";

export function ServerDetailPanel() {
  const { servers, tools, selectedServerId, setSelectedServerId } = useAppContext();
  const { toggleBinding, addServer, removeServer } = useServers();
  const server = servers.find((s) => s.id === selectedServerId);
  if (!server) return null;

  const handleSave = async (entry: ServerEntry) => {
    const boundToolIds = server.bindings.map((b) => b.tool_id);
    await removeServer(server.bindings[0]?.server_name ?? entry.name, boundToolIds);
    await addServer(entry, boundToolIds);
    setSelectedServerId(null);
  };

  const handleAddToTool = async (toolId: string) => {
    const entry: ServerEntry = {
      name: server.display_name, transport: server.transport,
      command: server.command, args: server.args, env: server.env,
      url: server.url, headers: server.headers,
    };
    await addServer(entry, [toolId]);
  };

  const unboundTools = tools.filter((t) => !server.bindings.some((b) => b.tool_id === t.tool_id));

  return (
    <div className="w-96 bg-gray-850 border-l border-gray-700 overflow-y-auto p-4">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-semibold">{server.display_name}</h2>
        <button onClick={() => setSelectedServerId(null)} className="text-gray-400 hover:text-white">x</button>
      </div>
      <ServerForm initial={{ name: server.display_name, transport: server.transport, command: server.command, args: server.args, env: server.env, url: server.url, headers: server.headers }}
        onSave={handleSave} onCancel={() => setSelectedServerId(null)} />
      <div className="mt-6">
        <h3 className="text-sm font-medium text-gray-400 mb-2">Bound to</h3>
        {server.bindings.map((binding) => (
          <div key={binding.tool_id} className="flex items-center justify-between py-1.5">
            <span className="text-sm">{binding.tool_id}</span>
            <div className="flex items-center gap-2">
              <button onClick={() => toggleBinding(binding.server_name, binding.tool_id, binding.status, {
                  name: binding.server_name, transport: server.transport, command: server.command,
                  args: server.args, env: server.env, url: server.url, headers: server.headers,
                })}
                className={`text-xs px-2 py-0.5 rounded ${binding.status === "enabled" ? "bg-green-600/30 text-green-400" : "bg-gray-600/30 text-gray-400"}`}>
                {binding.status}
              </button>
              <button onClick={() => removeServer(binding.server_name, [binding.tool_id])} className="text-xs text-red-400 hover:text-red-300">Remove</button>
            </div>
          </div>
        ))}
        {unboundTools.length > 0 && (
          <div className="mt-3">
            <select onChange={(e) => { if (e.target.value) handleAddToTool(e.target.value); e.target.value = ""; }}
              className="w-full px-2 py-1.5 bg-gray-700 border border-gray-600 rounded text-sm" defaultValue="">
              <option value="" disabled>Add to tool...</option>
              {unboundTools.map((t) => (<option key={t.tool_id} value={t.tool_id}>{t.display_name}</option>))}
            </select>
          </div>
        )}
      </div>
    </div>
  );
}
