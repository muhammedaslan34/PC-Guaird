import { POST as loginRoute } from "@/app/api/auth/login/route";
import { AUTH_COOKIE_NAME } from "@/lib/auth";

describe("login route", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    vi.stubEnv("BACKEND_URL", "http://backend.test");
  });

  it("stores the Laravel token in a secure cookie on successful login", async () => {
    vi.spyOn(global, "fetch").mockResolvedValue(
      new Response(
        JSON.stringify({
          user: { id: 1, name: "Parent", email: "parent@example.com" },
          token: "plain-laravel-token",
        }),
        {
          status: 200,
          headers: { "Content-Type": "application/json" },
        },
      ),
    );

    const response = await loginRoute(
      new Request("http://dashboard.test/api/auth/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          email: "parent@example.com",
          password: "secret1234",
        }),
      }),
    );

    expect(response.status).toBe(200);
    expect(await response.json()).toEqual({
      user: { id: 1, name: "Parent", email: "parent@example.com" },
    });
    expect(response.headers.get("set-cookie")).toContain(AUTH_COOKIE_NAME);
  });
});
