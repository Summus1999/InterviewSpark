<!--
  MarkdownNotes.vue - Markdown notes editor with preview
  
  Features:
  - Edit mode with markdown syntax
  - Preview mode with rendered HTML
  - Toggle between modes
-->
<template>
  <div class="markdown-notes">
    <div class="notes-header">
      <span class="notes-label">{{ label }}</span>
      <div class="mode-toggle">
        <button
          :class="['toggle-btn', { active: mode === 'edit' }]"
          @click="mode = 'edit'"
        >
          编辑
        </button>
        <button
          :class="['toggle-btn', { active: mode === 'preview' }]"
          @click="mode = 'preview'"
        >
          预览
        </button>
      </div>
    </div>

    <div v-if="mode === 'edit'" class="edit-mode">
      <textarea
        :value="modelValue"
        @input="$emit('update:modelValue', ($event.target as HTMLTextAreaElement).value)"
        @blur="$emit('blur')"
        :placeholder="placeholder"
        :rows="rows"
        class="notes-textarea"
      />
      <div class="markdown-hint">
        支持 Markdown 语法: **粗体** *斜体* - 列表 # 标题
      </div>
    </div>

    <div v-else class="preview-mode">
      <div
        v-if="modelValue"
        class="markdown-content"
        v-html="renderedContent"
      />
      <div v-else class="empty-preview">
        {{ placeholder }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { marked } from 'marked'

const props = withDefaults(defineProps<{
  modelValue?: string
  label?: string
  placeholder?: string
  rows?: number
}>(), {
  modelValue: '',
  label: '笔记',
  placeholder: '添加笔记...',
  rows: 4
})

defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'blur'): void
}>()

const mode = ref<'edit' | 'preview'>('edit')

const renderedContent = computed(() => {
  if (!props.modelValue) return ''
  return marked.parse(props.modelValue) as string
})
</script>

<style scoped>
.markdown-notes {
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 6px;
  overflow: hidden;
}

.notes-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0.8rem;
  background: var(--bg-secondary, #f8f9ff);
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.notes-label {
  font-size: 0.9rem;
  font-weight: 500;
  color: var(--text-primary, #333);
}

.mode-toggle {
  display: flex;
  gap: 0.25rem;
}

.toggle-btn {
  padding: 0.25rem 0.6rem;
  background: transparent;
  border: 1px solid var(--border-color, #ccc);
  border-radius: 4px;
  font-size: 0.8rem;
  color: var(--text-secondary, #666);
  cursor: pointer;
  transition: all 0.2s;
}

.toggle-btn.active {
  background: #667eea;
  border-color: #667eea;
  color: white;
}

.edit-mode {
  padding: 0.5rem;
}

.notes-textarea {
  width: 100%;
  padding: 0.8rem;
  border: none;
  font-size: 0.9rem;
  font-family: 'Monaco', 'Menlo', monospace;
  line-height: 1.6;
  resize: vertical;
  background: transparent;
}

.notes-textarea:focus {
  outline: none;
}

.markdown-hint {
  padding: 0.4rem 0.8rem;
  font-size: 0.75rem;
  color: var(--text-secondary, #999);
}

.preview-mode {
  padding: 1rem;
  min-height: 100px;
}

.markdown-content {
  font-size: 0.95rem;
  line-height: 1.7;
  color: var(--text-primary, #333);
}

.markdown-content :deep(h1),
.markdown-content :deep(h2),
.markdown-content :deep(h3) {
  margin-top: 1rem;
  margin-bottom: 0.5rem;
}

.markdown-content :deep(p) {
  margin-bottom: 0.8rem;
}

.markdown-content :deep(ul),
.markdown-content :deep(ol) {
  padding-left: 1.5rem;
  margin-bottom: 0.8rem;
}

.markdown-content :deep(li) {
  margin-bottom: 0.3rem;
}

.markdown-content :deep(code) {
  padding: 0.2rem 0.4rem;
  background: #f0f0f0;
  border-radius: 3px;
  font-family: monospace;
  font-size: 0.9em;
}

.markdown-content :deep(pre) {
  padding: 1rem;
  background: #f5f5f5;
  border-radius: 6px;
  overflow-x: auto;
}

.markdown-content :deep(blockquote) {
  padding-left: 1rem;
  border-left: 3px solid #667eea;
  color: var(--text-secondary, #666);
  margin: 0.8rem 0;
}

.empty-preview {
  color: var(--text-secondary, #999);
  font-style: italic;
}
</style>
