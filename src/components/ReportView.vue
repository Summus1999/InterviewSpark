<!--
  ReportView.vue - Comprehensive interview report viewer and exporter
  
  Features:
  - Display comprehensive interview reports
  - View overall performance, improvements, and key takeaways
  - Export report to text and HTML formats
-->

<template>
  <div class="report-view">
    <!-- Loading State -->
    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p class="loading-text">正在生成报告...</p>
      <p class="loading-subtext">已等待 {{ elapsedSeconds }} 秒 / 预计 {{ estimatedSeconds }} 秒</p>
      <div class="progress-bar">
        <div class="progress-fill" :style="{ width: progressPercent + '%' }"></div>
      </div>
    </div>

    <!-- Report Display -->
    <div v-else-if="report" class="report-content">
      <!-- Header -->
      <div class="report-header">
        <h2>面试复盘报告</h2>
        <div class="report-meta">
          <span class="meta-item">
            <label>生成时间</label>
            <span class="meta-value">{{ formatDate(report.generated_at) }}</span>
          </span>
          <span class="score-badge" :style="{ backgroundColor: scoreColor }">
            <span class="score-value">{{ report.overall_score.toFixed(1) }}</span>
            <span class="score-label">/10</span>
            <span class="score-grade">{{ scoreGrade }}</span>
          </span>
        </div>
      </div>

      <!-- Overall Summary -->
      <section class="report-section">
        <h3 class="section-title">整体表现</h3>
        <div class="section-content">
          <p>{{ report.summary }}</p>
        </div>
      </section>

      <!-- Improvements -->
      <section class="report-section">
        <h3 class="section-title">改进建议</h3>
        <div class="section-content">
          <ul class="improvements-list">
            <li v-for="(item, idx) in improvements" :key="idx">
              <span class="list-icon">→</span>
              <span>{{ item }}</span>
            </li>
          </ul>
        </div>
      </section>

      <!-- Key Takeaways -->
      <section class="report-section">
        <h3 class="section-title">关键要点</h3>
        <div class="section-content">
          <ul class="takeaways-list">
            <li v-for="(item, idx) in keyTakeaways" :key="idx">
              <span class="list-icon">✓</span>
              <span>{{ item }}</span>
            </li>
          </ul>
        </div>
      </section>

      <!-- Actions -->
      <div class="report-actions">
        <button @click="exportText" class="btn btn-text" :disabled="exporting">
          <span v-if="!exporting">📄 导出为文本</span>
          <span v-else>导出中...</span>
        </button>
        <button @click="exportHtml" class="btn btn-html" :disabled="exporting">
          <span v-if="!exporting">🌐 导出为网页</span>
          <span v-else>导出中...</span>
        </button>
        <button @click="printReport" class="btn btn-print">
          🖨️ 打印
        </button>
      </div>
    </div>

    <!-- Error State -->
    <div v-else class="error-state">
      <p>{{ errorMessage }}</p>
      <div class="error-actions">
        <button @click="retryGenerate" class="btn btn-retry">重新生成</button>
        <button @click="goHome" class="btn btn-home">返回主页</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Report view component
 * Displays comprehensive interview reports and export options
 */

import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { SessionReport } from '../services/database'

// Props
interface Props {
  sessionId: number
}

const props = withDefaults(defineProps<Props>(), {})

// Emits
const emit = defineEmits<{
  close: []
}>()

// State
const report = ref<SessionReport | null>(null)
const loading = ref(false)
const exporting = ref(false)
const errorMessage = ref('')
const retryCount = ref(0)
const timeoutId = ref<number | null>(null)
const maxRetries = 2
const elapsedSeconds = ref(0)
const estimatedSeconds = 45
const timerInterval = ref<number | null>(null)

// Computed properties
const improvements = computed(() => {
  if (!report.value) return []
  try {
    return JSON.parse(report.value.improvements) as string[]
  } catch {
    return []
  }
})

const keyTakeaways = computed(() => {
  if (!report.value) return []
  try {
    return JSON.parse(report.value.key_takeaways) as string[]
  } catch {
    return []
  }
})

const scoreColor = computed(() => {
  if (!report.value) return '#666'
  const score = report.value.overall_score
  if (score >= 8.5) return '#4CAF50'  // Green
  if (score >= 7.5) return '#8BC34A'  // Light Green
  if (score >= 6.5) return '#FFC107'  // Amber
  if (score >= 5.5) return '#FF9800'  // Orange
  return '#F44336'                    // Red
})

const scoreGrade = computed(() => {
  if (!report.value) return 'N/A'
  const score = report.value.overall_score
  if (score >= 9.0) return 'A+'
  if (score >= 8.5) return 'A'
  if (score >= 8.0) return 'A-'
  if (score >= 7.5) return 'B+'
  if (score >= 7.0) return 'B'
  if (score >= 6.5) return 'B-'
  if (score >= 6.0) return 'C+'
  if (score >= 5.0) return 'C'
  if (score >= 4.0) return 'D'
  return 'F'
})

const remainingSeconds = computed(() => {
  const remaining = estimatedSeconds - elapsedSeconds.value
  return Math.max(0, remaining)
})

const progressPercent = computed(() => {
  return Math.min(100, (elapsedSeconds.value / estimatedSeconds) * 100)
})

// Methods
const loadReport = async () => {
  errorMessage.value = ''
  
  try {
    // First, quickly check if report already exists in database
    const existingReport = await invoke<SessionReport | null>('db_get_report', {
      sessionId: props.sessionId
    })
    
    if (existingReport) {
      // Report exists, show directly without loading state
      report.value = existingReport
      return
    }
    
    // No existing report, need to generate - show loading state
    loading.value = true
    elapsedSeconds.value = 0
    
    // Start timer to show elapsed time
    if (timerInterval.value) {
      clearInterval(timerInterval.value)
    }
    timerInterval.value = window.setInterval(() => {
      elapsedSeconds.value++
    }, 1000)
    
    // Clear previous timeout
    if (timeoutId.value) {
      clearTimeout(timeoutId.value)
      timeoutId.value = null
    }
    
    // Set frontend timeout (60 seconds)
    const timeoutPromise = new Promise<never>((_, reject) => {
      timeoutId.value = window.setTimeout(() => {
        reject(new Error('网络超时，请重试'))
      }, 60000)
    })
    
    // Race between API call and timeout
    const result = await Promise.race([
      invoke<SessionReport>('generate_comprehensive_report', {
        sessionId: props.sessionId,
        usePremiumModel: false
      }),
      timeoutPromise
    ])
    
    report.value = result
    retryCount.value = 0
  } catch (error) {
    const errorStr = String(error)
    console.error('Failed to load report:', error)
    
    // Translate error messages to Chinese for better UX
    if (errorStr.includes('超时') || errorStr.includes('timeout')) {
      errorMessage.value = `网络超时，请检查网络连接后重试 (已重试 ${retryCount.value}/${maxRetries} 次)`
    } else if (errorStr.includes('API') || errorStr.includes('Failed to generate')) {
      errorMessage.value = '报告生成失败，AI 服务暂时不可用。请稍后重试。'
    } else if (errorStr.includes('No answers found')) {
      errorMessage.value = '该面试会话暂无答题记录，无法生成分析报告。'
    } else if (errorStr.includes('Session not found')) {
      errorMessage.value = '未找到该面试会话记录。'
    } else if (errorStr.includes('API client not initialized')) {
      errorMessage.value = 'AI 服务未初始化，请检查 API Key 配置。'
    } else {
      errorMessage.value = '报告生成失败，请稍后重试。'
    }
  } finally {
    loading.value = false
    // Clear timeout and timer
    if (timeoutId.value) {
      clearTimeout(timeoutId.value)
      timeoutId.value = null
    }
    if (timerInterval.value) {
      clearInterval(timerInterval.value)
      timerInterval.value = null
    }
  }
}

const exportText = async () => {
  if (!report.value) return
  
  exporting.value = true
  try {
    const fileName = `report_${props.sessionId}_${Date.now()}.txt`
    const filePath = `${fileName}`
    
    await invoke('export_report_text', {
      sessionId: props.sessionId,
      filePath: filePath
    })
    
    alert(`报告已导出为: ${fileName}`)
  } catch (error) {
    alert(`导出失败: ${String(error)}`)
    console.error('Export failed:', error)
  } finally {
    exporting.value = false
  }
}

const exportHtml = async () => {
  if (!report.value) return
  
  exporting.value = true
  try {
    const fileName = `report_${props.sessionId}_${Date.now()}.html`
    const filePath = `${fileName}`
    
    await invoke('export_report_html', {
      sessionId: props.sessionId,
      filePath: filePath
    })
    
    alert(`报告已导出为: ${fileName}`)
  } catch (error) {
    alert(`导出失败: ${String(error)}`)
    console.error('Export failed:', error)
  } finally {
    exporting.value = false
  }
}

const printReport = () => {
  window.print()
}

const retryGenerate = () => {
  if (retryCount.value < maxRetries) {
    retryCount.value++
    loadReport()
  } else {
    errorMessage.value = `已达到最大重试次数 (${maxRetries})。请稍后再试或联系支持。`
  }
}

const goHome = () => {
  emit('close')
}

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// Lifecycle
onMounted(() => {
  loadReport()
})
</script>

<style scoped>
.report-view {
  padding: 2rem 0;
  max-width: 900px;
  margin: 0 auto;
}

/* Loading State */
.loading-state {
  text-align: center;
  padding: 4rem 2rem;
}

.loading-text {
  font-size: 16px;
  margin: 1rem 0 0.5rem 0;
  color: var(--text-primary, #333);
}

.loading-subtext {
  font-size: 14px;
  color: var(--text-secondary, #666);
  margin: 0.5rem 0 1.5rem 0;
}

.progress-bar {
  width: 100%;
  max-width: 300px;
  height: 8px;
  background: var(--border-light, #e0e0e0);
  border-radius: 4px;
  overflow: hidden;
  margin: 0 auto;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
  transition: width 0.3s ease;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #e0e0e0;
  border-top: 4px solid #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 1rem;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* Error State */
.error-state {
  padding: 3rem;
  background: #ffebee;
  border-radius: 8px;
  text-align: center;
  color: #c62828;
}

.error-state p {
  margin-bottom: 1.5rem;
}

.error-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
}

/* Report Header */
.report-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 2rem;
  border-radius: 8px;
  margin-bottom: 2rem;
}

.report-header h2 {
  margin: 0 0 1rem 0;
  font-size: 28px;
}

.report-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 2rem;
}

.meta-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.meta-item label {
  font-size: 12px;
  opacity: 0.8;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.meta-value {
  font-size: 16px;
  font-weight: 500;
}

.score-badge {
  padding: 1rem 1.5rem;
  border-radius: 8px;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background: rgba(255, 255, 255, 0.2);
}

.score-value {
  font-size: 32px;
  font-weight: bold;
}

.score-label {
  font-size: 14px;
  opacity: 0.8;
}

.score-grade {
  margin-left: 0.5rem;
  padding: 0.25rem 0.75rem;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 4px;
  font-weight: bold;
  font-size: 14px;
}

/* Report Content */
.report-content {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.report-section {
  padding: 2rem;
  border-bottom: 1px solid #f0f0f0;
}

.report-section:last-of-type {
  border-bottom: none;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  color: #333;
  margin: 0 0 1rem 0;
  padding-bottom: 0.5rem;
  border-bottom: 2px solid #667eea;
}

.section-content {
  color: #666;
  line-height: 1.8;
}

.section-content p {
  margin: 0;
  text-align: justify;
}

.improvements-list,
.takeaways-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.improvements-list li,
.takeaways-list li {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  padding: 0.75rem 0;
  margin: 0;
}

.list-icon {
  color: #667eea;
  font-weight: bold;
  flex-shrink: 0;
  font-size: 16px;
}

/* Actions */
.report-actions {
  padding: 2rem;
  background: #f9f9f9;
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
  justify-content: center;
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-text {
  background: #667eea;
  color: white;
}

.btn-text:hover:not(:disabled) {
  background: #5568d3;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.btn-html {
  background: #764ba2;
  color: white;
}

.btn-html:hover:not(:disabled) {
  background: #65408a;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(118, 75, 162, 0.4);
}

.btn-print {
  background: #F5AF19;
  color: white;
}

.btn-print:hover:not(:disabled) {
  background: #d99415;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(245, 175, 25, 0.4);
}

.btn-retry {
  background: #ff9800;
  color: white;
}

.btn-retry:hover:not(:disabled) {
  background: #e68900;
  transform: translateY(-2px);
}

.btn-home {
  background: #607d8b;
  color: white;
}

.btn-home:hover:not(:disabled) {
  background: #546e7a;
  transform: translateY(-2px);
}

/* Print Styles */
@media print {
  .report-actions,
  .loading-state,
  .error-state {
    display: none;
  }

  .report-view {
    max-width: 100%;
    padding: 0;
  }

  .report-header {
    page-break-after: avoid;
  }

  .report-section {
    page-break-inside: avoid;
  }
}
</style>
