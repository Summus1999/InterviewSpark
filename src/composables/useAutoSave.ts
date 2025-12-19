/**
 * Auto-save composable with debounce
 * Automatically saves draft with configurable delay
 */

import { ref, watch, onBeforeUnmount } from 'vue'
import type { Ref } from 'vue'
import { saveDraft, getDraft, deleteDraft } from '../services/draftStorage'
import type { Draft } from '../services/draftStorage'

export interface AutoSaveOptions {
  draftId: string
  draftType: Draft['type']
  debounceMs?: number
  metadata?: Record<string, unknown>
}

export function useAutoSave(content: Ref<string>, options: AutoSaveOptions) {
  const isSaving = ref(false)
  const lastSaved = ref<Date | null>(null)
  const error = ref<string | null>(null)

  const debounceMs = options.debounceMs || 5000 // Default 5 seconds
  let debounceTimer: number | null = null

  /**
   * Save draft immediately
   */
  async function saveNow() {
    if (!content.value.trim()) {
      return
    }

    isSaving.value = true
    error.value = null

    try {
      await saveDraft({
        id: options.draftId,
        type: options.draftType,
        content: content.value,
        metadata: options.metadata,
        timestamp: Date.now()
      })

      lastSaved.value = new Date()
    } catch (err) {
      error.value = String(err)
      console.error('Failed to save draft:', err)
    } finally {
      isSaving.value = false
    }
  }

  /**
   * Save with debounce
   */
  function scheduleSave() {
    if (debounceTimer !== null) {
      clearTimeout(debounceTimer)
    }

    debounceTimer = window.setTimeout(() => {
      saveNow()
    }, debounceMs)
  }

  /**
   * Load draft on initialization
   */
  async function loadDraft() {
    try {
      const draft = await getDraft(options.draftId)
      if (draft) {
        content.value = draft.content
        lastSaved.value = new Date(draft.timestamp)
      }
    } catch (err) {
      console.error('Failed to load draft:', err)
    }
  }

  /**
   * Clear draft
   */
  async function clearDraft() {
    try {
      await deleteDraft(options.draftId)
      lastSaved.value = null
    } catch (err) {
      console.error('Failed to clear draft:', err)
    }
  }

  // Watch content changes
  watch(content, () => {
    scheduleSave()
  })

  // Cleanup on unmount
  onBeforeUnmount(() => {
    if (debounceTimer !== null) {
      clearTimeout(debounceTimer)
    }
    // Save immediately on unmount if there are unsaved changes
    if (content.value.trim() && !isSaving.value) {
      saveNow()
    }
  })

  return {
    isSaving,
    lastSaved,
    error,
    saveNow,
    loadDraft,
    clearDraft
  }
}
