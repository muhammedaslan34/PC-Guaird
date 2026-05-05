import { backendFetch } from "@/lib/backend-client";
import { getStoredAuthToken } from "@/lib/auth";
import type {
  DashboardAuditLog,
  DashboardCommand,
  DashboardDevice,
} from "@/lib/dashboard-types";

export async function getDevices(): Promise<DashboardDevice[]> {
  const token = await getStoredAuthToken();

  if (!token) {
    return [];
  }

  const response = await backendFetch("/api/devices", {
    method: "GET",
    token,
  });

  if (!response.ok) {
    throw new Error("Failed to load devices.");
  }

  const payload = (await response.json()) as { data: DashboardDevice[] };

  return payload.data;
}

export async function getDevice(deviceId: number): Promise<DashboardDevice> {
  const token = await getStoredAuthToken();

  if (!token) {
    throw new Error("Missing auth token.");
  }

  const response = await backendFetch(`/api/devices/${deviceId}`, {
    method: "GET",
    token,
  });

  if (!response.ok) {
    throw new Error("Failed to load device.");
  }

  const payload = (await response.json()) as { data: DashboardDevice };

  return payload.data;
}

export async function getDeviceCommands(
  deviceId: number,
): Promise<DashboardCommand[]> {
  const token = await getStoredAuthToken();

  if (!token) {
    return [];
  }

  const response = await backendFetch(`/api/devices/${deviceId}/commands`, {
    method: "GET",
    token,
  });

  if (!response.ok) {
    throw new Error("Failed to load device commands.");
  }

  const payload = (await response.json()) as { data: DashboardCommand[] };

  return payload.data;
}

export async function getDeviceAuditLogs(
  deviceId: number,
): Promise<DashboardAuditLog[]> {
  const token = await getStoredAuthToken();

  if (!token) {
    return [];
  }

  const response = await backendFetch(`/api/devices/${deviceId}/audit-logs`, {
    method: "GET",
    token,
  });

  if (!response.ok) {
    throw new Error("Failed to load device audit logs.");
  }

  const payload = (await response.json()) as { data: DashboardAuditLog[] };

  return payload.data;
}
