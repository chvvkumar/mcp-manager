import { useState, useEffect } from "react";
import { useAppContext } from "../context/AppContext";
import { listBackups, restoreBackup, savePreferences } from "../lib/invoke";
import type { BackupEntry } from "../types";

interface SettingsDialogProps { onClose: () => void; }

export function SettingsDialog({ onClose }: SettingsDialogProps) {
  const { tools, preferences, setPreferences, rescan } = useAppContext();
  const [autoScan, setAutoScan] = useState(preferences.auto_scan_on_launch);
  const [retentionDays, setRetentionDays] = useState(preferences.backup_retention_days);
  const [backups, setBackups] = useState<Record<string, BackupEntry[]>>({});
  const [selectedTool, setSelectedTool] = useState<string>("");

  useEffect(() => {
    if (selectedTool) {
      listBackups(selectedTool).then((b) => setBackups((prev) => ({ ...prev, [selectedTool]: b })));
    }
  }, [selectedTool]);

  const handleSave = async () => {
    const prefs = { auto_scan_on_launch: autoScan, backup_retention_days: retentionDays };
    await savePreferences(prefs);
    setPreferences(prefs);
    onClose();
  };

  const handleRestore = async (backup: BackupEntry) => {
    if (confirm(`Restore ${backup.tool_id} config from ${backup.timestamp}?`)) {
      await restoreBackup(backup.tool_id, backup.file_path);
      await rescan();
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 w-[500px] max-h-[80vh] overflow-y-auto">
        <h2 className="text-lg font-semibold mb-4">Settings</h2>
        <div className="space-y-3 mb-6">
          <label className="flex items-center gap-3">
            <input type="checkbox" checked={autoScan} onChange={(e) => setAutoScan(e.target.checked)} />
            <span className="text-sm">Auto-scan on launch</span>
          </label>
          <div>
            <label className="block text-xs text-gray-400 mb-1">Backup retention (days)</label>
            <input type="number" value={retentionDays} onChange={(e) => setRetentionDays(Number(e.target.value))}
              className="w-24 px-2 py-1 bg-gray-700 border border-gray-600 rounded text-sm" />
          </div>
        </div>
        <div className="mb-4">
          <h3 className="text-sm font-medium text-gray-400 mb-2">Backup & Restore</h3>
          <select value={selectedTool} onChange={(e) => setSelectedTool(e.target.value)}
            className="w-full px-2 py-1.5 bg-gray-700 border border-gray-600 rounded text-sm mb-2">
            <option value="">Select tool to view backups...</option>
            {tools.map((t) => (<option key={t.tool_id} value={t.tool_id}>{t.display_name}</option>))}
          </select>
          {selectedTool && backups[selectedTool] && (
            <div className="space-y-1 max-h-48 overflow-y-auto">
              {backups[selectedTool].length === 0 ? (
                <p className="text-sm text-gray-500">No backups found.</p>
              ) : (
                backups[selectedTool].map((b) => (
                  <div key={b.file_path} className="flex items-center justify-between py-1 px-2 bg-gray-700/50 rounded">
                    <span className="text-xs text-gray-300">{b.timestamp}</span>
                    <button onClick={() => handleRestore(b)} className="text-xs text-blue-400 hover:text-blue-300">Restore</button>
                  </div>
                ))
              )}
            </div>
          )}
        </div>
        <div className="flex gap-2">
          <button onClick={handleSave} className="px-4 py-1.5 bg-blue-600 text-white rounded text-sm hover:bg-blue-500">Save</button>
          <button onClick={onClose} className="px-4 py-1.5 bg-gray-700 text-gray-300 rounded text-sm hover:bg-gray-600">Cancel</button>
        </div>
      </div>
    </div>
  );
}
