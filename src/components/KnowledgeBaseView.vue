<template>
  <div class="knowledge-base-view">
    <div class="header">
      <h2>知识库管理</h2>
      <button @click="showImport = !showImport" class="import-toggle-btn">
        {{ showImport ? '隐藏导入' : '导入知识' }}
      </button>
    </div>

    <KnowledgeImport v-if="showImport" @import-complete="handleImportComplete" />

    <div class="stats-cards">
      <div class="stat-card">
        <div class="stat-label">总条目</div>
        <div class="stat-value">{{ stats?.total_vectors || 0 }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">问题</div>
        <div class="stat-value">{{ stats?.question_count || 0 }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">答案</div>
        <div class="stat-value">{{ stats?.answer_count || 0 }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">岗位描述</div>
        <div class="stat-value">{{ stats?.jd_count || 0 }}</div>
      </div>
    </div>

    <div class="controls">
      <div class="search-box">
        <input
          v-model="searchKeyword"
          type="text"
          placeholder="输入关键词搜索..."
          @keyup.enter="handleSearch"
        />
        <button @click="handleSearch" class="search-btn">搜索</button>
        <button v-if="isSearching" @click="clearSearch" class="clear-btn">清除</button>
      </div>
      <div class="filters">
        <select v-model="typeFilter" @change="loadEntries">
          <option value="">全部类型</option>
          <option value="question">问题</option>
          <option value="answer">答案</option>
          <option value="jd">岗位描述</option>
        </select>
      </div>
    </div>

    <div v-if="loading" class="loading">加载中...</div>

    <div v-else-if="entries.length === 0" class="empty-state">
      <p>暂无知识条目</p>
      <p class="hint">请导入知识以开始使用</p>
    </div>

    <div v-else class="entries-table">
      <table>
        <thead>
          <tr>
            <th style="width: 10%">类型</th>
            <th style="width: 50%">内容</th>
            <th style="width: 15%">元数据</th>
            <th style="width: 15%">创建时间</th>
            <th style="width: 10%">操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="entry in entries" :key="entry.id">
            <td>
              <span class="type-badge" :class="`type-${entry.content_type}`">
                {{ formatType(entry.content_type) }}
              </span>
            </td>
            <td class="content-cell">
              <div class="content-preview">{{ entry.content }}</div>
            </td>
            <td class="metadata-cell">
              {{ formatMetadata(entry.metadata) }}
            </td>
            <td class="date-cell">{{ formatDate(entry.created_at) }}</td>
            <td class="action-cell">
              <button @click="confirmDelete(entry.id)" class="delete-btn" title="Delete">
                🗑️
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="!isSearching && entries.length > 0" class="pagination">
      <button @click="prevPage" :disabled="currentPage === 1" class="page-btn">
        上一页
      </button>
      <span class="page-info">第 {{ currentPage }} 页</span>
      <button @click="nextPage" :disabled="entries.length < pageSize" class="page-btn">
        下一页
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import KnowledgeImport from './KnowledgeImport.vue'
import {
  listKnowledgeEntries,
  deleteKnowledgeEntry,
  searchKnowledgeByKeyword,
  getKnowledgeBaseStats,
  type KnowledgeEntry,
  type KnowledgeStats
} from '../services/database'

const showImport = ref(false)
const loading = ref(false)
const entries = ref<KnowledgeEntry[]>([])
const stats = ref<KnowledgeStats | null>(null)
const currentPage = ref(1)
const pageSize = ref(20)
const typeFilter = ref('')
const searchKeyword = ref('')
const isSearching = ref(false)

onMounted(() => {
  loadStats()
  loadEntries()
})

async function loadStats() {
  try {
    stats.value = await getKnowledgeBaseStats()
  } catch (error) {
    console.error('Failed to load stats:', error)
  }
}

async function loadEntries() {
  loading.value = true
  try {
    entries.value = await listKnowledgeEntries(
      currentPage.value,
      pageSize.value,
      typeFilter.value || undefined
    )
  } catch (error) {
    console.error('Failed to load entries:', error)
  } finally {
    loading.value = false
  }
}

async function handleSearch() {
  if (!searchKeyword.value.trim()) return

  loading.value = true
  isSearching.value = true
  try {
    entries.value = await searchKnowledgeByKeyword(searchKeyword.value, 50)
  } catch (error) {
    console.error('Search failed:', error)
  } finally {
    loading.value = false
  }
}

function clearSearch() {
  searchKeyword.value = ''
  isSearching.value = false
  currentPage.value = 1
  loadEntries()
}

async function confirmDelete(id: number) {
  if (!confirm('确定要删除这条知识吗？')) return

  try {
    await deleteKnowledgeEntry(id)
    await loadStats()
    await loadEntries()
  } catch (error) {
    console.error('Delete failed:', error)
    alert('Failed to delete entry: ' + error)
  }
}

function prevPage() {
  if (currentPage.value > 1) {
    currentPage.value--
    loadEntries()
  }
}

function nextPage() {
  if (entries.value.length === pageSize.value) {
    currentPage.value++
    loadEntries()
  }
}

function handleImportComplete() {
  showImport.value = false
  loadStats()
  loadEntries()
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

function formatType(type: string): string {
  const typeMap: Record<string, string> = {
    'question': '问题',
    'answer': '答案',
    'jd': '岗位描述'
  }
  return typeMap[type] || type
}

function formatMetadata(metadata: string | null): string {
  if (!metadata) return '-'
  
  try {
    const data = JSON.parse(metadata)
    const parts: string[] = []
    
    if (data.category) {
      const categoryMap: Record<string, string> = {
        'frontend': '前端',
        'backend': '后端',
        'pm': '产品',
        'fullstack': '全栈',
        'qa': '测试',
        'devops': '运维'
      }
      parts.push(categoryMap[data.category] || data.category)
    }
    
    if (data.jd_name) {
      parts.push(data.jd_name)
    }
    
    return parts.length > 0 ? parts.join(' · ') : '-'
  } catch {
    return metadata
  }
}
</script>

<style scoped>
.knowledge-base-view {
  padding: 24px;
  max-width: 1400px;
  margin: 0 auto;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.header h2 {
  margin: 0;
  color: var(--text-primary);
}

.import-toggle-btn {
  padding: 10px 20px;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  transition: background 0.2s;
}

.import-toggle-btn:hover {
  background: var(--primary-hover);
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  padding: 20px;
  background: var(--bg-secondary);
  border-radius: 8px;
  text-align: center;
}

.stat-label {
  font-size: 14px;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.stat-value {
  font-size: 32px;
  font-weight: 600;
  color: var(--primary-color);
}

.controls {
  display: flex;
  gap: 16px;
  margin-bottom: 20px;
}

.search-box {
  flex: 1;
  display: flex;
  gap: 8px;
}

.search-box input {
  flex: 1;
  padding: 10px 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 14px;
}

.search-btn,
.clear-btn {
  padding: 10px 20px;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  transition: background 0.2s;
}

.search-btn:hover {
  background: var(--primary-hover);
}

.clear-btn {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.clear-btn:hover {
  background: var(--bg-hover);
}

.filters select {
  padding: 10px 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  cursor: pointer;
}

.loading,
.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: var(--text-secondary);
}

.empty-state .hint {
  margin-top: 8px;
  font-size: 14px;
}

.entries-table {
  background: var(--bg-secondary);
  border-radius: 8px;
  overflow: hidden;
}

table {
  width: 100%;
  border-collapse: collapse;
}

thead {
  background: var(--bg-tertiary);
}

th {
  padding: 14px 16px;
  text-align: left;
  font-weight: 600;
  color: var(--text-primary);
  border-bottom: 2px solid var(--border-color);
}

tbody tr {
  border-bottom: 1px solid var(--border-color);
  transition: background 0.2s;
}

tbody tr:hover {
  background: var(--bg-hover);
}

td {
  padding: 12px 16px;
  color: var(--text-primary);
}

.type-badge {
  display: inline-block;
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.type-question {
  background: rgba(59, 130, 246, 0.2);
  color: #3b82f6;
}

.type-answer {
  background: rgba(34, 197, 94, 0.2);
  color: #22c55e;
}

.type-jd {
  background: rgba(168, 85, 247, 0.2);
  color: #a855f7;
}

.content-cell {
  max-width: 400px;
}

.content-preview {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.metadata-cell,
.date-cell {
  font-size: 13px;
  color: var(--text-secondary);
}

.action-cell {
  text-align: center;
}

.delete-btn {
  padding: 6px 10px;
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 16px;
  transition: transform 0.2s;
}

.delete-btn:hover {
  transform: scale(1.2);
}

.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 16px;
  margin-top: 24px;
}

.page-btn {
  padding: 8px 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  cursor: pointer;
  transition: background 0.2s;
}

.page-btn:hover:not(:disabled) {
  background: var(--bg-hover);
}

.page-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.page-info {
  color: var(--text-secondary);
  font-size: 14px;
}
</style>
