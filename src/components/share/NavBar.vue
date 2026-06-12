<template>
  <div class="navigator" data-tauri-drag-region>
    <div class="nav-left">
      <RouterLink
        v-for="item in navItems"
        :key="item.to"
        :to="item.to"
        custom
        v-slot="{ href, isActive, navigate }"
        class="router-link"
      >
        <a :href="href" :class="{ active: isActive }" @click="navigate">
          {{ $t(item.label) }}
        </a>
      </RouterLink>
    </div>
    <div class="nav-right">
      <label class="search-input" :aria-label="$t('app.search')">
        <input :placeholder="$t('app.search')" />
        <span class="search-icon">⌕</span>
      </label>

      <div v-if="isTauriRuntime" class="window-controls">
        <button class="win-btn" title="Minimize" @click="onMinimize()">⌄</button>
        <button class="win-btn" title="Maximize" @click="onMaximize()">⌃</button>
        <button class="win-btn close" title="Close" @click="onClose()">×</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';

const isTauriRuntime = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

const navItems = [
  { to: "/home", label: "app.home" },
  { to: "/category/working", label: "app.working" },
  { to: "/category/games", label: "app.games" },
  { to: "/category/video", label: "app.video" },
  { to: "/category/creating", label: "app.creating" },
  { to: "/view-all", label: "app.view-all" },
  { to: "/updates", label: "app.updates" },
];

const appWindow = isTauriRuntime ? getCurrentWindow() : null;

const onMinimize = () => { void appWindow?.minimize(); };
const onMaximize = () => { void appWindow?.toggleMaximize(); };
const onClose = () => { void appWindow?.close(); };
</script>

<style scoped>
.navigator {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  height: 42px;
  background:
    linear-gradient(90deg, rgba(2, 47, 17, 0.96), rgba(8, 83, 27, 0.84)),
    radial-gradient(circle at 75% 10%, rgba(255, 255, 255, 0.16), transparent 30%);
  position: fixed;
  top: 0;
  left: 0;
  z-index: 9999;
  padding: 0 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.14);
}

.nav-left,
.nav-right {
  display: flex;
  align-items: center;
}

.navigator a {
  display: inline-flex;
  align-items: center;
  height: 42px;
  padding: 0 10px;
  color: white;
  font-size: 17px;
  text-decoration: none;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.7);
  transition: background 0.2s, box-shadow 0.2s;
}

.navigator a:hover {
  background-color: rgba(0, 0, 0, 0.24);
}

.navigator a.active {
  box-shadow: inset 0 -4px 0 0 var(--color-accent);
}

.nav-right {
  gap: 10px;
}

.search-input {
  display: flex;
  align-items: center;
  width: 260px;
  height: 26px;
  border: 1px solid rgba(255, 255, 255, 0.7);
  border-radius: var(--radius-control);
  background-color: rgba(0, 0, 0, 0.16);
  color: white;
}

.search-input input {
  min-width: 0;
  flex: 1;
  height: 100%;
  padding: 0 8px;
  border: 0;
  outline: none;
  background: transparent;
  color: white;
}

.search-input input::placeholder {
  color: rgba(255, 255, 255, 0.74);
}

.search-icon {
  padding: 0 7px;
  color: rgba(255, 255, 255, 0.86);
}

.window-controls {
  display: flex;
  align-items: center;
  gap: 6px;
}

.win-btn {
  width: 22px;
  height: 22px;
  border: 0;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  cursor: pointer;
  line-height: 20px;
}

.win-btn:hover {
  background-color: rgba(255, 255, 255, 0.22);
}

.win-btn.close {
  background: rgba(255, 255, 255, 0.86);
  color: #0f3a19;
}
</style>
