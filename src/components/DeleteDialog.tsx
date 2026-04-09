import { useState } from "react";
import { useServers } from "../hooks/useServers";
import type { UnifiedServer } from "../types";

interface DeleteDialogProps { server: UnifiedServer; onClose: () => void; }

export function DeleteDialog({ server, onClose }: DeleteDialogProps) {
  const { removeServer } = useServers();
  const [selectedTools, setSelectedTools] = useState<Set<string>>(new Set(server.bindings.map((b) => b.tool_id)));

  const handleDelete = async () => {
    const toolIds = Array.from(selectedTools);
    const serverName = server.bindings[0]?.server_name ?? server.display_name;
    await removeServer(serverName, toolIds);
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 w-[400px]">
        <h2 className="text-lg font-semibold mb-2">Delete Server</h2>
        <p className="text-sm text-gray-400 mb-4">Remove <strong>{server.display_name}</strong> from selected tools:</p>
        {server.bindings.map((binding) => (
          <label key={binding.tool_id} className="flex items-center gap-3 py-1">
            <input type="checkbox" checked={selectedTools.has(binding.tool_id)}
              onChange={(e) => { const next = new Set(selectedTools); if (e.target.checked) next.add(binding.tool_id); else next.delete(binding.tool_id); setSelectedTools(next); }} className="rounded" />
            <span className="text-sm">{binding.tool_id}</span>
          </label>
        ))}
        <div className="flex gap-2 pt-4">
          <button onClick={handleDelete} disabled={selectedTools.size === 0}
            className="px-4 py-1.5 bg-red-600 text-white rounded text-sm hover:bg-red-500 disabled:opacity-50">
            Delete from {selectedTools.size} tool{selectedTools.size !== 1 ? "s" : ""}
          </button>
          <button onClick={onClose} className="px-4 py-1.5 bg-gray-700 text-gray-300 rounded text-sm hover:bg-gray-600">Cancel</button>
        </div>
      </div>
    </div>
  );
}
