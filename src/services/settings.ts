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
  API_SETTINGS: 'interview-spark-api'
}

export type Theme = 'light' | 'dark'

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
  { value: 'Qwen/Qwen3-8B', label: 'Qwen3-8B' },
  { value: 'Qwen/Qwen2.5-Plus', label: 'Qwen Plus' },
  { value: 'Qwen/Qwen-Max', label: 'Qwen Max' },
  { value: 'Moonshot/Kimi-Large', label: 'Kimi Large' },
  { value: 'THUDM/GLM-4V-Flash', label: 'GLM-4-6v' },
  { value: 'MiniMax/MiniMax-M2', label: 'MiniMax-M2' },
]

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
    model: 'Qwen/Qwen3-8B',
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
