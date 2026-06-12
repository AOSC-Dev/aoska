<template>
  <teleport to="body">
    <div v-if="operation" class="dialog-backdrop" role="presentation">
      <section class="dialog" role="dialog" aria-modal="true" :aria-label="heading">
        <header class="dialog-header">
          <div>
            <div class="eyebrow">{{ operation.unit }}</div>
            <h2>{{ heading }}</h2>
          </div>
          <button class="icon-button" type="button" @click="$emit('close')">×</button>
        </header>

        <div class="summary">
          <span class="pill" :class="status?.state ?? 'running'">{{ status?.state ?? 'running' }}</span>
          <span>{{ operation.packages.join(', ') || operation.operation }}</span>
        </div>

        <pre class="logs">{{ logText }}</pre>

        <footer class="actions">
          <button
            v-if="canCancel"
            class="secondary danger"
            type="button"
            @click="cancel"
          >
            {{ normalizeLocale(locale) === 'zh' ? '取消' : 'Cancel' }}
          </button>
          <button class="primary" type="button" @click="$emit('close')">
            {{ normalizeLocale(locale) === 'zh' ? '好' : 'OK' }}
          </button>
        </footer>
      </section>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import type { PmOperationStart, PmOperationStatus } from '../../types/pm';
import { getPackageManager } from '../../utils/pmClient';
import { normalizeLocale, operationHeading } from '../../utils/pmPresentation';

const props = defineProps<{
  operation: PmOperationStart | null;
}>();

defineEmits<{
  close: [];
}>();

const locale = navigator.language;
const status = ref<PmOperationStatus | null>(null);
const logs = ref<string[]>([]);

const heading = computed(() => {
  if (!props.operation) return '';
  return operationHeading(props.operation.operation, status.value?.state ?? 'running', locale);
});

const logText = computed(() => logs.value.join('\n'));
const canCancel = computed(() => ['pending', 'running'].includes(status.value?.state ?? 'running'));

async function refreshOperation() {
  if (!props.operation) return;
  const pm = getPackageManager();
  status.value = await pm.operationStatus(props.operation.unit);
  logs.value = (await pm.operationLogs(props.operation.unit)).lines;
}

async function cancel() {
  if (!props.operation) return;
  await getPackageManager().cancelOperation(props.operation.unit);
  await refreshOperation();
}

watch(
  () => props.operation?.unit,
  async () => {
    status.value = null;
    logs.value = [];
    await refreshOperation();
  },
  { immediate: true },
);
</script>

<style scoped>
.dialog-backdrop {
  position: fixed;
  inset: 0;
  z-index: 10000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  background: rgba(0, 0, 0, 0.62);
  backdrop-filter: blur(4px);
}

.dialog {
  width: min(680px, 100%);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  background: var(--color-surface);
  color: var(--color-text);
  box-shadow: var(--shadow-card);
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  gap: 18px;
  padding: 20px 22px 12px;
}

.eyebrow {
  color: var(--color-text-subtle);
  font-size: 13px;
}

h2 {
  margin-top: 4px;
  font-size: 28px;
  font-weight: 600;
}

.icon-button {
  width: 32px;
  height: 32px;
  border: 1px solid var(--color-border);
  border-radius: 50%;
  background: transparent;
  color: var(--color-text);
  cursor: pointer;
}

.summary {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 0 22px 16px;
  color: var(--color-text-muted);
}

.pill {
  padding: 3px 10px;
  border-radius: 999px;
  border: 1px solid var(--color-info-border);
  color: var(--color-text);
  background: var(--color-info);
  font-size: 12px;
}

.pill.cancelled,
.pill.failed {
  border-color: var(--color-danger-border);
  background: var(--color-danger);
}

.pill.succeeded,
.pill.already_terminal {
  border-color: var(--color-success-border);
  background: var(--color-success);
}

.logs {
  min-height: 150px;
  max-height: 260px;
  margin: 0 22px;
  padding: 14px;
  overflow: auto;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  background: #101214;
  color: var(--color-text-muted);
  white-space: pre-wrap;
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 22px 22px;
}

.primary,
.secondary {
  min-width: 96px;
  height: 35px;
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  color: var(--color-text);
  cursor: pointer;
}

.primary {
  background: var(--color-button-green);
}

.secondary {
  background: var(--color-button-grey);
}

.danger {
  background: var(--color-button-red);
}
</style>
