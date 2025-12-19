/**
 * Vue composable for streaming AI responses
 */

import { ref, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listenAnswerFeedback } from '../services/streaming'
import type { UnlistenFn } from '@tauri-apps/api/event'

export function useStreaming() {
  const content = ref('')
  const isStreaming = ref(false)
  const isComplete = ref(false)
  const error = ref<string | null>(null)

  let unlistenFn: UnlistenFn | null = null

  /**
   * Start streaming answer analysis
   */
  async function startStream(question: string, answer: string, jobDescription: string) {
    // Reset state
    content.value = ''
    isStreaming.value = true
    isComplete.value = false
    error.value = null

    try {
      // Setup listeners
      unlistenFn = await listenAnswerFeedback({
        onChunk: (chunk) => {
          content.value += chunk
        },
        onComplete: () => {
          isStreaming.value = false
          isComplete.value = true
        },
        onError: (err) => {
          error.value = err
          isStreaming.value = false
        }
      })

      // Start streaming from backend
      await invoke('analyze_answer_stream', {
        question,
        answer,
        jobDescription
      })
    } catch (err) {
      error.value = String(err)
      isStreaming.value = false
      cleanup()
    }
  }

  /**
   * Stop streaming and cleanup
   */
  function stopStream() {
    isStreaming.value = false
    cleanup()
  }

  /**
   * Cleanup listeners
   */
  function cleanup() {
    if (unlistenFn) {
      unlistenFn()
      unlistenFn = null
    }
  }

  // Auto cleanup on unmount
  onBeforeUnmount(() => {
    cleanup()
  })

  return {
    content,
    isStreaming,
    isComplete,
    error,
    startStream,
    stopStream
  }
}
