<template>
  <section class="banner" :aria-label="$t('home.banners')">
    <div ref="track" class="track" @scroll.passive="syncPosition">
      <WelcomeCard
        v-for="(slide, index) in slides"
        :key="index"
        :image="slide.image"
        :title="slide.title"
        :subtitle="slide.subtitle"
        :tabindex="index === active ? 0 : -1"
        @click="$emit('activate', slide)"
      />
    </div>
    <div v-if="slides.length > 1" class="controls">
      <button type="button" :aria-label="$t('home.previousBanner')" :disabled="active === 0" @click="step(-1)">‹</button>
      <span aria-live="polite">{{ active + 1 }} / {{ slides.length }}</span>
      <button
        type="button"
        :aria-label="$t('home.nextBanner')"
        :disabled="active === slides.length - 1"
        @click="step(1)"
      >›</button>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import type { CatalogBanner } from '../../utils/asmr';
import WelcomeCard from './WelcomeCard.vue';

const props = defineProps<{ slides: CatalogBanner[] }>();
defineEmits<{ activate: [slide: CatalogBanner] }>();

const track = ref<HTMLElement | null>(null);
const active = ref(0);

function syncPosition() {
  const el = track.value;
  if (el?.clientWidth) active.value = Math.round(el.scrollLeft / el.clientWidth);
}

function step(offset: number) {
  const el = track.value;
  const index = active.value + offset;
  if (el && index >= 0 && index < props.slides.length) {
    el.scrollTo({ left: index * el.clientWidth });
  }
}
</script>

<style scoped>
.banner {
  min-width: 0;
  height: 200px;
  position: relative;
}

.track {
  display: flex;
  height: 100%;
  overflow-x: auto;
  border-radius: var(--radius-card);
  scroll-snap-type: x mandatory;
  scroll-behavior: smooth;
  scrollbar-width: none;
}

.track > * {
  flex: 0 0 100%;
  scroll-snap-align: start;
}

.track > :focus-visible {
  outline-offset: -3px;
}

.controls {
  position: absolute;
  top: 10px;
  right: 10px;
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 3px;
  color: white;
  background: rgba(0, 0, 0, 0.72);
  border-radius: var(--radius-control);
  font-variant-numeric: tabular-nums;
  font-size: 12px;
}

.controls button {
  width: 32px;
  height: 32px;
  border: 0;
  border-radius: var(--radius-control);
  background: transparent;
  color: inherit;
  font-size: 24px;
  line-height: 1;
  cursor: pointer;
}

.controls button:not(:disabled):hover {
  background: rgba(255, 255, 255, 0.2);
}

.controls button:disabled {
  opacity: 0.45;
  cursor: default;
}

@media (prefers-reduced-motion: reduce) {
  .track { scroll-behavior: auto; }
}
</style>
