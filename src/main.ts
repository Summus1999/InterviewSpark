import { createApp } from 'vue'
import App from './App.vue'
import { ThemeManager } from './services/settings'
import './styles/themes.css'
import './styles/responsive.css'

// Initialize theme before mounting app
const themeManager = ThemeManager.getInstance()

createApp(App).mount('#app')
