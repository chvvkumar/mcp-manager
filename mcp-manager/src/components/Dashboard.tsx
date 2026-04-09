import { useAppContext } from "../context/AppContext";
import { ServerRow } from "./ServerRow";

export function Dashboard() {
  const { servers, tools, loading, error } = useAppContext();

  if (loading) {
    return <div className="flex items-center justify-center h-full text-gray-400">Scanning for MCP configurations...</div>;
  }
  if (error) {
    return <div className="flex items-center justify-center h-full text-red-400">Error: {error}</div>;
  }
  if (servers.length === 0) {
    return <div className="flex items-center justify-center h-full text-gray-400">No MCP servers found. Add one using the + button.</div>;
  }

  return (
    <div className="overflow-auto flex-1">
      <table className="w-full text-left">
        <thead className="bg-gray-800 sticky top-0">
          <tr>
            <th className="px-3 py-2 text-xs text-gray-400 font-medium w-12">All</th>
            <th className="px-3 py-2 text-xs text-gray-400 font-medium">Server</th>
            {tools.map((tool) => (
              <th key={tool.tool_id} className="px-3 py-2 text-xs text-gray-400 font-medium text-center whitespace-nowrap" title={tool.config_path}>
                {tool.display_name}
              </th>
            ))}
            <th className="px-3 py-2 text-xs text-gray-400 font-medium w-16"></th>
          </tr>
        </thead>
        <tbody>
          {servers.map((server) => (
            <ServerRow key={server.id} server={server} detectedTools={tools} />
          ))}
        </tbody>
      </table>
    </div>
  );
}
