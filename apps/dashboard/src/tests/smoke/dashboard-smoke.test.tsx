import { render, screen } from "@testing-library/react";
import Home from "@/app/page";

describe("dashboard scaffold", () => {
  it("renders the dashboard app marker", () => {
    render(<Home />);

    expect(screen.getByText("PC Guard Dashboard")).toBeInTheDocument();
  });
});
