/**
 * Interview Sessions Store
 * Preloads and caches interview session history
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface InterviewSession {
  id: number
  resume_id: number | null
  job_description_id: number | null
  questions: string
  created_at: string
  updated_at: string
}

export const useSessionStore = defineStore('session', () => {
  const sessions = ref<InterviewSession[]>([])
  const isLoading = ref(false)
  const isLoaded = ref(false)
  const lastFetchTime = ref<number | null>(null)
  const error = ref<string | null>(null)

  // Cache duration: 3 minutes
  const CACHE_DURATION = 3 * 60 * 1000

  /**
   * Check if cache is valid
   */
  const isCacheValid = computed(() => {
    if (!lastFetchTime.value) return false
    return Date.now() - lastFetchTime.value < CACHE_DURATION
  })

  /**
   * Load sessions from backend
   */
  async function loadSessions(forceRefresh = false) {
    if (!forceRefresh && isCacheValid.value && isLoaded.value) {
      return sessions.value
    }

    isLoading.value = true
    error.value = null

    try {
      const result = await invoke<InterviewSession[]>('db_get_sessions')
      sessions.value = result
      isLoaded.value = true
      lastFetchTime.value = Date.now()
      return result
    } catch (err) {
      error.value = String(err)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Get recent sessions
   */
  const recentSessions = computed(() => {
    return sessions.value.slice(0, 10)
  })

  /**
   * Get session by ID
   */
  function getSessionById(id: number) {
    return sessions.value.find((s) => s.id === id)
  }

  /**
   * Add session to local cache
   */
  function addSession(session: InterviewSession) {
    sessions.value.unshift(session)
  }

  /**
   * Delete session from local cache
   */
  function deleteSession(id: number) {
    const index = sessions.value.findIndex((s) => s.id === id)
    if (index !== -1) {
      sessions.value.splice(index, 1)
    }
  }

  /**
   * Clear cache
   */
  function clearCache() {
    sessions.value = []
    isLoaded.value = false
    lastFetchTime.value = null
  }

  return {
    sessions,
    isLoading,
    isLoaded,
    error,
    isCacheValid,
    recentSessions,
    loadSessions,
    getSessionById,
    addSession,
    deleteSession,
    clearCache
  }
})
