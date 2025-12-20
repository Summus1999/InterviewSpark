<template>
  <div class="star-score-container">
    <div class="star-header">
      <h3>STAR 法则评分</h3>
      <div class="overall-score">
        <span class="score-label">综合评分</span>
        <span class="score-value" :class="getScoreClass(starScore.overall_score)">
          {{ starScore.overall_score.toFixed(1) }}
        </span>
        <span class="score-max">/10</span>
      </div>
    </div>

    <div class="completeness-bar">
      <div class="completeness-label">结构完整度</div>
      <div class="bar-container">
        <div class="bar-fill" :style="{ width: `${starScore.completeness}%` }">
          {{ starScore.completeness.toFixed(0) }}%
        </div>
      </div>
    </div>

    <div class="star-radar">
      <div ref="radarChart" class="chart-container"></div>
    </div>

    <div class="star-breakdown">
      <h4>维度评分</h4>
      <div class="dimension-list">
        <div class="dimension-item">
          <div class="dimension-header">
            <span class="dimension-name">📍 Situation (情境)</span>
            <span class="dimension-score" :class="getScoreClass(starScore.breakdown.situation)">
              {{ starScore.breakdown.situation.toFixed(1) }}
            </span>
          </div>
          <div class="dimension-desc">背景描述：说明当时的环境、团队或项目情况</div>
        </div>

        <div class="dimension-item">
          <div class="dimension-header">
            <span class="dimension-name">🎯 Task (任务)</span>
            <span class="dimension-score" :class="getScoreClass(starScore.breakdown.task)">
              {{ starScore.breakdown.task.toFixed(1) }}
            </span>
          </div>
          <div class="dimension-desc">任务目标：明确需要完成什么、面临什么挑战</div>
        </div>

        <div class="dimension-item">
          <div class="dimension-header">
            <span class="dimension-name">⚡ Action (行动)</span>
            <span class="dimension-score" :class="getScoreClass(starScore.breakdown.action)">
              {{ starScore.breakdown.action.toFixed(1) }}
            </span>
          </div>
          <div class="dimension-desc">行动步骤：详细说明采取的具体措施和执行过程</div>
        </div>

        <div class="dimension-item">
          <div class="dimension-header">
            <span class="dimension-name">🏆 Result (结果)</span>
            <span class="dimension-score" :class="getScoreClass(starScore.breakdown.result)">
              {{ starScore.breakdown.result.toFixed(1) }}
            </span>
          </div>
          <div class="dimension-desc">成果量化：用数据说明效果和达成的成果</div>
        </div>
      </div>
    </div>

    <div class="star-suggestions">
      <h4>改进建议</h4>
      <ul class="suggestion-list">
        <li v-for="(suggestion, index) in starScore.suggestions" :key="index">
          {{ suggestion }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, nextTick } from 'vue'
import * as echarts from 'echarts'
import type { STARScoringResult } from '../services/database'

interface Props {
  starScore: STARScoringResult
}

const props = defineProps<Props>()
const radarChart = ref<HTMLElement | null>(null)
let chartInstance: echarts.ECharts | null = null

onMounted(() => {
  initChart()
})

watch(() => props.starScore, () => {
  nextTick(() => {
    updateChart()
  })
}, { deep: true })

function initChart() {
  if (!radarChart.value) return

  chartInstance = echarts.init(radarChart.value)
  updateChart()
}

function updateChart() {
  if (!chartInstance) return

  const option = {
    radar: {
      indicator: [
        { name: 'Situation', max: 10 },
        { name: 'Task', max: 10 },
        { name: 'Action', max: 10 },
        { name: 'Result', max: 10 }
      ],
      axisName: {
        color: '#666',
        fontSize: 12
      },
      splitArea: {
        areaStyle: {
          color: ['rgba(102, 126, 234, 0.05)', 'rgba(102, 126, 234, 0.1)']
        }
      }
    },
    series: [{
      type: 'radar',
      data: [{
        value: [
          props.starScore.breakdown.situation,
          props.starScore.breakdown.task,
          props.starScore.breakdown.action,
          props.starScore.breakdown.result
        ],
        name: 'STAR Score',
        areaStyle: {
          color: 'rgba(102, 126, 234, 0.3)'
        },
        lineStyle: {
          color: 'rgba(102, 126, 234, 1)',
          width: 2
        },
        itemStyle: {
          color: 'rgba(102, 126, 234, 1)'
        }
      }]
    }]
  }

  chartInstance.setOption(option)
}

function getScoreClass(score: number): string {
  if (score >= 8) return 'excellent'
  if (score >= 6) return 'good'
  if (score >= 4) return 'fair'
  return 'poor'
}
</script>

<style scoped>
.star-score-container {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 1.5rem;
  margin-top: 1.5rem;
  border: 1px solid var(--border-light);
}

.star-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
  padding-bottom: 1rem;
  border-bottom: 2px solid var(--border-light);
}

.star-header h3 {
  margin: 0;
  color: var(--text-primary);
  font-size: 1.3rem;
}

.overall-score {
  display: flex;
  align-items: baseline;
  gap: 0.5rem;
}

.score-label {
  font-size: 0.9rem;
  color: var(--text-light);
}

.score-value {
  font-size: 2rem;
  font-weight: bold;
}

.score-value.excellent {
  color: #10b981;
}

.score-value.good {
  color: #3b82f6;
}

.score-value.fair {
  color: #f59e0b;
}

.score-value.poor {
  color: #ef4444;
}

.score-max {
  font-size: 1.2rem;
  color: var(--text-muted);
}

.completeness-bar {
  margin-bottom: 1.5rem;
}

.completeness-label {
  font-size: 0.9rem;
  color: var(--text-light);
  margin-bottom: 0.5rem;
}

.bar-container {
  background: var(--bg-secondary);
  border-radius: 8px;
  height: 32px;
  overflow: hidden;
  position: relative;
}

.bar-fill {
  background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: 600;
  font-size: 0.9rem;
  transition: width 0.5s ease;
  min-width: 50px;
}

.star-radar {
  margin-bottom: 1.5rem;
}

.chart-container {
  width: 100%;
  height: 300px;
}

.star-breakdown h4 {
  margin: 0 0 1rem 0;
  color: var(--text-primary);
  font-size: 1.1rem;
}

.dimension-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.dimension-item {
  background: var(--bg-secondary);
  padding: 1rem;
  border-radius: 8px;
  border-left: 4px solid var(--accent-primary);
}

.dimension-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.dimension-name {
  font-weight: 600;
  color: var(--text-primary);
  font-size: 1rem;
}

.dimension-score {
  font-size: 1.3rem;
  font-weight: bold;
}

.dimension-desc {
  font-size: 0.85rem;
  color: var(--text-light);
  line-height: 1.4;
}

.star-suggestions {
  margin-top: 1.5rem;
  padding-top: 1.5rem;
  border-top: 2px solid var(--border-light);
}

.star-suggestions h4 {
  margin: 0 0 1rem 0;
  color: var(--text-primary);
  font-size: 1.1rem;
}

.suggestion-list {
  margin: 0;
  padding-left: 1.5rem;
  list-style: none;
}

.suggestion-list li {
  position: relative;
  padding-left: 1.5rem;
  margin-bottom: 0.8rem;
  color: var(--text-light);
  line-height: 1.6;
}

.suggestion-list li::before {
  content: '💡';
  position: absolute;
  left: 0;
  top: 0;
}

@media (max-width: 768px) {
  .star-score-container {
    padding: 1rem;
  }

  .star-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }

  .chart-container {
    height: 250px;
  }
}
</style>
