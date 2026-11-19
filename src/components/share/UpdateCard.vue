<template>
  <div class="update-card" :class="tone" aria-live="polite">
    <div class="card-content">
      <div class="title">{{ title }}</div>
      <p class="status">{{ detail }}</p>
      <progress v-if="checking" aria-label="正在检查更新" />
      <p v-if="!checking && !error && !recent" class="greeting">{{ securityClassificationAvailable ? '更新前请保存工作。' : '安全更新分类暂不可用。' }}</p>
      <p class="greeting">{{ checkedAt ? `上次由 aoska 检查：${formatActivityDate(checkedAt, now)}` : '尚未通过 aoska 检查更新' }}</p>
      <button class="button" type="button" :disabled="buttonDisabled" @click="act">{{ button }}</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onUnmounted, ref } from 'vue';
import { updateActivity, formatActivityDate, isStale, activityPresentation } from '../../utils/updateActivity';
const props = withDefaults(defineProps<{ update?: number; updateSecurity?: number; securityClassificationAvailable?: boolean; isLoading?: boolean; error?: string; disabled?: boolean; lastCheckedAt?: string; stale?: boolean; canRefresh?: boolean; canUpdate?: boolean }>(), { update: 0, updateSecurity: 0, securityClassificationAvailable: true, canRefresh: true, canUpdate: true });
const emit = defineEmits<{ action: [intent?: 'refresh' | 'update' | 'retry'] }>();
const activity = ref(updateActivity.get());
const stop = updateActivity.subscribe(() => { activity.value = updateActivity.get(); });
const now = ref(Date.now());
const timer = setInterval(() => { now.value = Date.now(); }, 1000);
onUnmounted(() => { stop(); clearInterval(timer); });
const checkedAt = computed(() => activity.value.lastCheckedAt ?? props.lastCheckedAt);
const recent = computed(() => activity.value.result);
const outdated = computed(() => props.stale || isStale(checkedAt.value, now.value));
const presentation = computed(() => activityPresentation({
  activity: activity.value,
  loading: props.isLoading,
  error: props.error,
  security: props.updateSecurity,
  updates: props.update,
  stale: outdated.value,
}));
const checking = computed(() => presentation.value.state === 'checking');
const tone = computed(() => presentation.value.tone);
const title = computed(() => {
  switch (presentation.value.state) {
    case 'unknown': return '操作状态未知';
    case 'checking': return '正在检查软件包更新';
    case 'error': return '无法获取更新状态';
    case 'completed': return '操作完成';
    case 'failed': return '操作未完成';
    case 'loading': return '正在读取更新状态';
    case 'security': return '有可用的安全补丁！';
    default: return props.update ? '有软件包更新！' : '一切如常！';
  }
});
const detail = computed(() => {
  switch (presentation.value.state) {
    case 'unknown': return activity.value.uncertain?.message ?? '无法确认操作结果，请检查系统状态。';
    case 'checking': {
      const elapsed = Math.max(0, Math.floor((now.value - Date.parse(activity.value.current!.startedAt)) / 1000));
      return `已用时间 ${elapsed} 秒`;
    }
    case 'error': return props.error;
    case 'completed': return `包管理操作已完成（${formatActivityDate(recent.value!.finishedAt!, now.value)}）。`;
    case 'failed': return recent.value?.message ?? '请查看操作日志。';
    case 'loading': return '正在读取软件包状态…';
    case 'security': return `${props.updateSecurity} 个安全更新，另有 ${props.update} 个常规更新`;
    default: {
      const counts = props.update ? `${props.update} 个可用软件包更新` : '当前软件包索引中没有可用更新。';
      return counts + (outdated.value ? ' 建议重新检查更新。' : '');
    }
  }
});
const refreshFailed = computed(() => recent.value?.state === 'failed' && recent.value.operation === 'refresh');
const intent = computed<'refresh' | 'update' | 'retry'>(() => {
  if (props.error) return 'retry';
  if (refreshFailed.value || outdated.value || (!props.update && !props.updateSecurity)) return 'refresh';
  return 'update';
});
const button = computed(() => {
  if (activity.value.uncertain) return '操作结果未知，请检查系统状态';
  if (checking.value) return '检查中…';
  if (props.error) return '重试';
  if (recent.value) return refreshFailed.value ? '重新检查' : '好';
  return intent.value === 'refresh' ? '查找更新' : '查看更新';
});
const buttonDisabled = computed(() => {
  if (props.disabled || props.isLoading || activity.value.current) return true;
  if (recent.value) return false;
  if (intent.value === 'refresh') return !props.canRefresh;
  if (intent.value === 'update') return !props.canUpdate;
  return false;
});
function act() {
  if (recent.value && !props.error) {
    const retry = refreshFailed.value;
    updateActivity.dismiss();
    if (retry) emit('action', 'refresh');
    return;
  }
  emit('action', intent.value);
}

</script>

<style scoped>
.update-card {
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 1px solid transparent;
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-card);
}

.card-content {
  display: flex;
  flex-direction: column;
  min-height: 100%;
  flex: 1;
  padding: 25px;
}

.checking-update,
.system-update {
  border-color: var(--color-info-border);
  background-color: var(--color-info);
}

.warning-update { border-color: var(--color-warning-border); background: var(--color-warning); }
progress { width: 100%; margin-top: 12px; }

.no-update {
  border-color: var(--color-success-border);
  background-color: var(--color-success);
}

.security-update {
  border-color: var(--color-security-border);
  background-color: var(--color-security);
}

.title {
  color: var(--color-text);
  font-size: 32px;
  line-height: 38px;
  font-weight: 700;
}

.status {
  margin-top: 14px;
  color: var(--color-text);
  font-size: 16px;
}

.greeting {
  margin-top: 8px;
  color: var(--color-text-muted);
  font-size: 16px;
  line-height: 24px;
}

.button {
  align-self: flex-end;
  min-width: 96px;
  height: 35px;
  margin-top: auto;
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: rgba(255, 255, 255, 0.03);
  color: var(--color-text);
  font-size: 16px;
  cursor: pointer;
}

.button:hover {
  border-color: var(--color-text);
}

</style>
