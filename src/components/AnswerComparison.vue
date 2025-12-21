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
      <p>{{ error }}</p>
    </div>

    <div v-else class="comparison-content">
      <!-- Best Answer Section -->
      <div class="best-answer-section">
        <div class="section-header">
          <h3>AI 优秀答案参考</h3>
          <div v-if="bestAnswer" class="version-info">
            <span class="version-badge">V{{ bestAnswer.version }}</span>
            <span class="source-count">基于 {{ bestAnswer.source_answer_count }} 次回答</span>
          </div>
        </div>
        
        <div v-if="generatingBestAnswer" class="generating">
          <div class="spinner-small"></div>
          <span>正在生成优秀答案...</span>
        </div>
        
        <div v-else-if="bestAnswer" class="best-answer-content">
          <div class="text-content best-text">{{ bestAnswer.generated_answer }}</div>
          <div class="update-time">
            更新于 {{ formatDate(bestAnswer.updated_at) }}
          </div>
        </div>
        
        <div v-else class="no-best-answer">
          <button @click="generateBestAnswer" class="generate-btn" :disabled="generatingBestAnswer">
            生成优秀答案
          </button>
        </div>
      </div>

      <!-- History Timeline -->
      <div v-if="comparisonData.length === 0" class="empty-state">
        <p>暂无该问题的历史记录</p>
      </div>

      <div v-else>
        <!-- Mode Switch Buttons -->
        <div class="mode-switch">
          <button 
            @click="switchToTimeline" 
            :class="{ active: comparisonMode === 'timeline' }"
            class="mode-btn"
          >
            📅 时间线视图
          </button>
          <button 
            @click="switchToSideBySide" 
            :class="{ active: comparisonMode === 'sideBySide' }"
            :disabled="!canCompare()"
            class="mode-btn"
          >
            🔀 并排对比 {{ selectedIndices.length > 0 ? `(${selectedIndices.length}/2)` : '' }}
          </button>
        </div>

        <!-- Timeline Mode -->
        <div v-if="comparisonMode === 'timeline'" class="comparison-timeline">
          <h3 class="timeline-title">历史回答记录</h3>
          <p class="selection-hint" v-if="processedComparisonData.length >= 2">
            💡 提示：选择 2 个回答后可切换到并排对比模式
          </p>
          <div
            v-for="(item, index) in processedComparisonData"
            :key="index"
            class="comparison-item"
            :class="{ selected: isSelected(index) }"
          >
            <div class="timeline-marker">
              <div class="dot"></div>
              <div v-if="index < processedComparisonData.length - 1" class="line"></div>
            </div>

            <div class="item-content">
              <div class="item-header">
                <div class="meta">
                  <input 
                    v-if="processedComparisonData.length >= 2"
                    type="checkbox" 
                    :checked="isSelected(index)"
                    @change="toggleSelection(index)"
                    class="select-checkbox"
                  />
                  <span class="timestamp">{{ item.formattedTimestamp }}</span>
                  <span class="score-badge" :class="item.scoreClass">
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
              已对比 {{ processedComparisonData.length }} 次尝试
              <span v-if="processedComparisonData.length > 1" class="improvement">
                最早评分: {{ lowestScore }} -> 最近评分: {{ highestScore }}
              </span>
            </p>
          </div>
        </div>

        <!-- Side by Side Mode -->
        <div v-else-if="comparisonMode === 'sideBySide'" class="side-by-side-container">
          <div class="comparison-grid">
            <div 
              v-for="(item, idx) in getSelectedItems()" 
              :key="idx"
              class="comparison-column"
            >
              <div class="column-header">
                <span class="column-title">回答 {{ idx + 1 }}</span>
                <span class="timestamp">{{ item.formattedTimestamp }}</span>
                <span class="score-badge" :class="item.scoreClass">
                  评分: {{ item.score }}
                </span>
              </div>
              
              <div class="column-content">
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
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { 
  getAnswersComparison, 
  getOrGenerateBestAnswer,
  type AnswerComparisonItem,
  type QuestionBestAnswer
} from '../services/database'

interface Props {
  question: string
  jobDescription?: string
}

const props = withDefaults(defineProps<Props>(), {
  jobDescription: ''
})

interface ProcessedComparisonItem extends AnswerComparisonItem {
  formattedTimestamp: string
  scoreValue: number
  scoreClass: string
}

const comparisonData = ref<AnswerComparisonItem[]>([])
const bestAnswer = ref<QuestionBestAnswer | null>(null)
const loading = ref(false)
const generatingBestAnswer = ref(false)
const error = ref<string | null>(null)
const comparisonMode = ref<'timeline' | 'sideBySide'>('timeline')
const selectedIndices = ref<number[]>([])

// Computed: Pre-process comparison data to avoid repeated calculations in template
const processedComparisonData = computed<ProcessedComparisonItem[]>(() => {
  return comparisonData.value.map(item => {
    const scoreValue = parseFloat(item.score)
    return {
      ...item,
      formattedTimestamp: formatDate(item.timestamp),
      scoreValue,
      scoreClass: getScoreClass(scoreValue)
    }
  })
})

// Computed: Lowest score
const lowestScore = computed<string>(() => {
  if (processedComparisonData.value.length === 0) return '-'
  return Math.min(...processedComparisonData.value.map(item => item.scoreValue)).toFixed(1)
})

// Computed: Highest score
const highestScore = computed<string>(() => {
  if (processedComparisonData.value.length === 0) return '-'
  return Math.max(...processedComparisonData.value.map(item => item.scoreValue)).toFixed(1)
})

onMounted(async () => {
  await loadComparison()
})

const loadComparison = async () => {
  loading.value = true
  error.value = null
  try {
    // Load history data first (fast, non-blocking)
    comparisonData.value = await getAnswersComparison(props.question)
    
    // Immediately show history data to user
    loading.value = false
    
    // Load best answer asynchronously in background (non-blocking)
    loadBestAnswerAsync()
  } catch (err: any) {
    error.value = err?.message || 'Failed to load comparison data'
    loading.value = false
  }
}

const loadBestAnswerAsync = async () => {
  try {
    bestAnswer.value = await getOrGenerateBestAnswer(props.question, props.jobDescription)
  } catch (err) {
    // Best answer generation failed, but history is still available
    console.warn('Failed to load best answer:', err)
    bestAnswer.value = null
  }
}

const generateBestAnswer = async () => {
  generatingBestAnswer.value = true
  try {
    bestAnswer.value = await getOrGenerateBestAnswer(props.question, props.jobDescription)
  } catch (err: any) {
    console.error('Failed to generate best answer:', err)
  } finally {
    generatingBestAnswer.value = false
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

const toggleSelection = (index: number) => {
  const idx = selectedIndices.value.indexOf(index)
  if (idx > -1) {
    selectedIndices.value.splice(idx, 1)
  } else {
    if (selectedIndices.value.length < 2) {
      selectedIndices.value.push(index)
    } else {
      selectedIndices.value = [selectedIndices.value[1], index]
    }
  }
}

const isSelected = (index: number): boolean => {
  return selectedIndices.value.includes(index)
}

const getSelectedItems = () => {
  return selectedIndices.value
    .sort((a, b) => a - b)
    .map(idx => processedComparisonData.value[idx])
}

const canCompare = (): boolean => {
  return selectedIndices.value.length === 2
}

const switchToSideBySide = () => {
  if (canCompare()) {
    comparisonMode.value = 'sideBySide'
  }
}

const switchToTimeline = () => {
  comparisonMode.value = 'timeline'
  selectedIndices.value = []
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

.spinner-small {
  width: 20px;
  height: 20px;
  border: 3px solid #f0f0f0;
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

.comparison-content {
  padding: 24px;
}

/* Best Answer Section */
.best-answer-section {
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(5, 150, 105, 0.05) 100%);
  border: 1px solid rgba(16, 185, 129, 0.2);
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  flex-wrap: wrap;
  gap: 12px;
}

.section-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #059669;
}

.version-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.version-badge {
  background: #059669;
  color: white;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
}

.source-count {
  font-size: 12px;
  color: #666;
}

.generating {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px;
  color: #666;
}

.best-answer-content {
  position: relative;
}

.best-text {
  background: white;
  padding: 16px;
  border-radius: 6px;
  border-left: 3px solid #10b981;
}

.update-time {
  margin-top: 8px;
  font-size: 12px;
  color: #888;
  text-align: right;
}

.no-best-answer {
  text-align: center;
  padding: 16px;
}

.generate-btn {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  border: none;
  padding: 10px 24px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: opacity 0.2s;
}

.generate-btn:hover {
  opacity: 0.9;
}

.generate-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.empty-state {
  padding: 40px 20px;
  text-align: center;
  color: #999;
  font-size: 16px;
}

.timeline-title {
  margin: 0 0 20px 0;
  font-size: 16px;
  font-weight: 600;
  color: #333;
}

.comparison-timeline {
  padding-top: 16px;
  border-top: 1px solid #e5e7eb;
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

/* Mode Switch */
.mode-switch {
  display: flex;
  gap: 12px;
  margin-bottom: 20px;
  padding: 12px;
  background: #f9f9f9;
  border-radius: 6px;
}

.mode-btn {
  flex: 1;
  padding: 10px 16px;
  border: 2px solid #e0e0e0;
  border-radius: 6px;
  background: white;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.3s;
  color: #666;
}

.mode-btn:hover:not(:disabled) {
  border-color: #667eea;
  background: #f0f0ff;
  color: #667eea;
}

.mode-btn.active {
  border-color: #667eea;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.mode-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.selection-hint {
  margin: 0 0 16px 0;
  padding: 8px 12px;
  background: #fff9e6;
  border-left: 3px solid #fbbf24;
  border-radius: 4px;
  font-size: 13px;
  color: #92400e;
}

.select-checkbox {
  width: 18px;
  height: 18px;
  cursor: pointer;
  margin-right: 8px;
}

.comparison-item.selected {
  border-left: 3px solid #667eea;
}

.comparison-item.selected .item-content {
  background: #f0f0ff;
}

/* Side by Side Mode */
.side-by-side-container {
  padding-top: 16px;
}

.comparison-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
}

.comparison-column {
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  overflow: hidden;
  background: white;
}

.column-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.column-title {
  font-size: 16px;
  font-weight: 600;
}

.column-header .timestamp {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.9);
  font-weight: normal;
}

.column-header .score-badge {
  align-self: flex-start;
  background: rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
}

.column-content {
  padding: 20px;
}

.column-content .answer-section,
.column-content .feedback-section {
  margin-bottom: 16px;
}

.column-content h4 {
  margin: 0 0 10px 0;
  font-size: 14px;
  font-weight: 600;
  color: #333;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.column-content .text-content {
  font-size: 14px;
  line-height: 1.7;
  color: #555;
  word-wrap: break-word;
  white-space: pre-wrap;
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
  
  .comparison-grid {
    grid-template-columns: 1fr;
  }
  
  .mode-switch {
    flex-direction: column;
  }
}
</style>
