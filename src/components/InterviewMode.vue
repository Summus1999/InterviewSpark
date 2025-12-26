<template>
  <div class="interview-mode-container">
    <!-- Step 1: Input Resume and JD -->
    <div v-if="interview.currentStep.value === 'input'" class="step-content">
      <ResumeInput v-model="interview.resume.value" />
      <JobDescription v-model="interview.jobDescription.value" />

      <!-- Timer Settings -->
      <div class="timer-settings-section">
        <TooltipBubble
          content="开启计时模式，模拟真实面试时间压力"
          tooltip-id="timer-settings-tip"
          position="bottom"
        >
          <button @click="showTimerSettings = !showTimerSettings" class="settings-toggle-btn">
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
          <button @click="showFollowUpSettings = !showFollowUpSettings" class="settings-toggle-btn">
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
            @click="interview.generateQuestions"
            :disabled="!interview.canGenerate.value || interview.isLoading.value"
            class="primary-btn"
          >
            <span v-if="interview.isLoading.value" class="loading-indicator">
              <span class="spinner"></span>
              问题生成中...
            </span>
            <span v-else>生成面试问题</span>
          </button>
        </TooltipBubble>
      </div>

      <p v-if="interview.error.value" class="error-message">{{ interview.error.value }}</p>
    </div>

    <!-- Step 2: Show Questions -->
    <div v-if="interview.currentStep.value === 'questions'" class="step-content">
      <QuestionList
        :questions="interview.questions.value"
        :current-index="interview.currentQuestionIndex.value"
        @select-question="interview.selectQuestion"
      />

      <div class="action-buttons">
        <button @click="interview.startInterview" class="primary-btn">
          开始面试
        </button>
        <button @click="interview.backToInput" class="secondary-btn">
          重新生成问题
        </button>
      </div>
    </div>

    <!-- Step 3: Interview -->
    <div v-if="interview.currentStep.value === 'interview'" class="step-content">
      <ProgressBar :current="interview.progress.value.current" :total="interview.progress.value.total" />

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
        <span>问题 {{ interview.progress.value.current }} / {{ interview.progress.value.total }}</span>
      </div>

      <div class="current-question">
        <h3>{{ interview.currentQuestion.value }}</h3>
      </div>

      <!-- Voice Controls -->
      <VoiceControls
        :current-question="interview.currentQuestion.value"
        :disabled="interview.isLoading.value"
        @transcript="interview.handleVoiceTranscript"
      />

      <div class="answer-input">
        <h4>您的回答：</h4>
        <textarea
          v-model="interview.currentAnswer.value"
          placeholder="请输入您的回答或使用语音回答..."
          rows="8"
          class="answer-textarea"
        />
      </div>

      <div class="action-buttons">
        <button
          @click="handleSubmitAnswer"
          :disabled="!interview.currentAnswer.value.trim() || interview.isLoading.value"
          class="primary-btn"
        >
          {{ interview.isLoading.value ? '分析中...' : '提交答案' }}
        </button>
        <button
          v-if="!interview.isLastQuestion.value"
          @click="interview.skipQuestion"
          class="secondary-btn"
        >
          跳过此题
        </button>
      </div>

      <p v-if="interview.error.value" class="error-message">{{ interview.error.value }}</p>
    </div>

    <!-- Step 4: Follow-up Panel (if enabled) -->
    <div v-if="interview.currentStep.value === 'followup'" class="step-content">
      <FollowUpPanel
        v-if="interview.followUpAnalysis.value"
        :analysis="interview.followUpAnalysis.value"
        @select="selectFollowUpQuestion"
        @skip="skipFollowUp"
        @custom="interview.currentStep.value = 'interview'"
      />
    </div>

    <!-- Final Report Modal -->
    <div v-if="interview.showFinalReport.value" class="report-overlay" @click.self="interview.closeReport">
      <div class="report-modal">
        <button class="report-close-btn" @click="interview.closeReport" aria-label="Close report">
          ✕
        </button>
        <ReportView
          v-if="interview.currentSessionId.value"
          :session-id="interview.currentSessionId.value"
          @close="interview.closeReport"
        />
      </div>
    </div>

    <!-- Completion Animation -->
    <CompletionAnimation
      :show="showCompletionAnimation"
      :message="`完成了 ${interview.answersHistory.value.length} 个问题的回答，继续加油！`"
      @close="interview.closeReport"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import ResumeInput from './ResumeInput.vue'
import JobDescription from './JobDescription.vue'
import QuestionList from './QuestionList.vue'
import VoiceControls from './VoiceControls.vue'
import ProgressBar from './ProgressBar.vue'
import CompletionAnimation from './CompletionAnimation.vue'
import TimerDisplay from './TimerDisplay.vue'
import TimerSettings from './TimerSettings.vue'
import FollowUpSettingsComp from './FollowUpSettings.vue'
import FollowUpPanel from './FollowUpPanel.vue'
import ReportView from './ReportView.vue'
import TooltipBubble from './TooltipBubble.vue'
import { useInterviewFlow } from '../composables/useInterviewFlow'
import { useSettingsStore } from '../stores/settings'
import type { TimerConfig } from '../stores/settings'
import type { FollowUpSettings } from '../types/follow-up'

const settingsStore = useSettingsStore()
const interview = useInterviewFlow()

// Timer state
const timerSettings = ref<TimerConfig>({ ...settingsStore.timerSettings })
const timerRef = ref<InstanceType<typeof TimerDisplay> | null>(null)
const showTimerSettings = ref(false)

// Follow-up state
const followUpSettings = ref<FollowUpSettings>({ ...settingsStore.followUpSettings })
const showFollowUpSettings = ref(false)

// Animation state
const showCompletionAnimation = ref(false)

// Timer handlers
function handleTimerSettingsChange(newSettings: TimerConfig) {
  timerSettings.value = newSettings
  settingsStore.updateTimerSettings(newSettings)

  // Restart timer if currently running
  if (timerRef.value && interview.currentStep.value === 'interview') {
    timerRef.value.reset()
  }
}

function handleTimerTimeout() {
  if (timerSettings.value.autoSubmit && interview.currentAnswer.value.trim()) {
    handleSubmitAnswer()
  } else {
    alert('时间到！请尽快提交答案。')
  }
}

function handleTimerWarning() {
  if (timerSettings.value.showWarning) {
    console.log('⚠️ 警告：剩余时间不足30秒')
  }
}

// Follow-up handlers
function handleFollowUpSettingsChange(newSettings: FollowUpSettings) {
  followUpSettings.value = newSettings
  settingsStore.updateFollowUpSettings(newSettings)
}

function selectFollowUpQuestion(question: string) {
  // Handle follow-up question selection
  console.log('Selected follow-up:', question)
}

function skipFollowUp() {
  // Skip follow-up and proceed
  interview.currentStep.value = 'interview'
}

// Submit answer with timer ref
function handleSubmitAnswer() {
  interview.submitAnswer(timerRef.value)
}
</script>

<style scoped>
.interview-mode-container {
  width: 100%;
}

.step-content {
  padding: 1rem 0;
}

.timer-settings-section,
.followup-settings-section {
  margin: 1rem 0;
}

.settings-toggle-btn {
  padding: 0.5rem 1rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-light);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s;
}

.settings-toggle-btn:hover {
  background: var(--bg-hover);
  border-color: var(--accent-primary);
}

.action-buttons {
  display: flex;
  gap: 1rem;
  justify-content: center;
  margin-top: 1.5rem;
}

.primary-btn {
  padding: 0.8rem 2rem;
  background: var(--accent-gradient);
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
  opacity: 0.6;
  cursor: not-allowed;
}

.secondary-btn {
  padding: 0.8rem 2rem;
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-light);
  border-radius: 8px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s;
}

.secondary-btn:hover {
  background: var(--bg-hover);
  border-color: var(--accent-primary);
}

.loading-indicator {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid transparent;
  border-top-color: currentColor;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-message {
  color: var(--error-color, #ef4444);
  margin-top: 1rem;
  text-align: center;
}

.interview-progress {
  text-align: center;
  margin: 1rem 0;
  color: var(--text-secondary);
}

.current-question {
  background: var(--bg-card);
  padding: 1.5rem;
  border-radius: 12px;
  margin: 1rem 0;
}

.current-question h3 {
  margin: 0;
  color: var(--text-primary);
  font-size: 1.2rem;
  line-height: 1.6;
}

.answer-input {
  margin: 1.5rem 0;
}

.answer-input h4 {
  margin-bottom: 0.5rem;
  color: var(--text-primary);
}

.answer-textarea {
  width: 100%;
  padding: 1rem;
  background: var(--bg-input);
  border: 1px solid var(--border-light);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 1rem;
  resize: vertical;
  min-height: 150px;
}

.answer-textarea:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.report-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.report-modal {
  position: relative;
  background: var(--bg-card-solid);
  border-radius: 16px;
  padding: 2rem;
  max-width: 90vw;
  max-height: 90vh;
  overflow-y: auto;
}

.report-close-btn {
  position: absolute;
  top: 1rem;
  right: 1rem;
  width: 32px;
  height: 32px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-light);
  border-radius: 50%;
  cursor: pointer;
  font-size: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s;
}

.report-close-btn:hover {
  background: var(--bg-hover);
  border-color: var(--accent-primary);
}
</style>
