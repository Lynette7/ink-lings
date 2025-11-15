"use client"

import { useState, useEffect, useMemo } from "react"
import { useRouter } from "next/navigation"
import { useAuth } from "@/components/auth-context"
import { Navbar } from "@/components/navbar"
import { exercises } from "@/lib/exercises"
import { storage } from "@/lib/storage"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { motion, AnimatePresence } from "framer-motion"

export default function ProgressPage() {
  const { user, isLoading } = useAuth()
  const router = useRouter()
  const [stats, setStats] = useState({
    totalExercises: exercises.length,
    completedExercises: 0,
    categoryProgress: {} as Record<string, { completed: number; total: number }>,
    totalAttempts: 0,
  })

  useEffect(() => {
    if (!isLoading && !user) {
      router.push("/login")
      return
    }

    if (user) {
      const allProgress = storage.getAllProgress(user.id)
      let completed = 0
      let totalAttempts = 0
      const categoryProgress: Record<string, { completed: number; total: number }> = {}

      exercises.forEach((exercise) => {
        if (!categoryProgress[exercise.category]) categoryProgress[exercise.category] = { completed: 0, total: 0 }
        categoryProgress[exercise.category].total++
      })

      Object.entries(allProgress).forEach(([exerciseId, progress]) => {
        if (progress.completed) {
          completed++
          const exercise = exercises.find((e) => e.id === exerciseId)
          if (exercise) categoryProgress[exercise.category].completed++
        }
        totalAttempts += progress.attempts || 0
      })

      setStats({ totalExercises: exercises.length, completedExercises: completed, categoryProgress, totalAttempts })
    }
  }, [user, isLoading, router])

  const categories = Array.from(new Set(exercises.map((e) => e.category)))
  const completionPercentage = stats.totalExercises
    ? Math.round((stats.completedExercises / stats.totalExercises) * 100)
    : 0

  const recentExercises = useMemo(() => {
    if (!user) return []
    const allProgress = storage.getAllProgress(user.id)
    return exercises
      .map((exercise) => ({ ...exercise, progress: allProgress[exercise.id] || { completed: false, attempts: 0, lastAttempted: null } }))
      .filter((e) => e.progress)
      .sort((a, b) => (new Date(b.progress?.lastAttempted || 0).getTime() - new Date(a.progress?.lastAttempted || 0).getTime()))
      .slice(0, 5)
  }, [user])

  const mostChallengingExercises = useMemo(() => {
    if (!user) return []
    const allProgress = storage.getAllProgress(user.id)
    return exercises
      .map((exercise) => ({ ...exercise, progress: allProgress[exercise.id] || { completed: false, attempts: 0 } }))
      .filter((e) => !e.progress.completed)
      .sort((a, b) => (b.progress?.attempts || 0) - (a.progress?.attempts || 0))
      .slice(0, 3)
  }, [user])

  if (!isLoading && !user) return null

  const palette = [
    { bar: "bg-[#89219E]", text: "text-[#89219E]" },
    { bar: "bg-[#89219E]/80", text: "text-[#89219E]" },
    { bar: "bg-[#89219E]/60", text: "text-[#89219E]" },
  ]

  return (
    <div className="min-h-screen relative">
      {/* Gradient overlay background */}
      <div className="absolute inset-0 bg-gradient-to-br from-[#1f0a2a]/20 via-[#89219E]/15 to-[#000000]/40 pointer-events-none z-0"></div>

      <div className="relative z-10 bg-background text-[#f6f4f2] min-h-screen">
        <Navbar />
        <main className="max-w-4xl mx-auto px-4 py-12 space-y-8">

          {/* Overall Progress */}
          <section className="bg-card p-6 rounded-lg shadow flex flex-col md:flex-row justify-between items-center">
            <div>
              <h1 className="text-3xl font-bold mb-2 text-[#89219E]">Your Progress</h1>
              <p className="text-[#f6f4f2]/80 text-sm">Track your learning journey</p>
            </div>
            <motion.div
              initial={{ scale: 0 }}
              animate={{ scale: 1 }}
              transition={{ type: "spring", stiffness: 120 }}
              className="relative w-24 h-24 flex items-center justify-center"
            >
              <svg className="w-24 h-24">
                <circle
                  className="text-[#f6f4f2]/30"
                  strokeWidth="8"
                  stroke="currentColor"
                  fill="transparent"
                  r="44"
                  cx="48"
                  cy="48"
                />
                <motion.circle
                  stroke="#89219E"
                  strokeWidth="8"
                  strokeLinecap="round"
                  fill="transparent"
                  r="44"
                  cx="48"
                  cy="48"
                  strokeDasharray={2 * Math.PI * 44}
                  strokeDashoffset={2 * Math.PI * 44}
                  animate={{ strokeDashoffset: 2 * Math.PI * 44 * (1 - completionPercentage / 100) }}
                  transition={{ duration: 1, ease: "easeOut" }}
                />
              </svg>
              <div className="absolute inset-0 flex items-center justify-center text-lg font-bold text-[#89219E]">
                {completionPercentage}%
              </div>
            </motion.div>
          </section>

          {/* Category Progress */}
          <section className="bg-card p-6 rounded-lg shadow space-y-4">
            <h2 className="text-lg font-semibold text-[#89219E]">Progress by Category</h2>
            <div className="space-y-3">
              {categories.map((category, idx) => {
                const progress = stats.categoryProgress[category] || { completed: 0, total: 0 }
                const percentage = progress.total ? Math.round((progress.completed / progress.total) * 100) : 0
                const color = palette[idx % palette.length]

                return (
                  <div key={category} className="space-y-1">
                    <div className="flex justify-between text-sm font-medium text-[#f6f4f2]">
                      <span>{category}</span>
                      <span className={`font-bold ${color.text}`}>{percentage}%</span>
                    </div>
                    <div className="h-2 bg-[#f6f4f2]/20 rounded-full overflow-hidden">
                      <motion.div
                        className={`h-full ${color.bar}`}
                        initial={{ width: 0 }}
                        animate={{ width: `${percentage}%` }}
                        transition={{ duration: 0.8, ease: "easeOut" }}
                      />
                    </div>
                  </div>
                )
              })}
            </div>
          </section>

          {/* Most Challenging & Recent Exercises */}
          <section className="grid md:grid-cols-2 gap-4">
            {mostChallengingExercises.length > 0 && (
              <div className="bg-card p-4 rounded-lg shadow space-y-2">
                <h3 className="font-semibold mb-2 text-[#89219E]">Most Challenging</h3>
                <ul className="space-y-2">
                  <AnimatePresence>
                    {mostChallengingExercises.map((ex, idx) => (
                      <motion.li
                        key={ex.id}
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0 }}
                        transition={{ duration: 0.3, delay: idx * 0.1 }}
                        className="flex justify-between items-center text-[#f6f4f2]"
                      >
                        <span>{idx + 1}. {ex.title}</span>
                        <Badge
                          variant="outline"
                          className="border-[#89219E] text-[#89219E] hover:bg-[#89219E]/10"
                        >
                          {ex.progress?.attempts || 0} attempts
                        </Badge>
                      </motion.li>
                    ))}
                  </AnimatePresence>
                </ul>
              </div>
            )}

            {recentExercises.length > 0 && (
              <div className="bg-card p-4 rounded-lg shadow space-y-2">
                <h3 className="font-semibold mb-2 text-[#f6f4f2]">Recently Worked On</h3>
                <ul className="space-y-2 text-[#f6f4f2]">
                  <AnimatePresence>
                    {recentExercises.map((ex, idx) => (
                      <motion.li
                        key={ex.id}
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0 }}
                        transition={{ duration: 0.3, delay: idx * 0.1 }}
                      >
                        {ex.title}
                      </motion.li>
                    ))}
                  </AnimatePresence>
                </ul>
              </div>
            )}
          </section>

          {/* Action Buttons */}
          <div className="flex flex-col sm:flex-row gap-4">
            <Button 
              onClick={() => router.push("/exercises")} 
              className="flex-1 bg-[#89219E] hover:bg-[#f6f4f2] hover:text-[#89219E] text-[#f6f4f2] font-medium"
            >
              Continue Learning
            </Button>
            <Button 
              variant="outline" 
              onClick={() => router.push("/")} 
              className="flex-1 border-[#89219E] text-[#89219E] hover:bg-[#89219E]/10"
            >
              Back to Home
            </Button>
          </div>

        </main>
      </div>
    </div>
  )
}
