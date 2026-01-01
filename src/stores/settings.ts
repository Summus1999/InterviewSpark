/**
 * Unified Settings Store
 * Centralized state management for all application settings
 */

import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import type { FollowUpSettings } from '../types/follow-up'
import { DEFAULT_FOLLOWUP_SETTINGS } from '../types/follow-up'

// Storage keys
const STORAGE_KEYS = {
  THEME: 'interview-spark-theme',
  ONBOARDING_COMPLETED: 'interview-spark-onboarding',
  VOICE_SETTINGS: 'interview-spark-voice',
  TIMER_SETTINGS: 'interview-spark-timer',
  FOLLOWUP_SETTINGS: 'interview-spark-followup',
  API_SETTINGS: 'interview-spark-api',
  INTERVIEWER_PERSONA: 'interview-spark-persona',
  MULTI_AGENT_MODE: 'interview-spark-multi-agent',
  TOOLTIP_DISMISSED: 'tooltip_dismissed_ids'
}

// Types
export type Theme = 'light' | 'dark'
export type InterviewerPersona = 'strict' | 'friendly' | 'stress' | 'balanced'

export interface VoiceSettings {
  rate: number
  volume: number
  enabled: boolean
}

export interface TimerConfig {
  enabled: boolean
  timePerQuestion: number
  autoSubmit: boolean
  showWarning: boolean
}

export interface ApiSettings {
  model: string
  apiKey: string
}

// Default values
const DEFAULT_VOICE_SETTINGS: VoiceSettings = {
  rate: 1.0,
  volume: 1.0,
  enabled: true
}

const DEFAULT_TIMER_SETTINGS: TimerConfig = {
  enabled: false,
  timePerQuestion: 180,
  autoSubmit: false,
  showWarning: true
}

const DEFAULT_API_SETTINGS: ApiSettings = {
  model: 'Pro/zai-org/GLM-4.7',
  apiKey: ''
}

// Available models
export const AVAILABLE_MODELS = [
  { value: 'Pro/zai-org/GLM-4.7', label: 'GLM-4.7', description: '智谱轻量' },
  { value: 'Qwen/Qwen3-VL-235B-A22B-Instruct', label: 'Qwen Plus', description: '通义视觉' },
  { value: 'Qwen/Qwen3-235B-A22B-Thinking-2507', label: 'Qwen Max', description: '通义推理' },
  { value: 'moonshotai/Kimi-K2-Thinking', label: 'Kimi Large', description: '月之暗面' },
  { value: 'zai-org/GLM-4.6V', label: 'GLM-4-6v', description: '智谱视觉' },
  { value: 'MiniMaxAI/MiniMax-M2', label: 'MiniMax-M2', description: '海螺AI' },
]

export const FLAGSHIP_MODEL = 'Qwen/Qwen3-235B-A22B-Thinking-2507'

// Persona labels and descriptions
const PERSONA_LABELS: Record<InterviewerPersona, string> = {
  strict: '严肃型',
  friendly: '友好型',
  stress: '压力型',
  balanced: '平衡型'
}

const PERSONA_DESCRIPTIONS: Record<InterviewerPersona, string> = {
  strict: '高标准评判，直接指出问题，要求精确，语气正式',
  friendly: '鼓励式反馈，肯定优点，温和建议，语气亲切',
  stress: '挑战式提问，追问细节，压力测试，语气直接',
  balanced: '平衡的建设性反馈，综合各种面试风格'
}

// Helper functions
function loadFromStorage<T>(key: string, defaultValue: T): T {
  const saved = localStorage.getItem(key)
  if (saved) {
    try {
      return JSON.parse(saved)
    } catch {
      return defaultValue
    }
  }
  return defaultValue
}

function saveToStorage<T>(key: string, value: T): void {
  localStorage.setItem(key, JSON.stringify(value))
}

function loadThemeFromStorage(): Theme {
  const saved = localStorage.getItem(STORAGE_KEYS.THEME) as Theme | null
  if (saved) return saved
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
  return prefersDark ? 'dark' : 'light'
}

function loadPersonaFromStorage(): InterviewerPersona {
  const saved = localStorage.getItem(STORAGE_KEYS.INTERVIEWER_PERSONA)
  if (saved && ['strict', 'friendly', 'stress', 'balanced'].includes(saved)) {
    return saved as InterviewerPersona
  }
  return 'balanced'
}

export const useSettingsStore = defineStore('settings', () => {
  // Theme
  const theme = ref<Theme>(loadThemeFromStorage())
  
  // Onboarding
  const onboardingCompleted = ref(localStorage.getItem(STORAGE_KEYS.ONBOARDING_COMPLETED) === 'true')
  
  // Voice settings
  const voiceSettings = ref<VoiceSettings>(loadFromStorage(STORAGE_KEYS.VOICE_SETTINGS, DEFAULT_VOICE_SETTINGS))
  
  // Timer settings
  const timerSettings = ref<TimerConfig>(loadFromStorage(STORAGE_KEYS.TIMER_SETTINGS, DEFAULT_TIMER_SETTINGS))
  
  // Follow-up settings
  const followUpSettings = ref<FollowUpSettings>(loadFromStorage(STORAGE_KEYS.FOLLOWUP_SETTINGS, DEFAULT_FOLLOWUP_SETTINGS))
  
  // API settings
  const apiSettings = ref<ApiSettings>(loadFromStorage(STORAGE_KEYS.API_SETTINGS, DEFAULT_API_SETTINGS))
  
  // Interviewer persona
  const persona = ref<InterviewerPersona>(loadPersonaFromStorage())
  
  // Dismissed tooltips
  const dismissedTooltips = ref<string[]>(loadFromStorage(STORAGE_KEYS.TOOLTIP_DISMISSED, []))
  
  // Multi-agent mode
  const multiAgentMode = ref<boolean>(loadFromStorage(STORAGE_KEYS.MULTI_AGENT_MODE, false))

  // Computed values
  const isDarkTheme = computed(() => theme.value === 'dark')
  const isApiConfigured = computed(() => apiSettings.value.apiKey.length > 0)
  const personaLabel = computed(() => PERSONA_LABELS[persona.value])
  const personaDescription = computed(() => PERSONA_DESCRIPTIONS[persona.value])

  // Theme actions
  function setTheme(newTheme: Theme) {
    theme.value = newTheme
    localStorage.setItem(STORAGE_KEYS.THEME, newTheme)
    document.documentElement.setAttribute('data-theme', newTheme)
  }

  function toggleTheme(): Theme {
    const newTheme: Theme = theme.value === 'light' ? 'dark' : 'light'
    setTheme(newTheme)
    return newTheme
  }

  function applyTheme() {
    document.documentElement.setAttribute('data-theme', theme.value)
  }

  // Onboarding actions
  function completeOnboarding() {
    onboardingCompleted.value = true
    localStorage.setItem(STORAGE_KEYS.ONBOARDING_COMPLETED, 'true')
  }

  function resetOnboarding() {
    onboardingCompleted.value = false
    localStorage.removeItem(STORAGE_KEYS.ONBOARDING_COMPLETED)
  }

  // Voice settings actions
  function updateVoiceSettings(settings: Partial<VoiceSettings>) {
    voiceSettings.value = { ...voiceSettings.value, ...settings }
    saveToStorage(STORAGE_KEYS.VOICE_SETTINGS, voiceSettings.value)
  }

  // Timer settings actions
  function updateTimerSettings(settings: Partial<TimerConfig>) {
    timerSettings.value = { ...timerSettings.value, ...settings }
    saveToStorage(STORAGE_KEYS.TIMER_SETTINGS, timerSettings.value)
  }

  function resetTimerSettings() {
    timerSettings.value = { ...DEFAULT_TIMER_SETTINGS }
    localStorage.removeItem(STORAGE_KEYS.TIMER_SETTINGS)
  }

  // Follow-up settings actions
  function updateFollowUpSettings(settings: Partial<FollowUpSettings>) {
    followUpSettings.value = { ...followUpSettings.value, ...settings }
    saveToStorage(STORAGE_KEYS.FOLLOWUP_SETTINGS, followUpSettings.value)
  }

  function resetFollowUpSettings() {
    followUpSettings.value = { ...DEFAULT_FOLLOWUP_SETTINGS }
    localStorage.removeItem(STORAGE_KEYS.FOLLOWUP_SETTINGS)
  }

  // API settings actions
  function updateApiSettings(settings: Partial<ApiSettings>) {
    apiSettings.value = { ...apiSettings.value, ...settings }
    saveToStorage(STORAGE_KEYS.API_SETTINGS, apiSettings.value)
  }

  function resetApiSettings() {
    apiSettings.value = { ...DEFAULT_API_SETTINGS }
    localStorage.removeItem(STORAGE_KEYS.API_SETTINGS)
  }

  // Persona actions
  function setPersona(newPersona: InterviewerPersona) {
    persona.value = newPersona
    localStorage.setItem(STORAGE_KEYS.INTERVIEWER_PERSONA, newPersona)
  }

  function getPersonaLabel(p: InterviewerPersona): string {
    return PERSONA_LABELS[p]
  }

  function getPersonaDescription(p: InterviewerPersona): string {
    return PERSONA_DESCRIPTIONS[p]
  }

  // Tooltip actions
  function isTooltipDismissed(tooltipId: string): boolean {
    return dismissedTooltips.value.includes(tooltipId)
  }

  function dismissTooltip(tooltipId: string) {
    if (!dismissedTooltips.value.includes(tooltipId)) {
      dismissedTooltips.value.push(tooltipId)
      saveToStorage(STORAGE_KEYS.TOOLTIP_DISMISSED, dismissedTooltips.value)
    }
  }

  function resetTooltips() {
    dismissedTooltips.value = []
    localStorage.removeItem(STORAGE_KEYS.TOOLTIP_DISMISSED)
  }
  
  // Multi-agent mode actions
  function setMultiAgentMode(enabled: boolean) {
    multiAgentMode.value = enabled
    saveToStorage(STORAGE_KEYS.MULTI_AGENT_MODE, enabled)
  }

  // Reset all settings
  function resetAllSettings() {
    resetTimerSettings()
    resetFollowUpSettings()
    resetApiSettings()
    resetTooltips()
    setTheme('light')
    setPersona('balanced')
  }

  // Watch for system theme changes
  function listenToSystemTheme(callback?: (theme: Theme) => void) {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    mediaQuery.addEventListener('change', (e) => {
      const newTheme: Theme = e.matches ? 'dark' : 'light'
      setTheme(newTheme)
      callback?.(newTheme)
    })
  }

  // Initialize theme on load
  applyTheme()

  return {
    // State
    theme,
    onboardingCompleted,
    voiceSettings,
    timerSettings,
    followUpSettings,
    apiSettings,
    persona,
    dismissedTooltips,
    multiAgentMode,

    // Computed
    isDarkTheme,
    isApiConfigured,
    personaLabel,
    personaDescription,

    // Theme actions
    setTheme,
    toggleTheme,
    applyTheme,
    listenToSystemTheme,

    // Onboarding actions
    completeOnboarding,
    resetOnboarding,

    // Voice settings actions
    updateVoiceSettings,

    // Timer settings actions
    updateTimerSettings,
    resetTimerSettings,

    // Follow-up settings actions
    updateFollowUpSettings,
    resetFollowUpSettings,

    // API settings actions
    updateApiSettings,
    resetApiSettings,

    // Persona actions
    setPersona,
    getPersonaLabel,
    getPersonaDescription,

    // Tooltip actions
    isTooltipDismissed,
    dismissTooltip,
    resetTooltips,
    
    // Multi-agent mode actions
    setMultiAgentMode,

    // Global reset
    resetAllSettings
  }
})
