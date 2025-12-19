<template>
  <div class="template-selector">
    <!-- Filter by industry -->
    <div class="filter-section">
      <label>选择行业：</label>
      <select v-model="selectedIndustry" class="industry-select">
        <option value="">全部行业</option>
        <option v-for="industry in industries" :key="industry" :value="industry">
          {{ industry }}
        </option>
      </select>
    </div>

    <!-- Template list -->
    <div class="template-list">
      <h4 v-if="templateType === 'resume'">简历模板</h4>
      <h4 v-else>岗位描述模板</h4>
      
      <div class="template-grid">
        <div 
          v-for="template in filteredTemplates" 
          :key="template.id"
          class="template-card"
          @click="selectTemplate(template)"
        >
          <div class="template-header">
            <h5>{{ template.name }}</h5>
            <span class="industry-badge">{{ template.industry }}</span>
          </div>
          <p class="template-position">{{ template.position }}</p>
          <div class="template-actions">
            <button class="preview-btn" @click.stop="showPreview(template)">
              预览
            </button>
            <button class="select-btn" @click.stop="selectTemplate(template)">
              选择此模板
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Preview Dialog -->
    <div v-if="previewTemplate" class="preview-dialog" @click="closePreview">
      <div class="preview-content" @click.stop>
        <div class="preview-header">
          <h3>{{ previewTemplate.name }}</h3>
          <button class="close-btn" @click="closePreview">✕</button>
        </div>
        <div class="preview-body">
          <pre>{{ previewTemplate.content }}</pre>
        </div>
        <div class="preview-actions">
          <button class="primary-btn" @click="selectTemplate(previewTemplate)">
            使用此模板
          </button>
          <button class="secondary-btn" @click="closePreview">
            取消
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { 
  resumeTemplates, 
  jdTemplates, 
  getIndustries,
  type ResumeTemplate, 
  type JDTemplate 
} from '../data/templates'

interface Props {
  templateType: 'resume' | 'jd'
}

const props = defineProps<Props>()

const emit = defineEmits<{
  select: [content: string, templateName: string]
}>()

const selectedIndustry = ref('')
const previewTemplate = ref<ResumeTemplate | JDTemplate | null>(null)

const industries = computed(() => getIndustries())

const allTemplates = computed(() => {
  return props.templateType === 'resume' ? resumeTemplates : jdTemplates
})

const filteredTemplates = computed(() => {
  if (!selectedIndustry.value) {
    return allTemplates.value
  }
  return allTemplates.value.filter(t => t.industry === selectedIndustry.value)
})

function selectTemplate(template: ResumeTemplate | JDTemplate) {
  emit('select', template.content, template.name)
  previewTemplate.value = null
}

function showPreview(template: ResumeTemplate | JDTemplate) {
  previewTemplate.value = template
}

function closePreview() {
  previewTemplate.value = null
}
</script>

<style scoped>
.template-selector {
  background: var(--bg-card-solid);
  padding: 2rem;
  border-radius: 12px;
  border: 1px solid var(--border-light);
}

.filter-section {
  margin-bottom: 2rem;
  display: flex;
  gap: 1rem;
  align-items: center;
}

.filter-section label {
  font-weight: 600;
  color: var(--text-primary);
}

.industry-select {
  padding: 0.6rem 1rem;
  border: 1px solid var(--border-light);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 0.95rem;
  cursor: pointer;
}

.template-list h4 {
  margin: 0 0 1.5rem 0;
  font-size: 1.3rem;
  color: var(--text-primary);
}

.template-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

.template-card {
  background: var(--bg-secondary);
  border: 2px solid var(--border-light);
  border-radius: 10px;
  padding: 1.5rem;
  cursor: pointer;
  transition: all 0.3s;
}

.template-card:hover {
  border-color: var(--accent-primary);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.2);
  transform: translateY(-2px);
}

.template-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 0.5rem;
  gap: 1rem;
}

.template-header h5 {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-primary);
  flex: 1;
}

.industry-badge {
  background: var(--accent-primary);
  color: white;
  padding: 0.3rem 0.8rem;
  border-radius: 20px;
  font-size: 0.8rem;
  white-space: nowrap;
  font-weight: 600;
}

.template-position {
  margin: 0.5rem 0 1rem 0;
  color: var(--text-secondary);
  font-size: 0.95rem;
}

.template-actions {
  display: flex;
  gap: 0.8rem;
}

.preview-btn,
.select-btn {
  flex: 1;
  padding: 0.6rem 1rem;
  border: none;
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.preview-btn {
  background: var(--bg-input);
  border: 1px solid var(--border-color);
  color: var(--text-primary);
}

.preview-btn:hover {
  background: var(--bg-hover);
  border-color: var(--border-hover);
}

.select-btn {
  background: var(--accent-gradient);
  color: white;
}

.select-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

/* Preview Dialog */
.preview-dialog {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1001;
  backdrop-filter: blur(5px);
}

.preview-content {
  background: var(--bg-card-solid);
  border-radius: 12px;
  max-width: 800px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  width: 90%;
  overflow: hidden;
}

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 2rem;
  border-bottom: 1px solid var(--border-light);
}

.preview-header h3 {
  margin: 0;
  color: var(--text-primary);
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.3s;
}

.close-btn:hover {
  color: var(--text-primary);
}

.preview-body {
  flex: 1;
  overflow-y: auto;
  padding: 2rem;
}

.preview-body pre {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 0.9rem;
  line-height: 1.6;
  color: var(--text-primary);
}

.preview-actions {
  display: flex;
  gap: 1rem;
  padding: 2rem;
  border-top: 1px solid var(--border-light);
}

.primary-btn,
.secondary-btn {
  flex: 1;
  padding: 0.8rem 1.5rem;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.primary-btn {
  background: var(--accent-gradient);
  color: white;
}

.primary-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.secondary-btn {
  background: var(--bg-input);
  color: var(--text-primary);
  border: 1px solid var(--border-light);
}

.secondary-btn:hover {
  background: var(--bg-hover);
}
</style>
