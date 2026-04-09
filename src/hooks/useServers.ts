import { useCallback } from "react";
import { useAppContext } from "../context/AppContext";
import { updateServerBinding } from "../lib/invoke";
import type { ServerEntry } from "../types";

export function useServers() {
  const { servers, tools, rescan } = useAppContext();

  const toggleBinding = useCallback(
    async (serverName: string, toolId: string, currentStatus: "enabled" | "disabled", serverConfig?: ServerEntry) => {
      const action = currentStatus === "enabled" ? "disable" : "enable";
      await updateServerBinding(serverName, toolId, action, serverConfig);
      await rescan();
    }, [rescan]
  );

  const toggleGlobal = useCallback(
    async (serverId: string, enable: boolean) => {
      const server = servers.find((s) => s.id === serverId);
      if (!server) return;
      for (const binding of server.bindings) {
        const action = enable ? "enable" : "disable";
        await updateServerBinding(binding.server_name, binding.tool_id, action,
          enable ? {
            name: binding.server_name, transport: server.transport,
            command: server.command, args: server.args, env: server.env,
            url: server.url, headers: server.headers,
          } : undefined
        );
      }
      await rescan();
    }, [servers, rescan]
  );

  const addServer = useCallback(
    async (serverConfig: ServerEntry, toolIds: string[]) => {
      for (const toolId of toolIds) {
        await updateServerBinding(serverConfig.name, toolId, "add", serverConfig);
      }
      await rescan();
    }, [rescan]
  );

  const removeServer = useCallback(
    async (serverName: string, toolIds: string[]) => {
      for (const toolId of toolIds) {
        await updateServerBinding(serverName, toolId, "remove");
      }
      await rescan();
    }, [rescan]
  );

  return { servers, tools, toggleBinding, toggleGlobal, addServer, removeServer };
}
