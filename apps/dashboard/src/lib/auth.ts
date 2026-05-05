import { cookies } from "next/headers";
import type { NextResponse } from "next/server";

export const AUTH_COOKIE_NAME = "pc_guard_session";

export async function getStoredAuthToken(): Promise<string | null> {
  return (await cookies()).get(AUTH_COOKIE_NAME)?.value ?? null;
}

export function getAuthTokenFromCookieHeader(cookieHeader: string | null): string | null {
  if (!cookieHeader) {
    return null;
  }

  const cookieValue = cookieHeader
    .split(";")
    .map((part) => part.trim())
    .find((part) => part.startsWith(`${AUTH_COOKIE_NAME}=`));

  if (!cookieValue) {
    return null;
  }

  return decodeURIComponent(cookieValue.slice(AUTH_COOKIE_NAME.length + 1));
}

export function storeAuthToken(response: NextResponse, token: string): void {
  response.cookies.set({
    name: AUTH_COOKIE_NAME,
    value: token,
    httpOnly: true,
    sameSite: "lax",
    path: "/",
    secure: process.env.NODE_ENV === "production",
  });
}

export function clearAuthToken(response: NextResponse): void {
  response.cookies.set({
    name: AUTH_COOKIE_NAME,
    value: "",
    httpOnly: true,
    sameSite: "lax",
    path: "/",
    secure: process.env.NODE_ENV === "production",
    expires: new Date(0),
  });
}
