<template>
  <Transition name="completion">
    <div v-if="show" class="completion-overlay" @click="onClose">
      <div class="completion-card" @click.stop>
        <div class="celebration">
          <div class="confetti-container">
            <span class="confetti">🎉</span>
            <span class="confetti">🎊</span>
            <span class="confetti">✨</span>
            <span class="confetti">🌟</span>
            <span class="confetti">⭐</span>
          </div>
          <h2>恭喜完成面试！</h2>
          <p class="message">{{ message }}</p>
          <button class="close-btn" @click="onClose">
            查看结果
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
interface Props {
  show: boolean
  message?: string
}

defineProps<Props>()

const emit = defineEmits<{
  close: []
}>()

function onClose() {
  emit('close')
}
</script>

<style scoped>
.completion-overlay {
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
  backdrop-filter: blur(5px);
}

.completion-card {
  background: var(--bg-card-solid);
  padding: 3rem;
  border-radius: 20px;
  box-shadow: var(--shadow-lg);
  max-width: 500px;
  text-align: center;
}

.celebration {
  position: relative;
}

.confetti-container {
  position: absolute;
  top: -40px;
  left: 50%;
  transform: translateX(-50%);
  width: 100%;
  height: 100px;
  pointer-events: none;
}

.confetti {
  position: absolute;
  font-size: 2rem;
  animation: confetti-fall 2s ease-in-out infinite;
}

.confetti:nth-child(1) {
  left: 10%;
  animation-delay: 0s;
}

.confetti:nth-child(2) {
  left: 30%;
  animation-delay: 0.3s;
}

.confetti:nth-child(3) {
  left: 50%;
  animation-delay: 0.6s;
}

.confetti:nth-child(4) {
  left: 70%;
  animation-delay: 0.9s;
}

.confetti:nth-child(5) {
  left: 90%;
  animation-delay: 1.2s;
}

@keyframes confetti-fall {
  0% {
    transform: translateY(0) rotate(0deg);
    opacity: 1;
  }
  100% {
    transform: translateY(100px) rotate(360deg);
    opacity: 0;
  }
}

h2 {
  margin: 2rem 0 1rem;
  font-size: 2rem;
  color: var(--accent-primary);
  font-weight: bold;
}

.message {
  font-size: 1.1rem;
  color: var(--text-secondary);
  margin-bottom: 2rem;
  line-height: 1.6;
}

.close-btn {
  padding: 1rem 2rem;
  background: var(--accent-gradient);
  color: var(--text-light);
  border: none;
  border-radius: 10px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.close-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
}

/* Transition animations */
.completion-enter-active,
.completion-leave-active {
  transition: all 0.3s ease;
}

.completion-enter-from,
.completion-leave-to {
  opacity: 0;
}

.completion-enter-from .completion-card,
.completion-leave-to .completion-card {
  transform: scale(0.8);
}

.completion-enter-to .completion-card,
.completion-leave-from .completion-card {
  transform: scale(1);
}
</style>
