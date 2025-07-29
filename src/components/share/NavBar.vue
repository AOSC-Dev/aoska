<template>
  <div class="navigator" data-tauri-drag-region>
    <div class="nav-left">
      <RouterLink v-for="item in navItems" :key="item.to" :to="item.to" custom v-slot="{ href, isActive, navigate }"
        class="router-link">
        <a :href="href" :class="{ active: isActive }" @click="navigate">
          {{ $t(item.label) }}
        </a>
      </RouterLink>
    </div>
    <div class="nav-right">
      <div class="search-input">
        <input class="search-input" :placeholder="$t('app.search')"></input>
      </div>
      
      <div class="window-controls">
        <button class="win-btn" title="Minimize" v-on:click="onMinimize()">
          <svg viewBox="0 0 24 24" width="18" height="18" fill="none">
            <path d="M6 10l6 6 6-6" stroke="white" stroke-width="2" stroke-linecap="round"/>
          </svg>
        </button>
        <button class="win-btn" title="Maximize" v-on:click="onMaximize()">
          <svg viewBox="0 0 24 24" width="18" height="18" fill="none">
            <path d="M6 14l6-6 6 6" stroke="white" stroke-width="2" stroke-linecap="round"/>
          </svg>
        </button>
        <button class="win-btn" title="Close" v-on:click="onClose()">
          <svg viewBox="0 0 24 24" width="18" height="18" fill="white">
            <path d="M6 6l12 12M18 6L6 18" stroke="white" stroke-width="2" stroke-linecap="round" />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const navItems = [
  { to: "/home", label: "app.home" },
  { to: "/category/working", label: "app.working" },
  { to: "/category/games", label: "app.games" },
  { to: "/category/video", label: "app.video" },
  { to: "/category/creating", label: "app.creating" },
  { to: "/view-all", label: "app.view-all" },
  { to: "/updates", label: "app.updates" },
];

import { getCurrentWindow } from '@tauri-apps/api/window';
const appWindow = getCurrentWindow();

const onMinimize = () => { appWindow.minimize(); };
const onMaximize = () => { appWindow.toggleMaximize(); };
const onClose = () => { appWindow.close(); };
</script>

<style>
.navigator {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  height: 40px;
  background-color: rgb(0, 53, 5);
  position: fixed;
  top: 0;
  left: 0;
  z-index: 9999;
  padding: 10px 10px;
  box-sizing: border-box;
  border-radius: 5px 5px 0px 0px;
}

.router-link {
  height: 100%;
}

.nav-left {
  display: flex;
  align-items: center;
}

.navigator a {
  display: inline-block;
  text-decoration: none;
  color: white;
  font-size: 18px;
  line-height: 40px;
  margin: 0 5px;
  padding: 0 8px;
  height: 100%;
  transition: background 0.2s, box-shadow 0.2s;
}

.navigator a:hover {
  background-color: #00000080;
}

.navigator a.active {
  box-shadow: inset 0 -5px 0 0 rgb(29, 148, 178);
}

.nav-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.nav-right input {
  height: 24px;
  padding: 0 8px;
  font-size: 14px;
  border-radius: 4px;
  border: none;
  outline: none;
}

.window-controls {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-left: 10px;
}

.win-btn {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  border: none;
  background-color: transparent;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 0;
  cursor: pointer;
  transition: background-color 0.2s;
}

.win-btn:hover {
  background-color: rgba(255, 255, 255, 0.2);
}

.search-input {
  background-color: transparent;
  color: white;
  border: 1px solid #a8bca8;
  border-radius: 5px;
  width: 218px;
  height: 24px;
}

</style>
