<template>
  <div class="welcome">
    <div class="scrim">
      <div class="sub-title">{{ $t("welcome.sub-title") }}</div>
      <div class="title">{{ $t("welcome.title") }}</div>
    </div>
  </div>
</template>

<script setup lang='ts'></script>

<style scoped>
.welcome {
  overflow: hidden;
  border-radius: var(--radius-card);
  background:
    linear-gradient(90deg, rgba(0, 0, 0, 0.65), rgba(0, 0, 0, 0.08)),
    radial-gradient(circle at 75% 30%, rgba(245, 180, 0, 0.28), transparent 24%),
    linear-gradient(135deg, #102b18, #27311f 48%, #765615);
  box-shadow: var(--shadow-card);
}

.scrim {
  height: 100%;
  padding: 66px 30px 24px;
}

.sub-title {
  font-size: 28px;
  color: var(--color-text);
  font-weight: 600;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.8);
}

.title {
  margin-top: 4px;
  font-size: 38px;
  color: var(--color-text);
  font-weight: 600;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.8);
}
</style>
