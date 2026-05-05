"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

export function LoginForm() {
  const router = useRouter();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitting(true);
    setError(null);

    const response = await fetch("/api/auth/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
    });

    const payload = await response.json();

    if (!response.ok) {
      setError(
        payload.message ??
          payload.error ??
          "Unable to sign in with the provided credentials.",
      );
      setSubmitting(false);
      return;
    }

    router.push("/devices");
    router.refresh();
  }

  return (
    <form
      onSubmit={handleSubmit}
      style={{
        display: "grid",
        gap: "1rem",
        width: "min(26rem, 100%)",
      }}
    >
      <div style={{ display: "grid", gap: "0.4rem" }}>
        <label htmlFor="email">Email</label>
        <input
          id="email"
          name="email"
          type="email"
          value={email}
          onChange={(event) => setEmail(event.target.value)}
          required
          style={{ padding: "0.75rem", borderRadius: "0.75rem" }}
        />
      </div>

      <div style={{ display: "grid", gap: "0.4rem" }}>
        <label htmlFor="password">Password</label>
        <input
          id="password"
          name="password"
          type="password"
          value={password}
          onChange={(event) => setPassword(event.target.value)}
          required
          style={{ padding: "0.75rem", borderRadius: "0.75rem" }}
        />
      </div>

      {error ? (
        <p
          role="alert"
          style={{ margin: 0, color: "#b91c1c" }}
        >
          {error}
        </p>
      ) : null}

      <button
        type="submit"
        disabled={submitting}
        style={{
          padding: "0.9rem 1rem",
          borderRadius: "999px",
          background: "#0f172a",
          color: "#fff",
          border: "none",
        }}
      >
        {submitting ? "Signing in..." : "Sign in"}
      </button>
    </form>
  );
}
