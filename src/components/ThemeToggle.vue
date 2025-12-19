<template>
  <button
    class="theme-toggle"
    @click="toggleTheme"
    :title="theme === 'light' ? '切换到暗色模式' : '切换到亮色模式'"
  >
    <svg v-if="theme === 'light'" class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
        d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
    </svg>
    <svg v-else class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
        d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
    </svg>
    <span class="label">{{ theme === 'light' ? '暗色' : '亮色' }}</span>
  </button>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ThemeManager, type Theme } from '../services/settings'

const theme = ref<Theme>('light')
const themeManager = ThemeManager.getInstance()

onMounted(() => {
  theme.value = themeManager.getTheme()
})

function toggleTheme() {
  theme.value = themeManager.toggleTheme()
}
</script>

<style scoped>
.theme-toggle {
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

.theme-toggle:hover {
  background: var(--bg-hover);
  border-color: var(--border-hover);
}

.icon {
  width: 18px;
  height: 18px;
}

.label {
  font-weight: 500;
}

@media (max-width: 768px) {
  .label {
    display: none;
  }
}
</style>
