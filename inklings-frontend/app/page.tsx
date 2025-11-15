"use client"

import { motion, useMotionValue, useTransform, useSpring } from "framer-motion"
import Link from "next/link"
import { useState } from "react"

export default function Home3DPage() {
  const x = useMotionValue(0)
  const y = useMotionValue(0)

  const rotateX = useTransform(y, [-200, 200], [15, -15])
  const rotateY = useTransform(x, [-200, 200], [-15, 15])
  const springX = useSpring(rotateX, { stiffness: 150, damping: 20 })
  const springY = useSpring(rotateY, { stiffness: 150, damping: 20 })

  const [user, setUser] = useState(null) // replace with actual auth state

  const handleMouseMove = (event) => {
    const rect = event.currentTarget.getBoundingClientRect()
    const offsetX = event.clientX - rect.left - rect.width / 2
    const offsetY = event.clientY - rect.top - rect.height / 2
    x.set(offsetX)
    y.set(offsetY)
  }

  const handleMouseLeave = () => {
    x.set(0)
    y.set(0)
  }

  // swimming octopuses 
  const octos = [
    { size: 45, speed: 32, y: 200, opacity: 0.5, fromLeft: true, delay: 1 },
    { size: 48, speed: 28, y: 150, opacity: 0.52, fromLeft: false, delay: 0.8 },
    { size: 50, speed: 30, y: 250, opacity: 0.55, fromLeft: true, delay: 0.5 },
    { size: 46, speed: 29, y: 180, opacity: 0.5, fromLeft: false, delay: 1.2 },
  ]

  return (
    <div
      className="relative h-screen w-full overflow-hidden bg-[#0b0934] text-white"
      onMouseMove={handleMouseMove}
      onMouseLeave={handleMouseLeave}
    >
      <div className="absolute inset-0 bg-gradient-to-b from-[#110d45] via-[#0d0a32] to-[#08061e] pointer-events-none" />
      
      {/* Floating squids */}
      {[{ left: 16, top: 12, delay: 0, rotateRange: [0, 4, 0, -4, 0] },
        { left: 70, top: 520, delay: 1.5, rotateRange: [0, -3, 0, 3, 0] }
      ].map((squid, i) => (
        <motion.div
          key={i}
          style={{ rotateX: springX, rotateY: springY }}
          className="absolute opacity-40 pointer-events-none select-none"
          style={{ top: `${squid.top}px`, left: `${squid.left}%` }}
        >
          <motion.img
            src="/squid.png"
            animate={{ y: [0, -10, 0, 10, 0], rotate: squid.rotateRange }}
            transition={{ repeat: Infinity, duration: 12, ease: "easeInOut", delay: squid.delay }}
            style={{ width: 180, height: "auto" }}
          />
        </motion.div>
      ))}

      {/* Bubbles, hills, corals... (same as before) */}
      {[...Array(26)].map((_, i) => (
        <motion.div
          key={i}
          className="absolute rounded-full bg-white/15 blur-[1px]"
          style={{
            width: `${5 + (i % 5) * 2}px`,
            height: `${5 + (i % 5) * 2}px`,
            left: `${Math.random() * 100}%`,
            bottom: -60,
          }}
          animate={{ y: [0, -900], opacity: [0, 1, 0] }}
          transition={{ duration: 8 + (i % 6) * 1.8, repeat: Infinity, ease: "easeOut", delay: i * 0.4 }}
        />
      ))}
      {[0.12, 0.1, 0.08].map((opacity, i) => (
        <motion.div
          key={i}
          className="absolute bottom-0 left-0 w-full"
          animate={{ y: [0, -10 - i * 4, 0] }}
          transition={{ repeat: Infinity, duration: 9 + i * 3, ease: "easeInOut" }}
          style={{ opacity }}
        >
          <svg viewBox="0 0 1440 320" className="fill-white/15">
            <path d="M0,256L80,250.7C160,245,320,235,480,213.3C640,192,800,160,960,165.3C1120,171,1280,213,1360,234.7L1440,256L1440,320L0,320Z"></path>
          </svg>
        </motion.div>
      ))}

      {/* Coral silhouettes */}
      <motion.div className="absolute bottom-0 left-0 opacity-[0.18] pointer-events-none"
        animate={{ scale: [1, 1.02, 1] }}
        transition={{ repeat: Infinity, duration: 7, ease: "easeInOut" }}
      >
        <svg width="400" height="300" viewBox="0 0 500 400" fill="#fff">
          <path d="M120 380 C80 260 200 260 160 170 C120 80 60 160 50 120" strokeWidth="45" stroke="white" fill="none" />
        </svg>
      </motion.div>
      <motion.div className="absolute bottom-0 right-0 opacity-[0.15] pointer-events-none"
        animate={{ scale: [1, 1.03, 1] }}
        transition={{ repeat: Infinity, duration: 8, ease: "easeInOut" }}
      >
        <svg width="380" height="280" viewBox="0 0 450 400" fill="#fff">
          <path d="M220 380 C260 280 140 260 200 180 C260 100 320 160 350 120" strokeWidth="45" stroke="white" fill="none" />
        </svg>
      </motion.div>

      {/* Swimming octopuses */}
      {octos.map((o, i) => (
        <motion.div
          key={i}
          className="absolute"
          style={{ bottom: o.y, opacity: o.opacity }}
          animate={{
            x: o.fromLeft ? ["-15%", "110%"] : ["110%", "-15%"],
            rotate: [0, 5, 0, -5, 0],
            scale: [1, 1.06, 1],
          }}
          transition={{ duration: o.speed, repeat: Infinity, ease: "easeInOut", delay: o.delay || 0 }}
        >
          <img
            src="/octopus.png"
            alt="octopus"
            style={{ width: o.size, height: "auto", filter: "drop-shadow(0px 0px 8px rgba(140, 90, 255, 0.35))" }}
          />
        </motion.div>
      ))}

      {/* Heading + buttons */}
      <motion.div style={{ rotateX: springX, rotateY: springY }} className="relative z-10 flex flex-col items-center justify-center h-full">
        <motion.h1
          className="text-6xl font-bold tracking-tight mb-4"
          initial={{ y: -20, opacity: 0 }}
          animate={{ y: [0, -6, 0], opacity: 1 }}
          transition={{ y: { repeat: Infinity, duration: 6, ease: "easeInOut" }, opacity: { duration: 1 } }}
        >
          ink!lings
        </motion.h1>

        <motion.p
          className="text-lg text-purple-200 max-w-lg text-center"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1, y: [0, 3, 0] }}
          transition={{ y: { repeat: Infinity, duration: 8, ease: "easeInOut" }, opacity: { duration: 1.2, delay: 0.2 } }}
        >
          Dive into the deep-sea world of ink! smart contracts
        </motion.p>

        <div className="mt-10 flex gap-4">
          <Link href="/signup">
            <motion.button
              className="px-6 py-3 bg-white text-black rounded-xl shadow-xl hover:shadow-[0_0_20px_rgba(140,90,255,0.7)] transition-shadow duration-300"
              whileHover={{ scale: 1.06 }}
              transition={{ type: "spring", stiffness: 300 }}
            >
              Start Learning
            </motion.button>
          </Link>

          <Link href={user ? "/exercises" : "/login"}>
            <motion.button
              className="px-6 py-3 bg-purple-800/60 text-white rounded-xl border border-purple-600 shadow-lg hover:shadow-[0_0_25px_rgba(140,90,255,0.8)] transition-shadow duration-300"
              whileHover={{ scale: 1.06 }}
              transition={{ type: "spring", stiffness: 300 }}
            >
              View Exercises
            </motion.button>
          </Link>
        </div>
      </motion.div>
    </div>
  )
}
