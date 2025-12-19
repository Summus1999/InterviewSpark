/**
 * Data preloader composable
 * Preloads frequently used data on app startup
 */

import { onMounted } from 'vue'
import { useQuestionBankStore } from '../stores/questionBank'
import { useSessionStore } from '../stores/session'

export function useDataPreloader() {
  const questionBankStore = useQuestionBankStore()
  const sessionStore = useSessionStore()

  /**
   * Preload all data
   */
  async function preloadAll() {
    try {
      // Preload in parallel
      await Promise.all([questionBankStore.loadQuestionBank(), sessionStore.loadSessions()])

      console.log('Data preloading completed')
    } catch (error) {
      console.error('Data preloading failed:', error)
    }
  }

  /**
   * Auto preload on mount
   */
  onMounted(() => {
    preloadAll()
  })

  return {
    preloadAll
  }
}
