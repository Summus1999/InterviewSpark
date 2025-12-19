<template>
  <div class="statistics-grid">
    <div class="stat-card">
      <div class="stat-label">Total Sessions</div>
      <div class="stat-value">{{ stats.totalSessions }}</div>
      <div class="stat-icon">🎯</div>
    </div>

    <div class="stat-card">
      <div class="stat-label">Average Score</div>
      <div class="stat-value">{{ stats.averageScore.toFixed(1) }}</div>
      <div class="stat-icon">📊</div>
    </div>

    <div class="stat-card">
      <div class="stat-label">Highest Score</div>
      <div class="stat-value">{{ stats.highestScore.toFixed(1) }}</div>
      <div class="stat-icon">🏆</div>
    </div>

    <div class="stat-card">
      <div class="stat-label">Recent Progress</div>
      <div class="stat-value" :class="progressClass">
        {{ formattedProgress }}
      </div>
      <div class="stat-icon">{{ progressIcon }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { DashboardStats } from '../services/database'

const props = defineProps<{
  stats: DashboardStats
}>()

const formattedProgress = computed(() => {
  const value = props.stats.recentImprovement
  const sign = value >= 0 ? '+' : ''
  return `${sign}${value.toFixed(1)}%`
})

const progressClass = computed(() => {
  return props.stats.recentImprovement >= 0 ? 'positive' : 'negative'
})

const progressIcon = computed(() => {
  const value = props.stats.recentImprovement
  if (value > 5) return '📈'
  if (value < -5) return '📉'
  return '➡️'
})
</script>

<style scoped>
.statistics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.stat-card {
  background: linear-gradient(135deg, #ffffff 0%, #f9fafb 100%);
  border: 1px solid #e5e7eb;
  border-radius: 12px;
  padding: 1.75rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.stat-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(90deg, #3b82f6, #2563eb);
}

.stat-card:hover {
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.12);
  transform: translateY(-4px);
  border-color: #3b82f6;
}

.stat-icon {
  position: absolute;
  top: 1rem;
  right: 1rem;
  font-size: 2rem;
  opacity: 0.6;
}

.stat-label {
  font-size: 0.875rem;
  color: #6b7280;
  margin-bottom: 0.75rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.stat-value {
  font-size: 2.5rem;
  font-weight: 700;
  color: #111827;
  line-height: 1;
  margin-bottom: 0.5rem;
}

.stat-value.positive {
  color: #10b981;
}

.stat-value.negative {
  color: #ef4444;
}

@media (max-width: 768px) {
  .statistics-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }

  .stat-card {
    padding: 1.25rem;
  }

  .stat-value {
    font-size: 1.75rem;
  }

  .stat-label {
    font-size: 0.75rem;
  }

  .stat-icon {
    font-size: 1.5rem;
  }
}

@media (max-width: 480px) {
  .statistics-grid {
    grid-template-columns: 1fr;
  }
}
</style>
