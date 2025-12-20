<template>
  <div class="activity-view">
    <div v-if="loading" class="loading">
      <div class="spinner"></div>
      <p>加载中...</p>
    </div>

    <div v-else-if="error" class="error">
      <p>{{ error }}</p>
      <button @click="loadData" class="retry-btn">重试</button>
    </div>

    <div v-else class="content">
      <ActivityHeatmap :data="activityData" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getActivityData } from '../services/database'
import type { ActivityData } from '../services/database'
import ActivityHeatmap from './ActivityHeatmap.vue'

const activityData = ref<ActivityData[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

const loadData = async () => {
  loading.value = true
  error.value = null

  try {
    activityData.value = await getActivityData()
  } catch (err) {
    error.value = err instanceof Error ? err.message : '加载活跃度数据失败'
    console.error('Failed to load activity data:', err)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadData()
})
</script>

<style scoped>
.activity-view {
  padding: 1rem 2rem;
  max-width: 1200px;
  margin: 0 auto;
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
  width: 32px;
  height: 32px;
  border: 3px solid #e5e7eb;
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

@media (max-width: 768px) {
  .activity-view {
    padding: 1rem;
  }
}
</style>
