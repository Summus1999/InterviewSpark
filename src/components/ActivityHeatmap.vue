<template>
  <div class="activity-heatmap">
    <div class="header">
      <h3 class="title">练习活跃度</h3>
      <span class="total-count">过去一年: {{ totalCount }} 次练习</span>
    </div>

    <div class="heatmap-wrapper">
      <div class="day-labels">
        <span class="day-label"></span>
        <span class="day-label">一</span>
        <span class="day-label"></span>
        <span class="day-label">三</span>
        <span class="day-label"></span>
        <span class="day-label">五</span>
        <span class="day-label"></span>
      </div>
      
      <div class="grid-container">
        <div class="months-row">
          <span
            v-for="month in monthLabels"
            :key="month.index"
            class="month-label"
            :style="{ left: month.position + 'px' }"
          >
            {{ month.name }}
          </span>
        </div>
        
        <div class="grid">
          <div
            v-for="(week, weekIndex) in weekData"
            :key="weekIndex"
            class="week-column"
          >
            <div
              v-for="(cell, dayIndex) in week"
              :key="dayIndex"
              class="cell"
              :class="[getIntensityClass(cell.count), { empty: !cell.date }]"
              :title="getCellTitle(cell)"
            ></div>
          </div>
        </div>
      </div>
    </div>

    <div class="legend">
      <span class="legend-label">Less</span>
      <div class="legend-cell intensity-0"></div>
      <div class="legend-cell intensity-1"></div>
      <div class="legend-cell intensity-2"></div>
      <div class="legend-cell intensity-3"></div>
      <div class="legend-cell intensity-4"></div>
      <span class="legend-label">More</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import type { ActivityData } from '../services/database'

const props = defineProps<{
  data: ActivityData[]
}>()

const CELL_SIZE = 11
const CELL_GAP = 3
const WEEK_WIDTH = CELL_SIZE + CELL_GAP

interface Cell {
  date: string
  count: number
}

/**
 * Calculate total practice count
 */
const totalCount = computed(() => {
  return props.data.reduce((sum, item) => sum + item.count, 0)
})

/**
 * Generate week-based grid data (53 weeks x 7 days)
 */
const weekData = computed(() => {
  const weeks: Cell[][] = []
  const endDate = new Date()
  const startDate = new Date(endDate)
  startDate.setDate(startDate.getDate() - 364)

  // Build data map
  const dataMap = new Map<string, number>()
  props.data.forEach((item) => {
    dataMap.set(item.date, item.count)
  })

  // Adjust start to Sunday of that week
  const startDayOfWeek = startDate.getDay()
  startDate.setDate(startDate.getDate() - startDayOfWeek)

  let currentDate = new Date(startDate)
  let currentWeek: Cell[] = []

  while (currentDate <= endDate || currentWeek.length > 0) {
    const dateStr = formatDate(currentDate)
    const isWithinRange = currentDate >= startDate && currentDate <= endDate
    
    currentWeek.push({
      date: isWithinRange ? dateStr : '',
      count: isWithinRange ? (dataMap.get(dateStr) || 0) : 0
    })

    if (currentWeek.length === 7) {
      weeks.push(currentWeek)
      currentWeek = []
    }

    currentDate.setDate(currentDate.getDate() + 1)
    
    if (currentDate > endDate && currentWeek.length === 0) {
      break
    }
  }

  // Fill remaining days
  if (currentWeek.length > 0) {
    while (currentWeek.length < 7) {
      currentWeek.push({ date: '', count: 0 })
    }
    weeks.push(currentWeek)
  }

  return weeks
})

/**
 * Generate month labels with pixel positions
 */
const monthLabels = computed(() => {
  const labels: { name: string; position: number; index: number }[] = []
  const endDate = new Date()
  const startDate = new Date(endDate)
  startDate.setDate(startDate.getDate() - 364)
  
  // Adjust to Sunday
  const startDayOfWeek = startDate.getDay()
  startDate.setDate(startDate.getDate() - startDayOfWeek)

  const monthNames = ['1月', '2月', '3月', '4月', '5月', '6月', '7月', '8月', '9月', '10月', '11月', '12月']
  
  let currentDate = new Date(startDate)
  let weekIndex = 0
  let lastMonth = -1

  while (currentDate <= endDate) {
    const month = currentDate.getMonth()
    const isFirstDayOfWeek = currentDate.getDay() === 0
    
    if (month !== lastMonth && isFirstDayOfWeek) {
      labels.push({
        name: monthNames[month],
        position: weekIndex * WEEK_WIDTH,
        index: labels.length
      })
      lastMonth = month
    }

    if (isFirstDayOfWeek) {
      weekIndex++
    }

    currentDate.setDate(currentDate.getDate() + 1)
  }

  return labels
})

/**
 * Get intensity class based on count
 */
function getIntensityClass(count: number): string {
  if (count === 0) return 'intensity-0'
  if (count <= 2) return 'intensity-1'
  if (count <= 4) return 'intensity-2'
  if (count <= 6) return 'intensity-3'
  return 'intensity-4'
}

/**
 * Get cell tooltip title
 */
function getCellTitle(cell: Cell): string {
  if (!cell.date) return ''
  return `${cell.date}: ${cell.count} 次练习`
}

/**
 * Format date to YYYY-MM-DD
 */
function formatDate(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}
</script>

<style scoped>
.activity-heatmap {
  background: var(--bg-card, white);
  border-radius: 12px;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.title {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary, #111827);
  margin: 0;
}

.total-count {
  font-size: 0.85rem;
  color: var(--text-secondary, #6b7280);
  font-weight: 500;
}

.heatmap-wrapper {
  display: flex;
  gap: 8px;
}

.day-labels {
  display: flex;
  flex-direction: column;
  gap: 3px;
  padding-top: 20px;
}

.day-label {
  height: 11px;
  font-size: 10px;
  color: var(--text-secondary, #6b7280);
  line-height: 11px;
  text-align: right;
  width: 20px;
}

.grid-container {
  flex: 1;
  overflow-x: auto;
  position: relative;
}

.months-row {
  height: 16px;
  position: relative;
  margin-bottom: 4px;
}

.month-label {
  position: absolute;
  font-size: 10px;
  color: var(--text-secondary, #6b7280);
  white-space: nowrap;
}

.grid {
  display: flex;
  gap: 3px;
}

.week-column {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.cell {
  width: 11px;
  height: 11px;
  border-radius: 2px;
  cursor: default;
  transition: transform 0.1s;
}

.cell:not(.empty):hover {
  transform: scale(1.3);
  outline: 1px solid rgba(0, 0, 0, 0.2);
}

.cell.empty {
  background-color: transparent;
}

.intensity-0 {
  background-color: #ebedf0;
}

.intensity-1 {
  background-color: #9be9a8;
}

.intensity-2 {
  background-color: #40c463;
}

.intensity-3 {
  background-color: #30a14e;
}

.intensity-4 {
  background-color: #216e39;
}

/* Blue theme option */
.activity-heatmap.blue-theme .intensity-0 { background-color: #e8f4fc; }
.activity-heatmap.blue-theme .intensity-1 { background-color: #a8d4f0; }
.activity-heatmap.blue-theme .intensity-2 { background-color: #5eb3e4; }
.activity-heatmap.blue-theme .intensity-3 { background-color: #2196f3; }
.activity-heatmap.blue-theme .intensity-4 { background-color: #0d47a1; }

.legend {
  display: flex;
  align-items: center;
  gap: 3px;
  margin-top: 1rem;
  justify-content: flex-end;
  font-size: 10px;
  color: var(--text-secondary, #6b7280);
}

.legend-label {
  margin: 0 4px;
}

.legend-cell {
  width: 11px;
  height: 11px;
  border-radius: 2px;
}

@media (max-width: 768px) {
  .activity-heatmap {
    padding: 1rem;
  }

  .header {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .cell, .legend-cell {
    width: 9px;
    height: 9px;
  }

  .day-label {
    height: 9px;
    line-height: 9px;
    font-size: 9px;
  }
}
</style>
