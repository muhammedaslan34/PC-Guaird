export default function DashboardLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <div
      style={{
        minHeight: "100vh",
        background: "#f8fafc",
        color: "#0f172a",
      }}
    >
      <header
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          padding: "1.25rem 1.5rem",
          borderBottom: "1px solid #e2e8f0",
          background: "#ffffff",
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
            Parent Control
          </p>
          <strong>PC Guard Dashboard</strong>
        </div>
      </header>

      <div style={{ padding: "1.5rem" }}>{children}</div>
    </div>
  );
}
