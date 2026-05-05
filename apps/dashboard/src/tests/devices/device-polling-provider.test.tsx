import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { DevicePollingProvider } from "@/components/devices/device-polling-provider";

describe("device polling provider", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("refreshes device state from dashboard proxy routes", async () => {
    vi.spyOn(global, "fetch")
      .mockResolvedValueOnce(
        new Response(
          JSON.stringify({
            data: {
              id: 7,
              device_uuid: "device-detail-7",
              device_name: "Office PC",
              status: "offline",
              last_seen_at: "2026-05-05T12:01:00.000Z",
              paired_at: "2026-05-05T10:00:00.000Z",
            },
          }),
          { status: 200, headers: { "Content-Type": "application/json" } },
        ),
      )
      .mockResolvedValueOnce(
        new Response(JSON.stringify({ data: [] }), {
          status: 200,
          headers: { "Content-Type": "application/json" },
        }),
      )
      .mockResolvedValueOnce(
        new Response(JSON.stringify({ data: [] }), {
          status: 200,
          headers: { "Content-Type": "application/json" },
        }),
      );

    render(
      <DevicePollingProvider
        deviceId={7}
        initialDevice={{
          id: 7,
          device_uuid: "device-detail-7",
          device_name: "Office PC",
          status: "online",
          last_seen_at: "2026-05-05T12:00:00.000Z",
          paired_at: "2026-05-05T10:00:00.000Z",
        }}
        initialCommands={[]}
        initialAuditLogs={[]}
      />,
    );

    expect(screen.getByText("online")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Refresh now" }));

    await waitFor(() => {
      expect(screen.getByText("offline")).toBeInTheDocument();
    });
  });
});
