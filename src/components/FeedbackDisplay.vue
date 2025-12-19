<template>
  <div class="feedback-display">
    <div v-if="isStreaming" class="streaming-indicator">
      <span class="pulse"></span>
      AI 正在分析...
    </div>

    <div class="feedback-content" v-if="content">
      <div class="typewriter-text">{{ displayedContent }}</div>
    </div>

    <div v-if="error" class="error-message">
      {{ error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'

const props = defineProps<{
  content: string
  isStreaming: boolean
  error: string | null
}>()

// Typewriter effect: gradually display content
const displayedContent = computed(() => props.content)
</script>

<style scoped>
.feedback-display {
  padding: 16px;
  background: #f9fafb;
  border-radius: 8px;
  min-height: 100px;
}

.streaming-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #6366f1;
  font-size: 14px;
  margin-bottom: 12px;
}

.pulse {
  width: 8px;
  height: 8px;
  background: #6366f1;
  border-radius: 50%;
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.3;
  }
}

.feedback-content {
  line-height: 1.6;
}

.typewriter-text {
  white-space: pre-wrap;
  word-break: break-word;
  font-size: 14px;
  color: #374151;
}

.error-message {
  padding: 12px;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 6px;
  color: #dc2626;
  font-size: 14px;
}
</style>
