<template>
  <div class="navigator">
    <RouterLink
      v-for="item in navItems"
      :key="item.to"
      :to="item.to"
      custom
      v-slot="{ href, isActive, navigate }"
    >
      <a
        :href="href"
        :class="{ active: isActive }"
        @click="navigate"
      >
        {{ $t(item.label) }}
      </a>
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
const navItems = [
  { to: "/home", label: "app.home" },
  { to: "/category/working", label: "app.working" },
  { to: "/category/games", label: "app.games" },
  { to: "/category/video", label: "app.video" },
  { to: "/category/creating", label: "app.creating" },
  { to: "/view-all", label: "app.observing" },
  { to: "/updates", label: "app.updates" },
];
</script>

<style>
.navigator {
  display: flex;
  width: 100%;
  height: 40px;
  background-color: rgb(0, 53, 5);
  position: fixed;
  top: 0;
  left: 0;
  z-index: 9999;
}
.navigator a {
  display: inline-block;
  text-decoration: none;
  color: white;
  font-size: 18px;
  line-height: 30px;
  margin: 0 5px;
  padding: 0 8px;
  border-top: 5px solid transparent;
  transition: background 0.2s;
}
.navigator a:hover {
  background-color: #00000080;
}
.navigator a.active {
  border-bottom: 5px solid rgb(29, 148, 178);
}
</style>
