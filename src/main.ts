import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import { ThemeManager } from './services/settings'
import './styles/themes.css'
import './styles/responsive.css'

// Initialize theme before mounting app
const themeManager = ThemeManager.getInstance()

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.mount('#app')
