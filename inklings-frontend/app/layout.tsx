import type React from "react"
import type { Metadata } from "next"
import { Geist, Geist_Mono } from "next/font/google"
import { Analytics } from "@vercel/analytics/next"
import { Providers } from "./providers"
import "./globals.css"

const _geist = Geist({ subsets: ["latin"] })
const _geistMono = Geist_Mono({ subsets: ["latin"] })

export const metadata: Metadata = {
  title: "inlinks - Learn Rust Smart Contracts",
  description: "Interactive web platform for learning Rust smart contract development with ink!",
  generator: "Next.app",
  icons: {
    icon: [
      {
        url: "/squid.png",
        media: "(prefers-color-scheme: light)",
      },
      {
        url: "/squid.png",
        media: "(prefers-color-scheme: dark)",
      },
      {
        url: "/squid.png",
        type: "image/svg+xml",
      },
    ],
    apple: "/apple-icon.png",
  },
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className={`font-sans antialiased`}>
        <Providers>{children}</Providers>
        <Analytics />
      </body>
    </html>
  )
}
