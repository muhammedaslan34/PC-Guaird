import { backendFetch } from "@/lib/backend-client";
import { getAuthTokenFromCookieHeader } from "@/lib/auth";
import { NextResponse } from "next/server";

type Params = {
  params: Promise<{ deviceId: string }>;
};

export async function GET(request: Request, { params }: Params) {
  const token = getAuthTokenFromCookieHeader(request.headers.get("cookie"));
  const { deviceId } = await params;

  const response = await backendFetch(`/api/devices/${deviceId}/commands`, {
    method: "GET",
    token,
  });

  const payload = await response.json();

  return NextResponse.json(payload, { status: response.status });
}

export async function POST(request: Request, { params }: Params) {
  const token = getAuthTokenFromCookieHeader(request.headers.get("cookie"));
  const { deviceId } = await params;
  const body = await request.json();

  const response = await backendFetch(`/api/devices/${deviceId}/commands`, {
    method: "POST",
    token,
    body,
  });

  const payload = await response.json();

  return NextResponse.json(payload, { status: response.status });
}
