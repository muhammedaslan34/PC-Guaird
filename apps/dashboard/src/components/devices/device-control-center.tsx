import type {
  DashboardAuditLog,
  DashboardCommand,
  DashboardDevice,
} from "@/lib/dashboard-types";
import { AuditTimeline } from "@/components/devices/audit-timeline";
import { CommandHistory } from "@/components/devices/command-history";
import { CommandPanel } from "@/components/devices/command-panel";

type DeviceControlCenterProps = {
  device: DashboardDevice;
  commands: DashboardCommand[];
  auditLogs: DashboardAuditLog[];
  onCommandCreated: (command: DashboardCommand) => void;
  onRefresh: () => void;
  stale: boolean;
};

export function DeviceControlCenter({
  device,
  commands,
  auditLogs,
  onCommandCreated,
  onRefresh,
  stale,
}: DeviceControlCenterProps) {
  return (
    <main style={{ display: "grid", gap: "1.5rem" }}>
      <section
        style={{
          display: "grid",
          gap: "0.75rem",
          padding: "1.25rem",
          borderRadius: "1.25rem",
          background: "#ffffff",
          border: "1px solid #e2e8f0",
        }}
      >
        <div
          style={{
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
            gap: "1rem",
          }}
        >
          <div>
            <p
              style={{
                margin: 0,
                fontSize: "0.75rem",
                textTransform: "uppercase",
                letterSpacing: "0.16em",
                color: "#64748b",
              }}
            >
              Device
            </p>
            <h1 style={{ margin: "0.35rem 0 0", fontSize: "2rem" }}>
              {device.device_name ?? "Unnamed device"}
            </h1>
          </div>

          <button
            type="button"
            onClick={onRefresh}
            style={{
              border: "1px solid #cbd5e1",
              borderRadius: "999px",
              padding: "0.8rem 1rem",
              background: "#ffffff",
            }}
          >
            Refresh now
          </button>
        </div>

        <div style={{ display: "flex", gap: "1.25rem", flexWrap: "wrap" }}>
          <span>{device.status}</span>
          <span>Last seen: {device.last_seen_at ?? "Never"}</span>
        </div>

        {stale ? (
          <p role="status" style={{ margin: 0, color: "#b45309" }}>
            Data may be stale. The latest refresh did not complete.
          </p>
        ) : null}
      </section>

      <CommandPanel deviceId={device.id} onCommandCreated={onCommandCreated} />
      <CommandHistory commands={commands} />
      <AuditTimeline auditLogs={auditLogs} />
    </main>
  );
}
