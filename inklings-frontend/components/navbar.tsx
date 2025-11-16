"use client"

import Link from "next/link"
import { useRouter, usePathname } from "next/navigation"
import { useAuth } from "@/components/auth-context"
import { Button } from "@/components/ui/button"
import { useTheme } from "next-themes"
import { Sun, Moon, LogOut, Menu, X } from "lucide-react"
import { useState } from "react"

export function Navbar() {
  const { user, logout } = useAuth()
  const router = useRouter()
  const pathname = usePathname()
  const { theme, setTheme } = useTheme()
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false)

  const handleLogout = () => {
    logout()
    router.push("/")
    setMobileMenuOpen(false)
  }

  if (!user) {
    return null
  }

  const isActive = (path: string) => pathname.startsWith(path)

  return (
    <nav className="border-b border-border bg-card sticky top-0 z-50">
      <div className="max-w-7xl mx-auto px-4 py-3 flex items-center justify-between">
        <Link href="/exercises" className="flex items-center gap-2">
          <div className="w-8 h-8 rounded-lg bg-[#6552D0] flex items-center justify-center">
            {/* <span className="text-white font-bold text-sm"></span> */}
            <img src="/squid.png" alt="ink!lings logo" className="w-6 h-6" />
          </div>
          <span className="font-bold text-lg hidden sm:inline">ink!lings</span>
        </Link>

        {/* Desktop Menu */}
        <div className="hidden md:flex items-center gap-6">
          <Link
            href="/exercises"
            className={`text-sm transition-colors ${isActive("/exercises") ? "text-[#89219E] font-medium" : "hover:text-[#89219E]"}`}
          >
            Exercises
          </Link>
          <Link
            href="/progress"
            className={`text-sm transition-colors ${isActive("/progress") ? "text-[#89219E] font-medium" : "hover:text-[#89219E]"}`}
          >
            Progress
          </Link>
          <Link
            href="/settings"
            className={`text-sm transition-colors ${isActive("/settings") ? "text-[#89219E] font-medium" : "hover:text-[#89219E]"}`}
          >
            Settings
          </Link>
        </div>

        <div className="flex items-center gap-3">
          {/* <button
            onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
            className="p-2 hover:bg-accent rounded-lg transition-colors"
          >
            {theme === "dark" ? <Sun size={18} /> : <Moon size={18} />}
          </button> */}
          <span className="text-sm text-muted-foreground hidden sm:inline">{user.username}</span>
          <Button variant="outline" size="sm" onClick={handleLogout} className="gap-2 bg-transparent">
            <LogOut size={16} />
            <span className="hidden sm:inline">Logout</span>
          </Button>

          {/* Mobile menu button */}
          <button
            onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
            className="md:hidden p-2 hover:bg-accent rounded-lg transition-colors"
          >
            {mobileMenuOpen ? <X size={18} /> : <Menu size={18} />}
          </button>
        </div>
      </div>

      {/* Mobile Menu */}
      {mobileMenuOpen && (
        <div className="md:hidden border-t border-border bg-card/95 backdrop-blur">
          <div className="max-w-7xl mx-auto px-4 py-3 space-y-2">
            <Link
              href="/exercises"
              onClick={() => setMobileMenuOpen(false)}
              className="block px-3 py-2 rounded-lg hover:bg-accent text-sm"
            >
              Exercises
            </Link>
            <Link
              href="/progress"
              onClick={() => setMobileMenuOpen(false)}
              className="block px-3 py-2 rounded-lg hover:bg-accent text-sm"
            >
              Progress
            </Link>
            <Link
              href="/settings"
              onClick={() => setMobileMenuOpen(false)}
              className="block px-3 py-2 rounded-lg hover:bg-accent text-sm"
            >
              Settings
            </Link>
          </div>
        </div>
      )}
    </nav>
  )
}
