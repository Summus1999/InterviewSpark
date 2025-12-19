<template>
  <div class="timer-display" :class="{ warning: isWarning, critical: isCritical }">
    <div class="timer-circle">
      <svg class="progress-ring" width="120" height="120">
        <circle
          class="progress-ring-bg"
          stroke="var(--border-light)"
          stroke-width="8"
          fill="transparent"
          r="52"
          cx="60"
          cy="60"
        />
        <circle
          class="progress-ring-progress"
          :stroke="progressColor"
          stroke-width="8"
          fill="transparent"
          r="52"
          cx="60"
          cy="60"
          :style="{ strokeDashoffset: strokeDashoffset }"
        />
      </svg>
      <div class="timer-text">
        <div class="time-value">{{ formattedTime }}</div>
        <div class="time-label">剩余时间</div>
      </div>
    </div>
    
    <div class="timer-controls">
      <button 
        v-if="!isRunning && !isFinished" 
        @click="start" 
        class="control-btn start-btn"
      >
        开始计时
      </button>
      <button 
        v-if="isRunning" 
        @click="pause" 
        class="control-btn pause-btn"
      >
        暂停
      </button>
      <button 
        v-if="!isRunning && remainingTime < initialTime && !isFinished" 
        @click="resume" 
        class="control-btn resume-btn"
      >
        继续
      </button>
      <button 
        @click="reset" 
        class="control-btn reset-btn"
      >
        重置
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onBeforeUnmount } from 'vue'

interface Props {
  timeLimit: number  // Total time in seconds
  autoStart?: boolean
  showWarning?: boolean
  warningThreshold?: number  // Show warning when remaining time < this value (seconds)
}

const props = withDefaults(defineProps<Props>(), {
  autoStart: false,
  showWarning: true,
  warningThreshold: 30
})

const emit = defineEmits<{
  timeout: []
  warning: []
  start: []
  pause: []
  reset: []
}>()

const remainingTime = ref(props.timeLimit)
const initialTime = ref(props.timeLimit)
const isRunning = ref(false)
const isFinished = ref(false)
const timerInterval = ref<number | null>(null)

const formattedTime = computed(() => {
  const minutes = Math.floor(remainingTime.value / 60)
  const seconds = remainingTime.value % 60
  return `${minutes}:${seconds.toString().padStart(2, '0')}`
})

const progressPercentage = computed(() => {
  return (remainingTime.value / initialTime.value) * 100
})

const circumference = computed(() => 2 * Math.PI * 52)

const strokeDashoffset = computed(() => {
  const progress = progressPercentage.value / 100
  return circumference.value * (1 - progress)
})

const isWarning = computed(() => {
  return props.showWarning && remainingTime.value <= props.warningThreshold && remainingTime.value > 10
})

const isCritical = computed(() => {
  return remainingTime.value <= 10 && remainingTime.value > 0
})

const progressColor = computed(() => {
  if (isCritical.value) return 'var(--error-color)'
  if (isWarning.value) return 'var(--warning-color)'
  return 'var(--accent-primary)'
})

function start() {
  if (isRunning.value) return
  
  isRunning.value = true
  isFinished.value = false
  emit('start')
  
  timerInterval.value = window.setInterval(() => {
    if (remainingTime.value > 0) {
      remainingTime.value--
      
      // Emit warning at threshold
      if (remainingTime.value === props.warningThreshold && props.showWarning) {
        emit('warning')
      }
    } else {
      stop()
      isFinished.value = true
      emit('timeout')
    }
  }, 1000)
}

function pause() {
  if (!isRunning.value) return
  
  isRunning.value = false
  if (timerInterval.value) {
    clearInterval(timerInterval.value)
    timerInterval.value = null
  }
  emit('pause')
}

function resume() {
  start()
}

function stop() {
  isRunning.value = false
  if (timerInterval.value) {
    clearInterval(timerInterval.value)
    timerInterval.value = null
  }
}

function reset() {
  stop()
  remainingTime.value = props.timeLimit
  initialTime.value = props.timeLimit
  isFinished.value = false
  emit('reset')
}

// Auto start if enabled
watch(() => props.autoStart, (newVal) => {
  if (newVal && !isRunning.value && remainingTime.value === initialTime.value) {
    start()
  }
}, { immediate: true })

// Update time limit if props change
watch(() => props.timeLimit, (newVal) => {
  if (!isRunning.value) {
    remainingTime.value = newVal
    initialTime.value = newVal
  }
})

onBeforeUnmount(() => {
  stop()
})

defineExpose({
  start,
  pause,
  reset,
  isRunning: computed(() => isRunning.value),
  remainingTime: computed(() => remainingTime.value)
})
</script>

<style scoped>
.timer-display {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.5rem;
  padding: 1.5rem;
  background: var(--bg-card);
  border-radius: 12px;
  border: 2px solid var(--border-light);
  transition: all 0.3s;
}

.timer-display.warning {
  border-color: var(--warning-color);
  background: rgba(255, 152, 0, 0.05);
}

.timer-display.critical {
  border-color: var(--error-color);
  background: rgba(244, 67, 54, 0.05);
  animation: pulse 1s infinite;
}

@keyframes pulse {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.02);
  }
}

.timer-circle {
  position: relative;
  width: 120px;
  height: 120px;
}

.progress-ring {
  transform: rotate(-90deg);
}

.progress-ring-bg {
  opacity: 0.2;
}

.progress-ring-progress {
  stroke-dasharray: 326.73;
  transition: stroke-dashoffset 0.5s ease, stroke 0.3s ease;
}

.timer-text {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
}

.time-value {
  font-size: 1.8rem;
  font-weight: bold;
  color: var(--text-primary);
  font-family: 'Monaco', 'Courier New', monospace;
}

.time-label {
  font-size: 0.75rem;
  color: var(--text-secondary);
  margin-top: 0.2rem;
}

.timer-controls {
  display: flex;
  gap: 0.8rem;
  flex-wrap: wrap;
  justify-content: center;
}

.control-btn {
  padding: 0.6rem 1.2rem;
  border: none;
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.start-btn,
.resume-btn {
  background: var(--success-color);
  color: white;
}

.start-btn:hover,
.resume-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(76, 175, 80, 0.3);
}

.pause-btn {
  background: var(--warning-color);
  color: white;
}

.pause-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(255, 152, 0, 0.3);
}

.reset-btn {
  background: var(--bg-input);
  color: var(--text-primary);
  border: 1px solid var(--border-light);
}

.reset-btn:hover {
  background: var(--bg-hover);
  border-color: var(--border-hover);
}
</style>
