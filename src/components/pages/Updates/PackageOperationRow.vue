<template>
  <article class="package-row" :class="{ selected }">
    <input
      class="checkbox"
      type="checkbox"
      :checked="selected"
      :aria-label="pkg.displayName"
      @change="$emit('toggle', pkg.name)"
    />
    <div class="icon" aria-hidden="true">●</div>
    <div class="main">
      <div class="name">{{ pkg.displayName }}</div>
      <div class="versions">
        <template v-if="pkg.currentVersion && pkg.newVersion">
          {{ pkg.currentVersion }} → {{ pkg.newVersion }}
        </template>
        <template v-else-if="pkg.currentVersion">
          {{ pkg.currentVersion }}
        </template>
        <template v-else>
          {{ pkg.source ?? pkg.packageName }}
        </template>
      </div>
    </div>
    <div class="size">{{ size }}</div>
    <div class="row-actions">
      <button v-if="pkg.upgradable" type="button" class="link update" @click="$emit('action', 'update', pkg.name)">
        {{ localeZh ? '更新' : 'Update' }}
      </button>
      <button v-if="pkg.installed" type="button" class="link remove" @click="$emit('action', 'remove', pkg.name)">
        {{ localeZh ? '卸载' : 'Remove' }}
      </button>
      <button v-if="pkg.openable" type="button" class="link open" @click="$emit('action', 'open', pkg.name)">
        {{ localeZh ? '打开' : 'Open' }}
      </button>
    </div>
  </article>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { filesize } from 'filesize';
import type { PackageOperationKind, PmPackageRecord } from '../../../types/pm';

const props = defineProps<{
  pkg: PmPackageRecord;
  selected: boolean;
}>();

defineEmits<{
  toggle: [name: string];
  action: [operation: Extract<PackageOperationKind, 'update' | 'remove'> | 'open', name: string];
}>();

const localeZh = navigator.language.startsWith('zh');
const size = computed(() => filesize(props.pkg.downloadSize ?? props.pkg.installedSize ?? 0, { standard: 'iec' }));
</script>

<style scoped>
.package-row {
  display: grid;
  grid-template-columns: 28px 52px minmax(0, 1fr) 110px 210px;
  align-items: center;
  min-height: 68px;
  gap: 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  color: var(--color-text);
}

.package-row.selected {
  background: rgba(61, 174, 233, 0.08);
}

.checkbox {
  width: 15px;
  height: 15px;
  accent-color: var(--color-focus);
}

.icon {
  display: grid;
  place-items: center;
  width: 48px;
  height: 48px;
  border-radius: 50%;
  color: #4285f4;
  background: conic-gradient(#4285f4, #34a853, #fbbc05, #ea4335, #4285f4);
  font-size: 20px;
}

.main {
  min-width: 0;
}

.name {
  font-size: 18px;
}

.versions {
  margin-top: 4px;
  color: var(--color-text-muted);
  font-size: 13px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.size {
  color: var(--color-text-muted);
  font-size: 13px;
}

.row-actions {
  display: flex;
  justify-content: flex-end;
  gap: 14px;
}

.link {
  border: 0;
  background: transparent;
  cursor: pointer;
  font-size: 14px;
}

.link:hover {
  text-decoration: underline;
}

.update { color: #2da8ff; }
.remove { color: #ffb300; }
.open { color: var(--color-text-subtle); }
</style>
