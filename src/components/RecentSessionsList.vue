<template>
  <div class="sessions-container">
    <h3 class="section-title">📝 Recent Sessions</h3>
    
    <div v-if="sessions.length === 0" class="empty-state">
      <p>No sessions recorded yet</p>
    </div>

    <div v-else class="sessions-list">
      <div
        v-for="session in sessions"
        :key="session.id"
        class="session-item"
        @click="$emit('view-session', session.id)"
      >
        <div class="session-date">{{ formatDate(session.createdAt) }}</div>
        <div class="session-info">
          <div class="question-count">{{ session.questionCount }} questions</div>
          <div v-if="session.overallScore" class="overall-score">
            Score: <span :class="scoreClass(session.overallScore)">{{ session.overallScore.toFixed(1) }}</span>
          </div>
        </div>
        <div class="view-btn">→</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { RecentSessionInfo } from '../services/database'

defineProps<{
  sessions: RecentSessionInfo[]
}>()

defineEmits<{
  'view-session': [sessionId: number]
}>()

const formatDate = (timestamp: string) => {
  const date = new Date(timestamp)
  return date.toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const scoreClass = (score: number) => {
  if (score >= 80) return 'excellent'
  if (score >= 60) return 'good'
  return 'fair'
}
</script>

<style scoped>
.sessions-container {
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

.sessions-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.session-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem;
  background: #f9fafb;
  border: 1px solid #f3f4f6;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.session-item:hover {
  background: #f3f4f6;
  border-color: #3b82f6;
  box-shadow: 0 4px 8px rgba(59, 130, 246, 0.1);
  transform: translateX(4px);
}

.session-date {
  flex-shrink: 0;
  font-size: 0.9rem;
  color: #6b7280;
  font-weight: 500;
  min-width: 120px;
}

.session-info {
  flex: 1;
  display: flex;
  gap: 1rem;
  align-items: center;
}

.question-count {
  font-size: 0.95rem;
  color: #374151;
  font-weight: 500;
  padding: 0.25rem 0.75rem;
  background: #e0f2fe;
  color: #0369a1;
  border-radius: 6px;
}

.overall-score {
  font-size: 0.95rem;
  color: #6b7280;
}

.overall-score span {
  font-weight: 700;
  margin-left: 0.25rem;
}

.overall-score .excellent {
  color: #10b981;
}

.overall-score .good {
  color: #3b82f6;
}

.overall-score .fair {
  color: #f59e0b;
}

.view-btn {
  flex-shrink: 0;
  font-size: 1.25rem;
  color: #9ca3af;
  transition: color 0.2s;
  margin-left: 0.5rem;
}

.session-item:hover .view-btn {
  color: #3b82f6;
}

@media (max-width: 768px) {
  .sessions-container {
    padding: 1rem;
  }

  .session-item {
    padding: 0.75rem;
    gap: 0.75rem;
  }

  .session-date {
    min-width: 100px;
    font-size: 0.85rem;
  }

  .session-info {
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .question-count {
    font-size: 0.85rem;
    padding: 0.2rem 0.5rem;
  }

  .overall-score {
    font-size: 0.85rem;
  }

  .view-btn {
    font-size: 1rem;
  }
}
</style>
