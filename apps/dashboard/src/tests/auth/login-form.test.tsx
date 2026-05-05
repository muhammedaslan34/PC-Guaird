import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { LoginForm } from "@/components/auth/login-form";

const push = vi.fn();

vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push,
    refresh: vi.fn(),
  }),
}));

describe("login form", () => {
  beforeEach(() => {
    push.mockReset();
    vi.restoreAllMocks();
  });

  it("shows an error returned by the dashboard auth route", async () => {
    vi.spyOn(global, "fetch").mockResolvedValue(
      new Response(
        JSON.stringify({
          message: "The provided credentials are incorrect.",
        }),
        {
          status: 422,
          headers: { "Content-Type": "application/json" },
        },
      ),
    );

    render(<LoginForm />);

    fireEvent.change(screen.getByLabelText("Email"), {
      target: { value: "parent@example.com" },
    });
    fireEvent.change(screen.getByLabelText("Password"), {
      target: { value: "wrong-password" },
    });
    fireEvent.submit(screen.getByRole("button", { name: "Sign in" }));

    await waitFor(() => {
      expect(
        screen.getByText("The provided credentials are incorrect."),
      ).toBeInTheDocument();
    });
  });

  it("navigates to /devices after a successful login", async () => {
    vi.spyOn(global, "fetch").mockResolvedValue(
      new Response(
        JSON.stringify({
          user: { id: 1, name: "Parent", email: "parent@example.com" },
        }),
        {
          status: 200,
          headers: { "Content-Type": "application/json" },
        },
      ),
    );

    render(<LoginForm />);

    fireEvent.change(screen.getByLabelText("Email"), {
      target: { value: "parent@example.com" },
    });
    fireEvent.change(screen.getByLabelText("Password"), {
      target: { value: "secret1234" },
    });
    fireEvent.submit(screen.getByRole("button", { name: "Sign in" }));

    await waitFor(() => {
      expect(push).toHaveBeenCalledWith("/devices");
    });
  });
});
