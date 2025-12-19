/**
 * Draft auto-save service using IndexedDB
 * Automatically saves user input with debounce
 */

const DB_NAME = 'InterviewSparkDB'
const DB_VERSION = 1
const DRAFT_STORE = 'drafts'

export interface Draft {
  id: string
  type: 'answer' | 'resume' | 'job_description'
  content: string
  metadata?: Record<string, unknown>
  timestamp: number
}

/**
 * Initialize IndexedDB
 */
function openDB(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(DB_NAME, DB_VERSION)

    request.onerror = () => reject(request.error)
    request.onsuccess = () => resolve(request.result)

    request.onupgradeneeded = (event) => {
      const db = (event.target as IDBOpenDBRequest).result

      // Create object store if not exists
      if (!db.objectStoreNames.contains(DRAFT_STORE)) {
        const store = db.createObjectStore(DRAFT_STORE, { keyPath: 'id' })
        store.createIndex('type', 'type', { unique: false })
        store.createIndex('timestamp', 'timestamp', { unique: false })
      }
    }
  })
}

/**
 * Save draft to IndexedDB
 */
export async function saveDraft(draft: Draft): Promise<void> {
  const db = await openDB()

  return new Promise((resolve, reject) => {
    const transaction = db.transaction([DRAFT_STORE], 'readwrite')
    const store = transaction.objectStore(DRAFT_STORE)

    const request = store.put(draft)

    request.onsuccess = () => resolve()
    request.onerror = () => reject(request.error)
  })
}

/**
 * Get draft by ID
 */
export async function getDraft(id: string): Promise<Draft | null> {
  const db = await openDB()

  return new Promise((resolve, reject) => {
    const transaction = db.transaction([DRAFT_STORE], 'readonly')
    const store = transaction.objectStore(DRAFT_STORE)

    const request = store.get(id)

    request.onsuccess = () => resolve(request.result || null)
    request.onerror = () => reject(request.error)
  })
}

/**
 * Get all drafts by type
 */
export async function getDraftsByType(type: Draft['type']): Promise<Draft[]> {
  const db = await openDB()

  return new Promise((resolve, reject) => {
    const transaction = db.transaction([DRAFT_STORE], 'readonly')
    const store = transaction.objectStore(DRAFT_STORE)
    const index = store.index('type')

    const request = index.getAll(type)

    request.onsuccess = () => resolve(request.result)
    request.onerror = () => reject(request.error)
  })
}

/**
 * Delete draft by ID
 */
export async function deleteDraft(id: string): Promise<void> {
  const db = await openDB()

  return new Promise((resolve, reject) => {
    const transaction = db.transaction([DRAFT_STORE], 'readwrite')
    const store = transaction.objectStore(DRAFT_STORE)

    const request = store.delete(id)

    request.onsuccess = () => resolve()
    request.onerror = () => reject(request.error)
  })
}

/**
 * Clear all drafts
 */
export async function clearAllDrafts(): Promise<void> {
  const db = await openDB()

  return new Promise((resolve, reject) => {
    const transaction = db.transaction([DRAFT_STORE], 'readwrite')
    const store = transaction.objectStore(DRAFT_STORE)

    const request = store.clear()

    request.onsuccess = () => resolve()
    request.onerror = () => reject(request.error)
  })
}
