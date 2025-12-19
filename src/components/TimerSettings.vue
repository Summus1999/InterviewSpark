<template>
  <div class="timer-settings">
    <div class="settings-header">
      <h4>⏱️ 计时设置</h4>
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
        <label>每题时长（分钟）</label>
        <div class="input-group">
          <input 
            type="number" 
            v-model.number="timeMinutes"
            @change="handleTimeChange"
            min="1"
            max="30"
            class="time-input"
          />
          <span class="unit">分钟</span>
        </div>
        <div class="time-display">= {{ localSettings.timePerQuestion }} 秒</div>
      </div>

      <div class="setting-item">
        <label class="checkbox-label">
          <input 
            type="checkbox" 
            v-model="localSettings.autoSubmit"
            @change="handleChange"
          />
          <span>时间到自动提交</span>
        </label>
        <p class="hint">如果关闭,时间到时仅提醒,不会自动提交答案</p>
      </div>

      <div class="setting-item">
        <label class="checkbox-label">
          <input 
            type="checkbox" 
            v-model="localSettings.showWarning"
            @change="handleChange"
          />
          <span>剩余30秒时提醒</span>
        </label>
        <p class="hint">在时间快结束时显示警告提示</p>
      </div>

      <div class="presets">
        <p class="presets-label">快速设置：</p>
        <div class="preset-buttons">
          <button @click="applyPreset(180)" class="preset-btn">3分钟</button>
          <button @click="applyPreset(300)" class="preset-btn">5分钟</button>
          <button @click="applyPreset(600)" class="preset-btn">10分钟</button>
          <button @click="applyPreset(900)" class="preset-btn">15分钟</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'

export interface TimerConfig {
  enabled: boolean
  timePerQuestion: number  // seconds
  autoSubmit: boolean
  showWarning: boolean
}

interface Props {
  modelValue: TimerConfig
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:modelValue': [value: TimerConfig]
}>()

const localSettings = ref<TimerConfig>({ ...props.modelValue })

const timeMinutes = computed({
  get: () => Math.round(localSettings.value.timePerQuestion / 60),
  set: (val) => {
    localSettings.value.timePerQuestion = val * 60
  }
})

function handleChange() {
  emit('update:modelValue', { ...localSettings.value })
}

function handleTimeChange() {
  if (localSettings.value.timePerQuestion < 60) {
    localSettings.value.timePerQuestion = 60
  } else if (localSettings.value.timePerQuestion > 1800) {
    localSettings.value.timePerQuestion = 1800
  }
  handleChange()
}

function applyPreset(seconds: number) {
  localSettings.value.timePerQuestion = seconds
  handleChange()
}

watch(() => props.modelValue, (newVal) => {
  localSettings.value = { ...newVal }
}, { deep: true })
</script>

<style scoped>
.timer-settings {
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

.time-input {
  width: 80px;
  padding: 0.6rem;
  border: 1px solid var(--border-light);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 1rem;
  text-align: center;
}

.time-input:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.unit {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.time-display {
  font-size: 0.85rem;
  color: var(--text-secondary);
  font-style: italic;
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
  padding-left: 1.8rem;
}

/* Presets */
.presets {
  padding-top: 1rem;
  border-top: 1px solid var(--border-light);
}

.presets-label {
  margin: 0 0 0.8rem 0;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-primary);
}

.preset-buttons {
  display: flex;
  gap: 0.8rem;
  flex-wrap: wrap;
}

.preset-btn {
  padding: 0.5rem 1rem;
  background: var(--bg-input);
  border: 1px solid var(--border-light);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 0.9rem;
  cursor: pointer;
  transition: all 0.3s;
}

.preset-btn:hover {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
  color: white;
  transform: translateY(-2px);
}
</style>
