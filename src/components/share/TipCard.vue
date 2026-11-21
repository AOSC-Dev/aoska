<template>
  <button class="card" type="button">
    <time class="date" :datetime="date">
      <span class="day">{{ day }}</span>
      <span class="month">{{ month }}</span>
    </time>
    <span class="content">
      <span class="title">{{ title }}</span>
      <span v-if="intro" class="intro">{{ intro }}</span>
    </span>
  </button>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";

const props = defineProps<{
  date: string,
  title: string,
  intro?: string,
}>();
const { locale } = useI18n();
const day = computed(() => Number(props.date.slice(8)));
const month = computed(() => new Intl.DateTimeFormat(locale.value, { month: "long", timeZone: "UTC" })
  .format(new Date(props.date)));
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
  border: 0;
  text-align: left;
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

.title, .intro {
  display: block;
}

.title {
  color: var(--color-text);
  font-size: 19px;
  line-height: 1.4;
  font-weight: 500;
  overflow-wrap: anywhere;
}

.intro {
  margin-top: 8px;
  color: var(--color-text-muted);
  font-size: 14px;
  line-height: 1.5;
  overflow-wrap: anywhere;
}
</style>
