/**
 * Streaming service for AI response
 * Handles event-based streaming from Tauri backend
 */

import { listen, UnlistenFn } from '@tauri-apps/api/event'

export interface StreamCallbacks {
  onChunk?: (chunk: string) => void
  onComplete?: () => void
  onError?: (error: string) => void
}

/**
 * Listen to streaming answer feedback
 * @returns Unlisten function to clean up
 */
export async function listenAnswerFeedback(
  callbacks: StreamCallbacks
): Promise<UnlistenFn> {
  const unlistenChunk = await listen<string>('answer-feedback-chunk', (event) => {
    callbacks.onChunk?.(event.payload)
  })

  const unlistenComplete = await listen('answer-feedback-complete', () => {
    callbacks.onComplete?.()
  })

  // Return combined unlisten function
  return () => {
    unlistenChunk()
    unlistenComplete()
  }
}
