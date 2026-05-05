import { POST as pairDeviceRoute } from "@/app/api/devices/pair/route";
import { AUTH_COOKIE_NAME } from "@/lib/auth";

describe("pair device route", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    vi.stubEnv("BACKEND_URL", "http://backend.test");
  });

  it("forwards pairing requests to Laravel using the auth cookie token", async () => {
    const fetchSpy = vi.spyOn(global, "fetch").mockResolvedValue(
      new Response(
        JSON.stringify({
          device: {
            id: 1,
            device_uuid: "device-pair-1",
            device_name: "Family PC",
          },
          token: "device-token",
        }),
        {
          status: 200,
          headers: { "Content-Type": "application/json" },
        },
      ),
    );

    const response = await pairDeviceRoute(
      new Request("http://dashboard.test/api/devices/pair", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          cookie: `${AUTH_COOKIE_NAME}=secure-token`,
        },
        body: JSON.stringify({
          code: "483921",
          device_name: "Family PC",
        }),
      }),
    );

    expect(fetchSpy).toHaveBeenCalledWith(
      "http://backend.test/api/devices/pair",
      expect.objectContaining({
        method: "POST",
        headers: expect.any(Headers),
      }),
    );
    expect(response.status).toBe(200);
    expect(await response.json()).toEqual({
      device: {
        id: 1,
        device_uuid: "device-pair-1",
        device_name: "Family PC",
      },
      token: "device-token",
    });
  });
});
