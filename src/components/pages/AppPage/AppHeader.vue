<template>
  <div class="header" :style="{ backgroundImage: `linear-gradient(90deg, rgba(32,35,38,0.98), rgba(32,35,38,0.72), rgba(32,35,38,0.3)), url(${appBannerImg})` }">
    <div class="app-banner">
      <img :src="appIcon" class="app-img" alt="App Icon">
      <div class="app-intro">
        <div class="name">{{ appName }}</div>
        <div class="intro">{{ appIntro }}</div>
        <div v-if="packageState" class="state-line">
          <span v-if="packageState.installed">{{ installedCopy }}</span>
          <span v-else>{{ notInstalledCopy }}</span>
          <span v-if="packageState.upgradable"> · {{ updateCopy }}</span>
        </div>
      </div>
    </div>
    <div class="actions">
      <CustomButton
        v-if="primaryAction !== 'none'"
        size="medium"
        :color="primaryColor"
        width="129"
        height="37"
        @click="$emit('action', primaryAction)"
      >
        {{ actionLabel(primaryAction) }}
      </CustomButton>
      <CustomButton
        v-for="action in secondaryActions"
        :key="action"
        size="medium"
        color="grey"
        width="129"
        height="37"
        @click="$emit('action', action)"
      >
        {{ actionLabel(action) }}
      </CustomButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import CustomButton from '../../share/CustomButton.vue';
import type { PackagePrimaryAction, PmPackageState } from '../../../types/pm';
import { actionLabel, isDestructiveAction } from '../../../utils/pmPresentation';

const props = defineProps<{
  appIcon?: string;
  appName?: string;
  appIntro?: string;
  appBannerImg?: string;
  packageState?: PmPackageState | null;
}>();

defineEmits<{
  action: [action: PackagePrimaryAction];
}>();

const primaryAction = computed<PackagePrimaryAction>(() => props.packageState?.action.primary ?? 'install');
const secondaryActions = computed<PackagePrimaryAction[]>(() => props.packageState?.action.secondary ?? ['download']);
const primaryColor = computed(() => {
  if (isDestructiveAction(primaryAction.value)) return 'red';
  if (primaryAction.value === 'update') return 'blue';
  return 'green';
});

const installedCopy = computed(() => navigator.language.startsWith('zh') ? '已安装' : 'Installed');
const notInstalledCopy = computed(() => navigator.language.startsWith('zh') ? '未安装' : 'Not installed');
const updateCopy = computed(() => navigator.language.startsWith('zh') ? '有可用更新' : 'Update available');
</script>

<style lang="css" scoped>
.header {
  display: flex;
  min-height: 192px;
  overflow: hidden;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  margin: 62px 20px 10px;
  background-size: cover;
  background-position: center;
}

.header .app-img {
  width: 128px;
  height: 128px;
  margin: 40px;
  filter: drop-shadow(5px 5px 10px rgba(0, 0, 0, 0.5));
  object-fit: contain;
}

.app-banner {
  min-width: 0;
  height: 100%;
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
</style>
