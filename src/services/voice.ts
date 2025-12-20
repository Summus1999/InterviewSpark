/**
 * Voice service - Text-to-Speech and Speech-to-Text
 * Using Web Speech API for browser-native voice capabilities
 */

// Speech synthesis (TTS)
export class TextToSpeech {
  private synthesis: SpeechSynthesis
  private utterance: SpeechSynthesisUtterance | null = null
  private isInitialized = false

  constructor() {
    this.synthesis = window.speechSynthesis
  }

  /**
   * Initialize and select Chinese voice
   */
  async init(): Promise<void> {
    if (this.isInitialized) return

    // Wait for voices to load
    return new Promise((resolve) => {
      if (this.synthesis.getVoices().length > 0) {
        this.isInitialized = true
        resolve()
      } else {
        this.synthesis.addEventListener('voiceschanged', () => {
          this.isInitialized = true
          resolve()
        }, { once: true })
      }
    })
  }

  /**
   * Get available Chinese voices
   */
  getChineseVoices(): SpeechSynthesisVoice[] {
    const voices = this.synthesis.getVoices()
    return voices.filter(voice => voice.lang.startsWith('zh'))
  }

  /**
   * Speak text
   */
  async speak(text: string, options?: {
    rate?: number
    pitch?: number
    volume?: number
    voice?: SpeechSynthesisVoice
  }): Promise<void> {
    await this.init()

    // Stop current speech
    this.stop()

    return new Promise((resolve, reject) => {
      this.utterance = new SpeechSynthesisUtterance(text)
      
      // Set options
      this.utterance.rate = options?.rate ?? 1.0
      this.utterance.pitch = options?.pitch ?? 1.0
      this.utterance.volume = options?.volume ?? 1.0

      // Select voice
      if (options?.voice) {
        this.utterance.voice = options.voice
      } else {
        // Auto-select Chinese voice
        const chineseVoices = this.getChineseVoices()
        if (chineseVoices.length > 0) {
          this.utterance.voice = chineseVoices[0]
        }
      }

      this.utterance.onend = () => resolve()
      this.utterance.onerror = (event) => reject(event.error)

      this.synthesis.speak(this.utterance)
    })
  }

  /**
   * Stop speaking
   */
  stop(): void {
    this.synthesis.cancel()
  }

  /**
   * Check if currently speaking
   */
  isSpeaking(): boolean {
    return this.synthesis.speaking
  }

  /**
   * Pause speaking
   */
  pause(): void {
    this.synthesis.pause()
  }

  /**
   * Resume speaking
   */
  resume(): void {
    this.synthesis.resume()
  }
}

// Speech recognition (ASR)
export class SpeechToText {
  private recognition: any // SpeechRecognition type
  private isListening = false
  private onResultCallback: ((text: string) => void) | null = null
  private onEndCallback: (() => void) | null = null
  private silenceTimer: ReturnType<typeof setTimeout> | null = null
  private silenceTimeout = 10000 // 10 seconds of silence before auto-stop

  constructor() {
    // Check browser support
    const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition
    
    if (!SpeechRecognition) {
      throw new Error('Speech recognition not supported in this browser')
    }

    this.recognition = new SpeechRecognition()
    this.recognition.lang = 'zh-CN'
    this.recognition.continuous = true  // Keep listening until manual stop or silence timeout
    this.recognition.interimResults = true  // Get interim results to reset silence timer
    this.recognition.maxAlternatives = 1

    // Setup event handlers
    this.recognition.onresult = (event: any) => {
      // Reset silence timer on any speech activity
      this.resetSilenceTimer()
      
      // Get the latest result
      const resultIndex = event.results.length - 1
      const result = event.results[resultIndex]
      
      // Only emit final results
      if (result.isFinal) {
        const transcript = result[0].transcript
        if (this.onResultCallback) {
          this.onResultCallback(transcript)
        }
      }
    }

    this.recognition.onspeechstart = () => {
      // Reset silence timer when speech starts
      this.resetSilenceTimer()
    }

    this.recognition.onspeechend = () => {
      // Start silence timer when speech ends
      this.startSilenceTimer()
    }

    this.recognition.onend = () => {
      this.clearSilenceTimer()
      this.isListening = false
      if (this.onEndCallback) {
        this.onEndCallback()
      }
    }

    this.recognition.onerror = (event: any) => {
      console.error('Speech recognition error:', event.error)
      this.clearSilenceTimer()
      this.isListening = false
    }
  }

  /**
   * Start silence timer
   */
  private startSilenceTimer(): void {
    this.clearSilenceTimer()
    this.silenceTimer = setTimeout(() => {
      console.log('Silence timeout reached, stopping recognition')
      this.stop()
    }, this.silenceTimeout)
  }

  /**
   * Reset silence timer
   */
  private resetSilenceTimer(): void {
    this.clearSilenceTimer()
    this.startSilenceTimer()
  }

  /**
   * Clear silence timer
   */
  private clearSilenceTimer(): void {
    if (this.silenceTimer) {
      clearTimeout(this.silenceTimer)
      this.silenceTimer = null
    }
  }

  /**
   * Start listening
   */
  start(
    onResult: (text: string) => void,
    onEnd?: () => void
  ): void {
    if (this.isListening) {
      this.stop()
    }

    this.onResultCallback = onResult
    this.onEndCallback = onEnd || null

    try {
      this.recognition.start()
      this.isListening = true
      // Start silence timer immediately
      this.startSilenceTimer()
    } catch (error) {
      console.error('Failed to start recognition:', error)
    }
  }

  /**
   * Stop listening
   */
  stop(): void {
    this.clearSilenceTimer()
    if (this.isListening) {
      this.recognition.stop()
      this.isListening = false
    }
  }

  /**
   * Check if currently listening
   */
  isActive(): boolean {
    return this.isListening
  }

  /**
   * Check if speech recognition is supported
   */
  static isSupported(): boolean {
    return !!(
      (window as any).SpeechRecognition ||
      (window as any).webkitSpeechRecognition
    )
  }
}

// Export singleton instances
export const tts = new TextToSpeech()
export const stt = SpeechToText.isSupported() ? new SpeechToText() : null
