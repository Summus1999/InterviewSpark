/**
 * Interview Flow Composable
 * Manages the complete interview flow state and logic
 */

import { ref, computed, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { createSession, saveAnswer, analyzeAnswerWithScoring, markBestAnswerNeedsUpdate } from '../services/database'
import { tts, stt } from '../services/voice'
import { useSettingsStore } from '../stores/settings'
import type { ConversationTurn, FollowUpAnalysis } from '../types/follow-up'

export type InterviewStep = 'input' | 'questions' | 'interview' | 'feedback' | 'followup'

export interface AnswerRecord {
  question: string
  answer: string
  feedback: string
}

export function useInterviewFlow() {
  const settingsStore = useSettingsStore()

  // Interview step state
  const currentStep = ref<InterviewStep>('input')
  
  // Input state
  const resume = ref('')
  const jobDescription = ref('')
  
  // Questions state
  const questions = ref<string[]>([])
  const currentQuestionIndex = ref(0)
  
  // Answer state
  const currentAnswer = ref('')
  const currentFeedback = ref('')
  
  // Loading and error state
  const isLoading = ref(false)
  const error = ref('')
  
  // Session tracking
  const currentSessionId = ref<number | null>(null)
  const answersHistory = ref<AnswerRecord[]>([])
  
  // Voice state
  const voiceEnabled = ref(true)
  
  // Follow-up state
  const conversationHistory = ref<ConversationTurn[]>([])
  const followUpAnalysis = ref<FollowUpAnalysis | null>(null)
  const followUpCount = ref(0)
  
  // Report state
  const showFinalReport = ref(false)
  const reportLoading = ref(false)

  // Computed values
  const canGenerate = computed(() => {
    return resume.value.trim().length > 50 && jobDescription.value.trim().length > 20
  })

  const currentQuestion = computed(() => {
    return questions.value[currentQuestionIndex.value] || ''
  })

  const isLastQuestion = computed(() => {
    return currentQuestionIndex.value >= questions.value.length - 1
  })

  const progress = computed(() => ({
    current: currentQuestionIndex.value + 1,
    total: questions.value.length
  }))

  // Generate interview questions
  async function generateQuestions() {
    if (!canGenerate.value) return

    isLoading.value = true
    error.value = ''

    try {
      const aiQuestions = await invoke<string[]>('generate_questions', {
        resume: resume.value,
        jobDescription: jobDescription.value,
        count: 5,
        persona: settingsStore.persona
      })

      // Add fixed opening and closing questions
      questions.value = [
        '请你做一下自我介绍',
        ...aiQuestions,
        '那你还有什么想问我的吗'
      ]

      currentStep.value = 'questions'
      currentQuestionIndex.value = 0
    } catch (err) {
      error.value = `生成问题失败: ${err}`
    } finally {
      isLoading.value = false
    }
  }

  // Start interview session
  async function startInterview() {
    try {
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

  // Play current question with voice
  async function playCurrentQuestion() {
    if (!voiceEnabled.value || !currentQuestion.value) return

    try {
      await tts.speak(currentQuestion.value)
    } catch (e) {
      console.error('Failed to play question:', e)
    }
  }

  // Handle voice transcript
  function handleVoiceTranscript(text: string) {
    if (currentAnswer.value.trim()) {
      currentAnswer.value = currentAnswer.value.trim() + ' ' + text
    } else {
      currentAnswer.value = text
    }
  }

  // Submit current answer
  async function submitAnswer(timerRef?: { reset: () => void } | null) {
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
      // Save answer to database if session exists
      if (currentSessionId.value) {
        const answerId = await saveAnswer(
          currentSessionId.value,
          currentQuestionIndex.value,
          currentQuestion.value,
          currentAnswer.value,
          ''
        )

        // Analyze and score answer for profile dimension calculation
        try {
          await analyzeAnswerWithScoring(
            answerId,
            currentAnswer.value,
            currentQuestion.value,
            jobDescription.value
          )
        } catch (analysisErr) {
          console.error('Failed to analyze answer:', analysisErr)
        }

        // Mark best answer as needing update
        try {
          await markBestAnswerNeedsUpdate(currentQuestion.value)
        } catch (markErr) {
          console.error('Failed to mark best answer for update:', markErr)
        }
      }

      // Track answer in memory
      answersHistory.value.push({
        question: currentQuestion.value,
        answer: currentAnswer.value,
        feedback: ''
      })

      // Proceed to next question or finish interview
      if (!isLastQuestion.value) {
        stopVoice()
        currentQuestionIndex.value++
        currentAnswer.value = ''
        followUpCount.value = 0
        conversationHistory.value = []

        // Restart timer if provided
        timerRef?.reset()

        // Auto-play next question
        if (voiceEnabled.value) {
          await nextTick()
          playCurrentQuestion()
        }
      } else {
        finishInterview()
      }
    } catch (err) {
      error.value = `保存答案失败: ${err}`
    } finally {
      isLoading.value = false
    }
  }

  // Skip to next question
  function skipQuestion() {
    stopVoice()
    if (!isLastQuestion.value) {
      currentQuestionIndex.value++
      currentAnswer.value = ''
    }
  }

  // Select specific question
  function selectQuestion(index: number) {
    stopVoice()
    currentQuestionIndex.value = index
  }

  // Stop voice playback and recording
  function stopVoice() {
    tts.stop()
    if (stt) {
      stt.stop()
    }
  }

  // Finish interview and show report
  function finishInterview() {
    if (!currentSessionId.value) {
      error.value = '未找到面试会话'
      return
    }
    showFinalReport.value = true
  }

  // Reset all state
  function reset() {
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
    showFinalReport.value = false
    reportLoading.value = false
    error.value = ''
  }

  // Close report and reset
  function closeReport() {
    reset()
  }

  // Go back to input step
  function backToInput() {
    currentStep.value = 'input'
  }

  return {
    // State
    currentStep,
    resume,
    jobDescription,
    questions,
    currentQuestionIndex,
    currentAnswer,
    currentFeedback,
    isLoading,
    error,
    currentSessionId,
    answersHistory,
    voiceEnabled,
    conversationHistory,
    followUpAnalysis,
    followUpCount,
    showFinalReport,
    reportLoading,

    // Computed
    canGenerate,
    currentQuestion,
    isLastQuestion,
    progress,

    // Actions
    generateQuestions,
    startInterview,
    playCurrentQuestion,
    handleVoiceTranscript,
    submitAnswer,
    skipQuestion,
    selectQuestion,
    stopVoice,
    finishInterview,
    reset,
    closeReport,
    backToInput
  }
}
