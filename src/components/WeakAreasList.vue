<template>
  <div class="weak-areas-container">
    <h3 class="section-title">⚠️ Areas for Improvement</h3>
    
    <div v-if="weakAreas.length === 0" class="empty-state">
      <p>No weak areas detected - great job!</p>
    </div>

    <div v-else class="areas-list">
      <div
        v-for="(item, index) in weakAreas"
        :key="index"
        class="area-card"
      >
        <div class="area-header">
          <div class="area-name">{{ item.area }}</div>
          <div class="area-score" :class="scoreClass(item.averageScore)">
            {{ item.averageScore.toFixed(1) }}
          </div>
        </div>
        <div class="suggestion">{{ item.suggestion }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { WeakArea } from '../services/database'

defineProps<{
  weakAreas: WeakArea[]
}>()

const scoreClass = (score: number) => {
  if (score < 40) return 'critical'
  if (score < 60) return 'warning'
  return 'caution'
}
</script>

<style scoped>
.weak-areas-container {
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
  color: #10b981;
  font-style: italic;
  background: #f0fdf4;
  border-radius: 8px;
}

.areas-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.area-card {
  padding: 1rem;
  background: #fef3c7;
  border: 1px solid #fcd34d;
  border-radius: 8px;
  border-left: 4px solid #f59e0b;
  transition: all 0.2s;
}

.area-card:hover {
  box-shadow: 0 4px 12px rgba(245, 158, 11, 0.15);
}

.area-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.75rem;
}

.area-name {
  font-size: 1rem;
  font-weight: 600;
  color: #92400e;
}

.area-score {
  font-size: 1.25rem;
  font-weight: 700;
  padding: 0.25rem 0.75rem;
  border-radius: 6px;
}

.area-score.critical {
  background: #fee2e2;
  color: #991b1b;
}

.area-score.warning {
  background: #fed7aa;
  color: #b45309;
}

.area-score.caution {
  background: #fef3c7;
  color: #92400e;
}

.suggestion {
  font-size: 0.9rem;
  color: #78350f;
  line-height: 1.5;
}

@media (max-width: 768px) {
  .weak-areas-container {
    padding: 1rem;
  }

  .area-card {
    padding: 0.75rem;
  }

  .area-name {
    font-size: 0.95rem;
  }

  .area-score {
    font-size: 1.1rem;
  }

  .suggestion {
    font-size: 0.85rem;
  }
}
</style>
