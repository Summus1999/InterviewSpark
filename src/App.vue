<template>
  <div id="app" class="app-container">
    <header>
      <h1>InterviewSpark</h1>
      <p>AI-Powered Mock Interview Platform</p>
    </header>
    <main>
      <section class="welcome">
        <h2>Welcome to InterviewSpark</h2>
        <p>Prepare for your interviews with AI-powered mock interviews and intelligent feedback.</p>
      </section>
    </main>
  </div>
</template>

<script setup lang="ts">
// Vue 3 component setup
</script>

<style scoped>
.app-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
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
</style>
