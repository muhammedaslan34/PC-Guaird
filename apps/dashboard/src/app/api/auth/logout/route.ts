import { backendFetch } from "@/lib/backend-client";
import { clearAuthToken, getAuthTokenFromCookieHeader } from "@/lib/auth";
import { NextResponse } from "next/server";

export async function POST(request: Request) {
  const token = getAuthTokenFromCookieHeader(request.headers.get("cookie"));

  const backendResponse = await backendFetch("/api/auth/logout", {
    method: "POST",
    token,
  });

  const payload = await backendResponse.json();
  const response = NextResponse.json(payload, { status: backendResponse.status });

  clearAuthToken(response);

  return response;
}
