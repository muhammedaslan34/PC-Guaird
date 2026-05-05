import { render, screen } from "@testing-library/react";
import DeviceDetailPage from "@/app/(dashboard)/devices/[deviceId]/page";

vi.mock("@/lib/dashboard-api", () => ({
  getDevice: vi.fn().mockResolvedValue({
    id: 7,
    device_uuid: "device-detail-7",
    device_name: "Office PC",
    status: "online",
    last_seen_at: "2026-05-05T12:00:00.000Z",
    paired_at: "2026-05-05T10:00:00.000Z",
  }),
  getDeviceCommands: vi.fn().mockResolvedValue([
    {
      id: 9,
      device_id: 7,
      command_type: "shutdown",
      status: "succeeded",
      payload: {},
      rejection_reason: null,
      dispatched_at: "2026-05-05T12:00:00.000Z",
      acknowledged_at: "2026-05-05T12:00:05.000Z",
      executed_at: "2026-05-05T12:00:10.000Z",
      result_message: "Completed",
      result_code: null,
    },
  ]),
  getDeviceAuditLogs: vi.fn().mockResolvedValue([
    {
      id: 4,
      event_type: "command_completed",
      actor_type: "device",
      actor_id: 7,
      device_id: 7,
      event_payload: { command_id: 9 },
      created_at: "2026-05-05T12:00:10.000Z",
    },
  ]),
}));

vi.mock("@/components/devices/device-polling-provider", () => ({
  DevicePollingProvider: ({
    initialDevice,
    initialCommands,
    initialAuditLogs,
  }: {
    initialDevice: { device_name: string; status: string };
    initialCommands: Array<{ command_type: string }>;
    initialAuditLogs: Array<{ event_type: string }>;
  }) => (
    <div>
      <h1>{initialDevice.device_name}</h1>
      <span>{initialDevice.status}</span>
      <span>{initialCommands[0].command_type}</span>
      <span>{initialAuditLogs[0].event_type}</span>
    </div>
  ),
}));

describe("device detail page", () => {
  it("loads the control-center data on the server before render", async () => {
    const page = await DeviceDetailPage({
      params: Promise.resolve({ deviceId: "7" }),
    });

    render(page);

    expect(screen.getByText("Office PC")).toBeInTheDocument();
    expect(screen.getByText("online")).toBeInTheDocument();
    expect(screen.getByText("shutdown")).toBeInTheDocument();
    expect(screen.getByText("command_completed")).toBeInTheDocument();
  });
});
