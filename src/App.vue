<template>
  <div
    id="app"
    class="app-container"
  >
    <header>
      <h1>InterviewSpark</h1>
      <p>AI-Powered Mock Interview Platform</p>
    </header>
    <main>
      <section class="welcome">
        <h2>Welcome to InterviewSpark</h2>
        <p>Prepare for your interviews with AI-powered mock interviews and intelligent feedback.</p>

        <div class="demo-section">
          <input
            v-model="userName"
            type="text"
            placeholder="Enter your name"
            class="name-input"
          >
          <button
            class="greet-btn"
            @click="handleGreet"
          >
            Test IPC Connection
          </button>
          <p
            v-if="greeting"
            class="greeting-message"
          >
            {{ greeting }}
          </p>
        </div>
      </section>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// User input state
const userName = ref('')
const greeting = ref('')

/**
 * Handle greet button click
 * Calls Rust backend via Tauri IPC to test communication
 */
const handleGreet = async () => {
  if (!userName.value.trim()) {
    greeting.value = 'Please enter your name'
    return
  }

  try {
    // Invoke Rust command 'greet' with user's name
    greeting.value = await invoke<string>('greet', { name: userName.value })
  } catch (error) {
    greeting.value = `Error: ${error}`
  }
}
</script>

<style scoped>
.app-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  font-family:
    -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
}

header {
  padding: 2rem;
  text-align: center;
  border-bottom: 2px solid rgba(255, 255, 255, 0.1);
}

header h1 {
  margin: 0;
  font-size: 2.5rem;
  font-weight: bold;
}

header p {
  margin: 0.5rem 0 0;
  font-size: 1rem;
  opacity: 0.9;
}

main {
  padding: 2rem;
}

.welcome {
  max-width: 800px;
  margin: 2rem auto;
  background: rgba(255, 255, 255, 0.1);
  padding: 2rem;
  border-radius: 8px;
  backdrop-filter: blur(10px);
}

.welcome h2 {
  margin-top: 0;
  font-size: 1.8rem;
}

.welcome p {
  font-size: 1.1rem;
  line-height: 1.6;
}

.demo-section {
  margin-top: 2rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  align-items: center;
}

.name-input {
  padding: 0.75rem 1rem;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 1rem;
  width: 100%;
  max-width: 300px;
  outline: none;
  transition: border-color 0.3s;
}

.name-input::placeholder {
  color: rgba(255, 255, 255, 0.6);
}

.name-input:focus {
  border-color: rgba(255, 255, 255, 0.6);
}

.greet-btn {
  padding: 0.75rem 2rem;
  background: rgba(255, 255, 255, 0.2);
  border: 2px solid rgba(255, 255, 255, 0.4);
  border-radius: 4px;
  color: white;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s;
}

.greet-btn:hover {
  background: rgba(255, 255, 255, 0.3);
  border-color: rgba(255, 255, 255, 0.6);
}

.greeting-message {
  margin-top: 1rem;
  font-size: 1.2rem;
  font-weight: 500;
  color: #fff;
}
</style>
