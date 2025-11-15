"use client"

import type React from "react"
import { createContext, useContext, useState, useEffect } from "react"
import { storage } from "@/lib/storage"

interface User {
  id: string
  email: string
  username: string
  walletAddress?: string
}

interface AuthContextType {
  user: User | null
  isLoading: boolean
  login: (email: string, password: string) => Promise<void>
  signup: (email: string, username: string, password: string) => Promise<void>
  logout: () => void
  connectWallet: (address: string) => Promise<void>
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null)
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    // Check for existing user on mount
    const existingUser = storage.getUser()
    if (existingUser) {
      setUser(existingUser)
    }
    setIsLoading(false)
  }, [])

  const login = async (email: string, password: string) => {
    // Mock login - in production would call backend API
    const newUser: User = {
      id: Math.random().toString(36).substring(7),
      email,
      username: email.split("@")[0],
    }
    storage.setUser(newUser)
    setUser(newUser)
  }

  const signup = async (email: string, username: string, password: string) => {
    const newUser: User = {
      id: Math.random().toString(36).substring(7),
      email,
      username,
    }
    storage.setUser(newUser)
    setUser(newUser)
  }

  const logout = () => {
    storage.clearUser()
    setUser(null)
  }

  const connectWallet = async (address: string) => {
    if (user) {
      const updatedUser = { ...user, walletAddress: address }
      storage.setUser(updatedUser)
      setUser(updatedUser)
    }
  }

  return (
    <AuthContext.Provider value={{ user, isLoading, login, signup, logout, connectWallet }}>
      {children}
    </AuthContext.Provider>
  )
}

export function useAuth() {
  const context = useContext(AuthContext)
  if (context === undefined) {
    throw new Error("useAuth must be used within an AuthProvider")
  }
  return context
}
