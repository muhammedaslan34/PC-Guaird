import type { DashboardCommand } from "@/lib/dashboard-types";

type CommandHistoryProps = {
  commands: DashboardCommand[];
};

export function CommandHistory({ commands }: CommandHistoryProps) {
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
          History
        </p>
        <h2 style={{ margin: "0.35rem 0 0", fontSize: "1.35rem" }}>
          Recent commands
        </h2>
      </div>

      <div style={{ display: "grid", gap: "0.75rem" }}>
        {commands.map((command) => (
          <article
            key={command.id}
            style={{
              display: "grid",
              gap: "0.35rem",
              border: "1px solid #e2e8f0",
              borderRadius: "1rem",
              padding: "0.95rem",
              background: "#ffffff",
            }}
          >
            <strong style={{ textTransform: "capitalize" }}>
              {command.command_type}
            </strong>
            <span>{command.status}</span>
            <span style={{ color: "#64748b", fontSize: "0.92rem" }}>
              {command.result_message ?? "No result message."}
            </span>
          </article>
        ))}
      </div>
    </section>
  );
}
