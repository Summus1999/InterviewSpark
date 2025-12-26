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
            <button @click="currentMode = 'analysis'" :class="{ active: currentMode === 'analysis' }" class="mode-btn">
              分析
            </button>
            <button @click="currentMode = 'activity'" :class="{ active: currentMode === 'activity' }" class="mode-btn">
              活跃度
            </button>
            <button @click="currentMode = 'bank'" :class="{ active: currentMode === 'bank' }" class="mode-btn">
              题库管理
            </button>
            <button @click="currentMode = 'dashboard'" :class="{ active: currentMode === 'dashboard' }" class="mode-btn">
              用户
            </button>
            <button v-if="isDev" @click="currentMode = 'knowledge'" :class="{ active: currentMode === 'knowledge' }" class="mode-btn">
              RAG引擎
            </button>
            <button v-if="isDev" @click="showTest = true" class="toggle-btn">
              测试模式
            </button>
          </div>
        </div>

        <!-- Interview Mode Content -->
        <div v-if="currentMode === 'interview'">
          <InterviewMode />
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

        <!-- Activity Mode -->
        <div v-if="currentMode === 'activity'">
          <ActivityView />
        </div>

        <!-- Knowledge Base Mode -->
        <div v-if="currentMode === 'knowledge'">
          <KnowledgeBaseView />
        </div>
      </section>
    </main>
    
    <!-- Onboarding Guide -->
    <OnboardingGuide 
      :show="showOnboarding"
      @complete="handleOnboardingComplete"
      @skip="handleOnboardingSkip"
    />
    
    <!-- Version Display -->
    <div class="version-display">{{ APP_VERSION }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import InterviewMode from './components/InterviewMode.vue'
import InterviewHistory from './components/InterviewHistory.vue'
import QuestionBank from './components/QuestionBank.vue'
import Dashboard from './components/Dashboard.vue'
import SettingsPanel from './components/SettingsPanel.vue'
import ProfileView from './components/ProfileView.vue'
import IndustryComparison from './components/IndustryComparison.vue'
import RecommendationList from './components/RecommendationList.vue'
import BestPracticesList from './components/BestPracticesList.vue'
import OnboardingGuide from './components/OnboardingGuide.vue'
import ActivityView from './components/ActivityView.vue'
import KnowledgeBaseView from './components/KnowledgeBaseView.vue'
import { initKnowledgeBaseBackground } from './services/database'
import { useSettingsStore } from './stores/settings'
import { APP_VERSION } from './version'

const settingsStore = useSettingsStore()

// Development mode detection
const isDev = import.meta.env.DEV

// Phase 1 test variables
const userName = ref('')
const greeting = ref('')
const showTest = ref(false)

// Mode management
const currentMode = ref<'interview' | 'history' | 'bank' | 'dashboard' | 'analysis' | 'activity' | 'knowledge'>('interview')
const analysisView = ref<'profile' | 'recommendation' | 'best-practices' | 'industry'>('profile')

// Onboarding state
const showOnboarding = ref(!settingsStore.onboardingCompleted)

// Background knowledge base initialization on app startup
onMounted(async () => {
  try {
    const result = await initKnowledgeBaseBackground()
    console.log('Knowledge base init:', result)
  } catch (err) {
    // Silent fail - knowledge base is optional enhancement
    console.warn('Knowledge base init skipped:', err)
  }
})

const modeTitle = computed(() => {
  switch (currentMode.value) {
    case 'interview': return '模拟面试'
    case 'history': return '历史记录'
    case 'bank': return '题库管理'
    case 'dashboard': return '用户'
    case 'analysis': return '分析'
    case 'activity': return '活跃度'
    case 'knowledge': return 'RAG 知识引擎'
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
    greeting.value = await invoke<string>('greet', { name: userName.value })
  } catch (error) {
    greeting.value = `Error: ${error}`
  }
}

/**
 * Handle onboarding completion
 */
const handleOnboardingComplete = () => {
  settingsStore.completeOnboarding()
  showOnboarding.value = false
}

const handleOnboardingSkip = () => {
  settingsStore.completeOnboarding()
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

.version-display {
  position: fixed;
  bottom: 1rem;
  left: 1rem;
  font-size: 1.1rem;
  color: silver;
  opacity: 0.8;
  font-family: monospace;
  z-index: 100;
  user-select: none;
}

.loading-indicator {
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: #fff;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
