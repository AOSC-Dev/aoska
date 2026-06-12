<template>
  <div class="card" tabindex="0" role="button">
    <img :src="image" class="app-icon" alt="" />
    <div class="copy">
      <div class="name">{{ name }}</div>
      <div class="intro">{{ intro }}</div>
    </div>
  </div>
</template>

<script setup lang='ts'>
defineProps<{
  name: string,
  intro: string,
  image: string
}>();
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

.copy {
  min-width: 0;
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
