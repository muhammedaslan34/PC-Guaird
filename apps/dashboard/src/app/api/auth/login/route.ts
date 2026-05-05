import { backendFetch } from "@/lib/backend-client";
import { storeAuthToken } from "@/lib/auth";
import { NextResponse } from "next/server";

export async function POST(request: Request) {
  const credentials = await request.json();
  const backendResponse = await backendFetch("/api/auth/login", {
    method: "POST",
    body: credentials,
  });

  const payload = await backendResponse.json();

  if (!backendResponse.ok) {
    return NextResponse.json(payload, { status: backendResponse.status });
  }

  const response = NextResponse.json(
    {
      user: payload.user,
    },
    { status: 200 },
  );

  storeAuthToken(response, payload.token);

  return response;
}
