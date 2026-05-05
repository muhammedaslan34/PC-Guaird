import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { PairDeviceForm } from "@/components/devices/pair-device-form";

const push = vi.fn();

vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push,
    refresh: vi.fn(),
  }),
}));

describe("pair device form", () => {
  beforeEach(() => {
    push.mockReset();
    vi.restoreAllMocks();
  });

  it("navigates to the new device detail page after a successful pairing", async () => {
    vi.spyOn(global, "fetch").mockResolvedValue(
      new Response(
        JSON.stringify({
          device: {
            id: 12,
            device_uuid: "device-pair-12",
            device_name: "Desk PC",
          },
        }),
        {
          status: 200,
          headers: { "Content-Type": "application/json" },
        },
      ),
    );

    render(<PairDeviceForm />);

    fireEvent.change(screen.getByLabelText("Pairing code"), {
      target: { value: "483921" },
    });
    fireEvent.change(screen.getByLabelText("Device name"), {
      target: { value: "Desk PC" },
    });
    fireEvent.submit(screen.getByRole("button", { name: "Pair device" }));

    await waitFor(() => {
      expect(push).toHaveBeenCalledWith("/devices/12");
    });
  });

  it("maps backend pairing errors to a direct message", async () => {
    vi.spyOn(global, "fetch").mockResolvedValue(
      new Response(JSON.stringify({ code: "expired_pairing_code" }), {
        status: 422,
        headers: { "Content-Type": "application/json" },
      }),
    );

    render(<PairDeviceForm />);

    fireEvent.change(screen.getByLabelText("Pairing code"), {
      target: { value: "483921" },
    });
    fireEvent.submit(screen.getByRole("button", { name: "Pair device" }));

    await waitFor(() => {
      expect(
        screen.getByText("This pairing code has expired. Request a new code."),
      ).toBeInTheDocument();
    });
  });
});
