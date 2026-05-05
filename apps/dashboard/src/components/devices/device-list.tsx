import Link from "next/link";
import type { DashboardDevice } from "@/lib/dashboard-types";

type DeviceListProps = {
  devices: DashboardDevice[];
};

export function DeviceList({ devices }: DeviceListProps) {
  return (
    <section
      style={{
        display: "grid",
        gap: "1rem",
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
              textTransform: "uppercase",
              letterSpacing: "0.16em",
              fontSize: "0.75rem",
              color: "#64748b",
            }}
          >
            Control Center
          </p>
          <h1 style={{ margin: "0.35rem 0 0", fontSize: "2rem" }}>
            Linked devices
          </h1>
        </div>

        <Link
          href="/pair-device"
          style={{
            borderRadius: "999px",
            padding: "0.85rem 1.1rem",
            textDecoration: "none",
            background: "#0f172a",
            color: "#ffffff",
          }}
        >
          Pair a device
        </Link>
      </div>

      <div
        style={{
          display: "grid",
          gap: "0.85rem",
        }}
      >
        {devices.map((device) => (
          <Link
            key={device.id}
            href={`/devices/${device.id}`}
            style={{
              display: "grid",
              gap: "0.45rem",
              padding: "1rem 1.1rem",
              borderRadius: "1rem",
              border: "1px solid #e2e8f0",
              background: "#ffffff",
              textDecoration: "none",
              color: "inherit",
            }}
          >
            <strong style={{ fontSize: "1.05rem" }}>
              {device.device_name ?? "Unnamed device"}
            </strong>
            <span style={{ color: "#475569" }}>{device.status}</span>
            <span style={{ color: "#64748b", fontSize: "0.92rem" }}>
              Last seen: {device.last_seen_at ?? "Never"}
            </span>
          </Link>
        ))}
      </div>
    </section>
  );
}
