<template>
  <div class="header">
    <img v-if="appBannerImg" :src="appBannerImg" class="backdrop" alt="">
    <div class="app-banner">
      <img v-if="appIcon" :src="appIcon" class="app-img" alt="">
      <span v-else class="app-img fallback-icon" aria-hidden="true">{{ appName?.slice(0, 1).toUpperCase() }}</span>
      <div class="app-intro">
        <div class="name">{{ appName }}</div>
        <div class="intro">{{ appIntro }}</div>
        <div v-if="packageState" class="state-line">
          <span v-if="packageState.installed">{{ installedCopy }}</span>
          <span v-else>{{ notInstalledCopy }}</span>
          <span v-if="packageState.upgradable"> · {{ updateCopy }}</span>
        </div>
        <p v-if="packageState?.action.disabledReason" class="state-line" role="status">{{ packageState.action.disabledReason }}</p>
      </div>
    </div>
    <div class="actions">
      <CustomButton
        v-if="primaryAction !== 'none'"
        :disabled="disabled || primaryAction === 'manual'"
        size="medium"
        :color="primaryColor"
        width="129"
        height="37"
        @click="$emit('action', primaryAction)"
      >
        {{ actionLabel(primaryAction, locale) }}
      </CustomButton>
      <CustomButton
        v-for="action in secondaryActions"
        :key="action"
        :disabled="disabled"
        size="medium"
        color="grey"
        width="129"
        height="37"
        @click="$emit('action', action)"
      >
        {{ actionLabel(action, locale) }}
      </CustomButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import CustomButton from '../../share/CustomButton.vue';
import type { PackagePrimaryAction, PmPackageState } from '../../../types/pm';
import { actionLabel, isDestructiveAction } from '../../../utils/pmPresentation';

const { locale } = useI18n();
const props = defineProps<{
  appIcon?: string;
  appName?: string;
  appIntro?: string;
  appBannerImg?: string;
  packageState?: PmPackageState | null;
  disabled?: boolean;
}>();

defineEmits<{
  action: [action: PackagePrimaryAction];
}>();

const primaryAction = computed<PackagePrimaryAction>(() => props.packageState?.action.primary ?? 'none');
const secondaryActions = computed(() => props.packageState?.action.secondary.filter(action => action !== 'download') ?? []);
const primaryColor = computed(() => {
  if (isDestructiveAction(primaryAction.value)) return 'red';
  if (primaryAction.value === 'update') return 'blue';
  return 'green';
});

const installedCopy = computed(() => locale.value.startsWith('zh') ? '已安装' : 'Installed');
const notInstalledCopy = computed(() => locale.value.startsWith('zh') ? '未安装' : 'Not installed');
const updateCopy = computed(() => locale.value.startsWith('zh') ? '有可用更新' : 'Update available');
</script>

<style lang="css" scoped>
.header {
  display: flex;
  min-height: 192px;
  overflow: hidden;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  margin: 62px 20px 10px;
  position: relative;
  isolation: isolate;
  background: var(--color-surface);
}
.header .backdrop {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
  z-index: -2;
}
.header::before {
  content: '';
  position: absolute;
  inset: 0;
  z-index: -1;
  background: linear-gradient(90deg, rgba(32,35,38,0.98), rgba(32,35,38,0.72), rgba(32,35,38,0.3));
}

.header .app-img {
  width: 128px;
  height: 128px;
  margin-right: 24px;
  filter: drop-shadow(5px 5px 10px rgba(0, 0, 0, 0.5));
  object-fit: contain;
}

.app-banner {
  min-width: 0;
  padding: 16px 24px;
  display: flex;
  align-items: center;
}

.app-intro {
  min-width: 0;
}

.name {
  color: var(--color-text);
  font-size: 40px;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.8);
}

.intro {
  margin-top: 4px;
  color: var(--color-text-muted);
  font-size: 18px;
}

.state-line {
  margin-top: 12px;
  color: var(--color-text-subtle);
  font-size: 15px;
}

.fallback-icon { display: grid; place-items: center; flex-shrink: 0; border-radius: var(--radius-card); background: var(--color-surface-raised); color: var(--color-text-muted); font-size: 56px; }

.actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  align-content: flex-end;
  gap: 10px;
  margin: auto 20px 20px auto;
}

.actions :deep(.button) {
  margin: 0;
}

@media (max-width: 760px) {
  .header { flex-direction: column; }
  .header .app-img { width: 80px; height: 80px; margin-right: 16px; }
  .name { font-size: 30px; overflow-wrap: anywhere; }
  .actions { margin: 0 20px 20px; justify-content: flex-start; }
}
</style>
