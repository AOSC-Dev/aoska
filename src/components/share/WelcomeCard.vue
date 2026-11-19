<template>
  <button class="welcome" type="button">
    <img :src="image" alt="" />
    <span class="scrim">
      <span v-if="subtitle" class="sub-title">{{ subtitle }}</span>
      <span class="title">{{ title }}</span>
    </span>
  </button>
</template>

<script setup lang="ts">
defineProps<{ image: string; title: string; subtitle?: string }>();
</script>

<style scoped>
.welcome {
  position: relative;
  overflow: hidden;
  border: 0;
  border-radius: var(--radius-card);
  background: var(--color-surface);
  box-shadow: var(--shadow-card);
  text-align: left;
  cursor: pointer;
  color: white;
}

img {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.scrim {
  position: relative;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
  height: 100%;
  padding: 24px 30px;
  background: linear-gradient(90deg, rgba(0, 0, 0, 0.8), rgba(0, 0, 0, 0.12));
}

.sub-title {
  font-size: 20px;
}

.title {
  margin-top: 4px;
  font-size: 38px;
  font-weight: 600;
}
</style>
