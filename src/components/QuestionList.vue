<!--
  QuestionList.vue - Interview questions display component
  
  Features:
  - Display list of AI-generated interview questions
  - Highlight currently selected question
  - Click to select a question
  - Empty state when no questions available
  
  Props:
  - questions: string[] - Array of interview questions
  - currentIndex?: number - Index of currently active question
  
  Emits:
  - selectQuestion: Fired when user clicks on a question (emits question index)
-->
<template>
  <div class="question-list">
    <h3>面试问题列表</h3>
    <div v-if="questions.length === 0" class="empty-state">
      <p>还没有生成问题，请先填写简历和岗位描述</p>
    </div>
    <div v-else class="questions">
      <div
        v-for="(question, index) in questions"
        :key="index"
        class="question-item"
        :class="{ active: index === currentIndex }"
        @click="$emit('selectQuestion', index)"
      >
        <div class="question-number">Q{{ index + 1 }}</div>
        <div class="question-text">{{ question }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Interview questions list component
 * Displays AI-generated questions with selection support
 */
defineProps<{
  questions: string[]
  currentIndex?: number
}>()

defineEmits<{
  (e: 'selectQuestion', index: number): void
}>()
</script>

<style scoped>
.question-list {
  margin: 2rem 0;
}

h3 {
  margin: 0 0 1.5rem;
  font-size: 1.2rem;
  color: #333;
}

.empty-state {
  padding: 3rem 2rem;
  text-align: center;
  background: #f5f5f5;
  border-radius: 8px;
  color: #999;
}

.questions {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.question-item {
  display: flex;
  gap: 1rem;
  padding: 1.2rem;
  background: white;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
}

.question-item:hover {
  border-color: #667eea;
  transform: translateX(4px);
}

.question-item.active {
  border-color: #667eea;
  background: #f8f9ff;
}

.question-number {
  flex-shrink: 0;
  width: 2.5rem;
  height: 2.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #667eea;
  color: white;
  border-radius: 50%;
  font-weight: 600;
  font-size: 0.9rem;
}

.question-item.active .question-number {
  background: #764ba2;
}

.question-text {
  flex: 1;
  font-size: 1rem;
  line-height: 1.6;
  color: #333;
}
</style>
