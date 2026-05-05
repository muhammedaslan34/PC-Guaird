export default function Home() {
  return (
    <main
      style={{
        minHeight: "100vh",
        display: "grid",
        placeItems: "center",
        padding: "2rem",
      }}
    >
      <section
        style={{
          width: "min(32rem, 100%)",
          border: "1px solid #d6d6d6",
          borderRadius: "1rem",
          padding: "2rem",
          background: "#ffffff",
          boxShadow: "0 18px 45px rgba(15, 23, 42, 0.08)",
        }}
      >
        <p
          style={{
            margin: 0,
            fontSize: "0.8rem",
            letterSpacing: "0.16em",
            textTransform: "uppercase",
            color: "#475569",
          }}
        >
          Parent Remote Control System
        </p>
        <h1
          style={{
            marginTop: "0.75rem",
            marginBottom: "0.5rem",
            fontSize: "2rem",
            lineHeight: 1.1,
            color: "#0f172a",
          }}
        >
          PC Guard Dashboard
        </h1>
        <p
          style={{
            margin: 0,
            color: "#334155",
            lineHeight: 1.6,
          }}
        >
          Dashboard scaffolding is ready. Protected routing, Laravel auth
          integration, and device control flows will attach here next.
        </p>
      </section>
    </main>
  );
}
