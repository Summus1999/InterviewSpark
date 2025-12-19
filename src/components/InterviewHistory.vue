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
    
    <!-- Action buttons -->
    <div class="action-bar">
      <button @click="openComparisonModal" class="btn btn-secondary" v-if="sessions.length > 0">
        📊 答案对比
      </button>
      <button @click="openBackupDialog" class="btn btn-secondary" v-if="sessions.length > 0">
        💾 备份数据
      </button>
      <button @click="openRestoreDialog" class="btn btn-secondary">
        📂 恢复数据
      </button>
      <button 
        v-if="sessions.length > 0"
        @click="confirmDeleteAll" 
        class="btn btn-danger"
      >
        🗑️ 清空所有
      </button>
    </div>
    
    <div v-if="sessions.length === 0" class="empty-state">
      <p>暂无面试记录</p>
    </div>
    
    <div v-else class="sessions-list">
      <div
        v-for="session in sessions"
        :key="session.id"
        class="session-card"
      >
        <div class="session-header">
          <div class="session-info-left" @click="viewSession(session.id!)">
            <span class="session-date">{{ formatDate(session.created_at) }}</span>
            <span class="question-count">{{ session.questions.length }} 个问题</span>
          </div>
          <div class="session-actions">
            <button 
              class="action-btn compare-btn" 
              @click.stop="openQuestionComparisonModal(session.questions[0])"
              title="对比该会话的首个问题"
            >
              📊
            </button>
            <button 
              class="action-btn delete-btn" 
              @click.stop="confirmDeleteSession(session.id!)"
              title="删除此会话"
            >
              🗑️
            </button>
          </div>
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
    
    <!-- Question Comparison Modal -->
    <div v-if="showComparisonModal" class="modal-overlay" @click="closeComparisonModal">
      <div class="modal-content modal-large" @click.stop>
        <div class="modal-header">
          <h4>选择问题对比</h4>
          <button @click="closeComparisonModal" class="close-btn">×</button>
        </div>
        <div class="modal-body">
          <div v-if="allQuestions.length === 0" class="no-answers">
            <p>暂无问题可对比</p>
          </div>
          <div v-else class="questions-grid">
            <button
              v-for="(question, index) in allQuestions"
              :key="index"
              @click="selectQuestionForComparison(question)"
              class="question-option"
            >
              {{ question }}
            </button>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Confirmation Dialog -->
    <div v-if="showConfirmDialog" class="modal-overlay" @click="closeConfirmDialog">
      <div class="modal-content modal-small" @click.stop>
        <div class="modal-header">
          <h4>{{ confirmTitle }}</h4>
          <button @click="closeConfirmDialog" class="close-btn">×</button>
        </div>
        <div class="modal-body">
          <p>{{ confirmMessage }}</p>
        </div>
        <div class="modal-footer">
          <button @click="closeConfirmDialog" class="btn btn-secondary">取消</button>
          <button @click="executeConfirmAction" class="btn btn-danger">确认删除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Interview history component
 * Displays past interview sessions and their answers
 * Features: delete sessions, backup/restore data, compare answers
 */
import { ref, onMounted, computed } from 'vue'
import type { InterviewSession, InterviewAnswer } from '../services/database'
import { getSessions, getSession, getAnswers, deleteSession, deleteAllSessions, backupData, restoreData } from '../services/database'

const sessions = ref<InterviewSession[]>([])
const selectedSession = ref<InterviewSession | null>(null)
const sessionAnswers = ref<InterviewAnswer[]>([])
const showComparisonModal = ref(false)
const showConfirmDialog = ref(false)
const confirmTitle = ref('')
const confirmMessage = ref('')
const confirmAction = ref<(() => Promise<void>) | null>(null)

const allQuestions = computed(() => {
  const questions: string[] = []
  const seen = new Set<string>()
  for (const session of sessions.value) {
    for (const question of session.questions) {
      if (!seen.has(question)) {
        questions.push(question)
        seen.add(question)
      }
    }
  }
  return questions
})

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

const openComparisonModal = () => {
  showComparisonModal.value = true
}

const closeComparisonModal = () => {
  showComparisonModal.value = false
}

const openQuestionComparisonModal = (question: string) => {
  closeComparisonModal()
  // Emit event to parent to show comparison for this question
  // This will be handled by the parent component or routing
  console.log('Compare question:', question)
}

const selectQuestionForComparison = (question: string) => {
  // Emit to parent or navigate to comparison view
  // For now, we'll just close the modal
  closeComparisonModal()
  openQuestionComparisonModal(question)
}

const confirmDeleteSession = (sessionId: number) => {
  confirmTitle.value = '删除面试会话'
  confirmMessage.value = '确定要删除此面试会话和相关数据吗？此操作无法撤销。'
  confirmAction.value = async () => {
    try {
      await deleteSession(sessionId)
      await loadSessions()
      closeConfirmDialog()
    } catch (error) {
      console.error('Failed to delete session:', error)
      alert('删除失败，请重试')
    }
  }
  showConfirmDialog.value = true
}

const confirmDeleteAll = () => {
  confirmTitle.value = '清空所有记录'
  confirmMessage.value = '确定要删除所有面试会话吗？此操作无法撤销，请先备份重要数据。'
  confirmAction.value = async () => {
    try {
      await deleteAllSessions()
      await loadSessions()
      closeConfirmDialog()
    } catch (error) {
      console.error('Failed to delete all sessions:', error)
      alert('清空失败，请重试')
    }
  }
  showConfirmDialog.value = true
}

const openBackupDialog = async () => {
  try {
    // For now, we'll show an alert with instructions
    alert('备份功能需要选择保存位置。请在文件管理器中选择保存位置。')
    // In a real implementation, we'd use file picker
  } catch (error) {
    console.error('Backup error:', error)
  }
}

const openRestoreDialog = async () => {
  try {
    alert('恢复功能需要选择备份文件。请在文件管理器中选择要恢复的备份文件。')
    // In a real implementation, we'd use file picker
  } catch (error) {
    console.error('Restore error:', error)
  }
}

const closeConfirmDialog = () => {
  showConfirmDialog.value = false
  confirmAction.value = null
}

const executeConfirmAction = async () => {
  if (confirmAction.value) {
    await confirmAction.value()
  }
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

.action-bar {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
  flex-wrap: wrap;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  gap: 4px;
}

.btn-secondary {
  background: #f0f0f0;
  color: #333;
  border: 1px solid #d0d0d0;
}

.btn-secondary:hover {
  background: #e0e0e0;
  border-color: #b0b0b0;
}

.btn-danger {
  background: #ef4444;
  color: white;
}

.btn-danger:hover {
  background: #dc2626;
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
  transition: all 0.3s;
}

.session-card:hover {
  border-color: #667eea;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.session-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.8rem;
}

.session-info-left {
  display: flex;
  gap: 16px;
  align-items: center;
  cursor: pointer;
  flex: 1;
}

.session-date {
  font-weight: 600;
  color: #667eea;
}

.question-count {
  font-size: 0.9rem;
  color: #666;
}

.session-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  background: none;
  border: 1px solid #ddd;
  width: 32px;
  height: 32px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-btn:hover {
  border-color: #667eea;
  background: #f0f0ff;
}

.delete-btn:hover {
  border-color: #ef4444;
  background: #fff0f0;
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

.modal-large {
  max-width: 900px;
}

.modal-small {
  max-width: 400px;
  max-height: 300px;
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

.modal-footer {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  padding: 1rem 1.5rem;
  border-top: 1px solid #e0e0e0;
  background: #fafafa;
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

.questions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 12px;
}

.question-option {
  padding: 12px;
  border: 2px solid #e0e0e0;
  border-radius: 6px;
  background: white;
  cursor: pointer;
  text-align: left;
  font-size: 14px;
  line-height: 1.5;
  transition: all 0.3s;
}

.question-option:hover {
  border-color: #667eea;
  background: #f0f0ff;
}

@media (max-width: 768px) {
  .action-bar {
    flex-direction: column;
  }

  .btn {
    width: 100%;
    justify-content: center;
  }

  .session-info-left {
    flex-direction: column;
    align-items: flex-start;
  }

  .questions-grid {
    grid-template-columns: 1fr;
  }
}
</style>
