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
  FOLLOWUP_SETTINGS: 'interview-spark-followup'
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
