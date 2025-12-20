<template>
  <teleport to="body">
    <div v-if="isActive" class="confetti-container">
      <!-- Firework particles -->
      <div
        v-for="particle in particles"
        :key="particle.id"
        class="particle"
        :style="{
          left: particle.x + 'px',
          top: particle.y + 'px',
          '--duration': particle.duration + 'ms',
          '--tx': particle.tx + 'px',
          '--ty': particle.ty + 'px',
          '--rotation': particle.rotation + 'deg'
        }"
      ></div>

      <!-- Success text -->
      <div v-if="showText" class="success-text">
        设置成功
      </div>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Particle {
  id: number
  x: number
  y: number
  tx: number
  ty: number
  duration: number
  rotation: number
}

const props = defineProps<{
  buttonElement?: HTMLElement
}>()

const isActive = ref(false)
const showText = ref(false)
const particles = ref<Particle[]>([])
let particleId = 0

const emit = defineEmits<{
  complete: []
}>()

/**
 * Trigger fireworks animation from button position
 */
const trigger = (buttonEl?: HTMLElement) => {
  const button = buttonEl || props.buttonElement
  if (!button) return

  isActive.value = true
  showText.value = false
  particles.value = []

  // Get button position
  const rect = button.getBoundingClientRect()
  const startX = rect.left + rect.width / 2
  const startY = rect.top + rect.height / 2

  // Generate particles
  const particleCount = 30
  for (let i = 0; i < particleCount; i++) {
    const angle = (i / particleCount) * Math.PI * 2
    const velocity = 4 + Math.random() * 4
    const tx = Math.cos(angle) * velocity * 60
    const ty = Math.sin(angle) * velocity * 60 - 100 // Upward bias

    particles.value.push({
      id: particleId++,
      x: startX,
      y: startY,
      tx,
      ty,
      duration: 800 + Math.random() * 400,
      rotation: Math.random() * 360
    })
  }

  // Show success text after burst
  setTimeout(() => {
    showText.value = true
  }, 400)

  // Complete animation and cleanup
  setTimeout(() => {
    isActive.value = false
    showText.value = false
    emit('complete')
  }, 2000)
}

defineExpose({
  trigger
})
</script>

<style scoped>
.confetti-container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 9999;
}

.particle {
  position: fixed;
  width: 10px;
  height: 10px;
  background: linear-gradient(135deg, #FFD700, #FFA500);
  border-radius: 50%;
  pointer-events: none;
  animation: burst var(--duration) ease-out forwards;
  box-shadow: 0 0 6px rgba(255, 215, 0, 0.8);
}

@keyframes burst {
  0% {
    opacity: 1;
    transform: translate(0, 0) rotate(0deg) scale(1);
  }
  100% {
    opacity: 0;
    transform: translate(var(--tx), var(--ty)) rotate(var(--rotation)) scale(0.2);
  }
}

.success-text {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 2.5rem;
  font-weight: 700;
  color: white;
  text-shadow: 
    0 0 10px rgba(255, 215, 0, 0.8),
    0 0 20px rgba(255, 165, 0, 0.6),
    2px 2px 4px rgba(0, 0, 0, 0.3);
  letter-spacing: 2px;
  animation: textAppear 1.5s ease-in-out forwards;
  pointer-events: none;
  z-index: 10000;
}

@keyframes textAppear {
  0% {
    opacity: 0;
    transform: translate(-50%, -50%) scale(0.5);
  }
  30% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1.1);
  }
  70% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1);
  }
  100% {
    opacity: 0;
    transform: translate(-50%, -50%) scale(0.8);
  }
}
</style>
