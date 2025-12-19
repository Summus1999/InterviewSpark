<template>
  <div class="conversation-history">
    <div class="history-header">
      <h4>💬 对话历史</h4>
      <button v-if="turns.length > 0" @click="$emit('clear')" class="clear-btn">
        清空
      </button>
    </div>

    <div v-if="turns.length === 0" class="empty-state">
      <p>暂无对话记录</p>
    </div>

    <div v-else class="history-list">
      <div 
        v-for="(turn, index) in turns" 
        :key="index"
        :class="['turn-item', turn.role]"
      >
        <div class="turn-header">
          <span class="role-badge">
            {{ turn.role === 'interviewer' ? '👤 面试官' : '🙋 候选人' }}
          </span>
          <span class="timestamp">{{ formatTime(turn.timestamp) }}</span>
          <span v-if="turn.questionType" class="question-type">
            {{ getQuestionTypeLabel(turn.questionType) }}
          </span>
        </div>
        <div class="turn-content">
          {{ turn.content }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ConversationTurn, FollowUpType } from '../types/follow-up'
import { FOLLOWUP_TYPE_LABELS } from '../types/follow-up'

interface Props {
  turns: ConversationTurn[]
}

defineProps<Props>()

defineEmits<{
  clear: []
}>()

function formatTime(timestamp: number): string {
  const date = new Date(timestamp)
  return date.toLocaleTimeString('zh-CN', { 
    hour: '2-digit', 
    minute: '2-digit',
    second: '2-digit'
  })
}

function getQuestionTypeLabel(type: FollowUpType): string {
  return FOLLOWUP_TYPE_LABELS[type] || type
}
</script>

<style scoped>
.conversation-history {
  background: var(--bg-card-solid);
  border-radius: 10px;
  padding: 1.5rem;
  border: 1px solid var(--border-light);
  max-height: 500px;
  display: flex;
  flex-direction: column;
}

.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  padding-bottom: 0.8rem;
  border-bottom: 1px solid var(--border-light);
}

.history-header h4 {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-primary);
}

.clear-btn {
  padding: 0.4rem 0.8rem;
  background: var(--bg-input);
  border: 1px solid var(--border-light);
  border-radius: 6px;
  color: var(--text-secondary);
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.3s;
}

.clear-btn:hover {
  background: var(--error-color);
  border-color: var(--error-color);
  color: white;
}

.empty-state {
  text-align: center;
  padding: 2rem;
  color: var(--text-secondary);
}

.empty-state p {
  margin: 0;
  font-size: 0.95rem;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  overflow-y: auto;
  padding-right: 0.5rem;
}

.turn-item {
  padding: 1rem;
  border-radius: 8px;
  border: 1px solid var(--border-light);
  transition: all 0.3s;
}

.turn-item.interviewer {
  background: linear-gradient(135deg, rgba(103, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
  border-left: 3px solid var(--accent-primary);
}

.turn-item.candidate {
  background: rgba(76, 175, 80, 0.05);
  border-left: 3px solid var(--success-color);
}

.turn-header {
  display: flex;
  align-items: center;
  gap: 0.8rem;
  margin-bottom: 0.8rem;
  flex-wrap: wrap;
}

.role-badge {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--text-primary);
}

.timestamp {
  font-size: 0.75rem;
  color: var(--text-secondary);
  font-family: 'Monaco', monospace;
}

.question-type {
  padding: 0.2rem 0.6rem;
  background: var(--accent-primary);
  color: white;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 600;
}

.turn-content {
  color: var(--text-primary);
  line-height: 1.6;
  font-size: 0.95rem;
  white-space: pre-wrap;
  word-break: break-word;
}

/* Scrollbar styling */
.history-list::-webkit-scrollbar {
  width: 6px;
}

.history-list::-webkit-scrollbar-track {
  background: var(--bg-input);
  border-radius: 3px;
}

.history-list::-webkit-scrollbar-thumb {
  background: var(--border-light);
  border-radius: 3px;
}

.history-list::-webkit-scrollbar-thumb:hover {
  background: var(--accent-primary);
}
</style>
