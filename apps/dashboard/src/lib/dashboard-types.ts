export type DashboardDevice = {
  id: number;
  device_uuid: string;
  device_name: string | null;
  status: string;
  last_seen_at: string | null;
  paired_at: string | null;
};

export type DashboardCommand = {
  id: number;
  device_id: number;
  command_type: string;
  status: string;
  payload: Record<string, unknown>;
  rejection_reason: string | null;
  dispatched_at: string | null;
  acknowledged_at: string | null;
  executed_at: string | null;
  result_message: string | null;
  result_code: string | null;
};

export type DashboardAuditLog = {
  id: number;
  event_type: string;
  actor_type: string;
  actor_id: number | null;
  device_id: number | null;
  event_payload: Record<string, unknown> | null;
  created_at: string | null;
};
