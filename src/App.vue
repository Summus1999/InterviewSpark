<template>
  <div id="app" class="app-container">
    <header>
      <h1>InterviewSpark</h1>
      <p>AI-Powered Mock Interview Platform</p>
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
            <button @click="showTest = true" class="toggle-btn">
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
        </div>

        <!-- History Mode -->
        <div v-if="currentMode === 'history'">
          <InterviewHistory />
        </div>

        <!-- Question Bank Mode -->
        <div v-if="currentMode === 'bank'">
          <QuestionBank />
        </div>
      </section>
    </main>
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
import VoiceControls from './components/VoiceControls.vue'
import { createSession, saveAnswer } from './services/database'
import { tts } from './services/voice'

// Phase 1 test variables
const userName = ref('')
const greeting = ref('')
const showTest = ref(false)

// Mode management
const currentMode = ref<'interview' | 'history' | 'bank'>('interview')

// Phase 2 interview variables
const currentStep = ref<'input' | 'questions' | 'interview' | 'feedback'>('input')
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
  currentQuestionIndex.value++
  currentAnswer.value = ''
  currentFeedback.value = ''
  currentStep.value = 'interview'
  
  // Auto-play next question
  if (voiceEnabled.value) {
    await nextTick()
    playCurrentQuestion()
  }
}

const finishInterview = () => {
  // Show completion message
  if (answersHistory.value.length > 0) {
    alert(`面试完成！已保存 ${answersHistory.value.length} 个回答记录。`)
  }
  
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
</script>

<style scoped>
.app-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  font-family:
    -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
}

header {
  padding: 2rem;
  text-align: center;
  border-bottom: 2px solid rgba(255, 255, 255, 0.1);
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

main {
  padding: 2rem;
}

.welcome {
  max-width: 800px;
  margin: 2rem auto;
  background: rgba(255, 255, 255, 0.1);
  padding: 2rem;
  border-radius: 8px;
  backdrop-filter: blur(10px);
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
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 1rem;
  width: 100%;
  max-width: 300px;
  outline: none;
  transition: border-color 0.3s;
}

.name-input::placeholder {
  color: rgba(255, 255, 255, 0.6);
}

.name-input:focus {
  border-color: rgba(255, 255, 255, 0.6);
}

.greet-btn {
  padding: 0.75rem 2rem;
  background: rgba(255, 255, 255, 0.2);
  border: 2px solid rgba(255, 255, 255, 0.4);
  border-radius: 4px;
  color: white;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s;
}

.greet-btn:hover {
  background: rgba(255, 255, 255, 0.3);
  border-color: rgba(255, 255, 255, 0.6);
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
  background: white;
  border-radius: 12px;
}

.mode-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  flex-wrap: wrap;
  gap: 1rem;
}

.mode-header h2 {
  margin: 0;
  font-size: 1.8rem;
  color: #333;
}

.mode-actions {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.mode-btn {
  padding: 0.6rem 1.2rem;
  background: white;
  border: 2px solid #e0e0e0;
  border-radius: 6px;
  color: #666;
  cursor: pointer;
  transition: all 0.3s;
  font-size: 0.95rem;
}

.mode-btn:hover {
  border-color: #667eea;
  color: #667eea;
}

.mode-btn.active {
  background: #667eea;
  border-color: #667eea;
  color: white;
}

.toggle-btn {
  padding: 0.6rem 1.2rem;
  background: #f5f5f5;
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  color: #666;
  cursor: pointer;
  transition: all 0.3s;
}

.toggle-btn:hover {
  background: #e0e0e0;
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
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
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
  background: white;
  color: #667eea;
  border: 2px solid #667eea;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.secondary-btn:hover {
  background: #f8f9ff;
}

.error-message {
  color: #e53e3e;
  text-align: center;
  margin-top: 1rem;
  padding: 1rem;
  background: #fff5f5;
  border-radius: 8px;
}

.interview-progress {
  text-align: center;
  font-size: 0.9rem;
  color: #666;
  margin-bottom: 2rem;
  padding: 0.8rem;
  background: #f8f9ff;
  border-radius: 8px;
}

.current-question {
  padding: 2rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
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
  color: #333;
}

.answer-textarea {
  width: 100%;
  padding: 1rem;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  font-size: 1rem;
  line-height: 1.6;
  font-family: inherit;
  resize: vertical;
  transition: border-color 0.3s;
}

.answer-textarea:focus {
  outline: none;
  border-color: #667eea;
}

.feedback-card {
  padding: 2rem;
  background: #f8f9ff;
  border-radius: 12px;
  border: 2px solid #667eea;
}

.feedback-card h3 {
  margin: 0 0 1.5rem;
  font-size: 1.4rem;
  color: #667eea;
}

.feedback-content {
  font-size: 1rem;
  line-height: 1.8;
  color: #333;
  white-space: pre-wrap;
}
</style>