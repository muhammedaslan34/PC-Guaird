import { POST as logoutRoute } from "@/app/api/auth/logout/route";
import { AUTH_COOKIE_NAME } from "@/lib/auth";

describe("logout route", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    vi.stubEnv("BACKEND_URL", "http://backend.test");
  });

  it("forwards logout to Laravel and clears the auth cookie", async () => {
    vi.spyOn(global, "fetch").mockResolvedValue(
      new Response(JSON.stringify({ message: "Logged out." }), {
        status: 200,
        headers: { "Content-Type": "application/json" },
      }),
    );

    const response = await logoutRoute(
      new Request("http://dashboard.test/api/auth/logout", {
        method: "POST",
        headers: {
          cookie: `${AUTH_COOKIE_NAME}=plain-laravel-token`,
        },
      }),
    );

    expect(response.status).toBe(200);
    expect(await response.json()).toEqual({ message: "Logged out." });
    expect(response.headers.get("set-cookie")).toContain(`${AUTH_COOKIE_NAME}=;`);
  });
});
