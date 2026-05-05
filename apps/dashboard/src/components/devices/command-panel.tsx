"use client";

import { useState } from "react";
import type { DashboardCommand } from "@/lib/dashboard-types";

type CommandPanelProps = {
  deviceId: number;
  onCommandCreated: (command: DashboardCommand) => void;
};

const commandTypes = ["shutdown", "restart", "lock"] as const;

export function CommandPanel({
  deviceId,
  onCommandCreated,
}: CommandPanelProps) {
  const [error, setError] = useState<string | null>(null);
  const [submittingCommand, setSubmittingCommand] = useState<string | null>(
    null,
  );

  async function sendCommand(commandType: (typeof commandTypes)[number]) {
    setSubmittingCommand(commandType);
    setError(null);

    const response = await fetch(`/api/devices/${deviceId}/commands`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        command_type: commandType,
      }),
    });

    const payload = await response.json();

    if (!response.ok) {
      setError(
        payload.code === "device_offline"
          ? "This device is offline, so the command was not queued."
          : "Unable to send this command right now.",
      );
      setSubmittingCommand(null);
      return;
    }

    onCommandCreated(payload.data);
    setSubmittingCommand(null);
  }

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
          Commands
        </p>
        <h2 style={{ margin: "0.35rem 0 0", fontSize: "1.35rem" }}>
          Device actions
        </h2>
      </div>

      <div style={{ display: "flex", flexWrap: "wrap", gap: "0.75rem" }}>
        {commandTypes.map((commandType) => (
          <button
            key={commandType}
            type="button"
            disabled={submittingCommand !== null}
            onClick={() => void sendCommand(commandType)}
            style={{
              border: "1px solid #cbd5e1",
              borderRadius: "999px",
              padding: "0.8rem 1rem",
              background: "#ffffff",
              textTransform: "capitalize",
            }}
          >
            {submittingCommand === commandType
              ? "Sending..."
              : commandType}
          </button>
        ))}
      </div>

      {error ? (
        <p role="alert" style={{ margin: 0, color: "#b91c1c" }}>
          {error}
        </p>
      ) : null}
    </section>
  );
}
