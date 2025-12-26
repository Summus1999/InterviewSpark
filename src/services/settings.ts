/**
 * Settings service - User preferences and app configuration
 */

import type { FollowUpSettings } from '../types/follow-up'
import { DEFAULT_FOLLOWUP_SETTINGS } from '../types/follow-up'

const STORAGE_KEYS = {
  THEME: 'interview-spark-theme',
  ONBOARDING_COMPLETED: 'interview-spark-onboarding',
  VOICE_SETTINGS: 'interview-spark-voice',
  TIMER_SETTINGS: 'interview-spark-timer',
  FOLLOWUP_SETTINGS: 'interview-spark-followup',
  API_SETTINGS: 'interview-spark-api',
  INTERVIEWER_PERSONA: 'interview-spark-persona'
}

export type Theme = 'light' | 'dark'

export type InterviewerPersona = 'strict' | 'friendly' | 'stress' | 'balanced'

export interface VoiceSettings {
  rate: number
  volume: number
  enabled: boolean
}

export interface TimerConfig {
  enabled: boolean
  timePerQuestion: number  // seconds
  autoSubmit: boolean
  showWarning: boolean
}

export interface ApiSettings {
  model: string
  apiKey: string
}

export const AVAILABLE_MODELS = [
  { value: 'Pro/zai-org/GLM-4.7', label: 'GLM-4.7', description: '智谱轻量' },
  { value: 'Qwen/Qwen3-VL-235B-A22B-Instruct', label: 'Qwen Plus', description: '通义视觉' },
  { value: 'Qwen/Qwen3-235B-A22B-Thinking-2507', label: 'Qwen Max', description: '通义推理' },
  { value: 'moonshotai/Kimi-K2-Thinking', label: 'Kimi Large', description: '月之暗面' },
  { value: 'zai-org/GLM-4.6V', label: 'GLM-4-6v', description: '智谱视觉' },
  { value: 'MiniMaxAI/MiniMax-M2', label: 'MiniMax-M2', description: '海螺AI' },
]

// The flagship model used for AI analysis reports
export const FLAGSHIP_MODEL = 'Qwen/Qwen3-235B-A22B-Thinking-2507'

/**
 * Theme management
 */
export class ThemeManager {
  private static instance: ThemeManager
  private currentTheme: Theme = 'light'

  private constructor() {
    this.loadTheme()
  }

  static getInstance(): ThemeManager {
    if (!ThemeManager.instance) {
      ThemeManager.instance = new ThemeManager()
    }
    return ThemeManager.instance
  }

  /**
   * Load theme from localStorage or system preference
   */
  private loadTheme(): void {
    const savedTheme = localStorage.getItem(STORAGE_KEYS.THEME) as Theme | null
    
    if (savedTheme) {
      this.currentTheme = savedTheme
    } else {
      // Detect system preference
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      this.currentTheme = prefersDark ? 'dark' : 'light'
    }
    
    this.applyTheme()
  }

  /**
   * Get current theme
   */
  getTheme(): Theme {
    return this.currentTheme
  }

  /**
   * Set theme and persist to localStorage
   */
  setTheme(theme: Theme): void {
    this.currentTheme = theme
    localStorage.setItem(STORAGE_KEYS.THEME, theme)
    this.applyTheme()
  }

  /**
   * Toggle between light and dark theme
   */
  toggleTheme(): Theme {
    const newTheme: Theme = this.currentTheme === 'light' ? 'dark' : 'light'
    this.setTheme(newTheme)
    return newTheme
  }

  /**
   * Apply theme to document
   */
  private applyTheme(): void {
    document.documentElement.setAttribute('data-theme', this.currentTheme)
  }

  /**
   * Listen to system theme changes
   */
  listenToSystemTheme(callback: (theme: Theme) => void): void {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    mediaQuery.addEventListener('change', (e) => {
      const newTheme: Theme = e.matches ? 'dark' : 'light'
      this.setTheme(newTheme)
      callback(newTheme)
    })
  }
}

/**
 * Onboarding state management
 */
export class OnboardingManager {
  static isCompleted(): boolean {
    return localStorage.getItem(STORAGE_KEYS.ONBOARDING_COMPLETED) === 'true'
  }

  static markCompleted(): void {
    localStorage.setItem(STORAGE_KEYS.ONBOARDING_COMPLETED, 'true')
  }

  static reset(): void {
    localStorage.removeItem(STORAGE_KEYS.ONBOARDING_COMPLETED)
  }
}

/**
 * Voice settings management
 */
export class VoiceSettingsManager {
  private static defaultSettings: VoiceSettings = {
    rate: 1.0,
    volume: 1.0,
    enabled: true
  }

  static getSettings(): VoiceSettings {
    const saved = localStorage.getItem(STORAGE_KEYS.VOICE_SETTINGS)
    if (saved) {
      try {
        return JSON.parse(saved)
      } catch {
        return this.defaultSettings
      }
    }
    return this.defaultSettings
  }

  static saveSettings(settings: VoiceSettings): void {
    localStorage.setItem(STORAGE_KEYS.VOICE_SETTINGS, JSON.stringify(settings))
  }
}

/**
 * Timer settings management
 */
export class TimerSettingsManager {
  private static defaultSettings: TimerConfig = {
    enabled: false,
    timePerQuestion: 180,  // 3 minutes default
    autoSubmit: false,
    showWarning: true
  }

  static getSettings(): TimerConfig {
    const saved = localStorage.getItem(STORAGE_KEYS.TIMER_SETTINGS)
    if (saved) {
      try {
        return JSON.parse(saved)
      } catch {
        return this.defaultSettings
      }
    }
    return this.defaultSettings
  }

  static saveSettings(settings: TimerConfig): void {
    localStorage.setItem(STORAGE_KEYS.TIMER_SETTINGS, JSON.stringify(settings))
  }

  static reset(): void {
    localStorage.removeItem(STORAGE_KEYS.TIMER_SETTINGS)
  }
}

/**
 * Follow-up settings management
 */
export class FollowUpSettingsManager {
  static getSettings(): FollowUpSettings {
    const saved = localStorage.getItem(STORAGE_KEYS.FOLLOWUP_SETTINGS)
    if (saved) {
      try {
        return JSON.parse(saved)
      } catch {
        return DEFAULT_FOLLOWUP_SETTINGS
      }
    }
    return DEFAULT_FOLLOWUP_SETTINGS
  }

  static saveSettings(settings: FollowUpSettings): void {
    localStorage.setItem(STORAGE_KEYS.FOLLOWUP_SETTINGS, JSON.stringify(settings))
  }

  static reset(): void {
    localStorage.removeItem(STORAGE_KEYS.FOLLOWUP_SETTINGS)
  }
}

/**
 * API settings management
 */
export class ApiSettingsManager {
  private static defaultSettings: ApiSettings = {
    model: 'Pro/zai-org/GLM-4.7',
    apiKey: ''
  }

  static getSettings(): ApiSettings {
    const saved = localStorage.getItem(STORAGE_KEYS.API_SETTINGS)
    if (saved) {
      try {
        return JSON.parse(saved)
      } catch {
        return this.defaultSettings
      }
    }
    return this.defaultSettings
  }

  static saveSettings(settings: ApiSettings): void {
    localStorage.setItem(STORAGE_KEYS.API_SETTINGS, JSON.stringify(settings))
  }

  static reset(): void {
    localStorage.removeItem(STORAGE_KEYS.API_SETTINGS)
  }
}

/**
 * Tooltip dismissed state management
 */
export class TooltipManager {
  private static DISMISSED_KEY = 'tooltip_dismissed_ids'

  static isDismissed(tooltipId: string): boolean {
    const dismissed = this.getDismissedIds()
    return dismissed.includes(tooltipId)
  }

  static dismiss(tooltipId: string): void {
    const dismissed = this.getDismissedIds()
    if (!dismissed.includes(tooltipId)) {
      dismissed.push(tooltipId)
      localStorage.setItem(this.DISMISSED_KEY, JSON.stringify(dismissed))
    }
  }

  static resetAll(): void {
    localStorage.removeItem(this.DISMISSED_KEY)
  }

  private static getDismissedIds(): string[] {
    const saved = localStorage.getItem(this.DISMISSED_KEY)
    if (saved) {
      try {
        return JSON.parse(saved)
      } catch {
        return []
      }
    }
    return []
  }
}

/**
 * Interviewer persona management
 */
export class InterviewerPersonaManager {
  private static defaultPersona: InterviewerPersona = 'balanced'

  static getPersona(): InterviewerPersona {
    const saved = localStorage.getItem(STORAGE_KEYS.INTERVIEWER_PERSONA)
    if (saved && this.isValidPersona(saved)) {
      return saved as InterviewerPersona
    }
    return this.defaultPersona
  }

  static setPersona(persona: InterviewerPersona): void {
    localStorage.setItem(STORAGE_KEYS.INTERVIEWER_PERSONA, persona)
  }

  static reset(): void {
    localStorage.removeItem(STORAGE_KEYS.INTERVIEWER_PERSONA)
  }

  private static isValidPersona(value: string): boolean {
    return ['strict', 'friendly', 'stress', 'balanced'].includes(value)
  }

  static getPersonaLabel(persona: InterviewerPersona): string {
    const labels: Record<InterviewerPersona, string> = {
      strict: '严肃型',
      friendly: '友好型',
      stress: '压力型',
      balanced: '平衡型'
    }
    return labels[persona]
  }

  static getPersonaDescription(persona: InterviewerPersona): string {
    const descriptions: Record<InterviewerPersona, string> = {
      strict: '高标准评判，直接指出问题，要求精确，语气正式',
      friendly: '鼓励式反馈，肯定优点，温和建议，语气亲切',
      stress: '挑战式提问，追问细节，压力测试，语气直接',
      balanced: '平衡的建设性反馈，综合各种面试风格'
    }
    return descriptions[persona]
  }
}
