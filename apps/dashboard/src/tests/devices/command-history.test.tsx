import { render, screen } from "@testing-library/react";
import { CommandHistory } from "@/components/devices/command-history";

describe("command history", () => {
  it("renders command status and result details", () => {
    render(
      <CommandHistory
        commands={[
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
        ]}
      />,
    );

    expect(screen.getByText("shutdown")).toBeInTheDocument();
    expect(screen.getByText("succeeded")).toBeInTheDocument();
    expect(screen.getByText("Completed")).toBeInTheDocument();
  });
});
