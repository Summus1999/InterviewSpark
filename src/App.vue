<template>
  <div id="app" class="app-container">
    <header>
      <h1>InterviewSpark</h1>
      <p>AI-Powered Mock Interview Platform</p>
      <div class="header-actions">
        <ThemeToggle />
      </div>
    </header>
    <main>
      <!-- Phase 1 Test Section -->
      <section v-if="showTest" class="welcome">
        <h2>IPC Connection Test</h2>
        <p>Prepare for your interviews with AI-powered mock interviews and intelligent feedback.</p>

        <div class="demo-section">
          <input
            v-model="userName"
            type="text"
            placeholder="Enter your name"
            class="name-input"
          >
          <button
            class="greet-btn"
            @click="handleGreet"
          >
            Test IPC Connection
          </button>
          <p
            v-if="greeting"
            class="greeting-message"
          >
            {{ greeting }}
          </p>
        </div>
      </section>

      <!-- Phase 2: Interview Mode -->
      <section v-else class="interview-mode">
        <div class="mode-header">
          <h2>{{ modeTitle }}</h2>
          <div class="mode-actions">
            <button @click="currentMode = 'interview'" :class="{ active: currentMode === 'interview' }" class="mode-btn">
              面试模式
            </button>
            <button @click="currentMode = 'history'" :class="{ active: currentMode === 'history' }" class="mode-btn">
              历史记录
            </button>
            <button @click="currentMode = 'bank'" :class="{ active: currentMode === 'bank' }" class="mode-btn">
              题库管理
            </button>
            <button @click="currentMode = 'dashboard'" :class="{ active: currentMode === 'dashboard' }" class="mode-btn">
              仪表板
            </button>
            <button v-if="isDev" @click="showTest = true" class="toggle-btn">
              测试模式
            </button>
          </div>
        </div>

        <!-- Interview Mode Content -->
        <div v-if="currentMode === 'interview'">
          <!-- Step 1: Input Resume and JD -->
          <div v-if="currentStep === 'input'" class="step-content">
            <ResumeInput v-model="resume" />
            <JobDescription v-model="jobDescription" />
            
            <!-- Timer Settings -->
            <div class="timer-settings-section">
              <button @click="toggleTimerSettings" class="settings-toggle-btn">
                ⏱️ {{ showTimerSettings ? '隐藏' : '显示' }}计时设置
              </button>
              <TimerSettings 
                v-if="showTimerSettings"
                v-model="timerSettings"
                @update:modelValue="handleTimerSettingsChange"
              />
            </div>
            
            <!-- Follow-up Settings -->
            <div class="followup-settings-section">
              <button @click="toggleFollowUpSettings" class="settings-toggle-btn">
                🔄 {{ showFollowUpSettings ? '隐藏' : '显示' }}追问设置
              </button>
              <FollowUpSettingsComp 
                v-if="showFollowUpSettings"
                v-model="followUpSettings"
                @update:modelValue="handleFollowUpSettingsChange"
              />
            </div>
            
            <div class="action-buttons">
              <button 
                @click="generateQuestions" 
                :disabled="!canGenerate || isLoading"
                class="primary-btn"
              >
                {{ isLoading ? '生成中...' : '生成面试问题' }}
              </button>
            </div>
            
            <p v-if="error" class="error-message">{{ error }}</p>
          </div>

          <!-- Step 2: Show Questions -->
          <div v-if="currentStep === 'questions'" class="step-content">
            <QuestionList 
              :questions="questions" 
              :current-index="currentQuestionIndex"
              @select-question="selectQuestion"
            />
            
            <div class="action-buttons">
              <button @click="startInterview" class="primary-btn">
                开始面试
              </button>
              <button @click="currentStep = 'input'" class="secondary-btn">
                重新生成问题
              </button>
            </div>
          </div>

          <!-- Step 3: Interview -->
          <div v-if="currentStep === 'interview'" class="step-content">
            <ProgressBar :current="currentQuestionIndex + 1" :total="questions.length" />
            
            <!-- Timer Display -->
            <TimerDisplay
              v-if="timerSettings.enabled"
              ref="timerRef"
              :time-limit="timerSettings.timePerQuestion"
              :auto-start="true"
              :show-warning="timerSettings.showWarning"
              @timeout="handleTimerTimeout"
              @warning="handleTimerWarning"
            />
            
            <div class="interview-progress">
              <span>问题 {{ currentQuestionIndex + 1 }} / {{ questions.length }}</span>
            </div>
            
            <div class="current-question">
              <h3>{{ questions[currentQuestionIndex] }}</h3>
            </div>

            <!-- Voice Controls -->
            <VoiceControls
              :current-question="questions[currentQuestionIndex]"
              :disabled="isLoading"
              @transcript="handleVoiceTranscript"
            />
            
            <div class="answer-input">
              <h4>您的回答：</h4>
              <textarea
                v-model="currentAnswer"
                placeholder="请输入您的回答或使用语音回答..."
                rows="8"
                class="answer-textarea"
              />
            </div>
            
            <div class="action-buttons">
              <button 
                @click="submitAnswer" 
                :disabled="!currentAnswer.trim() || isLoading"
                class="primary-btn"
              >
                {{ isLoading ? '分析中...' : '提交答案' }}
              </button>
              <button 
                v-if="currentQuestionIndex < questions.length - 1"
                @click="nextQuestion"
                class="secondary-btn"
              >
                跳过此题
              </button>
            </div>
            
            <p v-if="error" class="error-message">{{ error }}</p>
          </div>

          <!-- Step 4: Feedback -->
          <div v-if="currentStep === 'feedback'" class="step-content">
            <div class="feedback-card">
              <h3>AI 反馈</h3>
              <div class="feedback-content" v-html="formattedFeedback"></div>
            </div>
            
            <!-- Conversation History -->
            <ConversationHistory 
              v-if="conversationHistory.length > 0"
              :turns="conversationHistory"
              @clear="clearConversationHistory"
            />
            
            <div class="action-buttons">
              <button 
                v-if="currentQuestionIndex < questions.length - 1"
                @click="nextQuestionAfterFeedback"
                class="primary-btn"
              >
                下一题
              </button>
              <button 
                v-else
                @click="finishInterview"
                class="primary-btn"
              >
                完成面试
              </button>
              <button @click="currentStep = 'interview'" class="secondary-btn">
                返回答题
              </button>
            </div>
          </div>

          <!-- Step 5: Follow-up Panel -->
          <div v-if="currentStep === 'followup'" class="step-content">
            <FollowUpPanel 
              v-if="followUpAnalysis"
              :analysis="followUpAnalysis"
              @select="selectFollowUpQuestion"
              @skip="skipFollowUp"
              @custom="currentStep = 'interview'"
            />
          </div>
        </div>

        <!-- History Mode -->
        <div v-if="currentMode === 'history'">
          <InterviewHistory />
        </div>

        <!-- Question Bank Mode -->
        <div v-if="currentMode === 'bank'">
          <QuestionBank />
        </div>

        <!-- Dashboard Mode -->
        <div v-if="currentMode === 'dashboard'">
          <Dashboard />
        </div>
      </section>
    </main>
    
    <!-- Completion Animation -->
    <CompletionAnimation 
      :show="showCompletionAnimation"
      :message="`完成了 ${answersHistory.length} 个问题的回答，继续加油！`"
      @close="closeCompletionAnimation"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import ResumeInput from './components/ResumeInput.vue'
import JobDescription from './components/JobDescription.vue'
import QuestionList from './components/QuestionList.vue'
import InterviewHistory from './components/InterviewHistory.vue'
import QuestionBank from './components/QuestionBank.vue'
import Dashboard from './components/Dashboard.vue'
import VoiceControls from './components/VoiceControls.vue'
import ThemeToggle from './components/ThemeToggle.vue'
import ProgressBar from './components/ProgressBar.vue'
import CompletionAnimation from './components/CompletionAnimation.vue'
import TimerDisplay from './components/TimerDisplay.vue'
import TimerSettings from './components/TimerSettings.vue'
import FollowUpSettingsComp from './components/FollowUpSettings.vue'
import FollowUpPanel from './components/FollowUpPanel.vue'
import ConversationHistory from './components/ConversationHistory.vue'
import { createSession, saveAnswer } from './services/database'
import { tts } from './services/voice'
import { TimerSettingsManager, type TimerConfig, FollowUpSettingsManager } from './services/settings'
import type { ConversationTurn, FollowUpAnalysis, FollowUpSettings, FollowUpType } from './types/follow-up'
import { DEFAULT_FOLLOWUP_SETTINGS } from './types/follow-up'

// Development mode detection
const isDev = import.meta.env.DEV

// Phase 1 test variables
const userName = ref('')
const greeting = ref('')
const showTest = ref(false)

// Mode management
const currentMode = ref<'interview' | 'history' | 'bank' | 'dashboard'>('interview')

// Phase 2 interview variables
const currentStep = ref<'input' | 'questions' | 'interview' | 'feedback' | 'followup'>('input')
const resume = ref('')
const jobDescription = ref('')
const questions = ref<string[]>([])
const currentQuestionIndex = ref(0)
const currentAnswer = ref('')
const currentFeedback = ref('')
const isLoading = ref(false)
const error = ref('')

// Database tracking
const currentSessionId = ref<number | null>(null)
const answersHistory = ref<Array<{ question: string; answer: string; feedback: string }>>([])

// Voice feature toggle
const voiceEnabled = ref(true)

// Completion animation state
const showCompletionAnimation = ref(false)

// Timer settings
const timerSettings = ref<TimerConfig>(TimerSettingsManager.getSettings())
const timerRef = ref<InstanceType<typeof TimerDisplay> | null>(null)
const showTimerSettings = ref(false)

// Follow-up settings and state
const followUpSettings = ref<FollowUpSettings>(FollowUpSettingsManager.getSettings())
const showFollowUpSettings = ref(false)
const conversationHistory = ref<ConversationTurn[]>([])
const followUpAnalysis = ref<FollowUpAnalysis | null>(null)
const followUpCount = ref(0)  // Track how many follow-ups for current question

const canGenerate = computed(() => {
  return resume.value.trim().length > 50 && jobDescription.value.trim().length > 20
})

const formattedFeedback = computed(() => {
  return currentFeedback.value.replace(/\n/g, '<br>')
})

const modeTitle = computed(() => {
  switch (currentMode.value) {
    case 'interview': return '模拟面试'
    case 'history': return '历史记录'
    case 'bank': return '题库管理'
    case 'dashboard': return '仪表板'
    default: return '模拟面试'
  }
})

/**
 * Handle greet button click
 * Calls Rust backend via Tauri IPC to test communication
 */
const handleGreet = async () => {
  if (!userName.value.trim()) {
    greeting.value = 'Please enter your name'
    return
  }

  try {
    // Invoke Rust command 'greet' with user's name
    greeting.value = await invoke<string>('greet', { name: userName.value })
  } catch (error) {
    greeting.value = `Error: ${error}`
  }
}

/**
 * Generate interview questions
 */
const generateQuestions = async () => {
  if (!canGenerate.value) return
  
  isLoading.value = true
  error.value = ''
  
  try {
    questions.value = await invoke<string[]>('generate_questions', {
      resume: resume.value,
      jobDescription: jobDescription.value,
      count: 5
    })
    currentStep.value = 'questions'
    currentQuestionIndex.value = 0
  } catch (err) {
    error.value = `生成问题失败: ${err}`
  } finally {
    isLoading.value = false
  }
}

const startInterview = async () => {
  try {
    // Create interview session in database
    currentSessionId.value = await createSession(null, null, questions.value)
    answersHistory.value = []
    currentStep.value = 'interview'
    currentQuestionIndex.value = 0
    currentAnswer.value = ''
    
    // Auto-play first question with voice
    if (voiceEnabled.value) {
      await nextTick()
      playCurrentQuestion()
    }
  } catch (err) {
    error.value = `创建面试会话失败: ${err}`
  }
}

/**
 * Play current question with voice
 */
const playCurrentQuestion = async () => {
  if (!voiceEnabled.value || !questions.value[currentQuestionIndex.value]) return
  
  try {
    await tts.speak(questions.value[currentQuestionIndex.value])
  } catch (error) {
    console.error('Failed to play question:', error)
  }
}

/**
 * Play feedback with voice
 */
const playFeedback = async () => {
  if (!voiceEnabled.value || !currentFeedback.value) return
  
  try {
    // Speak feedback (limit length for better UX)
    const feedback = currentFeedback.value.substring(0, 500)
    await tts.speak(feedback)
  } catch (error) {
    console.error('Failed to play feedback:', error)
  }
}

const selectQuestion = (index: number) => {
  currentQuestionIndex.value = index
}

/**
 * Handle voice transcript from speech recognition
 */
const handleVoiceTranscript = (text: string) => {
  currentAnswer.value = text
}

const submitAnswer = async () => {
  if (!currentAnswer.value.trim()) return
  
  // Add candidate's answer to conversation history
  conversationHistory.value.push({
    role: 'candidate',
    content: currentAnswer.value,
    timestamp: Date.now()
  })
  
  isLoading.value = true
  error.value = ''
  
  try {
    currentFeedback.value = await invoke<string>('analyze_answer', {
      question: questions.value[currentQuestionIndex.value],
      answer: currentAnswer.value,
      jobDescription: jobDescription.value
    })
    
    // Save answer to database if session exists
    if (currentSessionId.value) {
      await saveAnswer(
        currentSessionId.value,
        currentQuestionIndex.value,
        questions.value[currentQuestionIndex.value],
        currentAnswer.value,
        currentFeedback.value
      )
    }
    
    // Track answer in memory
    answersHistory.value.push({
      question: questions.value[currentQuestionIndex.value],
      answer: currentAnswer.value,
      feedback: currentFeedback.value
    })
    
    currentStep.value = 'feedback'
    
    // Auto-play feedback with voice
    if (voiceEnabled.value) {
      await nextTick()
      playFeedback()
    }
  } catch (err) {
    error.value = `分析答案失败: ${err}`
  } finally {
    isLoading.value = false
  }
}

const nextQuestion = () => {
  if (currentQuestionIndex.value < questions.value.length - 1) {
    currentQuestionIndex.value++
    currentAnswer.value = ''
  }
}

const nextQuestionAfterFeedback = async () => {
  // Trigger follow-up analysis if enabled
  if (followUpSettings.value.enabled) {
    await analyzeForFollowUp()
  } else {
    proceedToNextQuestion()
  }
}

const finishInterview = () => {
  // Show completion animation
  showCompletionAnimation.value = true
}

const closeCompletionAnimation = () => {
  showCompletionAnimation.value = false
  
  // Reset state
  currentStep.value = 'input'
  resume.value = ''
  jobDescription.value = ''
  questions.value = []
  currentQuestionIndex.value = 0
  currentAnswer.value = ''
  currentFeedback.value = ''
  currentSessionId.value = null
  answersHistory.value = []
}

/**
 * Timer event handlers
 */
const handleTimerTimeout = () => {
  if (timerSettings.value.autoSubmit && currentAnswer.value.trim()) {
    submitAnswer()
  } else {
    alert('时间到！请尽快提交答案。')
  }
}

const handleTimerWarning = () => {
  if (timerSettings.value.showWarning) {
    console.log('⚠️ 警告：剩余时间不足30秒')
  }
}

const handleTimerSettingsChange = (newSettings: TimerConfig) => {
  timerSettings.value = newSettings
  TimerSettingsManager.saveSettings(newSettings)
  
  // Restart timer if currently running
  if (timerRef.value?.isRunning && currentStep.value === 'interview') {
    timerRef.value.reset()
  }
}

const toggleTimerSettings = () => {
  showTimerSettings.value = !showTimerSettings.value
}

/**
 * Follow-up event handlers
 */
const handleFollowUpSettingsChange = (newSettings: FollowUpSettings) => {
  followUpSettings.value = newSettings
  FollowUpSettingsManager.saveSettings(newSettings)
}

const toggleFollowUpSettings = () => {
  showFollowUpSettings.value = !showFollowUpSettings.value
}

const analyzeForFollowUp = async () => {
  if (!followUpSettings.value.enabled || followUpCount.value >= followUpSettings.value.maxFollowUps) {
    proceedToNextQuestion()
    return
  }

  isLoading.value = true
  error.value = ''

  try {
    const historyText = conversationHistory.value
      .map(turn => `${turn.role === 'interviewer' ? 'Interviewer' : 'Candidate'}: ${turn.content}`)
      .join('\n\n')

    const analysisJson = await invoke<string>('analyze_for_followup', {
      originalQuestion: questions.value[currentQuestionIndex.value],
      answer: currentAnswer.value,
      conversationHistory: historyText,
      jobDescription: jobDescription.value,
      maxFollowups: followUpSettings.value.maxFollowUps - followUpCount.value,
      preferredTypes: followUpSettings.value.preferredTypes
    })

    followUpAnalysis.value = JSON.parse(analysisJson)

    if (followUpSettings.value.autoTrigger && followUpAnalysis.value?.shouldFollowUp) {
      currentStep.value = 'followup'
    } else {
      proceedToNextQuestion()
    }
  } catch (err) {
    console.error('Follow-up analysis failed:', err)
    proceedToNextQuestion()
  } finally {
    isLoading.value = false
  }
}

const selectFollowUpQuestion = (question: string, type: FollowUpType) => {
  // Add interviewer's follow-up to history
  conversationHistory.value.push({
    role: 'interviewer',
    content: question,
    timestamp: Date.now(),
    questionType: type
  })

  // Update current question to follow-up
  questions.value[currentQuestionIndex.value] = question
  currentAnswer.value = ''
  followUpCount.value++
  currentStep.value = 'interview'

  // Restart timer if enabled
  if (timerRef.value) {
    timerRef.value.reset()
  }
}

const skipFollowUp = () => {
  proceedToNextQuestion()
}

const proceedToNextQuestion = () => {
  if (currentQuestionIndex.value < questions.value.length - 1) {
    currentQuestionIndex.value++
    currentAnswer.value = ''
    currentStep.value = 'interview'
    followUpCount.value = 0
    conversationHistory.value = []

    if (voiceEnabled.value) {
      nextTick().then(() => playCurrentQuestion())
    }
  } else {
    finishInterview()
  }
}

const clearConversationHistory = () => {
  conversationHistory.value = []
}
</script>

<style scoped>
.app-container {
  min-height: 100vh;
  background: var(--bg-primary);
  color: var(--text-light);
  font-family:
    -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
}

header {
  padding: 2rem;
  text-align: center;
  border-bottom: 2px solid var(--border-color);
  position: relative;
}

header h1 {
  margin: 0;
  font-size: 2.5rem;
  font-weight: bold;
}

header p {
  margin: 0.5rem 0 0;
  font-size: 1rem;
  opacity: 0.9;
}

.header-actions {
  position: absolute;
  top: 2rem;
  right: 2rem;
}

main {
  padding: 2rem;
}

.welcome {
  max-width: 800px;
  margin: 2rem auto;
  background: var(--bg-card);
  padding: 2rem;
  border-radius: 8px;
  backdrop-filter: var(--backdrop-blur);
}

.welcome h2 {
  margin-top: 0;
  font-size: 1.8rem;
}

.welcome p {
  font-size: 1.1rem;
  line-height: 1.6;
}

.demo-section {
  margin-top: 2rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  align-items: center;
}

.name-input {
  padding: 0.75rem 1rem;
  border: 2px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-input);
  color: var(--text-light);
  font-size: 1rem;
  width: 100%;
  max-width: 300px;
  outline: none;
  transition: border-color 0.3s;
}

.name-input::placeholder {
  color: var(--text-muted);
}

.name-input:focus {
  border-color: var(--border-hover);
}

.greet-btn {
  padding: 0.75rem 2rem;
  background: var(--bg-hover);
  border: 2px solid var(--border-color);
  border-radius: 4px;
  color: var(--text-light);
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s;
}

.greet-btn:hover {
  background: var(--bg-input);
  border-color: var(--border-hover);
}

.greeting-message {
  margin-top: 1rem;
  font-size: 1.2rem;
  font-weight: 500;
  color: #fff;
}

.interview-mode {
  max-width: 900px;
  margin: 0 auto;
  padding: 2rem;
  background: var(--bg-secondary);
  border-radius: 12px;
}

.mode-header {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  margin-bottom: 2rem;
  gap: 1rem;
}

.mode-header h2 {
  margin: 0;
  font-size: 1.8rem;
  color: var(--text-primary);
}

.mode-actions {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
  width: 100%;
}

.mode-btn {
  padding: 0.6rem 1.2rem;
  background: var(--bg-secondary);
  border: 2px solid var(--border-light);
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.3s;
  font-size: 0.95rem;
}

.mode-btn:hover {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.mode-btn.active {
  background: var(--accent-gradient);
  border-color: var(--accent-primary);
  color: var(--text-light);
}

.toggle-btn {
  padding: 0.6rem 1.2rem;
  background: var(--bg-input);
  border: 1px solid var(--border-light);
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.3s;
}

.toggle-btn:hover {
  background: var(--bg-hover);
}

.step-content {
  margin-top: 2rem;
}

.action-buttons {
  display: flex;
  gap: 1rem;
  margin-top: 2rem;
  justify-content: center;
}

.primary-btn {
  padding: 0.8rem 2rem;
  background: var(--accent-gradient);
  color: var(--text-light);
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.primary-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.primary-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.secondary-btn {
  padding: 0.8rem 2rem;
  background: var(--bg-secondary);
  color: var(--accent-primary);
  border: 2px solid var(--accent-primary);
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.secondary-btn:hover {
  background: var(--bg-hover);
}

.error-message {
  color: var(--error-color);
  text-align: center;
  margin-top: 1rem;
  padding: 1rem;
  background: rgba(244, 67, 54, 0.1);
  border-radius: 8px;
}

.interview-progress {
  text-align: center;
  font-size: 0.9rem;
  color: var(--text-secondary);
  margin-bottom: 2rem;
  padding: 0.8rem;
  background: var(--bg-card);
  border-radius: 8px;
}

.current-question {
  padding: 2rem;
  background: var(--accent-gradient);
  color: var(--text-light);
  border-radius: 12px;
  margin-bottom: 2rem;
}

.current-question h3 {
  margin: 0;
  font-size: 1.3rem;
  line-height: 1.6;
}

.answer-input h4 {
  margin: 0 0 1rem;
  font-size: 1.1rem;
  color: var(--text-primary);
}

.answer-textarea {
  width: 100%;
  padding: 1rem;
  border: 2px solid var(--border-light);
  border-radius: 8px;
  font-size: 1rem;
  line-height: 1.6;
  font-family: inherit;
  resize: vertical;
  transition: border-color 0.3s;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.answer-textarea:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.feedback-card {
  padding: 2rem;
  background: var(--bg-card-solid);
  border-radius: 12px;
  border: 2px solid var(--accent-primary);
}

.feedback-card h3 {
  margin: 0 0 1.5rem;
  font-size: 1.4rem;
  color: var(--accent-primary);
}

.feedback-content {
  font-size: 1rem;
  line-height: 1.8;
  color: var(--text-primary);
  white-space: pre-wrap;
}

/* Timer Settings Section */
.timer-settings-section,
.followup-settings-section {
  margin: 2rem 0;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.settings-toggle-btn {
  padding: 0.8rem 1.5rem;
  background: var(--bg-input);
  border: 1px solid var(--border-light);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 0.95rem;
  cursor: pointer;
  transition: all 0.3s;
  max-width: 200px;
}

.settings-toggle-btn:hover {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
  color: white;
  transform: translateY(-2px);
}
</style>
