<template>
  <div class="banner" tabindex="0" role="button">
    <img :src="icon" alt="" class="icon">
    <div class="copy">
      <div class="name">{{ name }}</div>
      <div class="intro">{{ intro }}</div>
    </div>
    <div class="button">{{ $t("appBanner.install") }}</div>
  </div>
</template>

<script setup lang='ts'>
defineProps<{
  name: string,
  intro: string,
  icon: string,
}>();
</script>

<style scoped>
.banner {
  display: flex;
  align-items: center;
  width: 100%;
  min-height: 82px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  background-color: var(--color-surface);
  cursor: pointer;
  transition: background-color 0.2s;
}

.banner:hover {
  background-color: var(--color-surface-hover);
}

.icon {
  width: 58px;
  height: 58px;
  margin: 12px;
  object-fit: contain;
}

.copy {
  min-width: 0;
}

.name {
  color: var(--color-text);
  font-size: 22px;
  line-height: 26px;
}

.intro {
  margin-top: 4px;
  color: var(--color-text-muted);
  font-size: 15px;
}

.button {
  margin: auto 15px auto auto;
  min-width: 120px;
  height: 35px;
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  color: var(--color-text);
  background-color: var(--color-button-green);
  font-size: 18px;
  text-align: center;
  line-height: 33px;
}

.button:hover {
  border-color: var(--color-text);
}
</style>
