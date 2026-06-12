<template>
  <div class="app-shell">
    <NavBar />
    <main class="app-main">
      <RouterView />
    </main>
  </div>
</template>

<script setup lang='ts'>
import { RouterView } from 'vue-router';
import NavBar from './components/share/NavBar.vue';
</script>

<style>
* {
  margin: 0;
  padding: 0;
}

body {
  background: transparent;
}

.app-shell {
  min-height: 100%;
  background:
    radial-gradient(circle at 18% 0%, rgba(24, 86, 45, 0.32), transparent 34%),
    linear-gradient(180deg, #111416 0%, var(--color-app-bg) 220px, var(--color-app-bg) 100%);
  color: var(--color-text);
}

.app-main {
  min-height: calc(100vh - 42px);
}
</style>
