import { useAppContext } from "../context/AppContext";

export function TopBar({ onSettingsClick }: { onSettingsClick: () => void }) {
  const { lastScanTime, loading, rescan } = useAppContext();

  return (
    <div className="flex items-center justify-between px-6 py-3 bg-gray-900 border-b border-gray-700">
      <div className="flex items-center gap-3">
        <h1 className="text-lg font-semibold text-white">MCP Manager</h1>
      </div>
      <div className="flex items-center gap-4">
        <span className="text-sm text-gray-400">
          {lastScanTime ? `Last scan: ${lastScanTime.toLocaleTimeString()}` : "Not scanned"}
        </span>
        <button onClick={rescan} disabled={loading}
          className="px-3 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-500 disabled:opacity-50">
          {loading ? "Scanning..." : "Rescan"}
        </button>
        <button onClick={onSettingsClick}
          className="p-1.5 text-gray-400 hover:text-white rounded hover:bg-gray-700" title="Settings">
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
        </button>
      </div>
    </div>
  );
}
