<!--
  VoiceControls.vue - Voice interaction controls
  
  Features:
  - Record answer button
  - Voice playback control
  - Volume adjustment
-->
<template>
  <div class="voice-controls">
    <div class="control-group">
      <!-- Record button -->
      <button
        @click="toggleRecording"
        :class="['record-btn', { recording: isRecording, transcribing: isTranscribing }]"
        :disabled="disabled || isTranscribing"
      >
        <span class="icon">{{ isTranscribing ? '⏳' : (isRecording ? '⏹' : '🎤') }}</span>
        <span class="label">{{ isTranscribing ? '识别中...' : (isRecording ? '停止录音' : '语音回答') }}</span>
      </button>

      <!-- Play/Stop question button -->
      <button
        v-if="canPlayQuestion"
        @click="toggleQuestionPlayback"
        :class="['play-btn', { playing: isPlayingQuestion }]"
        :disabled="disabled"
      >
        <span class="icon">{{ isPlayingQuestion ? '⏸' : '🔊' }}</span>
        <span class="label">{{ isPlayingQuestion ? '停止播放' : '播放问题' }}</span>
      </button>
    </div>

    <!-- Recording indicator -->
    <div v-if="isRecording" class="recording-indicator">
      <div class="pulse"></div>
      <span>正在录音...</span>
    </div>

    <!-- Transcribing indicator -->
    <div v-if="isTranscribing" class="transcribing-indicator">
      <div class="spinner"></div>
      <span>{{ webSpeechFailed ? '正在使用云端识别...' : '正在识别语音...' }}</span>
      <button v-if="canCancelTranscription" @click="cancelTranscription" class="cancel-btn">
        取消
      </button>
    </div>

    <!-- Error display -->
    <div v-if="transcribeError" class="error-message">
      {{ transcribeError }}
    </div>

    <!-- Transcript display -->
    <div v-if="transcript" class="transcript">
      <strong>识别结果：</strong>
      <p>{{ transcript }}</p>
    </div>

    <!-- Voice settings -->
    <div v-if="showSettings" class="settings">
      <label>
        语速:
        <input
          v-model.number="voiceRate"
          type="range"
          min="0.5"
          max="2"
          step="0.1"
        >
        <span>{{ voiceRate }}x</span>
      </label>
      
      <label>
        音量:
        <input
          v-model.number="voiceVolume"
          type="range"
          min="0"
          max="1"
          step="0.1"
        >
        <span>{{ Math.round(voiceVolume * 100) }}%</span>
      </label>
    </div>

    <!-- Settings toggle -->
    <button @click="showSettings = !showSettings" class="settings-toggle">
      {{ showSettings ? '隐藏设置' : '语音设置' }}
    </button>
  </div>
</template>

<script setup lang="ts">
/**
 * Voice controls component
 * Manages recording and playback
 * Uses AudioRecorder + API for reliable transcription in Tauri builds
 */
import { ref, computed } from 'vue'
import { tts, stt, audioRecorder, AudioRecorder } from '../services/voice'
import { transcribeAudio } from '../services/database'

const props = defineProps<{
  disabled?: boolean
  currentQuestion?: string
}>()

const emit = defineEmits<{
  transcript: [text: string]
  recordingStart: []
  recordingEnd: []
}>()

const isRecording = ref(false)
const isTranscribing = ref(false)
const isPlayingQuestion = ref(false)
const transcript = ref('')
const showSettings = ref(false)
const voiceRate = ref(1.0)
const voiceVolume = ref(1.0)
const transcribeError = ref('')
const webSpeechFailed = ref(false)
const webSpeechTimer = ref<ReturnType<typeof setTimeout> | null>(null)
const canCancelTranscription = ref(false)
let transcriptionAbortFlag = false

const canPlayQuestion = computed(() => !!props.currentQuestion)

/**
 * Toggle recording - tries Web Speech API first, falls back to SiliconFlow API
 */
const toggleRecording = async () => {
  if (isTranscribing.value) return

  if (isRecording.value) {
    // Stop recording
    await stopRecording()
  } else {
    // Start recording
    await startRecording()
  }
}

/**
 * Start recording - prioritize Web Speech API
 */
const startRecording = async () => {
  transcribeError.value = ''
  transcript.value = ''
  webSpeechFailed.value = false
  transcriptionAbortFlag = false
  
  // Strategy 1: Try Web Speech API first (real-time, no latency)
  if (stt) {
    try {
      let hasReceivedResult = false
      
      stt.start(
        (text) => {
          hasReceivedResult = true
          if (webSpeechTimer.value) {
            clearTimeout(webSpeechTimer.value)
            webSpeechTimer.value = null
          }
          transcript.value = text
          emit('transcript', text)
        },
        () => {
          isRecording.value = false
          emit('recordingEnd')
          if (webSpeechTimer.value) {
            clearTimeout(webSpeechTimer.value)
            webSpeechTimer.value = null
          }
        }
      )
      isRecording.value = true
      emit('recordingStart')
      
      // Set 3 second detection timer - if no result, switch to API fallback
      webSpeechTimer.value = setTimeout(() => {
        if (!hasReceivedResult && isRecording.value) {
          console.log('Web Speech API failed to respond in 3s, switching to SiliconFlow API')
          webSpeechFailed.value = true
          stt.stop()
          startApiRecording()
        }
      }, 3000)
      
      return
    } catch (error) {
      console.error('Web Speech API failed:', error)
      webSpeechFailed.value = true
    }
  }
  
  // Strategy 2: Direct API recording if Web Speech unavailable
  await startApiRecording()
}

/**
 * Start API-based recording (AudioRecorder + SiliconFlow)
 */
const startApiRecording = async () => {
  if (!audioRecorder) {
    transcribeError.value = '您的设备不支持语音录制功能'
    return
  }
  
  try {
    await audioRecorder.start()
    isRecording.value = true
    emit('recordingStart')
  } catch (error) {
    console.error('Failed to start recording:', error)
    transcribeError.value = '无法启动录音，请检查麦克风权限'
  }
}

/**
 * Stop recording
 */
const stopRecording = async () => {
  // Clear Web Speech detection timer
  if (webSpeechTimer.value) {
    clearTimeout(webSpeechTimer.value)
    webSpeechTimer.value = null
  }
  
  // If using Web Speech API
  if (stt && stt.isActive() && !webSpeechFailed.value) {
    stt.stop()
    isRecording.value = false
    emit('recordingEnd')
    return
  }
  
  // If using API recording
  if (audioRecorder && audioRecorder.isRecording()) {
    await stopAndTranscribe()
  }
}

/**
 * Stop API recording and transcribe
 */
const stopAndTranscribe = async () => {
  if (!audioRecorder || !audioRecorder.isRecording()) {
    return
  }

  try {
    isRecording.value = false
    isTranscribing.value = true
    canCancelTranscription.value = true
    transcriptionAbortFlag = false
    
    const audioBlob = await audioRecorder.stop()
    const base64 = await AudioRecorder.blobToBase64(audioBlob)
    
    // Frontend 15 second timeout wrapper
    const transcriptionPromise = transcribeAudio(base64)
    const timeoutPromise = new Promise<never>((_, reject) => {
      setTimeout(() => reject(new Error('Frontend timeout after 15 seconds')), 15000)
    })
    
    const text = await Promise.race([transcriptionPromise, timeoutPromise])
    
    if (!transcriptionAbortFlag) {
      transcript.value = text
      emit('transcript', text)
    }
  } catch (error: any) {
    if (!transcriptionAbortFlag) {
      console.error('Transcription failed:', error)
      if (error.message?.includes('timeout')) {
        transcribeError.value = '语音识别超时，请检查网络后重试'
      } else {
        transcribeError.value = '语音识别失败，请重试'
      }
    }
  } finally {
    isTranscribing.value = false
    canCancelTranscription.value = false
    emit('recordingEnd')
  }
}

/**
 * Cancel ongoing transcription
 */
const cancelTranscription = () => {
  transcriptionAbortFlag = true
  isTranscribing.value = false
  canCancelTranscription.value = false
  transcribeError.value = ''
  emit('recordingEnd')
}

/**
 * Toggle question playback
 */
const toggleQuestionPlayback = async () => {
  if (!props.currentQuestion) return

  if (isPlayingQuestion.value) {
    // Stop playback
    tts.stop()
    isPlayingQuestion.value = false
  } else {
    // Start playback
    isPlayingQuestion.value = true
    
    try {
      await tts.speak(props.currentQuestion, {
        rate: voiceRate.value,
        volume: voiceVolume.value
      })
    } catch (error) {
      console.error('TTS error:', error)
    } finally {
      isPlayingQuestion.value = false
    }
  }
}

/**
 * Play text (exposed method)
 */
const playText = async (text: string) => {
  try {
    await tts.speak(text, {
      rate: voiceRate.value,
      volume: voiceVolume.value
    })
  } catch (error) {
    console.error('TTS error:', error)
  }
}

/**
 * Stop all voice activities
 */
const stopAll = () => {
  if (isRecording.value) {
    if (audioRecorder && audioRecorder.isRecording()) {
      audioRecorder.stop().catch(() => {})
    }
    stt?.stop()
    isRecording.value = false
  }
  if (isPlayingQuestion.value) {
    tts.stop()
    isPlayingQuestion.value = false
  }
}

// Expose methods
defineExpose({
  playText,
  stopAll
})
</script>

<style scoped>
.voice-controls {
  padding: 1.5rem;
  background: #f8f9ff;
  border-radius: 8px;
  margin: 1rem 0;
}

.control-group {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
}

.record-btn,
.play-btn {
  flex: 1;
  padding: 1rem;
  border: 2px solid #667eea;
  border-radius: 8px;
  background: white;
  color: #667eea;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}

.record-btn:hover:not(:disabled),
.play-btn:hover:not(:disabled) {
  background: #667eea;
  color: white;
  transform: translateY(-2px);
}

.record-btn:disabled,
.play-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.record-btn.recording {
  background: #ff5757;
  color: white;
  border-color: #ff5757;
  animation: pulse 1.5s infinite;
}

.record-btn.transcribing {
  background: #667eea;
  color: white;
  border-color: #667eea;
}

.play-btn.playing {
  background: #667eea;
  color: white;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.7;
  }
}

.icon {
  font-size: 1.5rem;
}

.recording-indicator {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.8rem;
  background: #ffe5e5;
  border-radius: 6px;
  color: #ff5757;
  margin-bottom: 1rem;
}

.transcribing-indicator {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.8rem;
  background: #e5e9ff;
  border-radius: 6px;
  color: #667eea;
  margin-bottom: 1rem;
}

.cancel-btn {
  margin-left: auto;
  padding: 0.3rem 0.8rem;
  background: white;
  border: 1px solid #667eea;
  border-radius: 4px;
  color: #667eea;
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.2s;
}

.cancel-btn:hover {
  background: #667eea;
  color: white;
}

.error-message {
  padding: 0.8rem;
  background: #ffe5e5;
  border-radius: 6px;
  color: #cc3333;
  margin-bottom: 1rem;
  font-size: 0.9rem;
}

.pulse {
  width: 12px;
  height: 12px;
  background: #ff5757;
  border-radius: 50%;
  animation: pulse-dot 1s infinite;
}

@keyframes pulse-dot {
  0%, 100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.5);
    opacity: 0.5;
  }
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid #667eea;
  border-top-color: transparent;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.transcript {
  padding: 1rem;
  background: white;
  border-left: 3px solid #667eea;
  border-radius: 4px;
  margin-bottom: 1rem;
}

.transcript p {
  margin: 0.5rem 0 0;
  color: #333;
  line-height: 1.6;
}

.settings {
  padding: 1rem;
  background: white;
  border-radius: 6px;
  margin-bottom: 1rem;
}

.settings label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.8rem;
  color: #555;
}

.settings input[type="range"] {
  flex: 1;
}

.settings span {
  min-width: 3rem;
  text-align: right;
  font-weight: 500;
}

.settings-toggle {
  padding: 0.5rem 1rem;
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  color: #666;
  cursor: pointer;
  transition: all 0.3s;
}

.settings-toggle:hover {
  background: #f5f5f5;
  border-color: #667eea;
  color: #667eea;
}
</style>
