<!--
  QuestionBank.vue - Question bank management
  
  Features:
  - View saved questions
  - Add questions to bank
  - Edit best answers and notes
  - Delete questions from bank
-->
<template>
  <div class="question-bank">
    <h3>题库管理</h3>
    
    <div class="add-section">
      <h4>添加新问题</h4>
      <input
        v-model="newQuestion"
        type="text"
        placeholder="输入面试问题..."
        class="question-input"
      >
      <textarea
        v-model="newAnswer"
        placeholder="最佳答案（可选）"
        rows="3"
        class="answer-textarea"
      />
      <input
        v-model="newCategory"
        type="text"
        placeholder="岗位类别（可选）"
        class="category-input"
      >
      <button @click="addQuestion" :disabled="!newQuestion.trim()" class="add-btn">
        添加到题库
      </button>
    </div>
    
    <div v-if="bank.length === 0" class="empty-state">
      <p>题库为空，快来添加问题吧！</p>
    </div>
    
    <div v-else class="bank-list">
      <div
        v-for="item in bank"
        :key="item.id"
        class="bank-item"
      >
        <div class="item-header">
          <span class="item-question">{{ item.question }}</span>
          <button @click="deleteItem(item.id!)" class="delete-btn">删除</button>
        </div>
        
        <div v-if="item.job_category" class="item-category">
          分类: {{ item.job_category }}
        </div>
        
        <div class="item-answer">
          <strong>最佳答案:</strong>
          <textarea
            v-model="item.best_answer"
            @blur="updateItem(item)"
            placeholder="添加最佳答案..."
            rows="3"
            class="edit-textarea"
          />
        </div>
        
        <div class="item-notes">
          <strong>备注:</strong>
          <textarea
            v-model="item.notes"
            @blur="updateItem(item)"
            placeholder="添加备注..."
            rows="2"
            class="edit-textarea"
          />
        </div>
        
        <div class="item-meta">
          <span>创建时间: {{ formatDate(item.created_at) }}</span>
          <span v-if="item.updated_at !== item.created_at">
            更新时间: {{ formatDate(item.updated_at) }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Question bank management component
 * Allows users to save and manage frequently asked questions
 */
import { ref, onMounted } from 'vue'
import type { QuestionBankItem } from '../services/database'
import { getBank, addToBank, updateBankItem, deleteFromBank } from '../services/database'

const bank = ref<QuestionBankItem[]>([])
const newQuestion = ref('')
const newAnswer = ref('')
const newCategory = ref('')

onMounted(async () => {
  await loadBank()
})

const loadBank = async () => {
  try {
    bank.value = await getBank()
  } catch (error) {
    console.error('Failed to load question bank:', error)
  }
}

const addQuestion = async () => {
  if (!newQuestion.value.trim()) return
  
  try {
    await addToBank(
      newQuestion.value,
      newAnswer.value || undefined,
      undefined,
      newCategory.value || undefined
    )
    
    // Reset form
    newQuestion.value = ''
    newAnswer.value = ''
    newCategory.value = ''
    
    // Reload bank
    await loadBank()
  } catch (error) {
    console.error('Failed to add question:', error)
    alert('添加失败: ' + error)
  }
}

const updateItem = async (item: QuestionBankItem) => {
  if (!item.id) return
  
  try {
    await updateBankItem(
      item.id,
      item.best_answer || undefined,
      item.notes || undefined
    )
  } catch (error) {
    console.error('Failed to update item:', error)
  }
}

const deleteItem = async (id: number) => {
  if (!confirm('确定要删除这个问题吗？')) return
  
  try {
    await deleteFromBank(id)
    await loadBank()
  } catch (error) {
    console.error('Failed to delete item:', error)
    alert('删除失败: ' + error)
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
.question-bank {
  padding: 2rem 0;
}

h3 {
  margin: 0 0 1.5rem;
  font-size: 1.3rem;
  color: #333;
}

h4 {
  margin: 0 0 1rem;
  font-size: 1.1rem;
  color: #555;
}

.add-section {
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: #f8f9ff;
  border-radius: 8px;
}

.question-input,
.category-input {
  width: 100%;
  padding: 0.8rem;
  margin-bottom: 1rem;
  border: 2px solid #e0e0e0;
  border-radius: 6px;
  font-size: 0.95rem;
}

.answer-textarea {
  width: 100%;
  padding: 0.8rem;
  margin-bottom: 1rem;
  border: 2px solid #e0e0e0;
  border-radius: 6px;
  font-size: 0.95rem;
  font-family: inherit;
  resize: vertical;
}

.question-input:focus,
.category-input:focus,
.answer-textarea:focus {
  outline: none;
  border-color: #667eea;
}

.add-btn {
  padding: 0.8rem 1.5rem;
  background: #667eea;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 1rem;
  cursor: pointer;
  transition: background 0.3s;
}

.add-btn:hover:not(:disabled) {
  background: #5568d3;
}

.add-btn:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.empty-state {
  padding: 3rem;
  text-align: center;
  background: #f5f5f5;
  border-radius: 8px;
  color: #999;
}

.bank-list {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.bank-item {
  padding: 1.5rem;
  background: white;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
}

.item-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
  gap: 1rem;
}

.item-question {
  flex: 1;
  font-size: 1.05rem;
  font-weight: 600;
  color: #333;
  line-height: 1.5;
}

.delete-btn {
  padding: 0.4rem 1rem;
  background: #ff5757;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.3s;
  flex-shrink: 0;
}

.delete-btn:hover {
  background: #ff3838;
}

.item-category {
  margin-bottom: 1rem;
  padding: 0.4rem 0.8rem;
  background: #667eea;
  color: white;
  border-radius: 4px;
  display: inline-block;
  font-size: 0.85rem;
}

.item-answer,
.item-notes {
  margin-bottom: 1rem;
}

.item-answer strong,
.item-notes strong {
  display: block;
  margin-bottom: 0.5rem;
  color: #555;
  font-size: 0.95rem;
}

.edit-textarea {
  width: 100%;
  padding: 0.8rem;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  font-size: 0.9rem;
  font-family: inherit;
  resize: vertical;
  transition: border-color 0.3s;
}

.edit-textarea:focus {
  outline: none;
  border-color: #667eea;
}

.edit-textarea::placeholder {
  color: #999;
}

.item-meta {
  display: flex;
  gap: 2rem;
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid #e0e0e0;
  font-size: 0.85rem;
  color: #999;
}
</style>
