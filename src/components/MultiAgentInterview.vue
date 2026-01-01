<template>
  <div class="multi-agent-interview">
    <!-- Setup Panel (before interview starts) -->
    <div v-if="!sessionId" class="setup-panel">
      <h2>多面试官模式</h2>
      <p class="setup-description">技术面试官、HR、业务面试官轮流提问，模拟真实的多轮面试体验</p>
      
      <div class="setup-form">
        <div class="form-group">
          <label>简历内容</label>
          <textarea v-model="resume" placeholder="请输入您的简历内容..." rows="6"></textarea>
        </div>
        <div class="form-group">
          <label>岗位描述 (JD)</label>
          <textarea v-model="jobDescription" placeholder="请输入目标岗位的JD..." rows="6"></textarea>
        </div>
        <button 
          @click="startInterview" 
          :disabled="!resume.trim() || !jobDescription.trim() || isStarting"
          class="start-button"
        >
          {{ isStarting ? '启动中...' : '开始面试' }}
        </button>
        <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>
      </div>
    </div>

    <!-- Interview Panel (after interview starts) -->
    <template v-else>
      <div class="phase-progress">
        <div class="progress-bar">
          <div
            v-for="phase in phases"
            :key="phase.name"
            class="progress-step"
            :class="{ active: phase.name === currentPhase, completed: isPhaseCompleted(phase.name) }"
          >
            {{ phase.label }}
          </div>
        </div>
      </div>

      <div class="interview-content">
        <div class="messages-container" ref="messagesContainer">
        <div
          v-for="(turn, index) in conversation"
          :key="index"
          class="message-group"
        >
          <div class="message interviewer">
            <div class="avatar" :class="`avatar-${turn.role.toLowerCase()}`">
              <span>{{ getRoleEmoji(turn.role) }}</span>
            </div>
            <div class="message-content">
              <div class="role-name">{{ turn.role_name }}</div>
              <div class="question">{{ turn.question }}</div>
            </div>
          </div>

          <div v-if="turn.answer" class="message user">
            <div class="message-content">
              <div class="role-name">我的回答</div>
              <div class="answer">{{ turn.answer }}</div>
            </div>
            <div class="avatar avatar-user">
              <span>👤</span>
            </div>
          </div>

          <div v-if="turn.analysis" class="analysis-card">
            <div class="analysis-score">评分: {{ turn.analysis.score.toFixed(1) }}</div>
            <div v-if="turn.analysis.strengths.length" class="analysis-section">
              <strong>亮点:</strong>
              <ul>
                <li v-for="(s, i) in turn.analysis.strengths" :key="i">{{ s }}</li>
              </ul>
            </div>
            <div v-if="turn.analysis.improvements.length" class="analysis-section">
              <strong>改进建议:</strong>
              <ul>
                <li v-for="(imp, i) in turn.analysis.improvements" :key="i">{{ imp }}</li>
              </ul>
            </div>
          </div>
        </div>

        <div v-if="isCompleted" class="completion-message">
          <h3>面试结束</h3>
          <p>感谢参与多角色面试！</p>
          <button @click="resetInterview" class="reset-button">开始新面试</button>
        </div>
      </div>

      <div v-if="!isCompleted" class="input-area">
        <!-- Voice Controls Integration -->
        <VoiceControls
          :current-question="currentQuestion"
          :disabled="isProcessing || isLoadingQuestion"
          @transcript="handleVoiceTranscript"
        />
        
        <textarea
          v-model="userAnswer"
          placeholder="请输入您的回答或使用语音回答..."
          :disabled="isProcessing || isLoadingQuestion"
          @keydown.ctrl.enter="submitAnswer"
        ></textarea>
        <button
          @click="submitAnswer"
          :disabled="!userAnswer.trim() || isProcessing || isLoadingQuestion"
          class="submit-button"
        >
          {{ isProcessing ? '分析中...' : '提交回答' }}
        </button>
      </div>
    </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, onUnmounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import VoiceControls from './VoiceControls.vue'
import { tts } from '../services/voice'
import { useSettingsStore } from '../stores/settings'

interface ConversationTurn {
  role: string
  role_name: string
  question: string
  answer?: string
  analysis?: {
    score: number
    strengths: string[]
    improvements: string[]
    summary: string
  }
}

interface InterviewProgress {
  current_phase: string
  phase_question_count: number
  total_question_count: number
  is_completed: boolean
}

const phases = [
  { name: 'WarmUp', label: '暖场' },
  { name: 'Technical', label: '技术' },
  { name: 'Behavioral', label: '行为' },
  { name: 'Business', label: '业务' },
  { name: 'Questions', label: '反问' }
]

// Setup state
const resume = ref('')
const jobDescription = ref('')
const isStarting = ref(false)
const errorMessage = ref('')

// Interview state
const conversation = ref<ConversationTurn[]>([])
const currentPhase = ref<string>('WarmUp')
const userAnswer = ref('')
const isProcessing = ref(false)
const isLoadingQuestion = ref(false)
const isCompleted = ref(false)
const messagesContainer = ref<HTMLElement | null>(null)
const sessionId = ref('')

// Settings store for voice settings
const settingsStore = useSettingsStore()

// Computed current question for voice controls
const currentQuestion = computed(() => {
  if (conversation.value.length > 0) {
    const lastTurn = conversation.value[conversation.value.length - 1]
    return lastTurn.answer ? '' : lastTurn.question
  }
  return ''
})

const isPhaseCompleted = (phaseName: string) => {
  const phaseIndex = phases.findIndex(p => p.name === phaseName)
  const currentIndex = phases.findIndex(p => p.name === currentPhase.value)
  return phaseIndex < currentIndex
}

const getRoleEmoji = (role: string) => {
  const emojiMap: Record<string, string> = {
    Technical: '👨‍💼',
    HR: '👩‍💼',
    Business: '👨‍💻'
  }
  return emojiMap[role] || '👤'
}

// Voice transcript handler
const handleVoiceTranscript = (transcript: string) => {
  userAnswer.value = transcript
}

const scrollToBottom = () => {
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
  }
}

// Start multi-agent interview
const startInterview = async () => {
  if (!resume.value.trim() || !jobDescription.value.trim()) return
  
  isStarting.value = true
  errorMessage.value = ''
  
  try {
    // Call Tauri command to start session
    const id = await invoke<string>('start_multi_agent_interview', {
      resume: resume.value,
      jobDescription: jobDescription.value
    })
    
    sessionId.value = id
    conversation.value = []
    isCompleted.value = false
    currentPhase.value = 'WarmUp'
    
    // Get first question
    await getNextQuestion()
  } catch (error) {
    console.error('Failed to start interview:', error)
    errorMessage.value = `启动失败: ${error}`
  } finally {
    isStarting.value = false
  }
}

// Get next question from backend
const getNextQuestion = async () => {
  if (!sessionId.value) return
  
  isLoadingQuestion.value = true
  
  try {
    const turn = await invoke<ConversationTurn>('multi_agent_next_question', {
      sessionId: sessionId.value
    })
    
    conversation.value.push(turn)
    await nextTick()
    scrollToBottom()
    
    // Speak the question with role-specific voice
    if (settingsStore.voiceSettings.enabled && turn.question) {
      try {
        await tts.speakWithRole(
          turn.question,
          turn.role as 'Technical' | 'HR' | 'Business',
          {
            rate: settingsStore.voiceSettings.rate,
            volume: settingsStore.voiceSettings.volume
          }
        )
      } catch (error) {
        console.warn('TTS failed:', error)
      }
    }
    
    // Update progress
    await updateProgress()
  } catch (error) {
    console.error('Failed to get next question:', error)
    errorMessage.value = `获取问题失败: ${error}`
  } finally {
    isLoadingQuestion.value = false
  }
}

// Update interview progress
const updateProgress = async () => {
  if (!sessionId.value) return
  
  try {
    const progress = await invoke<InterviewProgress>('multi_agent_get_progress', {
      sessionId: sessionId.value
    })
    
    currentPhase.value = progress.current_phase
    isCompleted.value = progress.is_completed
  } catch (error) {
    console.error('Failed to get progress:', error)
  }
}

// Submit answer
const submitAnswer = async () => {
  if (!userAnswer.value.trim() || isProcessing.value || !sessionId.value) return

  isProcessing.value = true
  const answer = userAnswer.value.trim()
  userAnswer.value = ''

  try {
    // Update local conversation with answer
    if (conversation.value.length > 0) {
      conversation.value[conversation.value.length - 1].answer = answer
    }
    
    await nextTick()
    scrollToBottom()
    
    // Call Tauri command to submit answer and get analysis
    const analysis = await invoke<ConversationTurn['analysis']>('multi_agent_submit_answer', {
      sessionId: sessionId.value,
      answer: answer
    })
    
    // Update analysis in conversation
    if (conversation.value.length > 0) {
      conversation.value[conversation.value.length - 1].analysis = analysis
    }
    
    await nextTick()
    scrollToBottom()
    
    // Update progress
    await updateProgress()
    
    // If not completed, get next question
    if (!isCompleted.value) {
      await getNextQuestion()
    }
  } catch (error) {
    console.error('Error submitting answer:', error)
    errorMessage.value = `提交失败: ${error}`
  } finally {
    isProcessing.value = false
  }
}

// Reset interview
const resetInterview = async () => {
  if (sessionId.value) {
    try {
      await invoke('multi_agent_end_session', { sessionId: sessionId.value })
    } catch (error) {
      console.error('Failed to end session:', error)
    }
  }
  
  sessionId.value = ''
  conversation.value = []
  isCompleted.value = false
  currentPhase.value = 'WarmUp'
  errorMessage.value = ''
}

// Cleanup on unmount
onUnmounted(async () => {
  if (sessionId.value) {
    try {
      await invoke('multi_agent_end_session', { sessionId: sessionId.value })
    } catch (error) {
      console.error('Failed to cleanup session:', error)
    }
  }
})
</script>

<style scoped>
.multi-agent-interview {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary);
}

.setup-panel {
  max-width: 600px;
  margin: 2rem auto;
  padding: 2rem;
  background: var(--bg-secondary);
  border-radius: 12px;
}

.setup-panel h2 {
  margin-bottom: 0.5rem;
  color: var(--text-primary);
}

.setup-description {
  color: var(--text-secondary);
  margin-bottom: 1.5rem;
}

.setup-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  font-weight: 600;
  color: var(--text-primary);
}

.form-group textarea {
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: inherit;
  resize: vertical;
}

.start-button {
  margin-top: 1rem;
  padding: 1rem 2rem;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
  font-size: 1rem;
  transition: all 0.3s;
}

.start-button:hover:not(:disabled) {
  background: var(--primary-hover);
  transform: translateY(-2px);
}

.start-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.error-message {
  color: var(--error-color, #e74c3c);
  margin-top: 0.5rem;
}

.reset-button {
  margin-top: 1rem;
  padding: 0.75rem 1.5rem;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
}

.phase-progress {
  padding: 1rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.progress-bar {
  display: flex;
  justify-content: space-around;
  gap: 0.5rem;
}

.progress-step {
  flex: 1;
  padding: 0.75rem;
  text-align: center;
  background: var(--bg-tertiary);
  border-radius: 8px;
  font-size: 0.9rem;
  color: var(--text-secondary);
  transition: all 0.3s;
}

.progress-step.active {
  background: var(--primary-color);
  color: white;
  font-weight: 600;
}

.progress-step.completed {
  background: var(--success-color);
  color: white;
}

.interview-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.message-group {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.message {
  display: flex;
  gap: 1rem;
  align-items: flex-start;
}

.message.user {
  flex-direction: row-reverse;
}

.avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  flex-shrink: 0;
}

.avatar-technical {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.avatar-hr {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.avatar-business {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
}

.avatar-user {
  background: var(--primary-color);
}

.message-content {
  flex: 1;
  background: var(--bg-secondary);
  padding: 1rem;
  border-radius: 12px;
  max-width: 70%;
}

.role-name {
  font-weight: 600;
  color: var(--primary-color);
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
}

.question,
.answer {
  line-height: 1.6;
  color: var(--text-primary);
}

.analysis-card {
  margin-left: 56px;
  padding: 1rem;
  background: var(--bg-tertiary);
  border-left: 3px solid var(--primary-color);
  border-radius: 8px;
}

.analysis-score {
  font-weight: 600;
  color: var(--primary-color);
  margin-bottom: 0.5rem;
}

.analysis-section {
  margin-top: 0.5rem;
}

.analysis-section ul {
  margin: 0.25rem 0 0 1.5rem;
  padding: 0;
}

.analysis-section li {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.completion-message {
  text-align: center;
  padding: 2rem;
}

.completion-message h3 {
  color: var(--success-color);
  margin-bottom: 0.5rem;
}

.input-area {
  padding: 1rem;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
  display: flex;
  gap: 1rem;
}

.input-area textarea {
  flex: 1;
  min-height: 80px;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--bg-primary);
  color: var(--text-primary);
  resize: none;
  font-family: inherit;
}

.submit-button {
  padding: 0.75rem 2rem;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.3s;
}

.submit-button:hover:not(:disabled) {
  background: var(--primary-hover);
  transform: translateY(-1px);
}

.submit-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
