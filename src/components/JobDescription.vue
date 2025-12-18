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
    <h3>岗位描述 (JD)</h3>
    <textarea
      v-model="localValue"
      @input="handleInput"
      placeholder="请输入或粘贴岗位描述...&#10;&#10;建议包含：&#10;- 职位名称&#10;- 岗位职责&#10;- 任职要求&#10;- 技能要求&#10;- 其他要求"
      class="input-textarea"
      rows="10"
    />
    <div class="char-count">{{ charCount }} 字符</div>
  </div>
</template>

<script setup lang="ts">
/**
 * Job description input component with character count
 * Implements v-model pattern for parent component binding
 */
import { ref, computed, watch } from 'vue'

const props = defineProps<{
  modelValue: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const localValue = ref(props.modelValue)

const charCount = computed(() => localValue.value.length)

const handleInput = () => {
  emit('update:modelValue', localValue.value)
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

h3 {
  margin: 0 0 1rem;
  font-size: 1.2rem;
  color: #333;
}

.input-textarea {
  width: 100%;
  padding: 1rem;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  font-size: 0.95rem;
  line-height: 1.6;
  font-family: inherit;
  resize: vertical;
  transition: border-color 0.3s;
}

.input-textarea:focus {
  outline: none;
  border-color: #667eea;
}

.input-textarea::placeholder {
  color: #999;
}

.char-count {
  margin-top: 0.5rem;
  font-size: 0.85rem;
  color: #666;
  text-align: right;
}
</style>
