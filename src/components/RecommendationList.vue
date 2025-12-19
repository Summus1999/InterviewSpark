<!--
  RecommendationList.vue - Smart practice recommendations
  
  Features:
  - Display AI-recommended practice questions
  - Show priority and dimension targeting
  - Quick start practice action
-->
<template>
  <div class="recommendation-list">
    <div class="list-header">
      <h4>智能练习推荐</h4>
      <button @click="loadRecommendations" class="refresh-btn">刷新</button>
    </div>

    <div v-if="loading" class="loading">分析中...</div>

    <div v-else-if="result && result.recommendations.length > 0" class="recommendations">
      <!-- Weak areas summary -->
      <div v-if="result.weak_dimensions.length > 0" class="weak-summary">
        <span class="summary-label">待提升维度:</span>
        <span
          v-for="dim in result.weak_dimensions"
          :key="dim"
          class="weak-tag"
        >
          {{ dimensionName(dim) }}
        </span>
      </div>

      <!-- Recommendation cards -->
      <div
        v-for="rec in result.recommendations"
        :key="rec.question_id"
        class="rec-card"
      >
        <div class="rec-header">
          <span class="priority-badge" :class="'p' + rec.priority">
            P{{ rec.priority }}
          </span>
          <span class="dimension-label">{{ dimensionName(rec.dimension) }}</span>
        </div>
        <div class="rec-question">{{ rec.question }}</div>
        <div class="rec-reason">{{ rec.reason }}</div>
        <div class="rec-footer">
          <span class="improvement">
            预期提升: +{{ rec.estimated_improvement.toFixed(1) }}
          </span>
          <button @click="$emit('startPractice', rec)" class="practice-btn">
            开始练习
          </button>
        </div>
      </div>
    </div>

    <div v-else class="empty-state">
      <p v-if="result && result.total_available === 0">题库为空，请先添加练习题目</p>
      <p v-else>暂无推荐，请先完成面试练习以生成个人画像</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { RecommendationResult, PracticeRecommendation } from '../services/database'
import { generatePracticeRecommendations } from '../services/database'

const emit = defineEmits<{
  (e: 'startPractice', rec: PracticeRecommendation): void
}>()

const result = ref<RecommendationResult | null>(null)
const loading = ref(true)

const dimensionNames: Record<string, string> = {
  technical_depth: '技术深度',
  communication: '沟通表达',
  problem_solving: '问题解决',
  domain_knowledge: '领域知识',
  adaptability: '应变能力',
  general: '综合能力'
}

onMounted(async () => {
  await loadRecommendations()
})

const loadRecommendations = async () => {
  loading.value = true
  try {
    result.value = await generatePracticeRecommendations('default_user', 5)
  } catch (error) {
    console.error('Failed to load recommendations:', error)
  } finally {
    loading.value = false
  }
}

const dimensionName = (key: string): string => {
  return dimensionNames[key] || key
}
</script>

<style scoped>
.recommendation-list {
  padding: 1.5rem;
  background: var(--card-bg, #fff);
  border-radius: 8px;
  border: 2px solid var(--border-color, #e0e0e0);
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.list-header h4 {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-primary, #333);
}

.refresh-btn {
  padding: 0.4rem 0.8rem;
  background: #667eea;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 0.85rem;
  cursor: pointer;
}

.loading,
.empty-state {
  padding: 2rem;
  text-align: center;
  color: var(--text-secondary, #999);
}

.weak-summary {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  align-items: center;
  margin-bottom: 1.5rem;
  padding: 1rem;
  background: #fff5f5;
  border-radius: 6px;
}

.summary-label {
  font-size: 0.9rem;
  color: #c53030;
}

.weak-tag {
  padding: 0.2rem 0.6rem;
  background: #fed7d7;
  color: #c53030;
  border-radius: 10px;
  font-size: 0.8rem;
}

.recommendations {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.rec-card {
  padding: 1rem;
  background: var(--bg-secondary, #f8f9ff);
  border-radius: 8px;
  border: 1px solid var(--border-color, #e0e0e0);
}

.rec-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.8rem;
}

.priority-badge {
  padding: 0.15rem 0.5rem;
  border-radius: 4px;
  font-size: 0.75rem;
  font-weight: 600;
  color: white;
}

.priority-badge.p5 { background: #e53e3e; }
.priority-badge.p4 { background: #ed8936; }
.priority-badge.p3 { background: #ecc94b; color: #333; }
.priority-badge.p2 { background: #48bb78; }
.priority-badge.p1 { background: #a0aec0; }

.dimension-label {
  font-size: 0.8rem;
  color: var(--text-secondary, #666);
}

.rec-question {
  font-size: 1rem;
  font-weight: 500;
  color: var(--text-primary, #333);
  margin-bottom: 0.5rem;
  line-height: 1.5;
}

.rec-reason {
  font-size: 0.85rem;
  color: var(--text-secondary, #666);
  margin-bottom: 0.8rem;
}

.rec-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.improvement {
  font-size: 0.8rem;
  color: #48bb78;
  font-weight: 500;
}

.practice-btn {
  padding: 0.4rem 1rem;
  background: #667eea;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 0.85rem;
  cursor: pointer;
}

.practice-btn:hover {
  background: #5568d3;
}
</style>
