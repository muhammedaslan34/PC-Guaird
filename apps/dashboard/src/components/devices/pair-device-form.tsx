"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

const pairingErrorMessages: Record<string, string> = {
  invalid_pairing_code: "This pairing code is invalid. Check the child device and try again.",
  expired_pairing_code: "This pairing code has expired. Request a new code.",
  used_pairing_code: "This pairing code has already been used.",
};

export function PairDeviceForm() {
  const router = useRouter();
  const [code, setCode] = useState("");
  const [deviceName, setDeviceName] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitting(true);
    setError(null);

    const response = await fetch("/api/devices/pair", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        code,
        device_name: deviceName || undefined,
      }),
    });

    const payload = await response.json();

    if (!response.ok) {
      setError(
        pairingErrorMessages[payload.code] ??
          "Unable to pair this device right now.",
      );
      setSubmitting(false);
      return;
    }

    router.push(`/devices/${payload.device.id}`);
    router.refresh();
  }

  return (
    <form
      onSubmit={handleSubmit}
      style={{
        display: "grid",
        gap: "1rem",
        width: "min(32rem, 100%)",
      }}
    >
      <div style={{ display: "grid", gap: "0.4rem" }}>
        <label htmlFor="pairing-code">Pairing code</label>
        <input
          id="pairing-code"
          name="pairing-code"
          value={code}
          onChange={(event) => setCode(event.target.value)}
          required
          style={{ padding: "0.8rem", borderRadius: "0.8rem" }}
        />
      </div>

      <div style={{ display: "grid", gap: "0.4rem" }}>
        <label htmlFor="device-name">Device name</label>
        <input
          id="device-name"
          name="device-name"
          value={deviceName}
          onChange={(event) => setDeviceName(event.target.value)}
          style={{ padding: "0.8rem", borderRadius: "0.8rem" }}
        />
      </div>

      {error ? (
        <p role="alert" style={{ margin: 0, color: "#b91c1c" }}>
          {error}
        </p>
      ) : null}

      <button
        type="submit"
        disabled={submitting}
        style={{
          border: "none",
          borderRadius: "999px",
          padding: "0.95rem 1.15rem",
          background: "#0f172a",
          color: "#ffffff",
        }}
      >
        {submitting ? "Pairing..." : "Pair device"}
      </button>
    </form>
  );
}
