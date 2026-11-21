<template>
  <button class="card" type="button">
    <img v-if="image && !imageFailed" :src="image" class="app-icon" alt="" @error="imageFailed = true" />
    <span v-else class="app-icon fallback-icon" aria-hidden="true">{{ name.slice(0, 1).toLocaleUpperCase() }}</span>
    <span class="copy">
      <span class="name">{{ name }}</span>
      <span v-if="intro" class="intro">{{ intro }}</span>
    </span>
  </button>
</template>

<script setup lang='ts'>
import { ref, watch } from 'vue';

const props = defineProps<{
  name: string,
  intro?: string,
  image?: string | null
}>();
const imageFailed = ref(false);
watch(() => props.image, () => { imageFailed.value = false; });
</script>

<style scoped>
.card {
  display: flex;
  flex-direction: row;
  align-items: center;
  min-width: 0;
  height: 74px;
  padding: 10px;
  border: 1px solid transparent;
  border-radius: var(--radius-card);
  background-color: var(--color-surface-raised);
  box-shadow: 0 1px 0 rgba(255, 255, 255, 0.03) inset;
  cursor: pointer;
  text-align: left;
  transition: background-color 0.2s, border-color 0.2s, transform 0.2s;
}

.card:hover {
  border-color: var(--color-border);
  background-color: var(--color-surface-hover);
  transform: translateY(-1px);
}

.app-icon {
  width: 52px;
  height: 52px;
  margin-right: 12px;
  object-fit: contain;
}

.fallback-icon {
  display: grid;
  place-items: center;
  flex-shrink: 0;
  border-radius: 8px;
  background: var(--color-surface-soft);
  color: var(--color-text-muted);
  font-size: 28px;
}

.copy {
  min-width: 0;
}

.name, .intro {
  display: block;
}

.name {
  color: var(--color-text);
  font-size: 20px;
  line-height: 22px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.intro {
  margin-top: 5px;
  color: var(--color-text-muted);
  font-size: 14px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
