<template>
  <div class="comparison-container">
    <div class="header">
      <h2>答案对比</h2>
      <p class="question-text">{{ question }}</p>
    </div>

    <div v-if="loading" class="loading">
      <div class="spinner"></div>
      <p>加载中...</p>
    </div>

    <div v-else-if="error" class="error">
      <p>⚠️ {{ error }}</p>
    </div>

    <div v-else-if="comparisonData.length === 0" class="empty-state">
      <p>暂无该问题的历史记录</p>
    </div>

    <div v-else class="comparison-timeline">
      <div
        v-for="(item, index) in comparisonData"
        :key="index"
        class="comparison-item"
      >
        <div class="timeline-marker">
          <div class="dot"></div>
          <div v-if="index < comparisonData.length - 1" class="line"></div>
        </div>

        <div class="item-content">
          <div class="item-header">
            <div class="meta">
              <span class="timestamp">{{ formatDate(item.timestamp) }}</span>
              <span class="score-badge" :class="getScoreClass(parseFloat(item.score))">
                评分: {{ item.score }}
              </span>
            </div>
          </div>

          <div class="answer-section">
            <h4>你的回答</h4>
            <div class="text-content">{{ item.answer }}</div>
          </div>

          <div class="feedback-section">
            <h4>反馈</h4>
            <div class="text-content">{{ item.feedback }}</div>
          </div>
        </div>
      </div>

      <div class="progress-indicator">
        <p class="progress-text">
          已对比 {{ comparisonData.length }} 次尝试
          <span v-if="comparisonData.length > 1" class="improvement">
            最早评分: {{ getLowestScore() }} → 最近评分: {{ getHighestScore() }}
          </span>
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getAnswersComparison, type AnswerComparisonItem } from '../services/database'

interface Props {
  question: string
}

const props = defineProps<Props>()

const comparisonData = ref<AnswerComparisonItem[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

onMounted(async () => {
  await loadComparison()
})

const loadComparison = async () => {
  loading.value = true
  error.value = null
  try {
    comparisonData.value = await getAnswersComparison(props.question)
  } catch (err: any) {
    error.value = err?.message || '加载对比数据失败'
  } finally {
    loading.value = false
  }
}

const formatDate = (dateString: string): string => {
  try {
    const date = new Date(dateString)
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit'
    })
  } catch {
    return dateString
  }
}

const getScoreClass = (score: number): string => {
  if (score >= 8) return 'excellent'
  if (score >= 6) return 'good'
  if (score >= 4) return 'fair'
  return 'poor'
}

const getLowestScore = (): string => {
  if (comparisonData.value.length === 0) return '-'
  return Math.min(...comparisonData.value.map((item: AnswerComparisonItem) => parseFloat(item.score))).toFixed(1)
}

const getHighestScore = (): string => {
  if (comparisonData.value.length === 0) return '-'
  return Math.max(...comparisonData.value.map((item: AnswerComparisonItem) => parseFloat(item.score))).toFixed(1)
}
</script>

<style scoped>
.comparison-container {
  max-width: 900px;
  margin: 0 auto;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 24px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.header h2 {
  margin: 0 0 12px 0;
  font-size: 24px;
  font-weight: 600;
}

.question-text {
  margin: 0;
  font-size: 16px;
  opacity: 0.9;
  word-wrap: break-word;
}

.loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  gap: 16px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f0f0f0;
  border-top-color: #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.error {
  padding: 40px 20px;
  background: #fee;
  border-left: 4px solid #f87171;
  margin: 16px;
  border-radius: 4px;
}

.error p {
  margin: 0;
  color: #991b1b;
  font-size: 14px;
}

.empty-state {
  padding: 60px 20px;
  text-align: center;
  color: #999;
  font-size: 16px;
}

.comparison-timeline {
  padding: 24px;
}

.comparison-item {
  display: flex;
  margin-bottom: 32px;
}

.timeline-marker {
  position: relative;
  margin-right: 24px;
  min-width: 40px;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.dot {
  width: 16px;
  height: 16px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 50%;
  position: relative;
  z-index: 1;
  box-shadow: 0 0 0 4px rgba(102, 126, 234, 0.1);
}

.line {
  position: absolute;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  width: 2px;
  height: 100%;
  background: linear-gradient(to bottom, #667eea, transparent);
  min-height: 60px;
}

.item-content {
  flex: 1;
  background: #f9f9f9;
  border-left: 2px solid #e5e7eb;
  padding: 16px;
  border-radius: 4px;
}

.item-header {
  margin-bottom: 12px;
}

.meta {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.timestamp {
  font-size: 13px;
  color: #666;
  font-weight: 500;
}

.score-badge {
  display: inline-block;
  padding: 4px 12px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 600;
  color: white;
}

.score-badge.excellent {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
}

.score-badge.good {
  background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
}

.score-badge.fair {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
}

.score-badge.poor {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
}

.answer-section,
.feedback-section {
  margin-bottom: 12px;
}

.answer-section h4,
.feedback-section h4 {
  margin: 0 0 8px 0;
  font-size: 13px;
  font-weight: 600;
  color: #333;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.text-content {
  font-size: 14px;
  line-height: 1.6;
  color: #555;
  word-wrap: break-word;
  white-space: pre-wrap;
}

.progress-indicator {
  margin-top: 32px;
  padding-top: 16px;
  border-top: 1px solid #e5e7eb;
  background: linear-gradient(to right, rgba(102, 126, 234, 0.05), transparent);
  padding: 12px 16px;
  border-radius: 4px;
}

.progress-text {
  margin: 0;
  font-size: 13px;
  color: #666;
  font-weight: 500;
}

.improvement {
  display: block;
  margin-top: 4px;
  color: #10b981;
  font-weight: 600;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .comparison-container {
    border-radius: 0;
  }

  .header {
    padding: 16px;
  }

  .header h2 {
    font-size: 20px;
  }

  .comparison-timeline {
    padding: 16px;
  }

  .comparison-item {
    margin-bottom: 24px;
  }

  .meta {
    flex-direction: column;
    align-items: flex-start;
  }

  .answer-section,
  .feedback-section {
    margin-bottom: 8px;
  }

  .text-content {
    font-size: 13px;
  }
}
</style>
