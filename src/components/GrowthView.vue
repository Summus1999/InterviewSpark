<template>
  <div class="growth-view">
    <div class="header">
      <h1 class="title">Growth Curve</h1>
      <p class="subtitle">Track your interview performance over time</p>
    </div>

    <div v-if="loading" class="loading">
      <div class="spinner"></div>
      <p>Loading analytics data...</p>
    </div>

    <div v-else-if="error" class="error">
      <p>{{ error }}</p>
      <button @click="loadTrendData" class="retry-btn">Retry</button>
    </div>

    <div v-else-if="!trendData || trendData.dataPoints.length === 0" class="empty-state">
      <div class="empty-icon">📊</div>
      <h3>No Data Available</h3>
      <p>Complete some interviews to see your growth curve</p>
    </div>

    <div v-else class="content">
      <StatisticsCard :statistics="trendData.statistics" />

      <div class="controls">
        <div class="control-group">
          <label class="control-label">Time Range</label>
          <div class="button-group">
            <button
              :class="{ active: timeRange === undefined }"
              @click="timeRange = undefined"
              class="control-btn"
            >
              All Time
            </button>
            <button
              :class="{ active: timeRange === 30 }"
              @click="timeRange = 30"
              class="control-btn"
            >
              Last 30 Days
            </button>
            <button
              :class="{ active: timeRange === 7 }"
              @click="timeRange = 7"
              class="control-btn"
            >
              Last 7 Days
            </button>
          </div>
        </div>

        <div class="control-group">
          <label class="control-label">Dimension</label>
          <div class="button-group">
            <button
              :class="{ active: selectedDimension === 'overall' }"
              @click="selectedDimension = 'overall'"
              class="control-btn"
            >
              Overall
            </button>
            <button
              :class="{ active: selectedDimension === 'communication' }"
              @click="selectedDimension = 'communication'"
              class="control-btn"
            >
              Communication
            </button>
            <button
              :class="{ active: selectedDimension === 'problemSolving' }"
              @click="selectedDimension = 'problemSolving'"
              class="control-btn"
            >
              Problem Solving
            </button>
            <button
              :class="{ active: selectedDimension === 'technicalDepth' }"
              @click="selectedDimension = 'technicalDepth'"
              class="control-btn"
            >
              Technical Depth
            </button>
            <button
              :class="{ active: selectedDimension === 'presentation' }"
              @click="selectedDimension = 'presentation'"
              class="control-btn"
            >
              Presentation
            </button>
          </div>
        </div>
      </div>

      <div class="chart-container">
        <TrendChart :dataPoints="trendData.dataPoints" :dimension="selectedDimension" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { getTrendAnalytics } from '../services/database'
import type { TrendAnalytics } from '../services/database'
import TrendChart from './TrendChart.vue'
import StatisticsCard from './StatisticsCard.vue'

const trendData = ref<TrendAnalytics | null>(null)
const selectedDimension = ref<string>('overall')
const timeRange = ref<number | undefined>(undefined)
const loading = ref<boolean>(false)
const error = ref<string | null>(null)

const loadTrendData = async () => {
  loading.value = true
  error.value = null

  try {
    trendData.value = await getTrendAnalytics(timeRange.value)
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to load analytics data'
    console.error('Failed to load trend data:', err)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadTrendData()
})

watch(timeRange, () => {
  loadTrendData()
})
</script>

<style scoped>
.growth-view {
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
}

.header {
  margin-bottom: 2rem;
}

.title {
  font-size: 2rem;
  font-weight: 700;
  color: #111827;
  margin-bottom: 0.5rem;
}

.subtitle {
  font-size: 1rem;
  color: #6b7280;
}

.loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
}

.spinner {
  width: 48px;
  height: 48px;
  border: 4px solid #e5e7eb;
  border-top-color: #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.error {
  text-align: center;
  padding: 4rem 2rem;
  color: #ef4444;
}

.retry-btn {
  margin-top: 1rem;
  padding: 0.5rem 1.5rem;
  background-color: #3b82f6;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.875rem;
  font-weight: 500;
  transition: background-color 0.2s;
}

.retry-btn:hover {
  background-color: #2563eb;
}

.empty-state {
  text-align: center;
  padding: 4rem 2rem;
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.empty-state h3 {
  font-size: 1.5rem;
  font-weight: 600;
  color: #111827;
  margin-bottom: 0.5rem;
}

.empty-state p {
  color: #6b7280;
}

.content {
  animation: fadeIn 0.3s ease-in;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.controls {
  margin-bottom: 2rem;
}

.control-group {
  margin-bottom: 1.5rem;
}

.control-label {
  display: block;
  font-size: 0.875rem;
  font-weight: 600;
  color: #374151;
  margin-bottom: 0.75rem;
}

.button-group {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.control-btn {
  padding: 0.5rem 1rem;
  border: 1px solid #d1d5db;
  background-color: white;
  color: #374151;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.875rem;
  font-weight: 500;
  transition: all 0.2s;
}

.control-btn:hover {
  border-color: #3b82f6;
  color: #3b82f6;
}

.control-btn.active {
  background-color: #3b82f6;
  border-color: #3b82f6;
  color: white;
}

.chart-container {
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

@media (max-width: 768px) {
  .growth-view {
    padding: 1rem;
  }

  .title {
    font-size: 1.5rem;
  }

  .button-group {
    flex-direction: column;
  }

  .control-btn {
    width: 100%;
  }
}
</style>
