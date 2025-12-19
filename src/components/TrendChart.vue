<template>
  <div ref="chartContainer" class="trend-chart-container"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from 'vue'
import * as echarts from 'echarts'
import type { TrendDataPoint } from '../services/database'

const props = defineProps<{
  dataPoints: TrendDataPoint[]
  dimension: string
}>()

const chartContainer = ref<HTMLDivElement | null>(null)
let chartInstance: echarts.ECharts | null = null

const dimensionLabel = computed(() => {
  const labels: Record<string, string> = {
    overall: 'Overall Score',
    communication: 'Communication Score',
    problemSolving: 'Problem Solving Score',
    technicalDepth: 'Technical Depth Score',
    presentation: 'Presentation Score'
  }
  return labels[props.dimension] || 'Score'
})

const chartData = computed(() => {
  return props.dataPoints.map((point) => {
    const date = new Date(point.timestamp * 1000)
    const dateStr = date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric'
    })

    let score = point.overallScore
    if (props.dimension === 'communication') score = point.communicationScore
    else if (props.dimension === 'problemSolving') score = point.problemSolvingScore
    else if (props.dimension === 'technicalDepth') score = point.technicalDepthScore
    else if (props.dimension === 'presentation') score = point.presentationScore

    return {
      date: dateStr,
      score: score
    }
  })
})

const initChart = () => {
  if (!chartContainer.value) return

  chartInstance = echarts.init(chartContainer.value)
  updateChart()

  window.addEventListener('resize', handleResize)
}

const updateChart = () => {
  if (!chartInstance) return

  const dates = chartData.value.map((d) => d.date)
  const scores = chartData.value.map((d) => d.score)

  const option: echarts.EChartsOption = {
    title: {
      text: dimensionLabel.value,
      left: 'center',
      textStyle: {
        fontSize: 16,
        fontWeight: 'normal'
      }
    },
    tooltip: {
      trigger: 'axis',
      formatter: (params: any) => {
        const data = params[0]
        return `${data.name}<br/>${dimensionLabel.value}: ${data.value.toFixed(1)}`
      }
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '60px',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: dates,
      axisLabel: {
        rotate: 45,
        fontSize: 11
      }
    },
    yAxis: {
      type: 'value',
      min: 0,
      max: 100,
      axisLabel: {
        formatter: '{value}'
      }
    },
    dataZoom: [
      {
        type: 'inside',
        start: 0,
        end: 100
      },
      {
        start: 0,
        end: 100,
        height: 20,
        bottom: 10
      }
    ],
    series: [
      {
        name: dimensionLabel.value,
        type: 'line',
        smooth: true,
        symbol: 'circle',
        symbolSize: 8,
        lineStyle: {
          width: 3,
          color: '#3b82f6'
        },
        itemStyle: {
          color: '#3b82f6',
          borderWidth: 2,
          borderColor: '#fff'
        },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            {
              offset: 0,
              color: 'rgba(59, 130, 246, 0.3)'
            },
            {
              offset: 1,
              color: 'rgba(59, 130, 246, 0.05)'
            }
          ])
        },
        data: scores
      }
    ]
  }

  chartInstance.setOption(option)
}

const handleResize = () => {
  if (chartInstance) {
    chartInstance.resize()
  }
}

onMounted(() => {
  initChart()
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
  if (chartInstance) {
    chartInstance.dispose()
    chartInstance = null
  }
})

watch(
  () => [props.dataPoints, props.dimension],
  () => {
    updateChart()
  },
  { deep: true }
)
</script>

<style scoped>
.trend-chart-container {
  width: 100%;
  height: 400px;
  min-height: 300px;
}
</style>
