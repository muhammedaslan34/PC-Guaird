import { NextRequest } from "next/server";
import { middleware } from "@/middleware";
import { AUTH_COOKIE_NAME } from "@/lib/auth";

describe("protected routing", () => {
  it("redirects unauthenticated users from /devices to /login", () => {
    const request = new NextRequest("http://dashboard.test/devices");

    const response = middleware(request);

    expect(response?.status).toBe(307);
    expect(response?.headers.get("location")).toBe(
      "http://dashboard.test/login",
    );
  });

  it("redirects authenticated users away from /login", () => {
    const request = new NextRequest("http://dashboard.test/login", {
      headers: {
        cookie: `${AUTH_COOKIE_NAME}=secure-token`,
      },
    });

    const response = middleware(request);

    expect(response?.status).toBe(307);
    expect(response?.headers.get("location")).toBe(
      "http://dashboard.test/devices",
    );
  });
});
