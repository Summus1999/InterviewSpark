<template>
  <div id="app" class="app-container">
    <header>
      <h1>InterviewSpark</h1>
      <p>AI-Powered Mock Interview Platform</p>
      <div class="header-actions">
        <SettingsPanel />
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
            <button @click="currentMode = 'analysis'" :class="{ active: currentMode === 'analysis' }" class="mode-btn">
              分析
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
              <TooltipBubble
                content="开启计时模式，模拟真实面试时间压力"
                tooltip-id="timer-settings-tip"
                position="bottom"
              >
                <button @click="toggleTimerSettings" class="settings-toggle-btn">
                  ⏱️ {{ showTimerSettings ? '隐藏' : '显示' }}计时设置
                </button>
              </TooltipBubble>
              <TimerSettings 
                v-if="showTimerSettings"
                v-model="timerSettings"
                @update:modelValue="handleTimerSettingsChange"
              />
            </div>
            
            <!-- Follow-up Settings -->
            <div class="followup-settings-section">
              <TooltipBubble
                content="开启追问模式，AI 会根据你的回答追问细节"
                tooltip-id="followup-settings-tip"
                position="bottom"
              >
                <button @click="toggleFollowUpSettings" class="settings-toggle-btn">
                  🔄 {{ showFollowUpSettings ? '隐藏' : '显示' }}追问设置
                </button>
              </TooltipBubble>
              <FollowUpSettingsComp 
                v-if="showFollowUpSettings"
                v-model="followUpSettings"
                @update:modelValue="handleFollowUpSettingsChange"
              />
            </div>
            
            <div class="action-buttons">
              <TooltipBubble
                content="基于简历和 JD 生成个性化面试问题"
                tooltip-id="generate-questions-tip"
                position="top"
              >
                <button 
                  @click="generateQuestions" 
                  :disabled="!canGenerate || isLoading"
                  class="primary-btn"
                >
                  {{ isLoading ? '生成中...' : '生成面试问题' }}
                </button>
              </TooltipBubble>
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

          <!-- Step 4: Follow-up Panel (if enabled) -->
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

        <!-- Analysis Mode -->
        <div v-if="currentMode === 'analysis'">
          <div class="analysis-container">
            <div class="analysis-tabs">
              <button 
                @click="analysisView = 'profile'" 
                :class="{ active: analysisView === 'profile' }" 
                class="analysis-tab-btn"
              >
                个人画像
              </button>
              <button 
                @click="analysisView = 'recommendation'" 
                :class="{ active: analysisView === 'recommendation' }" 
                class="analysis-tab-btn"
              >
                智能推荐
              </button>
              <button 
                @click="analysisView = 'best-practices'" 
                :class="{ active: analysisView === 'best-practices' }" 
                class="analysis-tab-btn"
              >
                最佳实践
              </button>
              <button 
                @click="analysisView = 'industry'" 
                :class="{ active: analysisView === 'industry' }" 
                class="analysis-tab-btn"
              >
                行业对比
              </button>
            </div>
            <div class="analysis-content">
              <ProfileView v-if="analysisView === 'profile'" />
              <RecommendationList v-if="analysisView === 'recommendation'" />
              <BestPracticesList v-if="analysisView === 'best-practices'" />
              <IndustryComparison v-if="analysisView === 'industry'" />
            </div>
          </div>
        </div>
      </section>
    </main>
    
    <!-- Final Report Modal -->
    <div v-if="showFinalReport" class="report-overlay" @click.self="closeCompletionAnimation">
      <div class="report-modal">
        <button class="report-close-btn" @click="closeCompletionAnimation" aria-label="Close report">
          ✕
        </button>
        <ReportView v-if="currentSessionId" :session-id="currentSessionId" />
      </div>
    </div>
    
    <!-- Completion Animation -->
    <CompletionAnimation 
      :show="showCompletionAnimation"
      :message="`完成了 ${answersHistory.length} 个问题的回答，继续加油！`"
      @close="closeCompletionAnimation"
    />
    
    <!-- Onboarding Guide -->
    <OnboardingGuide 
      :show="showOnboarding"
      @complete="handleOnboardingComplete"
      @skip="handleOnboardingSkip"
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
import SettingsPanel from './components/SettingsPanel.vue'
import ProgressBar from './components/ProgressBar.vue'
import CompletionAnimation from './components/CompletionAnimation.vue'
import TimerDisplay from './components/TimerDisplay.vue'
import TimerSettings from './components/TimerSettings.vue'
import FollowUpSettingsComp from './components/FollowUpSettings.vue'
import FollowUpPanel from './components/FollowUpPanel.vue'
import ConversationHistory from './components/ConversationHistory.vue'
import ReportView from './components/ReportView.vue'
import ProfileView from './components/ProfileView.vue'
import IndustryComparison from './components/IndustryComparison.vue'
import RecommendationList from './components/RecommendationList.vue'
import BestPracticesList from './components/BestPracticesList.vue'
import OnboardingGuide from './components/OnboardingGuide.vue'
import TooltipBubble from './components/TooltipBubble.vue'
import STARScoreDisplay from './components/STARScoreDisplay.vue'
import { createSession, saveAnswer, analyzeSTARScore, type STARScoringResult } from './services/database'
import { tts } from './services/voice'
import { TimerSettingsManager, type TimerConfig, FollowUpSettingsManager, OnboardingManager, InterviewerPersonaManager } from './services/settings'
import type { ConversationTurn, FollowUpAnalysis, FollowUpSettings, FollowUpType } from './types/follow-up'

// Development mode detection
const isDev = import.meta.env.DEV

// Phase 1 test variables
const userName = ref('')
const greeting = ref('')
const showTest = ref(false)

// Mode management
const currentMode = ref<'interview' | 'history' | 'bank' | 'dashboard' | 'analysis'>('interview')
const analysisView = ref<'profile' | 'recommendation' | 'best-practices' | 'industry'>('profile')

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

// Onboarding state
const showOnboarding = ref(!OnboardingManager.isCompleted())

// Final report state
const showFinalReport = ref(false)
const reportLoading = ref(false)

// STAR scoring state (deprecated - will be in final report)
const starScore = ref<STARScoringResult | null>(null)
const showSTARScore = ref(false)

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
    case 'analysis': return '分析'
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
    const currentPersona = InterviewerPersonaManager.getPersona()
    questions.value = await invoke<string[]>('generate_questions', {
      resume: resume.value,
      jobDescription: jobDescription.value,
      count: 5,
      persona: currentPersona
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
    // Save answer to database if session exists (no immediate analysis)
    if (currentSessionId.value) {
      await saveAnswer(
        currentSessionId.value,
        currentQuestionIndex.value,
        questions.value[currentQuestionIndex.value],
        currentAnswer.value,
        '' // No feedback yet - will be in final report
      )
    }
    
    // Track answer in memory
    answersHistory.value.push({
      question: questions.value[currentQuestionIndex.value],
      answer: currentAnswer.value,
      feedback: '' // Will be filled in final report
    })
    
    // Proceed to next question or finish interview
    if (currentQuestionIndex.value < questions.value.length - 1) {
      // Go to next question
      currentQuestionIndex.value++
      currentAnswer.value = ''
      followUpCount.value = 0
      conversationHistory.value = []
      
      // Restart timer if enabled
      if (timerRef.value) {
        timerRef.value.reset()
      }
      
      // Auto-play next question
      if (voiceEnabled.value) {
        await nextTick()
        playCurrentQuestion()
      }
    } else {
      // All questions answered - finish interview
      finishInterview()
    }
  } catch (err) {
    error.value = `保存答案失败: ${err}`
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

const finishInterview = async () => {
  // Generate comprehensive report
  await generateFinalReport()
}

const generateFinalReport = async () => {
  if (!currentSessionId.value) {
    error.value = '未找到面试会话'
    return
  }
  
  reportLoading.value = true
  error.value = ''
  
  try {
    // Show completion animation first
    showCompletionAnimation.value = true
    
    // Wait for animation
    await new Promise(resolve => setTimeout(resolve, 2000))
    
    // Hide animation and show report loading
    showCompletionAnimation.value = false
    showFinalReport.value = true
    
    // Generate report will be handled by ReportView component
  } catch (err) {
    error.value = `生成报告失败: ${err}`
    showFinalReport.value = false
  } finally {
    reportLoading.value = false
  }
}

const closeCompletionAnimation = () => {
  showCompletionAnimation.value = false
  showFinalReport.value = false
  reportLoading.value = false
  
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
  conversationHistory.value = []
  followUpCount.value = 0
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

    const currentPersona = InterviewerPersonaManager.getPersona()
    const analysisJson = await invoke<string>('analyze_for_followup', {
      originalQuestion: questions.value[currentQuestionIndex.value],
      answer: currentAnswer.value,
      conversationHistory: historyText,
      jobDescription: jobDescription.value,
      maxFollowups: followUpSettings.value.maxFollowUps - followUpCount.value,
      preferredTypes: followUpSettings.value.preferredTypes,
      persona: currentPersona
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

/**
 * Handle onboarding completion
 */
const handleOnboardingComplete = () => {
  OnboardingManager.markCompleted()
  showOnboarding.value = false
}

/**
 * Handle onboarding skip
 */
const handleOnboardingSkip = () => {
  OnboardingManager.markCompleted()
  showOnboarding.value = false
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

/* Analysis Mode Styles */
.analysis-container {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.analysis-tabs {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
  padding: 0.5rem;
  background: var(--bg-card);
  border-radius: 12px;
}

.analysis-tab-btn {
  padding: 0.8rem 1.5rem;
  background: var(--bg-secondary);
  border: 2px solid var(--border-light);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 0.95rem;
  cursor: pointer;
  transition: all 0.3s;
  white-space: nowrap;
}

.analysis-tab-btn:hover {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.analysis-tab-btn.active {
  background: var(--accent-gradient);
  border-color: var(--accent-primary);
  color: var(--text-light);
  font-weight: 600;
}

.analysis-content {
  padding: 2rem;
  background: var(--bg-card);
  border-radius: 12px;
  min-height: 500px;
}

/* Final Report Modal */
.report-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.85);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
  animation: fadeIn 0.3s ease;
}

.report-modal {
  background: var(--bg-card, #ffffff);
  border-radius: 20px;
  width: 90%;
  max-width: 1000px;
  max-height: 90vh;
  overflow-y: auto;
  position: relative;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  animation: slideUp 0.4s ease;
}

.report-close-btn {
  position: absolute;
  top: 1.5rem;
  right: 1.5rem;
  background: rgba(0, 0, 0, 0.1);
  border: none;
  font-size: 1.5rem;
  color: var(--text-secondary, #999);
  cursor: pointer;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.2s;
  z-index: 1;
}

.report-close-btn:hover {
  background: rgba(0, 0, 0, 0.2);
  color: var(--text-primary, #333);
  transform: rotate(90deg);
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
