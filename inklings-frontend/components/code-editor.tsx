"use client"

import type React from "react"

import { useEffect, useRef, useState } from "react"

interface CodeEditorProps {
  value: string
  onChange: (value: string) => void
  language?: string
}

export default function CodeEditor({ value, onChange, language = "rust" }: CodeEditorProps) {
  const textareaRef = useRef<HTMLTextAreaElement>(null)
  const containerRef = useRef<HTMLDivElement>(null)
  const [isMounted, setIsMounted] = useState(false)

  useEffect(() => {
    setIsMounted(true)
  }, [])

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    onChange(e.target.value)
    syncScroll(e.target)
  }

  const syncScroll = (textarea: HTMLTextAreaElement) => {
    if (containerRef.current) {
      const highlight = containerRef.current.querySelector(".highlight-code") as HTMLElement
      if (highlight) {
        highlight.scrollTop = textarea.scrollTop
        highlight.scrollLeft = textarea.scrollLeft
      }
    }
  }

  const handleScroll = () => {
    if (textareaRef.current) {
      syncScroll(textareaRef.current)
    }
  }

  if (!isMounted) {
    return null
  }

  return (
    <div
      ref={containerRef}
      className="relative w-full h-[500px] border border-border rounded-lg overflow-hidden bg-card"
    >
      {/* Highlighted code background */}
      <pre className="highlight-code absolute inset-0 pointer-events-none overflow-hidden p-3 font-mono text-sm leading-relaxed whitespace-pre-wrap break-words text-transparent bg-background">
        <code className="text-transparent">{value}</code>
      </pre>

      {/* Textarea overlay */}
      <textarea
        ref={textareaRef}
        value={value}
        onChange={handleChange}
        onScroll={handleScroll}
        spellCheck="false"
        className="absolute inset-0 w-full h-full p-3 font-mono text-sm leading-relaxed resize-none outline-none bg-transparent text-foreground caret-foreground whitespace-pre-wrap break-words focus:ring-0"
        style={{
          fontFamily: "'Geist Mono', 'Monaco', 'Courier New', monospace",
          lineHeight: "1.5",
        }}
      />

      {/* Line numbers */}
      <div className="absolute left-0 top-0 bottom-0 w-12 bg-muted text-muted-foreground text-right pr-2 pt-3 font-mono text-sm overflow-hidden">
        {value.split("\n").map((_, i) => (
          <div key={i} className="h-6 leading-6 text-xs">
            {i + 1}
          </div>
        ))}
      </div>

      {/* Adjust textarea padding to account for line numbers */}
      <style jsx>{`
        textarea {
          padding-left: 3.5rem;
        }
      `}</style>
    </div>
  )
}
