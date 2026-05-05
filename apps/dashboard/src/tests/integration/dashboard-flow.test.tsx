import { cleanup, render, screen } from "@testing-library/react";
import LoginPage from "@/app/(auth)/login/page";
import DevicesPage from "@/app/(dashboard)/devices/page";
import PairDevicePage from "@/app/(dashboard)/pair-device/page";
import DeviceDetailPage from "@/app/(dashboard)/devices/[deviceId]/page";

vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: vi.fn(),
    refresh: vi.fn(),
  }),
}));

vi.mock("@/lib/dashboard-api", () => ({
  getDevices: vi.fn().mockResolvedValue([
    {
      id: 7,
      device_uuid: "device-7",
      device_name: "Office PC",
      status: "online",
      last_seen_at: "2026-05-05T12:00:00.000Z",
      paired_at: "2026-05-05T10:00:00.000Z",
    },
  ]),
  getDevice: vi.fn().mockResolvedValue({
    id: 7,
    device_uuid: "device-7",
    device_name: "Office PC",
    status: "online",
    last_seen_at: "2026-05-05T12:00:00.000Z",
    paired_at: "2026-05-05T10:00:00.000Z",
  }),
  getDeviceCommands: vi.fn().mockResolvedValue([
    {
      id: 1,
      device_id: 7,
      command_type: "shutdown",
      status: "succeeded",
      payload: {},
      rejection_reason: null,
      dispatched_at: null,
      acknowledged_at: null,
      executed_at: null,
      result_message: "Completed",
      result_code: null,
    },
  ]),
  getDeviceAuditLogs: vi.fn().mockResolvedValue([
    {
      id: 1,
      event_type: "device_paired",
      actor_type: "user",
      actor_id: 1,
      device_id: 7,
      event_payload: { device_uuid: "device-7" },
      created_at: "2026-05-05T12:00:00.000Z",
    },
  ]),
}));

describe("dashboard flow", () => {
  it("renders the main parent journey entry points", async () => {
    render(<LoginPage />);
    expect(
      screen.getByText("Sign in to PC Guard Dashboard"),
    ).toBeInTheDocument();

    cleanup();

    render(await DevicesPage());
    expect(screen.getByText("Linked devices")).toBeInTheDocument();
    expect(screen.getByText("Office PC")).toBeInTheDocument();

    cleanup();

    render(<PairDevicePage />);
    expect(screen.getByText("Link a child device")).toBeInTheDocument();

    cleanup();

    render(
      await DeviceDetailPage({
        params: Promise.resolve({ deviceId: "7" }),
      }),
    );
    expect(screen.getByText("Office PC")).toBeInTheDocument();
    expect(screen.getByText("device_paired")).toBeInTheDocument();
  });
});
