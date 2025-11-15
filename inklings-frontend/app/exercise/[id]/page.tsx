// "use client"

// import { useState, useEffect } from "react"
// import { Button } from "@/components/ui/button"
// import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
// import { Badge } from "@/components/ui/badge"
// import { Lightbulb, CheckCircle, AlertCircle, ChevronLeft } from "lucide-react"
// import CodeEditor from "@/components/code-editor"
// import { storage } from "@/lib/storage"
// import { verifyExercise } from "@/lib/verification"
// import { motion, AnimatePresence } from "framer-motion"
// import { exercises } from "@/lib/exercises"

// const DIFFICULTY_COLORS = {
//   Beginner: "bg-[#89219E] text-white",
//   Intermediate: "bg-[#060270] text-white",
//   Advanced: "bg-[#89219E] text-white",
// }

// interface Props {
//   selectedExercise: typeof exercises[0] | null
//   setSelectedExercise: (exercise: typeof exercises[0] | null) => void
// }

// export default function ExercisePanel({ selectedExercise, setSelectedExercise }: Props) {
//   const [code, setCode] = useState(selectedExercise?.starterCode || "")
//   const [showHints, setShowHints] = useState(false)
//   const [currentHintIndex, setCurrentHintIndex] = useState(0)
//   const [isVerifying, setIsVerifying] = useState(false)
//   const [verificationResult, setVerificationResult] = useState<any>(null)
//   const [attempts, setAttempts] = useState(0)
//   const [isCompleted, setIsCompleted] = useState(false)

//   // Reset code when a new exercise is selected
//   useEffect(() => {
//     if (selectedExercise) {
//       setCode(selectedExercise.starterCode)
//       setVerificationResult(null)
//       setAttempts(0)
//       setIsCompleted(false)
//       setCurrentHintIndex(0)
//     } else {
//       setCode("")
//       setVerificationResult(null)
//       setAttempts(0)
//       setIsCompleted(false)
//       setCurrentHintIndex(0)
//     }
//   }, [selectedExercise])

//   const handleVerify = async () => {
//     if (!selectedExercise) return

//     setIsVerifying(true)
//     const newAttempts = attempts + 1
//     setAttempts(newAttempts)

//     try {
//       const result = await verifyExercise(selectedExercise.id, code)
//       setVerificationResult(result)

//       storage.setProgress("userId", selectedExercise.id, {
//         code,
//         attempts: newAttempts,
//         completed: result.success,
//         lastAttempted: new Date().toISOString(),
//       })
//       if (result.success) setIsCompleted(true)
//     } catch (error) {
//       setVerificationResult({
//         success: false,
//         message: "Verification error",
//         errors: ["An error occurred during verification"],
//         executionTime: 0,
//       })
//     } finally {
//       setIsVerifying(false)
//     }
//   }

//   return (
//     <motion.div
//       key={selectedExercise?.id || "empty-editor"}
//       initial={{ x: 50, opacity: 0 }}
//       animate={{ x: 0, opacity: 1 }}
//       exit={{ x: 50, opacity: 0 }}
//       transition={{ duration: 0.4, ease: [0.4, 0, 0.2, 1] }}
//       className="lg:col-span-2 space-y-4"
//     >
//       {selectedExercise && (
//         <Button
//           variant="outline"
//           onClick={() => setSelectedExercise(null)}
//           className="mb-2 flex items-center gap-2"
//         >
//           <ChevronLeft size={18} /> Back
//         </Button>
//       )}

//       {/* Exercise Details */}
//       {selectedExercise && (
//         <AnimatePresence>
//           <motion.div
//             initial={{ opacity: 0, y: 20 }}
//             animate={{ opacity: 1, y: 0 }}
//             exit={{ opacity: 0, y: 20 }}
//             transition={{ duration: 0.3 }}
//           >
//             <Card>
//               <CardHeader className="flex justify-between items-center">
//                 <CardTitle className="flex items-center gap-2">
//                   {selectedExercise.title}
//                   {selectedExercise.hints?.length > 0 && (
//                     <div
//                       className="relative group"
//                       onMouseEnter={() => setShowHints(true)}
//                       onMouseLeave={() => setShowHints(false)}
//                     >
//                       <Lightbulb className="w-5 h-5 text-yellow-500 cursor-pointer" />
//                       {showHints && (
//                         <div className="absolute left-6 top-0 w-64 bg-yellow-50 border border-yellow-300 rounded p-2 text-sm text-yellow-900 z-10">
//                           {selectedExercise.hints[currentHintIndex]}
//                           {selectedExercise.hints.length > 1 && (
//                             <div className="text-xs text-yellow-800 font-medium mt-1 text-right">
//                               {currentHintIndex + 1}/{selectedExercise.hints.length}
//                             </div>
//                           )}
//                           {selectedExercise.hints.length > 1 && (
//                             <div className="flex justify-between mt-2">
//                               <Button
//                                 size="sm"
//                                 variant="outline"
//                                 onClick={() =>
//                                   setCurrentHintIndex((prev) =>
//                                     prev === 0 ? selectedExercise.hints!.length - 1 : prev - 1
//                                   )
//                                 }
//                               >
//                                 ◀
//                               </Button>
//                               <Button
//                                 size="sm"
//                                 variant="outline"
//                                 onClick={() =>
//                                   setCurrentHintIndex((prev) =>
//                                     prev === selectedExercise.hints!.length - 1 ? 0 : prev + 1
//                                   )
//                                 }
//                               >
//                                 ▶
//                               </Button>
//                             </div>
//                           )}
//                         </div>
//                       )}
//                     </div>
//                   )}
//                 </CardTitle>

//                 {isCompleted && (
//                   <div className="flex items-center gap-2 text-green-600">
//                     <CheckCircle size={16} />
//                     <span>Completed</span>
//                   </div>
//                 )}
//               </CardHeader>
//               <CardContent className="space-y-4">
//                 <p>{selectedExercise.description}</p>
//                 <div className="flex items-center gap-2">
//                   <Badge className={`${DIFFICULTY_COLORS[selectedExercise.difficulty as keyof typeof DIFFICULTY_COLORS]}`}>
//                     {selectedExercise.difficulty}
//                   </Badge>
//                   <Badge variant="outline">{selectedExercise.category}</Badge>
//                 </div>
//               </CardContent>
//             </Card>
//           </motion.div>
//         </AnimatePresence>
//       )}

//       {/* Code Editor (always shown) */}
//       <Card>
//         <CardHeader className="flex justify-between items-center">
//           <CardTitle>Code Editor</CardTitle>
//           {selectedExercise && (
//             <div className="flex gap-2">
//               <Button variant="outline" size="sm" onClick={() => setCode(selectedExercise.starterCode)}>
//                 Reset
//               </Button>
//               <Button onClick={handleVerify} disabled={isVerifying}>
//                 {isVerifying ? "Verifying..." : "Verify"}
//               </Button>
//             </div>
//           )}
//         </CardHeader>
//         <CardContent>
//           <CodeEditor
//             value={code}
//             onChange={setCode}
//             language="rust"
//             placeholder={!selectedExercise ? "// Start typing your Rust code here..." : ""}
//           />
//         </CardContent>
//       </Card>

//       {/* Verification Result */}
//       {selectedExercise && verificationResult && (
//         <AnimatePresence>
//           <motion.div
//             initial={{ opacity: 0, height: 0 }}
//             animate={{ opacity: 1, height: "auto" }}
//             exit={{ opacity: 0, height: 0 }}
//             transition={{ duration: 0.3 }}
//           >
//             <Card className={verificationResult.success ? "border-green-500/50 bg-green-500/5" : "border-red-500/50 bg-red-500/5"}>
//               <CardHeader>
//                 {verificationResult.success ? (
//                   <div className="flex items-center gap-2 text-green-600">
//                     <CheckCircle />
//                     <CardTitle>Success!</CardTitle>
//                   </div>
//                 ) : (
//                   <div className="flex items-center gap-2 text-red-600">
//                     <AlertCircle />
//                     <CardTitle>{verificationResult.message}</CardTitle>
//                   </div>
//                 )}
//               </CardHeader>
//               <CardContent>
//                 {verificationResult.errors?.length > 0 && (
//                   <div className="bg-red-100 border border-red-300 rounded p-2 text-xs">
//                     {verificationResult.errors.join("\n")}
//                   </div>
//                 )}
//                 <p className="text-xs text-muted-foreground">
//                   Execution time: {verificationResult.executionTime?.toFixed(0)}ms
//                 </p>
//               </CardContent>
//             </Card>
//           </motion.div>
//         </AnimatePresence>
//       )}
//     </motion.div>
//   )
// }
