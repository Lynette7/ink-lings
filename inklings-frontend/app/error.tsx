"use client"

import { Button } from "@/components/ui/button"
import { useRouter } from "next/navigation"

export default function Error({
  error,
  reset,
}: {
  error: Error & { digest?: string }
  reset?: () => void
}) {
  const router = useRouter()

  const handleRetry = () => {
    if (reset) {
      reset()
    } else {
      router.refresh()
    }
  }

  return (
    <main className="min-h-screen bg-gradient-to-br from-[#17203D] via-background to-[#F5E7DE]/10 flex items-center justify-center px-4">
      <div className="text-center space-y-6">
        <h1 className="text-4xl font-bold">Something went wrong</h1>
        <p className="text-muted-foreground max-w-md">An error occurred while processing your request.</p>
        <div className="flex gap-4 justify-center">
          <Button variant="outline" onClick={handleRetry}>
            Try again
          </Button>
          <Button asChild className="bg-[#6552D0] hover:bg-[#a350a3] text-white">
            <a href="/">Go Home</a>
          </Button>
        </div>
      </div>
    </main>
  )
}
