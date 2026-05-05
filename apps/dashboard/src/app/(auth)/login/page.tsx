import { LoginForm } from "@/components/auth/login-form";

export default function LoginPage() {
  return (
    <main
      style={{
        minHeight: "100vh",
        display: "grid",
        placeItems: "center",
        padding: "2rem",
        background:
          "linear-gradient(160deg, #f8fafc 0%, #e2e8f0 40%, #cbd5e1 100%)",
      }}
    >
      <section
        style={{
          width: "min(30rem, 100%)",
          padding: "2rem",
          borderRadius: "1.5rem",
          background: "#ffffff",
          boxShadow: "0 24px 60px rgba(15, 23, 42, 0.12)",
          border: "1px solid rgba(148, 163, 184, 0.25)",
        }}
      >
        <p
          style={{
            margin: 0,
            letterSpacing: "0.18em",
            textTransform: "uppercase",
            fontSize: "0.75rem",
            color: "#64748b",
          }}
        >
          Parent Access
        </p>
        <h1
          style={{
            marginTop: "0.75rem",
            marginBottom: "0.5rem",
            fontSize: "2rem",
            lineHeight: 1.05,
            color: "#0f172a",
          }}
        >
          Sign in to PC Guard Dashboard
        </h1>
        <p
          style={{
            marginTop: 0,
            marginBottom: "1.5rem",
            color: "#475569",
            lineHeight: 1.6,
          }}
        >
          Use your parent account from the Laravel backend to manage linked
          devices, pairing, commands, and audit history.
        </p>

        <LoginForm />
      </section>
    </main>
  );
}
