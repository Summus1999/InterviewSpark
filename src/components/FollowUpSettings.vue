<template>
  <div class="followup-settings">
    <div class="settings-header">
      <h4>🔄 追问设置</h4>
      <label class="toggle-switch">
        <input 
          type="checkbox" 
          v-model="localSettings.enabled"
          @change="handleChange"
        />
        <span class="slider"></span>
        <span class="label-text">{{ localSettings.enabled ? '已启用' : '已禁用' }}</span>
      </label>
    </div>

    <div v-if="localSettings.enabled" class="settings-content">
      <div class="setting-item">
        <label>每题最多追问次数</label>
        <div class="input-group">
          <input 
            type="number" 
            v-model.number="localSettings.maxFollowUps"
            @change="handleChange"
            min="1"
            max="3"
            class="number-input"
          />
          <span class="unit">次</span>
        </div>
        <p class="hint">建议1-2次，避免过度追问</p>
      </div>

      <div class="setting-item">
        <label class="checkbox-label">
          <input 
            type="checkbox" 
            v-model="localSettings.autoTrigger"
            @change="handleChange"
          />
          <span>自动触发追问</span>
        </label>
        <p class="hint">如果关闭，需要手动选择是否追问</p>
      </div>

      <div class="setting-item" v-if="localSettings.autoTrigger">
        <label>触发阈值（回答质量低于此值时追问）</label>
        <div class="slider-container">
          <input 
            type="range" 
            v-model.number="localSettings.triggerThreshold"
            @change="handleChange"
            min="1"
            max="5"
            step="1"
            class="range-slider"
          />
          <div class="slider-labels">
            <span>1 (总是)</span>
            <span>2</span>
            <span>3</span>
            <span>4</span>
            <span>5 (几乎不)</span>
          </div>
        </div>
        <div class="threshold-value">当前值: {{ localSettings.triggerThreshold }}</div>
      </div>

      <div class="setting-item">
        <label>追问类型偏好</label>
        <div class="type-checkboxes">
          <label 
            v-for="type in followUpTypes" 
            :key="type.value"
            class="type-checkbox"
          >
            <input 
              type="checkbox" 
              :value="type.value"
              v-model="localSettings.preferredTypes"
              @change="handleChange"
            />
            <span class="type-label">
              <strong>{{ type.label }}</strong>
              <small>{{ type.description }}</small>
            </span>
          </label>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import type { FollowUpSettings, FollowUpType } from '../types/follow-up'
import { FOLLOWUP_TYPE_LABELS, FOLLOWUP_TYPE_DESCRIPTIONS } from '../types/follow-up'

interface Props {
  modelValue: FollowUpSettings
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:modelValue': [value: FollowUpSettings]
}>()

const localSettings = ref<FollowUpSettings>({ ...props.modelValue })

const followUpTypes = [
  { value: 'clarification', label: FOLLOWUP_TYPE_LABELS.clarification, description: FOLLOWUP_TYPE_DESCRIPTIONS.clarification },
  { value: 'deepening', label: FOLLOWUP_TYPE_LABELS.deepening, description: FOLLOWUP_TYPE_DESCRIPTIONS.deepening },
  { value: 'scenario', label: FOLLOWUP_TYPE_LABELS.scenario, description: FOLLOWUP_TYPE_DESCRIPTIONS.scenario },
  { value: 'challenge', label: FOLLOWUP_TYPE_LABELS.challenge, description: FOLLOWUP_TYPE_DESCRIPTIONS.challenge },
  { value: 'extension', label: FOLLOWUP_TYPE_LABELS.extension, description: FOLLOWUP_TYPE_DESCRIPTIONS.extension }
] as const

function handleChange() {
  // Ensure at least one type is selected
  if (localSettings.value.preferredTypes.length === 0) {
    localSettings.value.preferredTypes = ['clarification']
  }
  emit('update:modelValue', { ...localSettings.value })
}

watch(() => props.modelValue, (newVal) => {
  localSettings.value = { ...newVal }
}, { deep: true })
</script>

<style scoped>
.followup-settings {
  background: var(--bg-card-solid);
  border-radius: 10px;
  padding: 1.5rem;
  border: 1px solid var(--border-light);
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
  gap: 1rem;
}

.settings-header h4 {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-primary);
}

/* Toggle Switch */
.toggle-switch {
  display: flex;
  align-items: center;
  gap: 0.8rem;
  cursor: pointer;
}

.toggle-switch input[type="checkbox"] {
  display: none;
}

.slider {
  position: relative;
  width: 48px;
  height: 24px;
  background: var(--bg-input);
  border-radius: 24px;
  transition: background 0.3s;
  border: 1px solid var(--border-light);
}

.slider::before {
  content: '';
  position: absolute;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: white;
  top: 2px;
  left: 2px;
  transition: transform 0.3s;
}

input[type="checkbox"]:checked + .slider {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
}

input[type="checkbox"]:checked + .slider::before {
  transform: translateX(24px);
}

.label-text {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
}

/* Settings Content */
.settings-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.setting-item > label {
  font-weight: 600;
  color: var(--text-primary);
  font-size: 0.95rem;
}

.input-group {
  display: flex;
  align-items: center;
  gap: 0.8rem;
}

.number-input {
  width: 80px;
  padding: 0.6rem;
  border: 1px solid var(--border-light);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 1rem;
  text-align: center;
}

.number-input:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.unit {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.8rem;
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
  accent-color: var(--accent-primary);
}

.checkbox-label span {
  font-weight: 500;
  color: var(--text-primary);
}

.hint {
  margin: 0;
  font-size: 0.85rem;
  color: var(--text-secondary);
  padding-left: 0.2rem;
}

/* Range Slider */
.slider-container {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.range-slider {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: var(--bg-input);
  outline: none;
  cursor: pointer;
}

.range-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: var(--accent-primary);
  cursor: pointer;
  border: 2px solid white;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.range-slider::-moz-range-thumb {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: var(--accent-primary);
  cursor: pointer;
  border: 2px solid white;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.slider-labels {
  display: flex;
  justify-content: space-between;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.threshold-value {
  text-align: center;
  font-weight: 600;
  color: var(--accent-primary);
  font-size: 0.9rem;
}

/* Type Checkboxes */
.type-checkboxes {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 0.5rem 0;
}

.type-checkbox {
  display: flex;
  align-items: flex-start;
  gap: 0.8rem;
  cursor: pointer;
  padding: 0.8rem;
  background: var(--bg-secondary);
  border-radius: 6px;
  border: 1px solid var(--border-light);
  transition: all 0.3s;
}

.type-checkbox:hover {
  border-color: var(--accent-primary);
  background: var(--bg-hover);
}

.type-checkbox input[type="checkbox"] {
  margin-top: 0.2rem;
  width: 18px;
  height: 18px;
  cursor: pointer;
  accent-color: var(--accent-primary);
}

.type-label {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
}

.type-label strong {
  color: var(--text-primary);
  font-size: 0.95rem;
}

.type-label small {
  color: var(--text-secondary);
  font-size: 0.85rem;
  line-height: 1.4;
}
</style>
