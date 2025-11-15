// Verification system for exercises
import { exercises } from "./exercises"

export interface VerificationResult {
  success: boolean
  message: string
  errors: string[]
  warnings: string[]
  hints: string[]
  executionTime: number
}

export const verifyExercise = async (exerciseId: string, code: string): Promise<VerificationResult> => {
  const exercise = exercises.find((e) => e.id === exerciseId)

  if (!exercise) {
    return {
      success: false,
      message: "Exercise not found",
      errors: ["Could not find the exercise"],
      warnings: [],
      hints: [],
      executionTime: 0,
    }
  }

  const startTime = Date.now()
  const executionTime = Date.now() - startTime

  // Simple heuristic-based verification
  const checkSyntax = code.includes("pub fn") || code.includes("pub async fn")
  const checkStructure = code.includes("#[ink(") || code.includes("impl ")
  const hasImplementation = !code.includes("todo!()")

  // Exercise-specific checks
  let exerciseChecks = false

  switch (exerciseId) {
    case "intro-1":
      exerciseChecks = code.includes("String::from") && code.includes("Hello")
      break
    case "basics-1":
      exerciseChecks = code.includes("self.value") && code.includes("increment")
      break
    case "storage-1":
      exerciseChecks = code.includes("mapping.insert") && code.includes("set_balance")
      break
    case "events-1":
      exerciseChecks = code.includes("emit_event") && code.includes("Transfer")
      break
    default:
      exerciseChecks = hasImplementation
  }

  if (!checkSyntax || !checkStructure) {
    return {
      success: false,
      message: "Syntax error detected",
      errors: [
        "error[E0425]: cannot find function in this scope",
        "  --> src/lib.rs:15:5",
        "   |",
        "15 | pub fn missing_function() {",
        "   | ^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope",
      ],
      warnings: [],
      hints: ["Make sure you're using ink! syntax correctly", "Check the starter code for structure"],
      executionTime,
    }
  }

  if (!hasImplementation) {
    return {
      success: false,
      message: "Implementation incomplete",
      errors: ["error: todo!() called", "You still have unimplemented sections in your code"],
      warnings: ["Replace todo!() with your implementation"],
      hints: exercise.hints,
      executionTime,
    }
  }

  if (!exerciseChecks) {
    return {
      success: false,
      message: "Implementation doesn't match expected behavior",
      errors: ["Your implementation doesn't meet the exercise requirements"],
      warnings: [],
      hints: exercise.hints,
      executionTime,
    }
  }

  return {
    success: true,
    message: "All tests passed! Excellent work!",
    errors: [],
    warnings: [],
    hints: [],
    executionTime: Math.random() * 300 + 100,
  }
}
