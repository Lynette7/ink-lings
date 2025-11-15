import { NextResponse } from "next/server"
import type { NextRequest } from "next/server"

export function middleware(request: NextRequest) {
  // This middleware would be used for server-side auth checks
  // For now, client-side auth is sufficient with useAuth hook
  return NextResponse.next()
}

export const config = {
  matcher: ["/exercises/:path*", "/exercise/:path*", "/progress/:path*", "/settings/:path*"],
}
