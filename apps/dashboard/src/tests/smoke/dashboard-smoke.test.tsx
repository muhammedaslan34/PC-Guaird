import { render, screen } from "@testing-library/react";
import LoginPage from "@/app/(auth)/login/page";

vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: vi.fn(),
    refresh: vi.fn(),
  }),
}));

describe("dashboard scaffold", () => {
  it("renders the dashboard login shell", () => {
    render(<LoginPage />);

    expect(
      screen.getByText("Sign in to PC Guard Dashboard"),
    ).toBeInTheDocument();
  });
});
