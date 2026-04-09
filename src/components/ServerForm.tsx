import { useState } from "react";
import { MaskedValue } from "./MaskedValue";
import type { ServerEntry, Transport } from "../types";

interface ServerFormProps { initial?: Partial<ServerEntry>; onSave: (entry: ServerEntry) => void; onCancel: () => void; }

export function ServerForm({ initial, onSave, onCancel }: ServerFormProps) {
  const [name, setName] = useState(initial?.name ?? "");
  const [transport, setTransport] = useState<Transport>(initial?.transport ?? "stdio");
  const [command, setCommand] = useState(initial?.command ?? "");
  const [args, setArgs] = useState(initial?.args?.join(" ") ?? "");
  const [url, setUrl] = useState(initial?.url ?? "");
  const [envEntries, setEnvEntries] = useState<[string, string][]>(Object.entries(initial?.env ?? {}));
  const [headerEntries, setHeaderEntries] = useState<[string, string][]>(Object.entries(initial?.headers ?? {}));

  const handleSubmit = () => {
    const entry: ServerEntry = {
      name, transport,
      ...(transport === "stdio"
        ? { command, args: args.split(/\s+/).filter(Boolean), env: envEntries.length > 0 ? Object.fromEntries(envEntries) : undefined }
        : { url, headers: headerEntries.length > 0 ? Object.fromEntries(headerEntries) : undefined }),
    };
    onSave(entry);
  };

  return (
    <div className="space-y-4">
      <div>
        <label className="block text-xs text-gray-400 mb-1">Server Name</label>
        <input value={name} onChange={(e) => setName(e.target.value)} className="w-full px-2 py-1.5 bg-gray-700 border border-gray-600 rounded text-sm" />
      </div>
      <div>
        <label className="block text-xs text-gray-400 mb-1">Transport</label>
        <select value={transport} onChange={(e) => setTransport(e.target.value as Transport)} className="w-full px-2 py-1.5 bg-gray-700 border border-gray-600 rounded text-sm">
          <option value="stdio">stdio (local command)</option>
          <option value="http">http (remote URL)</option>
        </select>
      </div>
      {transport === "stdio" && (
        <>
          <div>
            <label className="block text-xs text-gray-400 mb-1">Command</label>
            <input value={command} onChange={(e) => setCommand(e.target.value)} className="w-full px-2 py-1.5 bg-gray-700 border border-gray-600 rounded text-sm" placeholder="e.g., npx, docker, node" />
          </div>
          <div>
            <label className="block text-xs text-gray-400 mb-1">Arguments (space-separated)</label>
            <input value={args} onChange={(e) => setArgs(e.target.value)} className="w-full px-2 py-1.5 bg-gray-700 border border-gray-600 rounded text-sm" placeholder="e.g., -y @modelcontextprotocol/server-github" />
          </div>
          <div>
            <label className="block text-xs text-gray-400 mb-1">Environment Variables</label>
            {envEntries.map(([key, val], i) => (
              <div key={i} className="flex gap-2 mb-1">
                <input value={key} onChange={(e) => { const next = [...envEntries]; next[i] = [e.target.value, val]; setEnvEntries(next); }}
                  className="flex-1 px-2 py-1 bg-gray-700 border border-gray-600 rounded text-sm" placeholder="KEY" />
                <MaskedValue label={key} value={val} onChange={(v) => { const next = [...envEntries]; next[i] = [key, v]; setEnvEntries(next); }} />
                <button onClick={() => setEnvEntries(envEntries.filter((_, j) => j !== i))} className="text-red-400 hover:text-red-300 text-sm">x</button>
              </div>
            ))}
            <button onClick={() => setEnvEntries([...envEntries, ["", ""]])} className="text-sm text-blue-400 hover:text-blue-300">+ Add env var</button>
          </div>
        </>
      )}
      {transport === "http" && (
        <>
          <div>
            <label className="block text-xs text-gray-400 mb-1">URL</label>
            <input value={url} onChange={(e) => setUrl(e.target.value)} className="w-full px-2 py-1.5 bg-gray-700 border border-gray-600 rounded text-sm" placeholder="https://mcp.example.com/mcp" />
          </div>
          <div>
            <label className="block text-xs text-gray-400 mb-1">Headers</label>
            {headerEntries.map(([key, val], i) => (
              <div key={i} className="flex gap-2 mb-1">
                <input value={key} onChange={(e) => { const next = [...headerEntries]; next[i] = [e.target.value, val]; setHeaderEntries(next); }}
                  className="flex-1 px-2 py-1 bg-gray-700 border border-gray-600 rounded text-sm" placeholder="Header-Name" />
                <MaskedValue label={key} value={val} onChange={(v) => { const next = [...headerEntries]; next[i] = [key, v]; setHeaderEntries(next); }} />
                <button onClick={() => setHeaderEntries(headerEntries.filter((_, j) => j !== i))} className="text-red-400 hover:text-red-300 text-sm">x</button>
              </div>
            ))}
            <button onClick={() => setHeaderEntries([...headerEntries, ["", ""]])} className="text-sm text-blue-400 hover:text-blue-300">+ Add header</button>
          </div>
        </>
      )}
      <div className="flex gap-2 pt-2">
        <button onClick={handleSubmit} disabled={!name} className="px-4 py-1.5 bg-blue-600 text-white rounded text-sm hover:bg-blue-500 disabled:opacity-50">Save</button>
        <button onClick={onCancel} className="px-4 py-1.5 bg-gray-700 text-gray-300 rounded text-sm hover:bg-gray-600">Cancel</button>
      </div>
    </div>
  );
}
