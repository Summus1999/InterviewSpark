<template>
  <div class="statistics-grid">
    <div class="stat-card">
      <div class="stat-label">Total Sessions</div>
      <div class="stat-value">{{ statistics.totalSessions }}</div>
    </div>

    <div class="stat-card">
      <div class="stat-label">Average Score</div>
      <div class="stat-value">{{ statistics.averageOverall.toFixed(1) }}</div>
    </div>

    <div class="stat-card">
      <div class="stat-label">Highest Score</div>
      <div class="stat-value">{{ statistics.highestOverall.toFixed(1) }}</div>
    </div>

    <div class="stat-card">
      <div class="stat-label">Progress Rate</div>
      <div class="stat-value" :class="improvementClass">
        {{ formattedImprovement }}
        <span class="trend-icon" :class="trendIconClass">{{ trendIcon }}</span>
      </div>
      <div class="trend-indicator" :class="trendClass">
        {{ trendText }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { PerformanceStatistics } from '../services/database'

const props = defineProps<{
  statistics: PerformanceStatistics
}>()

const formattedImprovement = computed(() => {
  const value = props.statistics.improvementRate
  const sign = value >= 0 ? '+' : ''
  return `${sign}${value.toFixed(1)}%`
})

const improvementClass = computed(() => {
  return props.statistics.improvementRate >= 0 ? 'positive' : 'negative'
})

const trendText = computed(() => {
  const trend = props.statistics.recentTrend
  if (trend === 'improving') return 'Improving'
  if (trend === 'declining') return 'Declining'
  return 'Stable'
})

const trendClass = computed(() => {
  return `trend-${props.statistics.recentTrend}`
})

const trendIcon = computed(() => {
  const trend = props.statistics.recentTrend
  if (trend === 'improving') return '↑'
  if (trend === 'declining') return '↓'
  return '→'
})

const trendIconClass = computed(() => {
  return `icon-${props.statistics.recentTrend}`
})
</script>

<style scoped>
.statistics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
  background: #fff;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: all 0.2s;
}

.stat-card:hover {
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
}

.stat-label {
  font-size: 0.875rem;
  color: #6b7280;
  margin-bottom: 0.5rem;
  font-weight: 500;
}

.stat-value {
  font-size: 2rem;
  font-weight: 700;
  color: #111827;
  line-height: 1.2;
}

.stat-value.positive {
  color: #10b981;
}

.stat-value.negative {
  color: #ef4444;
}

.trend-icon {
  display: inline-block;
  margin-left: 0.5rem;
  font-size: 1.5rem;
  vertical-align: middle;
}

.trend-icon.icon-improving {
  color: #10b981;
}

.trend-icon.icon-declining {
  color: #ef4444;
}

.trend-icon.icon-stable {
  color: #f59e0b;
}

.trend-indicator {
  margin-top: 0.5rem;
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 600;
  display: inline-block;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.trend-improving {
  background-color: #d1fae5;
  color: #065f46;
}

.trend-declining {
  background-color: #fee2e2;
  color: #991b1b;
}

.trend-stable {
  background-color: #fef3c7;
  color: #92400e;
}

@media (max-width: 768px) {
  .statistics-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 480px) {
  .statistics-grid {
    grid-template-columns: 1fr;
  }
}
</style>
