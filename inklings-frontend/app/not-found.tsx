import Link from "next/link"
import { Button } from "@/components/ui/button"

export default function NotFound() {
  return (
    <main className="min-h-screen bg-gradient-to-br from-[#17203D] via-background to-[#F5E7DE]/10 flex items-center justify-center px-4">
      <div className="text-center space-y-6">
        <h1 className="text-6xl font-bold">404</h1>
        <p className="text-xl text-muted-foreground">Page not found</p>
        <p className="text-muted-foreground max-w-md">The page you're looking for doesn't exist or has been moved.</p>
        <Button asChild className="bg-[#6552D0] hover:bg-[#a350a3] text-white">
          <Link href="/">Return Home</Link>
        </Button>
      </div>
    </main>
  )
}
