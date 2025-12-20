<!--
  ProfileView.vue - Interview profile visualization
  
  Features:
  - Display dimension scores in radar chart
  - Show strongest/weakest areas
  - List improvement suggestions
-->
<template>
  <div class="profile-view">
    <div class="profile-header">
      <h3>个人面试画像</h3>
      <button @click="loadProfile" class="refresh-btn">刷新数据</button>
    </div>

    <div v-if="loading" class="loading">加载中...</div>

    <div v-else-if="profile" class="profile-content">
      <!-- Summary Stats -->
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-label">总面试次数</div>
          <div class="stat-value">{{ profile.total_sessions }}</div>
        </div>
        <div class="stat-card">
          <div class="stat-label">平均得分</div>
          <div class="stat-value">{{ profile.average_score.toFixed(1) }}</div>
        </div>
        <div class="stat-card highlight">
          <div class="stat-label">优势维度</div>
          <div class="stat-value">{{ dimensionName(profile.strongest_dimension) }}</div>
        </div>
        <div class="stat-card weak">
          <div class="stat-label">待提升</div>
          <div class="stat-value">{{ dimensionName(profile.weakest_dimension) }}</div>
        </div>
      </div>

      <!-- Radar Chart -->
      <div class="chart-container">
        <div ref="chartRef" class="radar-chart"></div>
      </div>

      <!-- Dimension Scores -->
      <div class="dimensions-list">
        <h4>维度得分详情</h4>
        <div
          v-for="[key, value] in Object.entries(profile.dimensions)"
          :key="key"
          class="dimension-item"
        >
          <span class="dimension-name">{{ dimensionName(key) }}</span>
          <div class="dimension-bar">
            <div 
              class="dimension-fill" 
              :style="{ width: value + '%', backgroundColor: getDimensionColor(value) }"
            ></div>
          </div>
          <span class="dimension-score">{{ value.toFixed(1) }}</span>
        </div>
      </div>

      <!-- Improvement Suggestions -->
      <div class="suggestions">
        <h4>改进建议</h4>
        <ul class="suggestions-list">
          <li v-for="(suggestion, index) in profile.improvement_suggestions" :key="index">
            {{ suggestion }}
          </li>
        </ul>
      </div>
    </div>

    <div v-else class="empty-state">
      <p>暂无数据，请先完成面试练习</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { InterviewProfile } from '../services/database'
import { generateInterviewProfile } from '../services/database'
import * as echarts from 'echarts'

const profile = ref<InterviewProfile | null>(null)
const loading = ref(true)
const chartRef = ref<HTMLElement | null>(null)
let chartInstance: echarts.ECharts | null = null

const dimensionNames: Record<string, string> = {
  technical_depth: '技术深度',
  communication: '沟通表达',
  problem_solving: '问题解决',
  domain_knowledge: '领域知识',
  adaptability: '应变能力',
  job_intention: '求职意向',
  none: '无',
  unknown: '未知'
}

onMounted(async () => {
  await loadProfile()
})

// Watch for profile changes and render chart when data is available
watch([profile, chartRef], async ([newProfile, newChartRef]) => {
  if (newProfile && newChartRef) {
    await nextTick()
    renderChart()
  }
}, { flush: 'post' })

const loadProfile = async () => {
  loading.value = true
  try {
    // First, analyze any answers that don't have analysis records
    const analyzed = await invoke<number>('analyze_missing_answers')
    console.log('Analyzed missing answers:', analyzed)
    
    // Then generate profile
    profile.value = await generateInterviewProfile()
    console.log('Profile loaded:', profile.value)
  } catch (error) {
    console.error('Failed to load profile:', error)
  } finally {
    loading.value = false
    // Ensure chart is rendered after DOM update
    await nextTick()
    // Use setTimeout to ensure DOM is fully painted
    setTimeout(() => {
      renderChart()
    }, 100)
  }
}

const renderChart = () => {
  console.log('renderChart called, chartRef:', chartRef.value, 'profile:', profile.value)
  
  if (!chartRef.value) {
    console.error('chartRef is null, cannot render chart')
    return
  }
  
  if (!profile.value) {
    console.error('profile is null, cannot render chart')
    return
  }

  if (chartInstance) {
    chartInstance.dispose()
  }

  chartInstance = echarts.init(chartRef.value)
  console.log('ECharts instance created')

  const option = {
    radar: {
      indicator: [
        { name: '技术深度', max: 100 },
        { name: '沟通表达', max: 100 },
        { name: '问题解决', max: 100 },
        { name: '领域知识', max: 100 },
        { name: '应变能力', max: 100 },
        { name: '求职意向', max: 100 }
      ],
      radius: '60%'
    },
    series: [{
      type: 'radar',
      data: [{
        value: [
          profile.value.dimensions.technical_depth,
          profile.value.dimensions.communication,
          profile.value.dimensions.problem_solving,
          profile.value.dimensions.domain_knowledge,
          profile.value.dimensions.adaptability,
          profile.value.dimensions.job_intention
        ],
        name: '能力维度',
        areaStyle: {
          color: 'rgba(102, 126, 234, 0.3)'
        },
        lineStyle: {
          color: '#667eea',
          width: 2
        },
        itemStyle: {
          color: '#667eea'
        }
      }]
    }]
  }

  console.log('Chart option:', option.series[0].data[0].value)
  chartInstance.setOption(option)
  console.log('Chart rendered successfully')
}

const dimensionName = (key: string): string => {
  return dimensionNames[key] || key
}

const getDimensionColor = (score: number): string => {
  if (score >= 70) return '#48bb78'
  if (score >= 50) return '#ed8936'
  return '#f56565'
}
</script>

<style scoped>
.profile-view {
  padding: 2rem 0;
}

.profile-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.profile-header h3 {
  margin: 0;
  font-size: 1.3rem;
  color: #333;
}

.refresh-btn {
  padding: 0.5rem 1rem;
  background: #667eea;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
}

.loading,
.empty-state {
  padding: 3rem;
  text-align: center;
  color: #999;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
  padding: 1.5rem;
  background: white;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  text-align: center;
}

.stat-card.highlight {
  border-color: #48bb78;
  background: #f0fff4;
}

.stat-card.weak {
  border-color: #ed8936;
  background: #fffaf0;
}

.stat-label {
  font-size: 0.9rem;
  color: #666;
  margin-bottom: 0.5rem;
}

.stat-value {
  font-size: 1.8rem;
  font-weight: 700;
  color: #333;
}

.chart-container {
  margin: 2rem 0;
  padding: 2rem;
  background: white;
  border-radius: 8px;
  border: 2px solid #e0e0e0;
}

.radar-chart {
  width: 100%;
  height: 400px;
}

.dimensions-list {
  margin: 2rem 0;
  padding: 1.5rem;
  background: white;
  border-radius: 8px;
  border: 2px solid #e0e0e0;
}

.dimensions-list h4 {
  margin: 0 0 1.5rem;
  font-size: 1.1rem;
  color: #333;
}

.dimension-item {
  display: grid;
  grid-template-columns: 120px 1fr 60px;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1rem;
}

.dimension-name {
  font-size: 0.95rem;
  color: #555;
}

.dimension-bar {
  height: 20px;
  background: #f0f0f0;
  border-radius: 10px;
  overflow: hidden;
}

.dimension-fill {
  height: 100%;
  transition: width 0.3s;
}

.dimension-score {
  font-weight: 600;
  color: #333;
  text-align: right;
}

.suggestions {
  padding: 1.5rem;
  background: #f8f9ff;
  border-radius: 8px;
  border: 2px solid #e0e0e0;
}

.suggestions h4 {
  margin: 0 0 1rem;
  font-size: 1.1rem;
  color: #333;
}

.suggestions-list {
  margin: 0;
  padding-left: 1.5rem;
}

.suggestions-list li {
  margin-bottom: 0.8rem;
  line-height: 1.6;
  color: #555;
}
</style>
