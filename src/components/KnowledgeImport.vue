<template>
  <div class="knowledge-import">
    <div class="import-header">
      <h3>导入知识</h3>
      <p class="hint">支持 .json 和 .txt 文件格式</p>
    </div>

    <div class="import-body">
      <div v-if="!importing" class="upload-area">
        <input
          ref="fileInput"
          type="file"
          accept=".json,.txt"
          @change="handleFileSelect"
          style="display: none"
        />
        <button @click="selectFile" class="select-btn">
          <span class="icon">📁</span>
          <span>选择文件</span>
        </button>
        <div v-if="selectedFile" class="file-info">
          <span class="file-name">{{ selectedFile.name }}</span>
          <button @click="startImport" class="import-btn">开始导入</button>
        </div>
      </div>

      <div v-if="importing" class="importing-status">
        <div class="spinner"></div>
        <p>正在导入知识条目...</p>
      </div>

      <div v-if="result" class="import-result">
        <div class="result-summary" :class="{ success: result.fail_count === 0 }">
          <h4>导入完成</h4>
          <p>成功: {{ result.success_count }} | 失败: {{ result.fail_count }}</p>
        </div>
        <div v-if="result.errors.length > 0" class="error-list">
          <h5>错误详情：</h5>
          <ul>
            <li v-for="(error, idx) in result.errors" :key="idx">{{ error }}</li>
          </ul>
        </div>
        <button @click="reset" class="reset-btn">导入其他文件</button>
      </div>
    </div>

    <div class="format-guide">
      <details>
        <summary>文件格式说明</summary>
        <div class="guide-content">
          <h5>JSON 格式：</h5>
          <pre>[
  {
    "content_type": "question",
    "content": "What is your experience with Rust?",
    "metadata": null
  }
]</pre>
          <h5>TXT 格式（竖线分隔）：</h5>
          <pre>question|What is your experience with Rust?|
answer|I have 3 years of Rust development|backend</pre>
        </div>
      </details>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { importKnowledgeFile, type ImportResult } from '../services/database'

const emit = defineEmits<{
  importComplete: []
}>()

const fileInput = ref<HTMLInputElement | null>(null)
const selectedFile = ref<File | null>(null)
const selectedPath = ref<string>('')
const importing = ref(false)
const result = ref<ImportResult | null>(null)

async function selectFile() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: 'Knowledge Files',
        extensions: ['json', 'txt']
      }
    ]
  })

  if (selected && typeof selected === 'string') {
    selectedPath.value = selected
    // Extract filename from path
    const filename = selected.split(/[\\/]/).pop() || 'unknown'
    selectedFile.value = { name: filename } as File
  }
}

function handleFileSelect(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    selectedFile.value = target.files[0]
  }
}

async function startImport() {
  if (!selectedPath.value) return

  importing.value = true
  result.value = null

  try {
    const importResult = await importKnowledgeFile(selectedPath.value)
    result.value = importResult
    if (importResult.success_count > 0) {
      emit('importComplete')
    }
  } catch (error) {
    result.value = {
      success_count: 0,
      fail_count: 1,
      errors: [String(error)]
    }
  } finally {
    importing.value = false
  }
}

function reset() {
  selectedFile.value = null
  selectedPath.value = ''
  result.value = null
  if (fileInput.value) {
    fileInput.value.value = ''
  }
}
</script>

<style scoped>
.knowledge-import {
  padding: 20px;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.import-header h3 {
  margin: 0 0 8px 0;
  color: var(--text-primary);
}

.hint {
  margin: 0;
  font-size: 14px;
  color: var(--text-secondary);
}

.import-body {
  margin: 20px 0;
  min-height: 120px;
}

.upload-area {
  text-align: center;
  padding: 30px;
  border: 2px dashed var(--border-color);
  border-radius: 8px;
  background: var(--bg-primary);
}

.select-btn {
  padding: 12px 24px;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  transition: background 0.2s;
}

.select-btn:hover {
  background: var(--primary-hover);
}

.file-info {
  margin-top: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
}

.file-name {
  color: var(--text-primary);
  font-weight: 500;
}

.import-btn {
  padding: 8px 16px;
  background: var(--success-color);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.import-btn:hover {
  opacity: 0.9;
}

.importing-status {
  text-align: center;
  padding: 40px;
}

.spinner {
  width: 40px;
  height: 40px;
  margin: 0 auto 16px;
  border: 4px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.import-result {
  padding: 20px;
  background: var(--bg-primary);
  border-radius: 8px;
}

.result-summary {
  padding: 16px;
  background: var(--warning-bg);
  border-left: 4px solid var(--warning-color);
  border-radius: 4px;
  margin-bottom: 16px;
}

.result-summary.success {
  background: var(--success-bg);
  border-left-color: var(--success-color);
}

.result-summary h4 {
  margin: 0 0 8px 0;
  color: var(--text-primary);
}

.result-summary p {
  margin: 0;
  color: var(--text-secondary);
}

.error-list {
  margin: 16px 0;
  padding: 16px;
  background: var(--error-bg);
  border-radius: 4px;
}

.error-list h5 {
  margin: 0 0 8px 0;
  color: var(--error-color);
}

.error-list ul {
  margin: 0;
  padding-left: 20px;
  max-height: 200px;
  overflow-y: auto;
}

.error-list li {
  color: var(--text-secondary);
  font-size: 14px;
  margin: 4px 0;
}

.reset-btn {
  padding: 10px 20px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
}

.reset-btn:hover {
  background: var(--bg-hover);
}

.format-guide {
  margin-top: 24px;
  padding: 16px;
  background: var(--bg-primary);
  border-radius: 8px;
}

.format-guide summary {
  cursor: pointer;
  color: var(--primary-color);
  font-weight: 500;
  user-select: none;
}

.guide-content {
  margin-top: 12px;
}

.guide-content h5 {
  margin: 12px 0 6px 0;
  color: var(--text-primary);
  font-size: 14px;
}

.guide-content pre {
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 4px;
  overflow-x: auto;
  font-size: 13px;
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
}
</style>
