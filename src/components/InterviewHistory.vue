<!--
  InterviewHistory.vue - Interview history records viewer
  
  Features:
  - Display all interview sessions
  - View session details and answers
  - Filter and search functionality
-->
<template>
  <div class="interview-history">
    <h3>面试历史记录</h3>
    
    <div v-if="sessions.length === 0" class="empty-state">
      <p>暂无面试记录</p>
    </div>
    
    <div v-else class="sessions-list">
      <div
        v-for="session in sessions"
        :key="session.id"
        class="session-card"
        @click="viewSession(session.id!)"
      >
        <div class="session-header">
          <span class="session-date">{{ formatDate(session.created_at) }}</span>
          <span class="question-count">{{ session.questions.length }} 个问题</span>
        </div>
        <div class="session-preview">
          <p>{{ session.questions[0] || '暂无问题' }}</p>
        </div>
      </div>
    </div>
    
    <!-- Session Detail Modal -->
    <div v-if="selectedSession" class="modal-overlay" @click="closeDetail">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h4>面试详情</h4>
          <button @click="closeDetail" class="close-btn">×</button>
        </div>
        
        <div class="modal-body">
          <div class="session-info">
            <p><strong>面试时间：</strong>{{ formatDate(selectedSession.created_at) }}</p>
            <p><strong>问题数量：</strong>{{ selectedSession.questions.length }}</p>
          </div>
          
          <div v-if="sessionAnswers.length > 0" class="answers-section">
            <h5>答题记录</h5>
            <div
              v-for="(answer, index) in sessionAnswers"
              :key="answer.id"
              class="answer-item"
            >
              <div class="question-text">
                <strong>Q{{ index + 1 }}:</strong> {{ answer.question }}
              </div>
              <div class="answer-text">
                <strong>回答:</strong> {{ answer.answer }}
              </div>
              <div class="feedback-text">
                <strong>反馈:</strong> {{ answer.feedback }}
              </div>
            </div>
          </div>
          <div v-else class="no-answers">
            <p>该面试会话暂无答题记录</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Interview history component
 * Displays past interview sessions and their answers
 */
import { ref, onMounted } from 'vue'
import type { InterviewSession, InterviewAnswer } from '../services/database'
import { getSessions, getSession, getAnswers } from '../services/database'

const sessions = ref<InterviewSession[]>([])
const selectedSession = ref<InterviewSession | null>(null)
const sessionAnswers = ref<InterviewAnswer[]>([])

onMounted(async () => {
  await loadSessions()
})

const loadSessions = async () => {
  try {
    sessions.value = await getSessions()
  } catch (error) {
    console.error('Failed to load sessions:', error)
  }
}

const viewSession = async (sessionId: number) => {
  try {
    selectedSession.value = await getSession(sessionId)
    if (selectedSession.value) {
      sessionAnswers.value = await getAnswers(sessionId)
    }
  } catch (error) {
    console.error('Failed to load session details:', error)
  }
}

const closeDetail = () => {
  selectedSession.value = null
  sessionAnswers.value = []
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
</script>

<style scoped>
.interview-history {
  padding: 2rem 0;
}

h3 {
  margin: 0 0 1.5rem;
  font-size: 1.3rem;
  color: #333;
}

.empty-state {
  padding: 3rem;
  text-align: center;
  background: #f5f5f5;
  border-radius: 8px;
  color: #999;
}

.sessions-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.session-card {
  padding: 1.2rem;
  background: white;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
}

.session-card:hover {
  border-color: #667eea;
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.session-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.8rem;
}

.session-date {
  font-weight: 600;
  color: #667eea;
}

.question-count {
  font-size: 0.9rem;
  color: #666;
}

.session-preview {
  color: #666;
  font-size: 0.95rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: white;
  border-radius: 12px;
  width: 90%;
  max-width: 800px;
  max-height: 80vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #e0e0e0;
}

.modal-header h4 {
  margin: 0;
  font-size: 1.3rem;
  color: #333;
}

.close-btn {
  background: none;
  border: none;
  font-size: 2rem;
  color: #999;
  cursor: pointer;
  line-height: 1;
  padding: 0;
  width: 2rem;
  height: 2rem;
}

.close-btn:hover {
  color: #333;
}

.modal-body {
  padding: 1.5rem;
  overflow-y: auto;
}

.session-info {
  margin-bottom: 1.5rem;
  padding: 1rem;
  background: #f8f9ff;
  border-radius: 6px;
}

.session-info p {
  margin: 0.5rem 0;
  color: #333;
}

.answers-section h5 {
  margin: 0 0 1rem;
  font-size: 1.1rem;
  color: #333;
}

.answer-item {
  margin-bottom: 1.5rem;
  padding: 1rem;
  background: #fafafa;
  border-left: 3px solid #667eea;
  border-radius: 4px;
}

.question-text,
.answer-text,
.feedback-text {
  margin: 0.8rem 0;
  line-height: 1.6;
  color: #333;
}

.question-text {
  color: #667eea;
  font-weight: 500;
}

.no-answers {
  padding: 2rem;
  text-align: center;
  color: #999;
}
</style>
