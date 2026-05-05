"use client";

import { useCallback, useEffect, useState } from "react";
import { DeviceControlCenter } from "@/components/devices/device-control-center";
import type {
  DashboardAuditLog,
  DashboardCommand,
  DashboardDevice,
} from "@/lib/dashboard-types";

type DevicePollingProviderProps = {
  deviceId: number;
  initialDevice: DashboardDevice;
  initialCommands: DashboardCommand[];
  initialAuditLogs: DashboardAuditLog[];
};

export function DevicePollingProvider({
  deviceId,
  initialDevice,
  initialCommands,
  initialAuditLogs,
}: DevicePollingProviderProps) {
  const [device, setDevice] = useState(initialDevice);
  const [commands, setCommands] = useState(initialCommands);
  const [auditLogs, setAuditLogs] = useState(initialAuditLogs);
  const [stale, setStale] = useState(false);

  const refreshData = useCallback(async () => {
    try {
      const [deviceResponse, commandsResponse, auditResponse] =
        await Promise.all([
          fetch(`/api/devices/${deviceId}`),
          fetch(`/api/devices/${deviceId}/commands`),
          fetch(`/api/devices/${deviceId}/audit-logs`),
        ]);

      const [devicePayload, commandsPayload, auditPayload] = await Promise.all([
        deviceResponse.json(),
        commandsResponse.json(),
        auditResponse.json(),
      ]);

      setDevice(devicePayload.data);
      setCommands(commandsPayload.data);
      setAuditLogs(auditPayload.data);
      setStale(false);
    } catch {
      setStale(true);
    }
  }, [deviceId]);

  useEffect(() => {
    const interval = window.setInterval(() => {
      void refreshData();
    }, 15000);

    return () => window.clearInterval(interval);
  }, [refreshData]);

  return (
    <DeviceControlCenter
      device={device}
      commands={commands}
      auditLogs={auditLogs}
      stale={stale}
      onRefresh={() => void refreshData()}
      onCommandCreated={(command) => {
        setCommands((current) => [command, ...current]);
      }}
    />
  );
}
