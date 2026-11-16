<template>
  <teleport to="body">
    <div v-if="operation || plan" class="dialog-backdrop" role="presentation">
      <section class="dialog" role="dialog" aria-modal="true" :aria-label="heading">
        <header class="dialog-header">
          <div><h2>{{ heading }}</h2></div>
          <button class="icon-button" type="button" :disabled="running || submitting" aria-label="关闭" @click="$emit('close')">×</button>
        </header>
        <template v-if="plan">
          <div class="plan-body">
            <p>请确认以下全部软件包变更（包括依赖）：</p>
            <h3 v-if="plan.install.length">安装或更新</h3>
            <ul><li v-for="pkg in plan.install" :key="pkg.name">{{ pkg.name }} {{ pkg.currentVersion }} → {{ pkg.newVersion }}</li></ul>
            <h3 v-if="plan.remove.length">卸载</h3>
            <ul><li v-for="pkg in plan.remove" :key="pkg.name">{{ pkg.name }} {{ pkg.currentVersion }}</li></ul>
            <p v-if="!plan.install.length && !plan.remove.length">无需变更软件包。</p>
            <p>下载：{{ filesize(plan.totalDownloadSize) }} · 磁盘空间变化：{{ filesize(plan.diskSizeDelta) }}</p>
          </div>
          <p v-if="error" class="error" role="alert">{{ error }}</p>
          <footer class="actions">
            <button class="secondary" type="button" :disabled="submitting" @click="$emit('close')">取消</button>
            <button class="primary" type="button" :disabled="submitting || (!plan.install.length && !plan.remove.length)" @click="$emit('confirm')">{{ submitting ? '正在提交…' : '确认执行' }}</button>
          </footer>
        </template>
        <template v-else-if="operation">
          <div class="summary"><span class="pill" :class="status?.state ?? 'pending'">{{ heading }}</span><span>{{ operation.packages.join(', ') }}</span></div>
          <p v-if="status?.message" class="error" :role="['failed', 'unknown'].includes(status.state) ? 'alert' : undefined">{{ status.message }}</p>
          <p v-if="observationError" class="error" role="alert">{{ observationError }}（正在重试读取状态，任务可能仍在运行）</p>
          <progress v-if="status?.progress !== undefined" :value="status.progress" max="100" aria-label="操作进度" />
          <pre class="logs">{{ logs.join('\n') }}</pre>
          <footer class="actions">
            <button v-if="running && canCancel" class="secondary danger" type="button" @click="cancel">取消操作</button>
            <button class="primary" type="button" :disabled="running" @click="$emit('close')">{{ status?.state === 'unknown' ? '操作结果未知，请检查系统状态' : running ? '等待操作完成' : '好' }}</button>
          </footer>
        </template>
      </section>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { computed, onUnmounted, ref, watch } from 'vue';
import { filesize } from 'filesize';
import type { PmOperationPlan, PmOperationStart, PmOperationStatus } from '../../types/pm';
import { getPackageManager } from '../../utils/pmClient';
import { operationHeading } from '../../utils/pmPresentation';
import { isTerminalOperation, observeOperation } from '../../utils/pmOperations';

const props = defineProps<{ operation: PmOperationStart | null; plan?: PmOperationPlan | null; submitting?: boolean; error?: string }>();
const emit = defineEmits<{ close: []; confirm: []; finished: [] }>();
const status = ref<PmOperationStatus | null>(null);
const logs = ref<string[]>([]);
const observationError = ref('');
const canCancel = ref(false);
const running = computed(() => Boolean(props.operation) && !isTerminalOperation(status.value?.state ?? 'pending'));
const heading = computed(() => props.plan ? '确认软件包变更' : props.operation ? operationHeading(props.operation.operation, status.value?.state ?? 'pending', 'zh') : '');
let stop: (() => void) | undefined;

async function cancel() {
  if (!props.operation || !canCancel.value) return;
  try { await getPackageManager().cancelOperation(props.operation.id); }
  catch (cause) { observationError.value = String(cause); }
}

watch(() => props.operation?.id, (id) => {
  stop?.();
  status.value = null;
  logs.value = [];
  observationError.value = '';
  canCancel.value = false;
  if (!id) return;
  const pm = getPackageManager();
  void pm.capabilities().then(value => {
    if (props.operation?.id === id) canCancel.value = value.capabilities.includes('operation.cancel.v1');
  }).catch(() => { /* Operation observation remains available if capabilities cannot be refreshed. */ });
  stop = observeOperation(pm, id, {
    update(nextStatus, nextLogs) { status.value = nextStatus; logs.value = nextLogs; observationError.value = ''; },
    error(cause) { observationError.value = String(cause); },
    finished() { emit('finished'); },
  });
}, { immediate: true });
onUnmounted(() => stop?.());
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

.plan-body { max-height: 45vh; overflow: auto; padding: 0 22px; }
.plan-body ul { padding-left: 24px; }
.plan-body p, .plan-body h3 { margin: 12px 0; }
.error { margin: 12px 22px; overflow-wrap: anywhere; }
progress { width: calc(100% - 44px); margin: 0 22px 12px; }
button:disabled { opacity: 0.5; cursor: not-allowed; }
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

.pill.unknown,
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
