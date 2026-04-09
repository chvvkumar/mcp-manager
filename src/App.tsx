import { useState, useEffect } from "react";
import { TopBar } from "./components/TopBar";
import { Dashboard } from "./components/Dashboard";
import { ServerDetailPanel } from "./components/ServerDetailPanel";
import { AddServerDialog } from "./components/AddServerDialog";
import { DeleteDialog } from "./components/DeleteDialog";
import { SettingsDialog } from "./components/SettingsDialog";
import { useAppContext } from "./context/AppContext";
import type { UnifiedServer } from "./types";

export default function App() {
  const [showSettings, setShowSettings] = useState(false);
  const [showAddDialog, setShowAddDialog] = useState(false);
  const [deleteTarget, setDeleteTarget] = useState<UnifiedServer | null>(null);
  const { selectedServerId } = useAppContext();

  useEffect(() => {
    const handler = (e: Event) => {
      setDeleteTarget((e as CustomEvent).detail as UnifiedServer);
    };
    window.addEventListener("delete-server", handler);
    return () => window.removeEventListener("delete-server", handler);
  }, []);

  return (
    <div className="flex flex-col h-screen bg-gray-900 text-gray-100">
      <TopBar onSettingsClick={() => setShowSettings(!showSettings)} />
      <div className="flex flex-1 overflow-hidden">
        <Dashboard />
        {selectedServerId && <ServerDetailPanel />}
      </div>
      <button onClick={() => setShowAddDialog(true)}
        className="fixed bottom-6 right-6 w-12 h-12 bg-blue-600 text-white rounded-full text-2xl shadow-lg hover:bg-blue-500 flex items-center justify-center"
        title="Add MCP Server">+</button>
      {showAddDialog && <AddServerDialog onClose={() => setShowAddDialog(false)} />}
      {showSettings && <SettingsDialog onClose={() => setShowSettings(false)} />}
      {deleteTarget && <DeleteDialog server={deleteTarget} onClose={() => setDeleteTarget(null)} />}
    </div>
  );
}
