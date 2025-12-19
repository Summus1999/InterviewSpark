/**
 * Question Bank Store
 * Preloads and caches question bank data
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface QuestionBankItem {
  id: number
  question: string
  best_answer: string | null
  notes: string | null
  job_category: string | null
  created_at: string
  updated_at: string
}

export const useQuestionBankStore = defineStore('questionBank', () => {
  const questions = ref<QuestionBankItem[]>([])
  const isLoading = ref(false)
  const isLoaded = ref(false)
  const lastFetchTime = ref<number | null>(null)
  const error = ref<string | null>(null)

  // Cache duration: 5 minutes
  const CACHE_DURATION = 5 * 60 * 1000

  /**
   * Check if cache is valid
   */
  const isCacheValid = computed(() => {
    if (!lastFetchTime.value) return false
    return Date.now() - lastFetchTime.value < CACHE_DURATION
  })

  /**
   * Load question bank from backend
   */
  async function loadQuestionBank(forceRefresh = false) {
    // Use cache if valid and not forcing refresh
    if (!forceRefresh && isCacheValid.value && isLoaded.value) {
      return questions.value
    }

    isLoading.value = true
    error.value = null

    try {
      const result = await invoke<QuestionBankItem[]>('db_get_bank')
      questions.value = result
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
   * Get questions by category
   */
  const getQuestionsByCategory = computed(() => {
    return (category: string) => {
      return questions.value.filter((q) => q.job_category === category)
    }
  })

  /**
   * Search questions by keyword
   */
  function searchQuestions(keyword: string) {
    const lowerKeyword = keyword.toLowerCase()
    return questions.value.filter(
      (q) =>
        q.question.toLowerCase().includes(lowerKeyword) ||
        q.notes?.toLowerCase().includes(lowerKeyword)
    )
  }

  /**
   * Add question to local cache
   */
  function addQuestion(question: QuestionBankItem) {
    questions.value.push(question)
  }

  /**
   * Update question in local cache
   */
  function updateQuestion(id: number, updates: Partial<QuestionBankItem>) {
    const index = questions.value.findIndex((q) => q.id === id)
    if (index !== -1) {
      questions.value[index] = { ...questions.value[index], ...updates }
    }
  }

  /**
   * Delete question from local cache
   */
  function deleteQuestion(id: number) {
    const index = questions.value.findIndex((q) => q.id === id)
    if (index !== -1) {
      questions.value.splice(index, 1)
    }
  }

  /**
   * Clear cache
   */
  function clearCache() {
    questions.value = []
    isLoaded.value = false
    lastFetchTime.value = null
  }

  return {
    questions,
    isLoading,
    isLoaded,
    error,
    isCacheValid,
    loadQuestionBank,
    getQuestionsByCategory,
    searchQuestions,
    addQuestion,
    updateQuestion,
    deleteQuestion,
    clearCache
  }
})
