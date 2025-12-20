<template>
  <div v-if="show" class="onboarding-overlay" @click.self="handleSkip">
    <div class="onboarding-container">
      <!-- Close Button -->
      <button class="close-btn" @click="handleSkip" aria-label="Skip tutorial">
        ✕
      </button>

      <!-- Step Content -->
      <div class="step-content">
        <div class="step-icon">{{ currentStepData.icon }}</div>
        <h2>{{ currentStepData.title }}</h2>
        <p>{{ currentStepData.description }}</p>
      </div>

      <!-- Step Indicator -->
      <div class="step-indicator">
        <span
          v-for="(step, index) in steps"
          :key="index"
          :class="['indicator-dot', { active: index === currentStep }]"
        />
      </div>

      <!-- Navigation Buttons -->
      <div class="step-actions">
        <button
          v-if="currentStep > 0"
          @click="prevStep"
          class="nav-btn secondary"
        >
          上一步
        </button>
        <button
          v-if="currentStep < steps.length - 1"
          @click="nextStep"
          class="nav-btn primary"
        >
          下一步
        </button>
        <button
          v-else
          @click="complete"
          class="nav-btn primary complete"
        >
          开始使用
        </button>
      </div>

      <!-- Skip Button -->
      <button class="skip-btn" @click="handleSkip">
        跳过引导
      </button>
    </div>

    <!-- Completion Animation -->
    <transition name="celebration">
      <div v-if="showCelebration" class="celebration-overlay">
        <div class="celebration-content">
          <div class="celebration-icon">🎉</div>
          <h3>准备就绪！</h3>
          <p>开始你的第一次模拟面试吧</p>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface OnboardingStep {
  icon: string
  title: string
  description: string
}

const props = defineProps<{
  show: boolean
}>()

const emit = defineEmits<{
  complete: []
  skip: []
}>()

const currentStep = ref(0)
const showCelebration = ref(false)

const steps: OnboardingStep[] = [
  {
    icon: '👋',
    title: '欢迎使用 InterviewSpark',
    description: '这是一款 AI 驱动的模拟面试应用，帮助你通过真实的面试场景提升技能和自信心。'
  },
  {
    icon: '📝',
    title: '输入简历信息',
    description: '粘贴你的简历内容，建议包含项目经验、技术栈和工作经历。也可以点击"选择模板"快速填充示例简历。'
  },
  {
    icon: '💼',
    title: '输入岗位描述',
    description: '粘贴目标岗位的 JD（Job Description），AI 会根据岗位要求生成针对性的面试问题。'
  },
  {
    icon: '⏱️',
    title: '可选设置',
    description: '你可以开启计时模式模拟真实面试时间压力，或开启 AI 追问模式进行深度对话。这些功能可以在设置中调整。'
  },
  {
    icon: '🚀',
    title: '开始面试',
    description: '点击"生成面试问题"，AI 会为你准备 5 个面试问题。完成所有问题后，面试官将为你生成一份全面的综合分析报告。'
  }
]

const currentStepData = computed(() => steps[currentStep.value])

const nextStep = () => {
  if (currentStep.value < steps.length - 1) {
    currentStep.value++
  }
}

const prevStep = () => {
  if (currentStep.value > 0) {
    currentStep.value--
  }
}

const complete = () => {
  showCelebration.value = true
  setTimeout(() => {
    showCelebration.value = false
    emit('complete')
  }, 1500)
}

const handleSkip = () => {
  emit('skip')
}
</script>

<style scoped>
.onboarding-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.onboarding-container {
  background: var(--bg-card, #ffffff);
  border-radius: 20px;
  padding: 3rem;
  max-width: 600px;
  width: 90%;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  position: relative;
  animation: slideUp 0.4s ease;
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

.close-btn {
  position: absolute;
  top: 1.5rem;
  right: 1.5rem;
  background: transparent;
  border: none;
  font-size: 1.5rem;
  color: var(--text-secondary, #999);
  cursor: pointer;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.2s;
}

.close-btn:hover {
  background: var(--bg-hover, #f0f0f0);
  color: var(--text-primary, #333);
}

.step-content {
  text-align: center;
  margin-bottom: 2.5rem;
}

.step-icon {
  font-size: 4rem;
  margin-bottom: 1.5rem;
  animation: bounce 0.6s ease;
}

@keyframes bounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}

.step-content h2 {
  margin: 0 0 1rem 0;
  font-size: 1.8rem;
  color: var(--text-primary, #333);
  font-weight: 600;
}

.step-content p {
  margin: 0;
  font-size: 1.1rem;
  line-height: 1.7;
  color: var(--text-secondary, #666);
}

.step-indicator {
  display: flex;
  justify-content: center;
  gap: 0.5rem;
  margin-bottom: 2rem;
}

.indicator-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--border-light, #ddd);
  transition: all 0.3s;
}

.indicator-dot.active {
  width: 24px;
  border-radius: 4px;
  background: var(--accent-primary, #667eea);
}

.step-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  margin-bottom: 1rem;
}

.nav-btn {
  padding: 0.8rem 2rem;
  border: none;
  border-radius: 10px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.nav-btn.primary {
  background: var(--accent-gradient, linear-gradient(135deg, #667eea 0%, #764ba2 100%));
  color: white;
}

.nav-btn.primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
}

.nav-btn.primary.complete {
  background: linear-gradient(135deg, #48bb78 0%, #38a169 100%);
}

.nav-btn.secondary {
  background: var(--bg-secondary, #f5f5f5);
  color: var(--text-primary, #333);
  border: 2px solid var(--border-light, #ddd);
}

.nav-btn.secondary:hover {
  background: var(--bg-hover, #ebebeb);
}

.skip-btn {
  width: 100%;
  padding: 0.6rem;
  background: transparent;
  border: none;
  color: var(--text-secondary, #999);
  font-size: 0.9rem;
  cursor: pointer;
  transition: color 0.2s;
}

.skip-btn:hover {
  color: var(--text-primary, #333);
}

/* Celebration Animation */
.celebration-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.85);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
}

.celebration-content {
  text-align: center;
  color: white;
}

.celebration-icon {
  font-size: 6rem;
  animation: celebrate 0.6s ease;
}

@keyframes celebrate {
  0% {
    transform: scale(0);
    opacity: 0;
  }
  50% {
    transform: scale(1.2);
  }
  100% {
    transform: scale(1);
    opacity: 1;
  }
}

.celebration-content h3 {
  font-size: 2rem;
  margin: 1rem 0 0.5rem 0;
}

.celebration-content p {
  font-size: 1.2rem;
  opacity: 0.9;
}

.celebration-enter-active,
.celebration-leave-active {
  transition: opacity 0.3s;
}

.celebration-enter-from,
.celebration-leave-to {
  opacity: 0;
}

/* Responsive */
@media (max-width: 768px) {
  .onboarding-container {
    padding: 2rem 1.5rem;
  }

  .step-content h2 {
    font-size: 1.5rem;
  }

  .step-content p {
    font-size: 1rem;
  }

  .step-icon {
    font-size: 3rem;
  }
}
</style>
