<!--
  JobDescription.vue - Job description input component
  
  Features:
  - Text area for user to paste/input job description (JD)
  - Character count display
  - v-model support for two-way data binding
  
  Props:
  - modelValue: string - Current job description text value
  
  Emits:
  - update:modelValue: Fired when text content changes
-->
<template>
  <div class="job-description">
    <div class="header-section">
      <h3>岗位描述 (JD)</h3>
      <button @click="showSelector = true" class="template-btn">
        📋 选择模板
      </button>
    </div>
    
    <textarea
      v-model="localValue"
      @input="handleInput"
      placeholder="请输入或粘贴岗位描述...&#10;&#10;建议包含：&#10;- 职位名称&#10;- 岗位职责&#10;- 任职要求&#10;- 技能要求&#10;- 其他要求"
      class="input-textarea"
      rows="10"
    />
    <div class="char-count">{{ charCount }} 字符</div>
    
    <!-- Template Selector Modal -->
    <div v-if="showSelector" class="template-modal" @click="showSelector = false">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h4>选择岗位描述模板</h4>
          <button class="close-btn" @click="showSelector = false">✕</button>
        </div>
        <TemplateSelector 
          template-type="jd"
          @select="applyTemplate"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Job description input component with character count
 * Implements v-model pattern for parent component binding
 */
import { ref, computed, watch } from 'vue'
import TemplateSelector from './TemplateSelector.vue'

const props = defineProps<{
  modelValue: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const localValue = ref(props.modelValue)
const showSelector = ref(false)

const charCount = computed(() => localValue.value.length)

const handleInput = () => {
  emit('update:modelValue', localValue.value)
}

const applyTemplate = (content: string) => {
  localValue.value = content
  emit('update:modelValue', content)
  showSelector.value = false
}

watch(
  () => props.modelValue,
  (newValue) => {
    localValue.value = newValue
  }
)
</script>

<style scoped>
.job-description {
  margin-bottom: 2rem;
}

.header-section {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  gap: 1rem;
}

.header-section h3 {
  margin: 0;
  font-size: 1.2rem;
  color: var(--text-primary);
}

.template-btn {
  padding: 0.6rem 1rem;
  background: var(--accent-primary);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
  white-space: nowrap;
}

.template-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.input-textarea {
  width: 100%;
  padding: 1rem;
  border: 2px solid var(--border-light);
  border-radius: 8px;
  font-size: 0.95rem;
  line-height: 1.6;
  font-family: inherit;
  resize: vertical;
  transition: border-color 0.3s;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.input-textarea:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.input-textarea::placeholder {
  color: var(--text-secondary);
}

.char-count {
  margin-top: 0.5rem;
  font-size: 0.85rem;
  color: var(--text-secondary);
  text-align: right;
}

/* Template Modal */
.template-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(5px);
}

.modal-content {
  background: var(--bg-card-solid);
  border-radius: 12px;
  max-width: 900px;
  width: 95%;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: var(--shadow-lg);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 2rem;
  border-bottom: 1px solid var(--border-light);
  background: var(--bg-secondary);
}

.modal-header h4 {
  margin: 0;
  font-size: 1.3rem;
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
</style>
