"use client"

import { useState, useMemo } from "react"
import { useAuth } from "@/components/auth-context"
import { Navbar } from "@/components/navbar"
import { exercises } from "@/lib/exercises"
import { storage } from "@/lib/storage"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Badge } from "@/components/ui/badge"
import { ChevronLeft, Search, Lightbulb, CheckCircle, AlertCircle } from "lucide-react"
import CodeEditor from "@/components/code-editor"
import { verifyExercise } from "@/lib/verification"
import { motion, AnimatePresence } from "framer-motion"

const DIFFICULTY_COLORS = {
  Beginner: "bg-[#89219E] text-white",
  Intermediate: "bg-[#060270] text-white",
  Advanced: "bg-[#89219E] text-white",
}

export default function ExercisesPage() {
  const { user, isLoading } = useAuth()
  const [searchTerm, setSearchTerm] = useState("")
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null)
  const [selectedExercise, setSelectedExercise] = useState<typeof exercises[0] | null>(null)
  const [code, setCode] = useState("")
  const [showHints, setShowHints] = useState(false)
  const [currentHintIndex, setCurrentHintIndex] = useState(0)
  const [isVerifying, setIsVerifying] = useState(false)
  const [verificationResult, setVerificationResult] = useState<any>(null)
  const [attempts, setAttempts] = useState(0)
  const [isCompleted, setIsCompleted] = useState(false)

  const categories = ["All", ...Array.from(new Set(exercises.map((e) => e.category)))]

  const filteredExercises = useMemo(() => {
    return exercises.filter((exercise) => {
      const matchesSearch =
        exercise.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
        exercise.description.toLowerCase().includes(searchTerm.toLowerCase())
      const matchesCategory =
        !selectedCategory || selectedCategory === "All" || exercise.category === selectedCategory
      return matchesSearch && matchesCategory
    })
  }, [searchTerm, selectedCategory])

  const getExerciseStatus = (exerciseId: string) => {
    if (!user) return false
    const progress = storage.getProgress(user.id, exerciseId)
    return progress?.completed || false
  }

  const handleSelectExercise = (exercise: typeof exercises[0]) => {
    setSelectedExercise(exercise)
    setCode(exercise.starterCode)
    setVerificationResult(null)
    setAttempts(0)
    setIsCompleted(getExerciseStatus(exercise.id))
    setShowHints(false)
    setCurrentHintIndex(0)

    if (user) {
      const progress = storage.getProgress(user.id, exercise.id)
      if (progress?.code) {
        setCode(progress.code)
        setAttempts(progress.attempts || 0)
        setIsCompleted(progress.completed || false)
      }
    }
  }

  const handleVerify = async () => {
    if (!user || !selectedExercise) return

    setIsVerifying(true)
    const newAttempts = attempts + 1
    setAttempts(newAttempts)

    try {
      const result = await verifyExercise(selectedExercise.id, code)
      setVerificationResult(result)

      storage.setProgress(user.id, selectedExercise.id, {
        code,
        attempts: newAttempts,
        completed: result.success,
        lastAttempted: new Date().toISOString(),
      })
      if (result.success) setIsCompleted(true)
    } catch (error) {
      setVerificationResult({
        success: false,
        message: "Verification error",
        errors: ["An error occurred during verification"],
        executionTime: 0,
      })
    } finally {
      setIsVerifying(false)
    }
  }

  if (!isLoading && !user) return <p>Loading...</p>

  return (
    <div className="min-h-screen bg-background">
      <Navbar />
      <main className="max-w-7xl mx-auto px-4 py-12 grid lg:grid-cols-3 gap-8">
        {/* Left: Exercises List */}
        <div className="lg:col-span-1 space-y-4">
          <div className="relative w-full">
            <Search className="absolute left-3 top-3 w-5 h-5 text-muted-foreground" />
            <Input
              placeholder="Search exercises..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="pl-10"
            />
          </div>

          <div className="flex flex-wrap gap-2">
            {categories.map((category) => (
              <Button
                key={category}
                variant={selectedCategory === category || (category === "All" && !selectedCategory) ? "default" : "outline"}
                className={
                  selectedCategory === category || (category === "All" && !selectedCategory)
                    ? "bg-[#89219E] text-white hover:bg-white hover:text-[#89219E]"
                    : ""
                }
                onClick={() => setSelectedCategory(category === "All" ? null : category)}
              >
                {category}
              </Button>
            ))}
          </div>

          <div className="space-y-2 overflow-y-auto max-h-[80vh]">
            {filteredExercises.map((exercise) => {
              const completed = getExerciseStatus(exercise.id)
              return (
                <Card
                  key={exercise.id}
                  className="cursor-pointer hover:shadow-md"
                  onClick={() => handleSelectExercise(exercise)}
                >
                  <CardHeader className="pb-2 flex justify-between items-center">
                    <CardTitle className="text-sm">{exercise.title}</CardTitle>
                    {completed && <CheckCircle className="w-4 h-4 text-green-500" />}
                  </CardHeader>
                  <CardContent className="text-xs text-muted-foreground">{exercise.description}</CardContent>
                </Card>
              )
            })}
          </div>
        </div>

        {/* Right: Exercise Panel */}
        <motion.div
          key={selectedExercise?.id || "empty-editor"}
          initial={{ x: 50, opacity: 0 }}
          animate={{ x: 0, opacity: 1 }}
          exit={{ x: 50, opacity: 0 }}
          transition={{ duration: 0.4, ease: [0.4, 0, 0.2, 1] }}
          className="lg:col-span-2 space-y-4"
        >
          {selectedExercise && (
            <Button
              variant="outline"
              onClick={() => setSelectedExercise(null)}
              className="mb-2 flex items-center gap-2"
            >
              <ChevronLeft size={18} /> Back
            </Button>
          )}

          <AnimatePresence>
            {selectedExercise && (
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: 20 }}
                transition={{ duration: 0.3 }}
              >
                <Card>
                  <CardHeader className="flex justify-between items-center">
                    <CardTitle className="flex items-center gap-2">
                      {selectedExercise.title}

                      {/* Hoverable Hint Icon */}
                      {selectedExercise.hints?.length > 0 && (
  <div
    className="relative group"
    onMouseEnter={() => setShowHints(true)}
    onMouseLeave={() => setShowHints(false)}
  >
    <Lightbulb className="w-5 h-5 text-yellow-500 cursor-pointer" />
    {showHints && (
      <div className="absolute left-6 top-0 w-64 bg-yellow-50 border border-yellow-300 rounded p-2 text-sm text-yellow-900 z-10">
        {selectedExercise.hints[currentHintIndex]}

        {/* Counter */}
        {selectedExercise.hints.length > 1 && (
          <div className="text-xs text-yellow-800 font-medium mt-1 text-right">
            {currentHintIndex + 1}/{selectedExercise.hints.length}
          </div>
        )}

        {/* Prev/Next Buttons */}
        {selectedExercise.hints.length > 1 && (
          <div className="flex justify-between mt-2">
            <Button
              size="sm"
              variant="outline"
              onClick={() =>
                setCurrentHintIndex((prev) =>
                  prev === 0 ? selectedExercise.hints!.length - 1 : prev - 1
                )
              }
            >
              ◀
            </Button>
            <Button
              size="sm"
              variant="outline"
              onClick={() =>
                setCurrentHintIndex((prev) =>
                  prev === selectedExercise.hints!.length - 1 ? 0 : prev + 1
                )
              }
            >
              ▶
            </Button>
          </div>
        )}
      </div>
    )}
  </div>
)}

                    </CardTitle>

                    {isCompleted && (
                      <div className="flex items-center gap-2 text-green-600">
                        <CheckCircle size={16} />
                        <span>Completed</span>
                      </div>
                    )}
                  </CardHeader>
                  <CardContent className="space-y-4">
                    <p>{selectedExercise.description}</p>
                    <div className="flex items-center gap-2">
                      <Badge className={`${DIFFICULTY_COLORS[selectedExercise.difficulty as keyof typeof DIFFICULTY_COLORS]}`}>
                        {selectedExercise.difficulty}
                      </Badge>
                      <Badge variant="outline">{selectedExercise.category}</Badge>
                    </div>
                  </CardContent>
                </Card>
              </motion.div>
            )}
          </AnimatePresence>

          {/* Code Editor */}
          <Card>
            <CardHeader className="flex justify-between items-center">
              <CardTitle>Code Editor</CardTitle>
              {selectedExercise && (
                <div className="flex gap-2">
                  <Button variant="outline" size="sm" onClick={() => setCode(selectedExercise.starterCode)}>
                    Reset
                  </Button>
                  <Button onClick={handleVerify} disabled={isVerifying}>
                    {isVerifying ? "Verifying..." : "Verify"}
                  </Button>
                </div>
              )}
            </CardHeader>
            <CardContent>
              <CodeEditor
                value={code}
                onChange={setCode}
                language="rust"
                placeholder={!selectedExercise ? "// Start typing your Rust code here..." : ""}
              />
            </CardContent>
          </Card>

          {/* Verification Result */}
          <AnimatePresence>
            {selectedExercise && verificationResult && (
              <motion.div
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: "auto" }}
                exit={{ opacity: 0, height: 0 }}
                transition={{ duration: 0.3 }}
              >
                <Card className={verificationResult.success ? "border-green-500/50 bg-green-500/5" : "border-red-500/50 bg-red-500/5"}>
                  <CardHeader>
                    {verificationResult.success ? (
                      <div className="flex items-center gap-2 text-green-600">
                        <CheckCircle />
                        <CardTitle>Success!</CardTitle>
                      </div>
                    ) : (
                      <div className="flex items-center gap-2 text-red-600">
                        <AlertCircle />
                        <CardTitle>{verificationResult.message}</CardTitle>
                      </div>
                    )}
                  </CardHeader>
                  <CardContent>
                    {verificationResult.errors?.length > 0 && (
                      <div className="bg-red-100 border border-red-300 rounded p-2 text-xs">
                        {verificationResult.errors.join("\n")}
                      </div>
                    )}
                    <p className="text-xs text-muted-foreground">
                      Execution time: {verificationResult.executionTime?.toFixed(0)}ms
                    </p>
                  </CardContent>
                </Card>
              </motion.div>
            )}
          </AnimatePresence>
        </motion.div>
      </main>
    </div>
  )
}
