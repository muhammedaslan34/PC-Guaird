import type { DashboardAuditLog } from "@/lib/dashboard-types";

type AuditTimelineProps = {
  auditLogs: DashboardAuditLog[];
};

export function AuditTimeline({ auditLogs }: AuditTimelineProps) {
  return (
    <section style={{ display: "grid", gap: "0.75rem" }}>
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
          Audit
        </p>
        <h2 style={{ margin: "0.35rem 0 0", fontSize: "1.35rem" }}>
          Activity timeline
        </h2>
      </div>

      <div style={{ display: "grid", gap: "0.75rem" }}>
        {auditLogs.map((log) => (
          <article
            key={log.id}
            style={{
              display: "grid",
              gap: "0.35rem",
              border: "1px solid #e2e8f0",
              borderRadius: "1rem",
              padding: "0.95rem",
              background: "#ffffff",
            }}
          >
            <strong>{log.event_type}</strong>
            <span style={{ color: "#64748b", fontSize: "0.92rem" }}>
              {log.actor_type}
            </span>
          </article>
        ))}
      </div>
    </section>
  );
}
