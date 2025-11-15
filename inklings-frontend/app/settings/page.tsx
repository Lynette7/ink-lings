"use client"

import { useState, useEffect } from "react"
import { useRouter } from "next/navigation"
import { useAuth } from "@/components/auth-context"
import { Navbar } from "@/components/navbar"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { useTheme } from "next-themes"
import { Wallet, Trash2 } from "lucide-react"

export default function SettingsPage() {
  const { user, isLoading, connectWallet, logout } = useAuth()
  const router = useRouter()
  const { theme, setTheme } = useTheme()
  const [walletAddress, setWalletAddress] = useState("")
  const [showWalletInput, setShowWalletInput] = useState(false)
  const [copied, setCopied] = useState(false)

  useEffect(() => {
    if (!isLoading && !user) {
      router.push("/login")
      return
    }

    if (user?.walletAddress) {
      setWalletAddress(user.walletAddress)
    }
  }, [user, isLoading, router])

  const handleConnectWallet = async () => {
    if (walletAddress.trim()) {
      await connectWallet(walletAddress)
      setShowWalletInput(false)
      setCopied(false)
    }
  }

  const handleCopyAddress = () => {
    if (walletAddress) {
      navigator.clipboard.writeText(walletAddress)
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    }
  }

  const handleResetProgress = () => {
    if (user && window.confirm("Are you sure? This will delete all your progress.")) {
      const prefix = `inlinks_progress_${user.id}_`
      const keysToDelete: string[] = []
      for (let i = 0; i < localStorage.length; i++) {
        const key = localStorage.key(i)
        if (key?.startsWith(prefix)) {
          keysToDelete.push(key)
        }
      }
      keysToDelete.forEach((key) => localStorage.removeItem(key))
      window.location.reload()
    }
  }

  const handleLogout = () => {
    logout()
    router.push("/")
  }

  if (!isLoading && !user) {
    return null
  }

  return (
    <div className="min-h-screen bg-background">
      <Navbar />

      <main className="max-w-2xl mx-auto px-4 py-12">
        <div className="space-y-8">
          {/* Header */}
          <div className="space-y-4">
            <h1 className="text-4xl font-bold">Settings</h1>
            <p className="text-muted-foreground">Manage your account and preferences</p>
          </div>

          {/* Account Settings */}
          <Card>
            <CardHeader>
              <CardTitle>Account</CardTitle>
              <CardDescription>Your account information</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <label className="text-sm font-medium">Email</label>
                <Input disabled value={user?.email || ""} />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Username</label>
                <Input disabled value={user?.username || ""} />
              </div>
            </CardContent>
          </Card>

          {/* Wallet Connection */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Wallet size={20} />
                Blockchain Wallet
              </CardTitle>
              <CardDescription>Connect your crypto wallet to track achievements on-chain</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {user?.walletAddress ? (
                <div className="space-y-3">
                  <div className="bg-accent/50 rounded-lg p-4 space-y-2">
                    <p className="text-sm font-medium">Connected Wallet</p>
                    <div className="flex items-center justify-between">
                      <code className="text-sm font-mono bg-background px-2 py-1 rounded">
                        {walletAddress.slice(0, 6)}...{walletAddress.slice(-4)}
                      </code>
                      <button
                        onClick={handleCopyAddress}
                        className="text-sm text-[#6552D0] hover:text-[#a350a3] transition-colors"
                      >
                        {copied ? "Copied!" : "Copy"}
                      </button>
                    </div>
                  </div>
                  <Button
                    variant="outline"
                    className="w-full bg-transparent"
                    onClick={() => {
                      setShowWalletInput(true)
                      setWalletAddress("")
                    }}
                  >
                    Change Wallet
                  </Button>
                </div>
              ) : (
                <div className="space-y-3">
                  {!showWalletInput ? (
                    <Button
                      className="w-full bg-white hover:bg-[#a350a3] hover:text-white text-black"
                      onClick={() => setShowWalletInput(true)}
                    >
                      <Wallet className="mr-2 w-4 h-4" />
                      Connect Wallet
                    </Button>
                  ) : (
                    <div className="space-y-3">
                      <Input
                        placeholder="Paste your wallet address (0x...)"
                        value={walletAddress}
                        onChange={(e) => setWalletAddress(e.target.value)}
                      />
                      <div className="flex gap-2">
                        <Button
                          className="flex-1 bg-[#6552D0] hover:bg-[#a350a3] text-white"
                          onClick={handleConnectWallet}
                          disabled={!walletAddress.trim()}
                        >
                          Confirm
                        </Button>
                        <Button
                          variant="outline"
                          className="flex-1 bg-transparent"
                          onClick={() => {
                            setShowWalletInput(false)
                            setWalletAddress("")
                          }}
                        >
                          Cancel
                        </Button>
                      </div>
                    </div>
                  )}
                  <p className="text-xs text-muted-foreground">
                    Your wallet address will be used to record your achievements on the blockchain.
                  </p>
                </div>
              )}
            </CardContent>
          </Card>

          {/* Theme Settings */}
          <Card>
            <CardHeader>
              <CardTitle>Appearance</CardTitle>
              <CardDescription>Customize your learning environment</CardDescription>
            </CardHeader>
            {/* <CardContent className="space-y-4">
              <div className="space-y-2">
                <label className="text-sm font-medium">Theme</label>
                <div className="flex gap-2">
                  <Button
                    variant={theme === "light" ? "default" : "outline"}
                    className={theme === "light" ? "bg-[#6552D0] hover:bg-[#a350a3]" : ""}
                    onClick={() => setTheme("light")}
                  >
                    Light
                  </Button>
                  <Button
                    variant={theme === "dark" ? "default" : "outline"}
                    className={theme === "dark" ? "bg-[#a350a3] hover:bg-[#a350a3]" : ""}
                    onClick={() => setTheme("dark")}
                  >
                    Dark
                  </Button>
                </div>
              </div>
            </CardContent> */}
          </Card>

          {/* Danger Zone */}
          <Card className="border-red-500/20">
            <CardHeader>
              <CardTitle className="text-red-600 dark:text-red-400">Danger Zone</CardTitle>
              <CardDescription>Irreversible actions</CardDescription>
            </CardHeader>
            <CardContent className="space-y-3">
              <Button
                variant="outline"
                className="w-full border-red-500/20 text-red-600 dark:text-red-400 hover:bg-red-500/10 bg-transparent"
                onClick={handleResetProgress}
              >
                <Trash2 className="mr-2 w-4 h-4" />
                Reset All Progress
              </Button>
              <Button className="w-full bg-red-600 hover:bg-red-700 text-white" onClick={handleLogout}>
                Logout
              </Button>
            </CardContent>
          </Card>
        </div>
      </main>
    </div>
  )
}
