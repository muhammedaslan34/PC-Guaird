import { PairDeviceForm } from "@/components/devices/pair-device-form";

export default function PairDevicePage() {
  return (
    <main
      style={{
        display: "grid",
        gap: "1.5rem",
        maxWidth: "42rem",
      }}
    >
      <div>
        <p
          style={{
            margin: 0,
            fontSize: "0.75rem",
            letterSpacing: "0.16em",
            textTransform: "uppercase",
            color: "#64748b",
          }}
        >
          Pairing
        </p>
        <h1 style={{ marginBottom: "0.5rem", fontSize: "2rem" }}>
          Link a child device
        </h1>
        <p style={{ margin: 0, color: "#475569", lineHeight: 1.6 }}>
          Enter the temporary code shown by the child device. Once pairing
          succeeds, the dashboard will take you directly to the new device
          control center.
        </p>
      </div>

      <PairDeviceForm />
    </main>
  );
}
