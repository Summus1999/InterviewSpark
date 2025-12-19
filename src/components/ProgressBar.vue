<template>
  <div class="progress-container">
    <div class="progress-bar">
      <div 
        class="progress-fill" 
        :style="{ width: progressPercentage + '%' }"
      >
        <span class="progress-text">{{ current }} / {{ total }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  current: number
  total: number
}

const props = defineProps<Props>()

const progressPercentage = computed(() => {
  if (props.total === 0) return 0
  return Math.round((props.current / props.total) * 100)
})
</script>

<style scoped>
.progress-container {
  width: 100%;
  margin-bottom: 1.5rem;
}

.progress-bar {
  width: 100%;
  height: 32px;
  background: var(--bg-card);
  border-radius: 16px;
  overflow: hidden;
  border: 1px solid var(--border-light);
}

.progress-fill {
  height: 100%;
  background: var(--accent-gradient);
  transition: width 0.5s ease-in-out;
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 60px;
}

.progress-text {
  color: var(--text-light);
  font-size: 0.9rem;
  font-weight: 600;
}
</style>
