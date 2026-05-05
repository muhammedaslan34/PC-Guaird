import { backendFetch } from "@/lib/backend-client";
import { getAuthTokenFromCookieHeader } from "@/lib/auth";
import { NextResponse } from "next/server";

export async function GET(request: Request) {
  const token = getAuthTokenFromCookieHeader(request.headers.get("cookie"));

  const response = await backendFetch("/api/devices", {
    method: "GET",
    token,
  });

  const payload = await response.json();

  return NextResponse.json(payload, { status: response.status });
}
