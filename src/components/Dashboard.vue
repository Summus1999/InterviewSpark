<template>
  <div class="dashboard">
    <div class="header">
      <h1 class="title">Dashboard</h1>
      <p class="subtitle">Your interview performance overview</p>
      <button @click="refreshData" class="refresh-btn" :disabled="loading">
        {{ loading ? 'Loading...' : '🔄 Refresh' }}
      </button>
    </div>

    <div v-if="loading" class="loading">
      <div class="spinner"></div>
      <p>Loading dashboard data...</p>
    </div>

    <div v-else-if="error" class="error">
      <p>{{ error }}</p>
      <button @click="refreshData" class="retry-btn">Retry</button>
    </div>

    <div v-else-if="!dashboardData" class="empty-state">
      <p>No data available</p>
    </div>

    <div v-else class="content">
      <!-- Overview Cards -->
      <DashboardCards :stats="dashboardData.stats" />

      <!-- Main Grid -->
      <div class="main-grid">
        <!-- Left Column: Hot Questions -->
        <TopQuestionsList :questions="dashboardData.topQuestions" />

        <!-- Right Column: Weak Areas -->
        <WeakAreasList :weak-areas="dashboardData.weakAreas" />
      </div>

      <!-- Recent Sessions -->
      <RecentSessionsList
        :sessions="dashboardData.recentSessions"
        @view-session="handleViewSession"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getDashboardData } from '../services/database'
import type { DashboardData } from '../services/database'
import DashboardCards from './DashboardCards.vue'
import TopQuestionsList from './TopQuestionsList.vue'
import WeakAreasList from './WeakAreasList.vue'
import RecentSessionsList from './RecentSessionsList.vue'

const dashboardData = ref<DashboardData | null>(null)
const loading = ref<boolean>(false)
const error = ref<string | null>(null)

const loadData = async () => {
  loading.value = true
  error.value = null

  try {
    dashboardData.value = await getDashboardData()
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to load dashboard data'
    console.error('Failed to load dashboard:', err)
  } finally {
    loading.value = false
  }
}

const refreshData = async () => {
  await loadData()
}

const handleViewSession = (sessionId: number) => {
  // Emit event to parent or navigate to session details
  console.log('View session:', sessionId)
  // Could navigate to session details page or emit event
}

onMounted(() => {
  loadData()
})
</script>

<style scoped>
.dashboard {
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
  background: #f3f4f6;
  min-height: 100vh;
}

.header {
  margin-bottom: 2.5rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.title {
  font-size: 2.5rem;
  font-weight: 700;
  color: #111827;
  margin: 0;
}

.subtitle {
  font-size: 1rem;
  color: #6b7280;
  margin: 0.5rem 0 0 0;
}

.refresh-btn {
  padding: 0.75rem 1.5rem;
  background-color: #3b82f6;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}

.refresh-btn:hover:not(:disabled) {
  background-color: #2563eb;
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  transform: translateY(-2px);
}

.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
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
  padding: 2rem;
  background: white;
  border: 1px solid #fee2e2;
  border-radius: 12px;
  color: #dc2626;
  margin-bottom: 2rem;
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
  background: white;
  border-radius: 12px;
  color: #9ca3af;
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

.main-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(450px, 1fr));
  gap: 2rem;
  margin-bottom: 2rem;
}

@media (max-width: 1200px) {
  .main-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 768px) {
  .dashboard {
    padding: 1rem;
  }

  .header {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .title {
    font-size: 1.75rem;
  }

  .subtitle {
    font-size: 0.9rem;
  }

  .refresh-btn {
    align-self: flex-start;
  }

  .main-grid {
    gap: 1rem;
  }
}
</style>
