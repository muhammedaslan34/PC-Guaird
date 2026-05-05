import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { CommandPanel } from "@/components/devices/command-panel";

describe("command panel", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("calls the create command route and reports the created command", async () => {
    const onCommandCreated = vi.fn();

    vi.spyOn(global, "fetch").mockResolvedValue(
      new Response(
        JSON.stringify({
          data: {
            id: 3,
            device_id: 7,
            command_type: "shutdown",
            status: "dispatched",
            payload: {},
            rejection_reason: null,
            dispatched_at: "2026-05-05T12:00:00.000Z",
            acknowledged_at: null,
            executed_at: null,
            result_message: null,
            result_code: null,
          },
        }),
        {
          status: 200,
          headers: { "Content-Type": "application/json" },
        },
      ),
    );

    render(<CommandPanel deviceId={7} onCommandCreated={onCommandCreated} />);

    fireEvent.click(screen.getByRole("button", { name: "shutdown" }));

    await waitFor(() => {
      expect(onCommandCreated).toHaveBeenCalledWith(
        expect.objectContaining({
          command_type: "shutdown",
        }),
      );
    });
  });

  it("shows the offline error returned by the backend", async () => {
    vi.spyOn(global, "fetch").mockResolvedValue(
      new Response(JSON.stringify({ code: "device_offline" }), {
        status: 422,
        headers: { "Content-Type": "application/json" },
      }),
    );

    render(<CommandPanel deviceId={7} onCommandCreated={vi.fn()} />);

    fireEvent.click(screen.getByRole("button", { name: "shutdown" }));

    await waitFor(() => {
      expect(
        screen.getByText(
          "This device is offline, so the command was not queued.",
        ),
      ).toBeInTheDocument();
    });
  });
});
