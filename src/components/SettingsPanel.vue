<template>
  <div class="settings-panel-container">
    <button
      class="settings-trigger"
      @click="togglePanel"
      :title="'设置'"
    >
      <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
          d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
          d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
      </svg>
      <span class="label">设置</span>
    </button>

    <!-- Settings Dropdown Panel -->
    <transition name="slide-fade">
      <div v-if="showPanel" class="settings-dropdown" @click.stop>
        <div class="settings-item">
          <label class="settings-label">主题</label>
          <select v-model="localTheme" @change="handleThemeChange" class="settings-select">
            <option value="light">亮色</option>
            <option value="dark">暗色</option>
          </select>
        </div>

        <div class="settings-item">
          <label class="settings-label">模型</label>
          <select v-model="localSettings.model" class="settings-select">
            <option v-for="model in availableModels" :key="model.value" :value="model.value">
              {{ model.label }}
            </option>
          </select>
        </div>

        <div class="settings-item">
          <label class="settings-label">API Key</label>
          <div class="api-key-input-wrapper">
            <input
              v-model="localSettings.apiKey"
              :type="showApiKey ? 'text' : 'password'"
              placeholder="请输入硅基流动 API Key"
              class="settings-input"
            />
            <button @click="toggleApiKeyVisibility" class="toggle-visibility-btn" type="button">
              <svg v-if="showApiKey" class="icon-small" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
              </svg>
              <svg v-else class="icon-small" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
              </svg>
            </button>
          </div>
        </div>

        <div class="settings-actions">
          <button @click="handleSave" class="save-btn">
            保存设置
          </button>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { ThemeManager, ApiSettingsManager, AVAILABLE_MODELS, type Theme, type ApiSettings } from '../services/settings'
import { invoke } from '@tauri-apps/api/core'

const showPanel = ref(false)
const showApiKey = ref(false)
const localTheme = ref<Theme>('light')
const localSettings = ref<ApiSettings>({
  model: 'Qwen/Qwen2.5-7B-Instruct',
  apiKey: ''
})

const availableModels = AVAILABLE_MODELS
const themeManager = ThemeManager.getInstance()

onMounted(() => {
  localTheme.value = themeManager.getTheme()
  localSettings.value = ApiSettingsManager.getSettings()
  
  // Add click outside listener
  document.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
})

function togglePanel() {
  showPanel.value = !showPanel.value
}

function handleClickOutside(event: MouseEvent) {
  const target = event.target as HTMLElement
  const container = document.querySelector('.settings-panel-container')
  if (container && !container.contains(target)) {
    showPanel.value = false
  }
}

function handleThemeChange() {
  themeManager.setTheme(localTheme.value)
}

function toggleApiKeyVisibility() {
  showApiKey.value = !showApiKey.value
}

async function handleSave() {
  try {
    // Save to localStorage
    ApiSettingsManager.saveSettings(localSettings.value)
    
    // Update backend configuration via Tauri command
    await invoke('update_api_config', {
      model: localSettings.value.model,
      apiKey: localSettings.value.apiKey
    })
    
    alert('设置已保存')
    showPanel.value = false
  } catch (error) {
    console.error('Failed to save settings:', error)
    alert('保存失败：' + String(error))
  }
}
</script>

<style scoped>
.settings-panel-container {
  position: relative;
}

.settings-trigger {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  padding: 0.6rem 1rem;
  background: var(--bg-input);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-light);
  cursor: pointer;
  transition: all 0.3s;
  font-size: 0.9rem;
}

.settings-trigger:hover {
  background: var(--bg-hover);
  border-color: var(--border-hover);
}

.icon {
  width: 18px;
  height: 18px;
}

.icon-small {
  width: 16px;
  height: 16px;
}

.label {
  font-weight: 500;
}

.settings-dropdown {
  position: absolute;
  top: calc(100% + 0.5rem);
  right: 0;
  min-width: 320px;
  background: var(--bg-card-solid);
  border: 2px solid var(--border-light);
  border-radius: 12px;
  padding: 1.5rem;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  z-index: 1000;
}

.settings-item {
  margin-bottom: 1.2rem;
}

.settings-item:last-of-type {
  margin-bottom: 1.5rem;
}

.settings-label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-primary);
}

.settings-select,
.settings-input {
  width: 100%;
  padding: 0.6rem 0.8rem;
  background: var(--bg-input);
  border: 1px solid var(--border-light);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 0.9rem;
  transition: border-color 0.3s;
}

.settings-select:focus,
.settings-input:focus {
  outline: none;
  border-color: var(--accent-primary);
}

.api-key-input-wrapper {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.api-key-input-wrapper .settings-input {
  flex: 1;
}

.toggle-visibility-btn {
  padding: 0.6rem;
  background: var(--bg-input);
  border: 1px solid var(--border-light);
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.toggle-visibility-btn:hover {
  background: var(--bg-hover);
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.settings-actions {
  display: flex;
  justify-content: center;
  margin-top: 1rem;
}

.save-btn {
  padding: 0.7rem 2rem;
  background: var(--accent-gradient);
  color: var(--text-light);
  border: none;
  border-radius: 8px;
  font-size: 0.95rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.save-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

/* Transition animations */
.slide-fade-enter-active {
  transition: all 0.2s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.15s ease-in;
}

.slide-fade-enter-from {
  transform: translateY(-10px);
  opacity: 0;
}

.slide-fade-leave-to {
  transform: translateY(-5px);
  opacity: 0;
}

@media (max-width: 768px) {
  .label {
    display: none;
  }
  
  .settings-dropdown {
    min-width: 280px;
  }
}
</style>
