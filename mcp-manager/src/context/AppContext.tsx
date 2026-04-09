import { createContext, useContext, useState, useCallback, useEffect, type ReactNode } from "react";
import type { UnifiedServer, DetectedTool, Preferences } from "../types";
import { scanConfigs, getPreferences } from "../lib/invoke";

interface AppState {
  servers: UnifiedServer[];
  tools: DetectedTool[];
  preferences: Preferences;
  loading: boolean;
  error: string | null;
  lastScanTime: Date | null;
  selectedServerId: string | null;
  rescan: () => Promise<void>;
  setSelectedServerId: (id: string | null) => void;
  setPreferences: (prefs: Preferences) => void;
}

const AppContext = createContext<AppState | null>(null);

export function AppProvider({ children }: { children: ReactNode }) {
  const [servers, setServers] = useState<UnifiedServer[]>([]);
  const [tools, setTools] = useState<DetectedTool[]>([]);
  const [preferences, setPreferencesState] = useState<Preferences>({
    auto_scan_on_launch: true, backup_retention_days: 30,
  });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [lastScanTime, setLastScanTime] = useState<Date | null>(null);
  const [selectedServerId, setSelectedServerId] = useState<string | null>(null);

  const rescan = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await scanConfigs();
      setServers(result.servers);
      setTools(result.tools);
      setLastScanTime(new Date());
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    getPreferences().then(setPreferencesState).catch(() => {});
    rescan();
  }, [rescan]);

  return (
    <AppContext.Provider value={{
      servers, tools, preferences, loading, error, lastScanTime,
      selectedServerId, rescan, setSelectedServerId,
      setPreferences: setPreferencesState,
    }}>
      {children}
    </AppContext.Provider>
  );
}

export function useAppContext() {
  const ctx = useContext(AppContext);
  if (!ctx) throw new Error("useAppContext must be used within AppProvider");
  return ctx;
}
