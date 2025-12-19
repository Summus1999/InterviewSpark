<!--
  IndustryComparison.vue - Industry benchmark comparison
  
  Features:
  - Compare user scores to industry averages
  - Show percentile rankings
  - Display user level
-->
<template>
  <div class="industry-comparison">
    <div class="header">
      <h4>行业水平对比</h4>
      <button @click="loadComparison" class="refresh-btn">刷新</button>
    </div>

    <div v-if="loading" class="loading">分析中...</div>

    <div v-else-if="result && result.benchmarks.length > 0" class="comparison-content">
      <!-- Overall Summary -->
      <div class="summary-card">
        <div class="level-badge" :class="getLevelClass(result.user_level)">
          {{ result.user_level }}
        </div>
        <div class="percentile-info">
          <span class="percentile-value">Top {{ (100 - result.overall_percentile).toFixed(0) }}%</span>
          <span class="percentile-label">超越了 {{ result.overall_percentile.toFixed(0) }}% 的面试者</span>
        </div>
        <div class="session-count">
          基于 {{ result.comparison_count }} 次面试数据
        </div>
      </div>

      <!-- Dimension Benchmarks -->
      <div class="benchmarks-list">
        <div
          v-for="bench in result.benchmarks"
          :key="bench.dimension"
          class="benchmark-item"
        >
          <div class="benchmark-header">
            <span class="dimension-name">{{ dimensionName(bench.dimension) }}</span>
            <span class="percentile-tag">
              Top {{ (100 - bench.percentile).toFixed(0) }}%
            </span>
          </div>
          
          <div class="benchmark-bar">
            <div class="bar-track">
              <!-- Industry Average Marker -->
              <div 
                class="marker avg-marker"
                :style="{ left: bench.industry_avg + '%' }"
                title="行业平均"
              >
                <span class="marker-label">均</span>
              </div>
              <!-- Industry Top Marker -->
              <div 
                class="marker top-marker"
                :style="{ left: bench.industry_top + '%' }"
                title="行业Top"
              >
                <span class="marker-label">优</span>
              </div>
              <!-- User Score Fill -->
              <div 
                class="user-fill"
                :style="{ width: bench.user_score + '%' }"
              />
              <!-- User Score Indicator -->
              <div 
                class="user-indicator"
                :style="{ left: bench.user_score + '%' }"
              >
                <span class="user-score">{{ bench.user_score.toFixed(0) }}</span>
              </div>
            </div>
          </div>
          
          <div class="benchmark-legend">
            <span>你: {{ bench.user_score.toFixed(1) }}</span>
            <span>均值: {{ bench.industry_avg.toFixed(1) }}</span>
            <span>Top: {{ bench.industry_top.toFixed(1) }}</span>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="empty-state">
      <p>暂无数据，完成更多面试练习后可查看行业对比</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { IndustryComparisonResult } from '../services/database'
import { generateIndustryComparison } from '../services/database'

const result = ref<IndustryComparisonResult | null>(null)
const loading = ref(true)

const dimensionNames: Record<string, string> = {
  technical_depth: '技术深度',
  communication: '沟通表达',
  problem_solving: '问题解决',
  domain_knowledge: '领域知识',
  adaptability: '应变能力'
}

onMounted(async () => {
  await loadComparison()
})

const loadComparison = async () => {
  loading.value = true
  try {
    result.value = await generateIndustryComparison()
  } catch (error) {
    console.error('Failed to load comparison:', error)
  } finally {
    loading.value = false
  }
}

const dimensionName = (key: string): string => {
  return dimensionNames[key] || key
}

const getLevelClass = (level: string): string => {
  const levelMap: Record<string, string> = {
    '入门级': 'level-beginner',
    '初级': 'level-junior',
    '中级': 'level-mid',
    '高级': 'level-senior',
    '专家级': 'level-expert'
  }
  return levelMap[level] || 'level-beginner'
}
</script>

<style scoped>
.industry-comparison {
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

.summary-card {
  padding: 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 12px;
  color: white;
  text-align: center;
  margin-bottom: 2rem;
}

.level-badge {
  display: inline-block;
  padding: 0.5rem 1.5rem;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 20px;
  font-size: 1.2rem;
  font-weight: 600;
  margin-bottom: 1rem;
}

.level-beginner { background: rgba(160, 174, 192, 0.3); }
.level-junior { background: rgba(72, 187, 120, 0.3); }
.level-mid { background: rgba(66, 153, 225, 0.3); }
.level-senior { background: rgba(237, 137, 54, 0.3); }
.level-expert { background: rgba(229, 62, 62, 0.3); }

.percentile-info {
  margin-bottom: 0.8rem;
}

.percentile-value {
  display: block;
  font-size: 2rem;
  font-weight: 700;
}

.percentile-label {
  font-size: 0.9rem;
  opacity: 0.9;
}

.session-count {
  font-size: 0.85rem;
  opacity: 0.8;
}

.benchmarks-list {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.benchmark-item {
  padding: 1rem;
  background: var(--bg-secondary, #f8f9ff);
  border-radius: 8px;
}

.benchmark-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.8rem;
}

.dimension-name {
  font-weight: 500;
  color: var(--text-primary, #333);
}

.percentile-tag {
  padding: 0.2rem 0.6rem;
  background: #667eea;
  color: white;
  border-radius: 10px;
  font-size: 0.75rem;
}

.benchmark-bar {
  margin-bottom: 0.5rem;
}

.bar-track {
  position: relative;
  height: 24px;
  background: #e0e0e0;
  border-radius: 12px;
  overflow: visible;
}

.user-fill {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  background: linear-gradient(90deg, #667eea, #764ba2);
  border-radius: 12px;
  transition: width 0.5s;
}

.marker {
  position: absolute;
  top: -6px;
  transform: translateX(-50%);
  width: 2px;
  height: 36px;
  z-index: 1;
}

.avg-marker {
  background: #ed8936;
}

.top-marker {
  background: #48bb78;
}

.marker-label {
  position: absolute;
  top: -18px;
  left: 50%;
  transform: translateX(-50%);
  font-size: 0.65rem;
  font-weight: 600;
  padding: 0.1rem 0.3rem;
  border-radius: 3px;
}

.avg-marker .marker-label {
  background: #ed8936;
  color: white;
}

.top-marker .marker-label {
  background: #48bb78;
  color: white;
}

.user-indicator {
  position: absolute;
  top: 50%;
  transform: translate(-50%, -50%);
  z-index: 2;
}

.user-score {
  display: inline-block;
  padding: 0.15rem 0.4rem;
  background: white;
  border: 2px solid #667eea;
  border-radius: 10px;
  font-size: 0.75rem;
  font-weight: 600;
  color: #667eea;
}

.benchmark-legend {
  display: flex;
  justify-content: space-between;
  font-size: 0.8rem;
  color: var(--text-secondary, #666);
}
</style>
