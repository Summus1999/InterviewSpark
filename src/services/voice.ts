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
   * Speak text with role-specific voice
   * @param text - Text to speak
   * @param role - Interviewer role (Technical, HR, Business)
   * @param options - Additional speech options
   */
  async speakWithRole(text: string, role?: 'Technical' | 'HR' | 'Business', options?: {
    rate?: number
    pitch?: number
    volume?: number
  }): Promise<void> {
    await this.init()

    // Stop current speech
    this.stop()

    return new Promise((resolve, reject) => {
      this.utterance = new SpeechSynthesisUtterance(text)
      
      // Set options
      this.utterance.rate = options?.rate ?? 1.0
      this.utterance.volume = options?.volume ?? 1.0
      
      // Set role-specific voice characteristics
      const chineseVoices = this.getChineseVoices()
      if (chineseVoices.length > 0) {
        if (role === 'Technical') {
          // Technical: lower pitch, moderate speed
          this.utterance.pitch = options?.pitch ?? 0.9
          // Prefer male voice if available
          const maleVoice = chineseVoices.find(v => v.name.includes('Male') || v.name.includes('\u7537'))
          this.utterance.voice = maleVoice || chineseVoices[0]
        } else if (role === 'HR') {
          // HR: higher pitch, friendly
          this.utterance.pitch = options?.pitch ?? 1.1
          // Prefer female voice if available
          const femaleVoice = chineseVoices.find(v => v.name.includes('Female') || v.name.includes('\u5973'))
          this.utterance.voice = femaleVoice || chineseVoices[1] || chineseVoices[0]
        } else if (role === 'Business') {
          // Business: neutral pitch, clear
          this.utterance.pitch = options?.pitch ?? 1.0
          this.utterance.voice = chineseVoices[2] || chineseVoices[0]
        } else {
          // Default
          this.utterance.pitch = options?.pitch ?? 1.0
          this.utterance.voice = chineseVoices[0]
        }
      } else {
        this.utterance.pitch = options?.pitch ?? 1.0
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
  private manualStop = false // Flag to distinguish manual stop from auto-end

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
      
      // Auto-restart if not manually stopped (browser auto-ended due to pause)
      if (this.isListening && !this.manualStop) {
        console.log('Speech recognition auto-ended, restarting...')
        try {
          this.recognition.start()
          this.startSilenceTimer()
          return // Don't trigger end callback
        } catch (error) {
          console.error('Failed to restart recognition:', error)
        }
      }
      
      this.isListening = false
      this.manualStop = false
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
    this.manualStop = false // Reset manual stop flag

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
    this.manualStop = true // Mark as manual stop to prevent auto-restart
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

/**
 * Audio Recorder using MediaRecorder API
 * Records audio and converts to base64 for API transcription
 */
export class AudioRecorder {
  private mediaRecorder: MediaRecorder | null = null
  private audioChunks: Blob[] = []
  private stream: MediaStream | null = null
  private _isRecording = false

  /**
   * Check if MediaRecorder is supported
   */
  static isSupported(): boolean {
    return !!(navigator.mediaDevices && navigator.mediaDevices.getUserMedia && window.MediaRecorder)
  }

  /**
   * Start recording audio
   */
  async start(): Promise<void> {
    if (this._isRecording) {
      return
    }

    try {
      this.stream = await navigator.mediaDevices.getUserMedia({ audio: true })
      
      // Prefer webm format for better compatibility
      const mimeType = MediaRecorder.isTypeSupported('audio/webm') 
        ? 'audio/webm' 
        : 'audio/mp4'
      
      this.mediaRecorder = new MediaRecorder(this.stream, { mimeType })
      this.audioChunks = []
      
      this.mediaRecorder.ondataavailable = (event) => {
        if (event.data.size > 0) {
          this.audioChunks.push(event.data)
        }
      }
      
      this.mediaRecorder.start(100) // Collect data every 100ms
      this._isRecording = true
    } catch (error) {
      console.error('Failed to start audio recording:', error)
      throw error
    }
  }

  /**
   * Stop recording and return audio blob
   */
  async stop(): Promise<Blob> {
    return new Promise((resolve, reject) => {
      if (!this.mediaRecorder || !this._isRecording) {
        reject(new Error('Not recording'))
        return
      }

      this.mediaRecorder.onstop = () => {
        const mimeType = this.mediaRecorder?.mimeType || 'audio/webm'
        const audioBlob = new Blob(this.audioChunks, { type: mimeType })
        
        // Stop all tracks
        if (this.stream) {
          this.stream.getTracks().forEach(track => track.stop())
          this.stream = null
        }
        
        this._isRecording = false
        resolve(audioBlob)
      }

      this.mediaRecorder.onerror = (event) => {
        this._isRecording = false
        reject(event)
      }

      this.mediaRecorder.stop()
    })
  }

  /**
   * Check if currently recording
   */
  isRecording(): boolean {
    return this._isRecording
  }

  /**
   * Convert blob to base64 string
   */
  static async blobToBase64(blob: Blob): Promise<string> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader()
      reader.onloadend = () => {
        const base64 = reader.result as string
        // Remove data URL prefix (e.g., "data:audio/webm;base64,")
        const base64Data = base64.split(',')[1]
        resolve(base64Data)
      }
      reader.onerror = reject
      reader.readAsDataURL(blob)
    })
  }
}

// Export singleton recorder instance
export const audioRecorder = AudioRecorder.isSupported() ? new AudioRecorder() : null
