import { render, screen } from "@testing-library/react";
import DevicesPage from "@/app/(dashboard)/devices/page";

vi.mock("@/lib/dashboard-api", () => ({
  getDevices: vi.fn().mockResolvedValue([
    {
      id: 1,
      device_uuid: "device-1",
      device_name: "Family PC",
      status: "online",
      last_seen_at: "2026-05-05T12:00:00.000Z",
      paired_at: "2026-05-05T11:00:00.000Z",
    },
  ]),
}));

describe("device list page", () => {
  it("renders linked devices from the server data layer", async () => {
    const page = await DevicesPage();

    render(page);

    expect(screen.getByText("Linked devices")).toBeInTheDocument();
    expect(screen.getByText("Family PC")).toBeInTheDocument();
    expect(screen.getByText("online")).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Pair a device" })).toHaveAttribute(
      "href",
      "/pair-device",
    );
  });
});
