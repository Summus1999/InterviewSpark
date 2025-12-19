<!--
  BestPracticesList.vue - Display extracted best practices
  
  Features:
  - Show high-scoring answers
  - Display key points
  - Copy to clipboard
-->
<template>
  <div class="best-practices">
    <div class="header">
      <h4>最佳实践库</h4>
      <button @click="loadPractices" class="refresh-btn">刷新</button>
    </div>

    <div v-if="loading" class="loading">提取中...</div>

    <div v-else-if="result && result.practices.length > 0" class="practices-list">
      <div class="summary">
        已分析 {{ result.total_analyzed }} 个回答，
        提取得分 ≥ {{ result.threshold_score }} 的最佳实践
      </div>

      <div
        v-for="(practice, index) in result.practices"
        :key="index"
        class="practice-card"
      >
        <div class="practice-header">
          <span class="score-badge">{{ practice.score.toFixed(1) }} 分</span>
          <button @click="copyToClipboard(practice)" class="copy-btn">复制</button>
        </div>
        
        <div class="practice-question">
          <strong>Q:</strong> {{ practice.question }}
        </div>
        
        <div class="practice-answer">
          <strong>A:</strong> {{ truncateText(practice.answer, 200) }}
          <button 
            v-if="practice.answer.length > 200"
            @click="toggleExpand(index)"
            class="expand-btn"
          >
            {{ expandedIndex === index ? '收起' : '展开' }}
          </button>
          <div v-if="expandedIndex === index" class="full-answer">
            {{ practice.answer }}
          </div>
        </div>

        <div v-if="practice.key_points.length > 0" class="key-points">
          <strong>关键要点:</strong>
          <ul>
            <li v-for="(point, i) in practice.key_points" :key="i">
              {{ point }}
            </li>
          </ul>
        </div>
      </div>
    </div>

    <div v-else class="empty-state">
      <p>暂无高分回答，继续练习以积累最佳实践</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { BestPracticesResult, BestPractice } from '../services/database'
import { extractBestPractices } from '../services/database'

const result = ref<BestPracticesResult | null>(null)
const loading = ref(true)
const expandedIndex = ref<number | null>(null)

onMounted(async () => {
  await loadPractices()
})

const loadPractices = async () => {
  loading.value = true
  try {
    result.value = await extractBestPractices(7.5, 10)
  } catch (error) {
    console.error('Failed to extract best practices:', error)
  } finally {
    loading.value = false
  }
}

const truncateText = (text: string, maxLength: number): string => {
  if (text.length <= maxLength) return text
  return text.substring(0, maxLength) + '...'
}

const toggleExpand = (index: number) => {
  expandedIndex.value = expandedIndex.value === index ? null : index
}

const copyToClipboard = async (practice: BestPractice) => {
  const text = `问题: ${practice.question}\n\n回答: ${practice.answer}`
  try {
    await navigator.clipboard.writeText(text)
    alert('已复制到剪贴板')
  } catch (error) {
    console.error('Failed to copy:', error)
  }
}
</script>

<style scoped>
.best-practices {
  padding: 1.5rem;
  background: var(--card-bg, #fff);
  border-radius: 8px;
  border: 2px solid var(--border-color, #e0e0e0);
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.header h4 {
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
  cursor: pointer;
}

.loading,
.empty-state {
  padding: 2rem;
  text-align: center;
  color: var(--text-secondary, #999);
}

.summary {
  margin-bottom: 1.5rem;
  padding: 0.8rem;
  background: #f0fff4;
  border-radius: 6px;
  color: #2f855a;
  font-size: 0.9rem;
}

.practices-list {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.practice-card {
  padding: 1.5rem;
  background: var(--bg-secondary, #f8f9ff);
  border-radius: 8px;
  border: 1px solid var(--border-color, #e0e0e0);
}

.practice-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.score-badge {
  padding: 0.3rem 0.8rem;
  background: #48bb78;
  color: white;
  border-radius: 15px;
  font-size: 0.9rem;
  font-weight: 600;
}

.copy-btn {
  padding: 0.3rem 0.8rem;
  background: #4299e1;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.85rem;
}

.practice-question {
  margin-bottom: 1rem;
  font-size: 1rem;
  color: var(--text-primary, #333);
  line-height: 1.5;
}

.practice-answer {
  margin-bottom: 1rem;
  font-size: 0.95rem;
  color: var(--text-secondary, #555);
  line-height: 1.6;
}

.expand-btn {
  margin-left: 0.5rem;
  padding: 0.2rem 0.5rem;
  background: transparent;
  border: 1px solid var(--border-color, #ccc);
  border-radius: 4px;
  font-size: 0.8rem;
  cursor: pointer;
  color: var(--text-secondary, #666);
}

.full-answer {
  margin-top: 1rem;
  padding: 1rem;
  background: white;
  border-radius: 4px;
  white-space: pre-wrap;
}

.key-points {
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border-color, #e0e0e0);
}

.key-points strong {
  display: block;
  margin-bottom: 0.5rem;
  color: var(--text-primary, #333);
  font-size: 0.9rem;
}

.key-points ul {
  margin: 0;
  padding-left: 1.5rem;
}

.key-points li {
  margin-bottom: 0.4rem;
  font-size: 0.9rem;
  color: var(--text-secondary, #555);
  line-height: 1.5;
}
</style>
