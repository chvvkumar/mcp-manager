import { useState } from "react";
import { ServerForm } from "./ServerForm";
import { useServers } from "../hooks/useServers";
import { useAppContext } from "../context/AppContext";
import type { ServerEntry } from "../types";

interface AddServerDialogProps { onClose: () => void; }

export function AddServerDialog({ onClose }: AddServerDialogProps) {
  const { tools } = useAppContext();
  const { addServer } = useServers();
  const [step, setStep] = useState<"form" | "tools">("form");
  const [serverEntry, setServerEntry] = useState<ServerEntry | null>(null);
  const [selectedTools, setSelectedTools] = useState<Set<string>>(new Set());

  const handleFormSave = (entry: ServerEntry) => { setServerEntry(entry); setStep("tools"); };

  const handleConfirm = async () => {
    if (serverEntry && selectedTools.size > 0) {
      await addServer(serverEntry, Array.from(selectedTools));
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 w-[480px] max-h-[80vh] overflow-y-auto">
        <h2 className="text-lg font-semibold mb-4">Add MCP Server</h2>
        {step === "form" && <ServerForm onSave={handleFormSave} onCancel={onClose} />}
        {step === "tools" && (
          <div className="space-y-4">
            <p className="text-sm text-gray-400">Select which tools should get <strong>{serverEntry?.name}</strong>:</p>
            {tools.map((tool) => (
              <label key={tool.tool_id} className="flex items-center gap-3 py-1">
                <input type="checkbox" checked={selectedTools.has(tool.tool_id)}
                  onChange={(e) => { const next = new Set(selectedTools); if (e.target.checked) next.add(tool.tool_id); else next.delete(tool.tool_id); setSelectedTools(next); }} className="rounded" />
                <span className="text-sm">{tool.display_name}</span>
              </label>
            ))}
            <div className="flex gap-2 pt-2">
              <button onClick={handleConfirm} disabled={selectedTools.size === 0}
                className="px-4 py-1.5 bg-blue-600 text-white rounded text-sm hover:bg-blue-500 disabled:opacity-50">
                Add to {selectedTools.size} tool{selectedTools.size !== 1 ? "s" : ""}
              </button>
              <button onClick={() => setStep("form")} className="px-4 py-1.5 bg-gray-700 text-gray-300 rounded text-sm hover:bg-gray-600">Back</button>
              <button onClick={onClose} className="px-4 py-1.5 bg-gray-700 text-gray-300 rounded text-sm hover:bg-gray-600">Cancel</button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
