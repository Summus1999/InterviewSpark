<template>
  <div class="followup-panel">
    <div class="panel-header">
      <h3>🔍 AI 追问建议</h3>
      <span class="quality-badge" :class="analysis.answerQuality">
        回答质量: {{ getQualityLabel(analysis.answerQuality) }}
      </span>
    </div>

    <div class="analysis-reasoning">
      <p>{{ analysis.reasoning }}</p>
    </div>

    <div v-if="analysis.shouldFollowUp && analysis.followUpQuestions.length > 0" class="followup-questions">
      <h4>建议追问（{{ analysis.followUpQuestions.length }} 个）</h4>
      
      <div class="questions-list">
        <div 
          v-for="(fq, index) in analysis.followUpQuestions" 
          :key="index"
          class="question-card"
        >
          <div class="question-header">
            <span class="question-type-badge" :class="fq.type">
              {{ FOLLOWUP_TYPE_LABELS[fq.type] }}
            </span>
            <span class="question-number">#{{ index + 1 }}</span>
          </div>
          
          <div class="question-content">
            <p class="question-text">{{ fq.question }}</p>
          </div>
          
          <div class="question-meta">
            <div class="meta-item">
              <strong>追问原因:</strong>
              <span>{{ fq.reason }}</span>
            </div>
            <div class="meta-item">
              <strong>背景:</strong>
              <span>{{ fq.context }}</span>
            </div>
          </div>

          <div class="question-actions">
            <button 
              @click="$emit('select', fq.question, fq.type)" 
              class="select-btn"
            >
              选择此问题
            </button>
          </div>
        </div>
      </div>

      <div class="panel-actions">
        <button @click="$emit('skip')" class="skip-btn">
          跳过追问
        </button>
        <button @click="$emit('custom')" class="custom-btn">
          自定义追问
        </button>
      </div>
    </div>

    <div v-else class="no-followup">
      <p>✅ 回答质量良好，无需追问</p>
      <button @click="$emit('skip')" class="continue-btn">
        继续下一题
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { FollowUpAnalysis, FollowUpType } from '../types/follow-up'
import { FOLLOWUP_TYPE_LABELS } from '../types/follow-up'

interface Props {
  analysis: FollowUpAnalysis
}

defineProps<Props>()

defineEmits<{
  select: [question: string, type: FollowUpType]
  skip: []
  custom: []
}>()

function getQualityLabel(quality: string): string {
  const labels: Record<string, string> = {
    excellent: '优秀',
    good: '良好',
    acceptable: '可接受',
    poor: '较差'
  }
  return labels[quality] || quality
}
</script>

<style scoped>
.followup-panel {
  background: var(--bg-card-solid);
  border-radius: 12px;
  padding: 2rem;
  border: 2px solid var(--accent-primary);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
  gap: 1rem;
}

.panel-header h3 {
  margin: 0;
  font-size: 1.3rem;
  color: var(--text-primary);
}

.quality-badge {
  padding: 0.4rem 1rem;
  border-radius: 20px;
  font-size: 0.85rem;
  font-weight: 600;
}

.quality-badge.excellent {
  background: var(--success-color);
  color: white;
}

.quality-badge.good {
  background: #4caf50;
  color: white;
}

.quality-badge.acceptable {
  background: var(--warning-color);
  color: white;
}

.quality-badge.poor {
  background: var(--error-color);
  color: white;
}

.analysis-reasoning {
  padding: 1rem;
  background: var(--bg-secondary);
  border-radius: 8px;
  border-left: 3px solid var(--accent-primary);
  margin-bottom: 1.5rem;
}

.analysis-reasoning p {
  margin: 0;
  color: var(--text-primary);
  line-height: 1.6;
}

.followup-questions h4 {
  margin: 0 0 1rem;
  font-size: 1.1rem;
  color: var(--text-primary);
}

.questions-list {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  margin-bottom: 1.5rem;
}

.question-card {
  padding: 1.5rem;
  background: var(--bg-secondary);
  border-radius: 10px;
  border: 1px solid var(--border-light);
  transition: all 0.3s;
}

.question-card:hover {
  border-color: var(--accent-primary);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(103, 126, 234, 0.2);
}

.question-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.question-type-badge {
  padding: 0.3rem 0.8rem;
  border-radius: 16px;
  font-size: 0.8rem;
  font-weight: 600;
  color: white;
}

.question-type-badge.clarification {
  background: #2196f3;
}

.question-type-badge.deepening {
  background: #9c27b0;
}

.question-type-badge.scenario {
  background: #ff9800;
}

.question-type-badge.challenge {
  background: #f44336;
}

.question-type-badge.extension {
  background: #4caf50;
}

.question-number {
  font-size: 0.85rem;
  color: var(--text-secondary);
  font-weight: 600;
}

.question-content {
  margin-bottom: 1rem;
}

.question-text {
  margin: 0;
  font-size: 1.05rem;
  font-weight: 500;
  color: var(--text-primary);
  line-height: 1.6;
}

.question-meta {
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
  margin-bottom: 1rem;
  padding: 1rem;
  background: var(--bg-input);
  border-radius: 6px;
}

.meta-item {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.meta-item strong {
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.meta-item span {
  font-size: 0.9rem;
  color: var(--text-primary);
  line-height: 1.5;
}

.question-actions {
  display: flex;
  justify-content: flex-end;
}

.select-btn {
  padding: 0.6rem 1.5rem;
  background: var(--accent-gradient);
  border: none;
  border-radius: 6px;
  color: white;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.select-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(103, 126, 234, 0.4);
}

.panel-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  padding-top: 1rem;
  border-top: 1px solid var(--border-light);
}

.skip-btn,
.custom-btn {
  padding: 0.8rem 1.5rem;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.skip-btn {
  background: var(--bg-input);
  border: 1px solid var(--border-light);
  color: var(--text-secondary);
}

.skip-btn:hover {
  background: var(--bg-hover);
  border-color: var(--border-hover);
}

.custom-btn {
  background: var(--bg-secondary);
  border: 2px solid var(--accent-primary);
  color: var(--accent-primary);
}

.custom-btn:hover {
  background: var(--accent-primary);
  color: white;
}

.no-followup {
  text-align: center;
  padding: 2rem;
}

.no-followup p {
  margin: 0 0 1.5rem;
  font-size: 1.1rem;
  color: var(--success-color);
}

.continue-btn {
  padding: 0.8rem 2rem;
  background: var(--success-color);
  border: none;
  border-radius: 6px;
  color: white;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.continue-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(76, 175, 80, 0.4);
}
</style>
