<template>
  <div class="questions-container">
    <h3 class="section-title">🔥 Hot Questions</h3>
    
    <div v-if="questions.length === 0" class="empty-state">
      <p>No question data available yet</p>
    </div>

    <div v-else class="questions-list">
      <div
        v-for="(item, index) in questions"
        :key="index"
        class="question-item"
      >
        <div class="rank">{{ index + 1 }}</div>
        <div class="content">
          <div class="question-text">{{ truncateText(item.question, 60) }}</div>
          <div class="meta">Asked {{ item.count }} times</div>
        </div>
        <div class="count-badge">{{ item.count }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { TopQuestion } from '../services/database'

defineProps<{
  questions: TopQuestion[]
}>()

const truncateText = (text: string, length: number) => {
  return text.length > length ? text.substring(0, length) + '...' : text
}
</script>

<style scoped>
.questions-container {
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 12px;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.section-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: #111827;
  margin: 0 0 1.5rem 0;
}

.empty-state {
  text-align: center;
  padding: 2rem;
  color: #9ca3af;
  font-style: italic;
}

.questions-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.question-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: #f9fafb;
  border-radius: 8px;
  border: 1px solid #f3f4f6;
  transition: all 0.2s;
}

.question-item:hover {
  background: #f3f4f6;
  border-color: #e5e7eb;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.rank {
  flex-shrink: 0;
  width: 2.5rem;
  height: 2.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #3b82f6, #2563eb);
  color: white;
  border-radius: 50%;
  font-weight: 700;
  font-size: 1rem;
}

.content {
  flex: 1;
  min-width: 0;
}

.question-text {
  font-size: 0.95rem;
  color: #374151;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.meta {
  font-size: 0.8rem;
  color: #9ca3af;
  margin-top: 0.25rem;
}

.count-badge {
  flex-shrink: 0;
  background: #fef08a;
  color: #92400e;
  padding: 0.375rem 0.75rem;
  border-radius: 16px;
  font-size: 0.875rem;
  font-weight: 600;
}

@media (max-width: 768px) {
  .questions-container {
    padding: 1rem;
  }

  .question-item {
    padding: 0.75rem;
    gap: 0.75rem;
  }

  .rank {
    width: 2rem;
    height: 2rem;
    font-size: 0.875rem;
  }

  .question-text {
    font-size: 0.9rem;
  }

  .count-badge {
    font-size: 0.75rem;
    padding: 0.25rem 0.5rem;
  }
}
</style>
