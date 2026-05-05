import { GET as devicesRoute } from "@/app/api/devices/route";
import { AUTH_COOKIE_NAME } from "@/lib/auth";

describe("devices route", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    vi.stubEnv("BACKEND_URL", "http://backend.test");
  });

  it("proxies the linked device list through the Laravel API", async () => {
    const fetchSpy = vi.spyOn(global, "fetch").mockResolvedValue(
      new Response(
        JSON.stringify({
          data: [
            {
              id: 1,
              device_uuid: "device-1",
              device_name: "Family PC",
              status: "online",
              last_seen_at: "2026-05-05T12:00:00.000000Z",
              paired_at: "2026-05-05T11:00:00.000000Z",
            },
          ],
        }),
        {
          status: 200,
          headers: { "Content-Type": "application/json" },
        },
      ),
    );

    const response = await devicesRoute(
      new Request("http://dashboard.test/api/devices", {
        headers: {
          cookie: `${AUTH_COOKIE_NAME}=secure-token`,
        },
      }),
    );

    expect(fetchSpy).toHaveBeenCalledWith(
      "http://backend.test/api/devices",
      expect.objectContaining({
        headers: expect.any(Headers),
      }),
    );
    expect(response.status).toBe(200);
    expect(await response.json()).toEqual({
      data: [
        {
          id: 1,
          device_uuid: "device-1",
          device_name: "Family PC",
          status: "online",
          last_seen_at: "2026-05-05T12:00:00.000000Z",
          paired_at: "2026-05-05T11:00:00.000000Z",
        },
      ],
    });
  });
});
