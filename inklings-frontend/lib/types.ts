// Exercise types and interfaces
export interface Exercise {
  id: string
  title: string
  description: string
  difficulty: "Beginner" | "Intermediate" | "Advanced"
  category: string
  instructions: string
  starterCode: string
  hints: string[]
  tags: string[]
}

export interface UserProgress {
  userId: string
  exerciseId: string
  completed: boolean
  attempts: number
  lastAttempted: string
  timeSpent: number
}

export interface UserStats {
  totalExercises: number
  completedExercises: number
  currentStreak: number
  totalTimeSpent: number
  categoryProgress: Record<string, { completed: number; total: number }>
}

export interface VerificationResult {
  success: boolean
  output: string
  errors: string[]
  warnings: string[]
  executionTime: number
}
