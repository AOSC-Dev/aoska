<template>
  <section class="storage-card">
    <div>
      <div class="size">{{ usedFormatted }}</div>
      <div class="label">{{ localeZh ? '本地软件使用空间' : 'Local software usage' }}</div>
    </div>
    <div class="bar" :title="tooltip">
      <div
        v-for="segment in normalizedSegments"
        :key="segment.id"
        class="segment"
        :style="{ width: `${segment.percent}%`, backgroundColor: segment.color }"
      ></div>
    </div>
    <div class="legend">
      <span v-for="segment in storage.segments" :key="segment.id">
        <i :style="{ backgroundColor: segment.color }"></i>{{ segment.label }}
      </span>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { filesize } from 'filesize';
import type { PmStorageSummary } from '../../../types/pm';

const props = defineProps<{
  storage: PmStorageSummary;
}>();

const localeZh = navigator.language.startsWith('zh');
const usedFormatted = computed(() => filesize(props.storage.totalUsedBytes, { standard: 'iec' }));
const tooltip = computed(() => localeZh ? '此处以系统所在磁盘的空间作估算' : 'Estimated from the system disk.');
const normalizedSegments = computed(() => props.storage.segments.map((segment) => ({
  ...segment,
  percent: props.storage.totalBytes > 0 ? Math.max(1, (segment.bytes / props.storage.totalBytes) * 100) : 0,
})));
</script>

<style scoped>
.storage-card {
  min-height: 190px;
  padding: 28px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  background: var(--color-surface);
}

.size {
  color: #008b8b;
  font-size: 42px;
  line-height: 48px;
  font-weight: 700;
}

.label {
  color: #008b8b;
  font-size: 18px;
}

.bar {
  display: flex;
  height: 12px;
  margin-top: 42px;
  overflow: hidden;
  border-radius: 2px;
  background: #4b4f52;
}

.segment {
  height: 100%;
}

.legend {
  display: flex;
  flex-wrap: wrap;
  gap: 10px 18px;
  margin-top: 12px;
  color: var(--color-text-muted);
  font-size: 13px;
}

.legend span {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.legend i {
  width: 12px;
  height: 12px;
  border-radius: 2px;
}
</style>
