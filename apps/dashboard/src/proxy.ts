import { NextResponse, type NextRequest } from "next/server";
import { AUTH_COOKIE_NAME } from "@/lib/auth";

export function proxy(request: NextRequest) {
  const token = request.cookies.get(AUTH_COOKIE_NAME)?.value;
  const { pathname } = request.nextUrl;

  const isProtectedRoute =
    pathname.startsWith("/devices") || pathname.startsWith("/pair-device");
  const isAuthRoute = pathname.startsWith("/login");

  if (isProtectedRoute && !token) {
    return NextResponse.redirect(new URL("/login", request.url));
  }

  if (isAuthRoute && token) {
    return NextResponse.redirect(new URL("/devices", request.url));
  }

  return NextResponse.next();
}

export const config = {
  matcher: ["/login", "/devices/:path*", "/pair-device/:path*"],
};
