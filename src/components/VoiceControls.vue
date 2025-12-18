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
        :class="['record-btn', { recording: isRecording }]"
        :disabled="disabled"
      >
        <span class="icon">{{ isRecording ? '⏹' : '🎤' }}</span>
        <span class="label">{{ isRecording ? '停止录音' : '语音回答' }}</span>
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
 */
import { ref, computed } from 'vue'
import { tts, stt } from '../services/voice'

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
const isPlayingQuestion = ref(false)
const transcript = ref('')
const showSettings = ref(false)
const voiceRate = ref(1.0)
const voiceVolume = ref(1.0)

const canPlayQuestion = computed(() => !!props.currentQuestion)

/**
 * Toggle recording
 */
const toggleRecording = () => {
  if (!stt) {
    alert('您的浏览器不支持语音识别功能')
    return
  }

  if (isRecording.value) {
    // Stop recording
    stt.stop()
    isRecording.value = false
    emit('recordingEnd')
  } else {
    // Start recording
    transcript.value = ''
    emit('recordingStart')
    
    stt.start(
      (text) => {
        transcript.value = text
        emit('transcript', text)
      },
      () => {
        isRecording.value = false
        emit('recordingEnd')
      }
    )
    
    isRecording.value = true
  }
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
