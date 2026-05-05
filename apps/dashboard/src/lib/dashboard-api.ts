import { backendFetch } from "@/lib/backend-client";
import { getStoredAuthToken } from "@/lib/auth";
import type { DashboardDevice } from "@/lib/dashboard-types";

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
