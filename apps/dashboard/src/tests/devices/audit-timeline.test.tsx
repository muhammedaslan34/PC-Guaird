import { render, screen } from "@testing-library/react";
import { AuditTimeline } from "@/components/devices/audit-timeline";

describe("audit timeline", () => {
  it("renders audit events for the device timeline", () => {
    render(
      <AuditTimeline
        auditLogs={[
          {
            id: 1,
            event_type: "device_paired",
            actor_type: "user",
            actor_id: 2,
            device_id: 7,
            event_payload: { device_uuid: "device-7" },
            created_at: "2026-05-05T12:00:00.000Z",
          },
        ]}
      />,
    );

    expect(screen.getByText("device_paired")).toBeInTheDocument();
    expect(screen.getByText("user")).toBeInTheDocument();
  });
});
