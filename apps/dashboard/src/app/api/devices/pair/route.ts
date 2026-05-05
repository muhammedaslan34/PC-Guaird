import { backendFetch } from "@/lib/backend-client";
import { getAuthTokenFromCookieHeader } from "@/lib/auth";
import { NextResponse } from "next/server";

export async function POST(request: Request) {
  const token = getAuthTokenFromCookieHeader(request.headers.get("cookie"));
  const body = await request.json();

  const response = await backendFetch("/api/devices/pair", {
    method: "POST",
    token,
    body,
  });

  const payload = await response.json();

  return NextResponse.json(payload, { status: response.status });
}
