<template>
  <div class="card" tabindex="0" role="button">
    <div class="date">
      <div class="day">{{ day }}</div>
      <div class="month">{{ month }}</div>
    </div>
    <div class="content">
      <div class="title">{{ title }}</div>
      <div class="intro">{{ intro }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  month: string,
  day: number,
  title: string,
  intro: string,
}>();
</script>

<style scoped>
.card {
  position: relative;
  width: 100%;
  min-height: 78px;
  border-radius: var(--radius-card);
  display: flex;
  overflow: hidden;
  cursor: pointer;
  background-color: var(--color-surface);
}

.card::before {
  content: "";
  position: absolute;
  inset: 0 auto 0 0;
  width: 78px;
  background-color: var(--color-surface-raised);
  transition: width 0.3s ease;
  z-index: 0;
}

.card:hover::before {
  width: 100%;
}

.card > * {
  position: relative;
  z-index: 1;
}

.date {
  width: 78px;
  min-width: 78px;
  min-height: 78px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 5px;
}

.day {
  color: var(--color-text);
  font-size: 28px;
  line-height: 28px;
  text-align: center;
}

.month {
  color: var(--color-text-muted);
  font-size: 15px;
  line-height: 15px;
  text-align: center;
}

.content {
  min-width: 0;
  padding: 15px 12px 12px 10px;
}

.title {
  color: var(--color-text);
  font-size: 19px;
  line-height: 20px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.intro {
  margin-top: 8px;
  color: var(--color-text-muted);
  font-size: 14px;
  line-height: 14px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
