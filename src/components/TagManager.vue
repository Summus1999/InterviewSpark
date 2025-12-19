<!--
  TagManager.vue - Tag management component
  
  Features:
  - Create new tags with custom colors
  - Edit existing tags
  - Delete tags
-->
<template>
  <div class="tag-manager">
    <div class="manager-header">
      <h4>标签管理</h4>
      <button @click="showAddForm = !showAddForm" class="toggle-btn">
        {{ showAddForm ? '取消' : '+ 新建标签' }}
      </button>
    </div>

    <div v-if="showAddForm" class="add-form">
      <input
        v-model="newTagName"
        type="text"
        placeholder="标签名称"
        class="tag-input"
        @keyup.enter="createNewTag"
      />
      <div class="color-picker">
        <span
          v-for="color in colorOptions"
          :key="color"
          :class="['color-option', { active: newTagColor === color }]"
          :style="{ backgroundColor: color }"
          @click="newTagColor = color"
        />
      </div>
      <button @click="createNewTag" :disabled="!newTagName.trim()" class="create-btn">
        创建
      </button>
    </div>

    <div v-if="loading" class="loading">加载中...</div>

    <div v-else-if="tags.length === 0" class="empty-state">
      <p>暂无标签，点击上方按钮创建</p>
    </div>

    <div v-else class="tags-list">
      <div
        v-for="tag in tags"
        :key="tag.id"
        class="tag-item"
      >
        <template v-if="editingId === tag.id">
          <input
            v-model="editName"
            type="text"
            class="edit-input"
            @keyup.enter="saveEdit(tag.id!)"
          />
          <div class="color-picker inline">
            <span
              v-for="color in colorOptions"
              :key="color"
              :class="['color-option small', { active: editColor === color }]"
              :style="{ backgroundColor: color }"
              @click="editColor = color"
            />
          </div>
          <div class="edit-actions">
            <button @click="saveEdit(tag.id!)" class="save-btn">保存</button>
            <button @click="cancelEdit" class="cancel-btn">取消</button>
          </div>
        </template>
        <template v-else>
          <span class="tag-badge" :style="{ backgroundColor: tag.color }">
            {{ tag.name }}
          </span>
          <div class="tag-actions">
            <button @click="startEdit(tag)" class="action-btn edit">编辑</button>
            <button @click="confirmDelete(tag)" class="action-btn delete">删除</button>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { QuestionTag } from '../services/database'
import { getAllTags, createTag, updateTag, deleteTag } from '../services/database'

const emit = defineEmits<{
  (e: 'tagsUpdated'): void
}>()

const tags = ref<QuestionTag[]>([])
const loading = ref(true)
const showAddForm = ref(false)

const newTagName = ref('')
const newTagColor = ref('#667eea')

const editingId = ref<number | null>(null)
const editName = ref('')
const editColor = ref('')

const colorOptions = [
  '#667eea', '#f56565', '#48bb78', '#ed8936',
  '#9f7aea', '#38b2ac', '#ed64a6', '#4299e1'
]

onMounted(async () => {
  await loadTags()
})

const loadTags = async () => {
  loading.value = true
  try {
    tags.value = await getAllTags()
  } catch (error) {
    console.error('Failed to load tags:', error)
  } finally {
    loading.value = false
  }
}

const createNewTag = async () => {
  if (!newTagName.value.trim()) return
  
  try {
    await createTag(newTagName.value.trim(), newTagColor.value)
    newTagName.value = ''
    newTagColor.value = '#667eea'
    showAddForm.value = false
    await loadTags()
    emit('tagsUpdated')
  } catch (error) {
    console.error('Failed to create tag:', error)
    alert('创建标签失败')
  }
}

const startEdit = (tag: QuestionTag) => {
  editingId.value = tag.id!
  editName.value = tag.name
  editColor.value = tag.color
}

const cancelEdit = () => {
  editingId.value = null
  editName.value = ''
  editColor.value = ''
}

const saveEdit = async (id: number) => {
  if (!editName.value.trim()) return
  
  try {
    await updateTag(id, editName.value.trim(), editColor.value)
    cancelEdit()
    await loadTags()
    emit('tagsUpdated')
  } catch (error) {
    console.error('Failed to update tag:', error)
    alert('更新标签失败')
  }
}

const confirmDelete = async (tag: QuestionTag) => {
  if (!confirm(`确定要删除标签"${tag.name}"吗？`)) return
  
  try {
    await deleteTag(tag.id!)
    await loadTags()
    emit('tagsUpdated')
  } catch (error) {
    console.error('Failed to delete tag:', error)
    alert('删除标签失败')
  }
}
</script>

<style scoped>
.tag-manager {
  padding: 1rem;
  background: var(--card-bg, #fff);
  border-radius: 8px;
  border: 1px solid var(--border-color, #e0e0e0);
}

.manager-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.manager-header h4 {
  margin: 0;
  font-size: 1rem;
  color: var(--text-primary, #333);
}

.toggle-btn {
  padding: 0.4rem 0.8rem;
  background: #667eea;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 0.85rem;
  cursor: pointer;
}

.toggle-btn:hover {
  background: #5568d3;
}

.add-form {
  padding: 1rem;
  background: var(--bg-secondary, #f8f9ff);
  border-radius: 6px;
  margin-bottom: 1rem;
}

.tag-input,
.edit-input {
  width: 100%;
  padding: 0.6rem;
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 4px;
  font-size: 0.9rem;
  margin-bottom: 0.8rem;
}

.tag-input:focus,
.edit-input:focus {
  outline: none;
  border-color: #667eea;
}

.color-picker {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 0.8rem;
}

.color-picker.inline {
  margin-bottom: 0;
}

.color-option {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  cursor: pointer;
  border: 2px solid transparent;
  transition: transform 0.2s;
}

.color-option:hover {
  transform: scale(1.1);
}

.color-option.active {
  border-color: var(--text-primary, #333);
}

.color-option.small {
  width: 18px;
  height: 18px;
}

.create-btn {
  padding: 0.5rem 1rem;
  background: #48bb78;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.create-btn:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.loading,
.empty-state {
  padding: 1.5rem;
  text-align: center;
  color: var(--text-secondary, #666);
  font-size: 0.9rem;
}

.tags-list {
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
}

.tag-item {
  display: flex;
  align-items: center;
  gap: 0.8rem;
  padding: 0.6rem;
  background: var(--bg-secondary, #f8f9ff);
  border-radius: 6px;
}

.tag-badge {
  padding: 0.3rem 0.8rem;
  color: white;
  border-radius: 12px;
  font-size: 0.85rem;
  font-weight: 500;
}

.tag-actions,
.edit-actions {
  margin-left: auto;
  display: flex;
  gap: 0.5rem;
}

.action-btn {
  padding: 0.3rem 0.6rem;
  border: none;
  border-radius: 4px;
  font-size: 0.8rem;
  cursor: pointer;
}

.action-btn.edit {
  background: #4299e1;
  color: white;
}

.action-btn.delete {
  background: #f56565;
  color: white;
}

.save-btn {
  padding: 0.3rem 0.6rem;
  background: #48bb78;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.cancel-btn {
  padding: 0.3rem 0.6rem;
  background: #a0aec0;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}
</style>