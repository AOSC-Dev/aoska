<template>
  <main class="updates-page">
    <section class="hero-grid">
      <UpdateCard
        class="update-status"
        :update="summary?.total ?? 0"
        :update-security="summary?.security ?? 0"
        :security-classification-available="summary?.securityClassificationAvailable ?? true"
        :is-loading="loading"
      />
      <StorageSummary
        v-if="storage"
        :storage="storage"
      />
    </section>

    <section class="management-card">
      <header class="management-header">
        <div>
          <p class="eyebrow">{{ $t('updates.managementEyebrow') }}</p>
          <h1>{{ $t('updates.managementTitle') }}</h1>
          <p class="intro">{{ $t('updates.managementIntro') }}</p>
        </div>
        <button class="refresh" type="button" :disabled="operationBusy" @click="refresh">
          {{ $t('updates.refresh') }}
        </button>
      </header>

      <div v-if="errorMessage" class="error">
        {{ errorMessage }}
      </div>

      <div class="toolbar">
        <div class="tabs" role="tablist" aria-label="Package list type">
          <button
            type="button"
            role="tab"
            :aria-selected="activeTab === 'updates'"
            :class="{ active: activeTab === 'updates' }"
            @click="activeTab = 'updates'"
          >
            {{ $t('updates.upgradable') }} <span>{{ updates.length }}</span>
          </button>
          <button
            type="button"
            role="tab"
            :aria-selected="activeTab === 'installed'"
            :class="{ active: activeTab === 'installed' }"
            @click="activeTab = 'installed'"
          >
            {{ $t('updates.installed') }} <span>{{ installed.length }}</span>
          </button>
        </div>

        <div class="batch-actions">
          <button
            v-if="activeTab === 'updates'"
            type="button"
            class="primary"
            :disabled="!updateBatch.enabled || operationBusy"
            @click="runBatchUpdate"
          >
            {{ $t('updates.updateSelected') }} ({{ updateBatch.count }})
          </button>
          <button
            v-else
            type="button"
            class="danger"
            :disabled="!removeBatch.enabled || operationBusy"
            @click="runBatchRemove"
          >
            {{ $t('updates.removeSelected') }} ({{ removeBatch.count }})
          </button>
        </div>
      </div>

      <div v-if="loading" class="list-state">
        {{ $t('updates.loading') }}
      </div>
      <div v-else-if="visiblePackages.length === 0" class="list-state">
        {{ activeTab === 'updates' ? $t('updates.noUpdates') : $t('updates.noInstalled') }}
      </div>
      <div v-else class="package-list">
        <PackageOperationRow
          v-for="pkg in visiblePackages"
          :key="pkg.name"
          :pkg="pkg"
          :selected="activeSelection.has(pkg.name)"
          @toggle="toggleSelection(activeTab, $event)"
          @action="handleRowAction"
        />
      </div>
    </section>

    <OperationDialog :operation="activeOperation" @close="closeOperationDialog" />
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import OperationDialog from '../share/OperationDialog.vue';
import UpdateCard from '../share/UpdateCard.vue';
import type { PackageOperationKind, PmOperationStart, PmPackageRecord, PmStorageSummary, PmUpdateSummary } from '../../types/pm';
import { getPackageManager } from '../../utils/pmClient';
import { batchActionState, selectedPackageNames } from '../../utils/pmSelection';
import PackageOperationRow from './Updates/PackageOperationRow.vue';
import StorageSummary from './Updates/StorageSummary.vue';

type ActiveTab = 'updates' | 'installed';
type RowOperation = Extract<PackageOperationKind, 'update' | 'remove'> | 'open';

const router = useRouter();
const loading = ref(true);
const operationBusy = ref(false);
const errorMessage = ref('');
const activeTab = ref<ActiveTab>('updates');
const summary = ref<PmUpdateSummary | null>(null);
const updates = ref<PmPackageRecord[]>([]);
const installed = ref<PmPackageRecord[]>([]);
const storage = ref<PmStorageSummary | null>(null);
const selectedUpdates = ref<Set<string>>(new Set());
const selectedInstalled = ref<Set<string>>(new Set());
const activeOperation = ref<PmOperationStart | null>(null);

const visiblePackages = computed(() => activeTab.value === 'updates' ? updates.value : installed.value);
const activeSelection = computed(() => activeTab.value === 'updates' ? selectedUpdates.value : selectedInstalled.value);
const updateBatch = computed(() => batchActionState('update', updates.value, selectedUpdates.value));
const removeBatch = computed(() => batchActionState('remove', installed.value, selectedInstalled.value));

onMounted(async () => {
  await loadPackageState();
});

async function loadPackageState() {
  loading.value = true;
  errorMessage.value = '';
  try {
    const pm = getPackageManager();
    const [nextSummary, nextUpdates, nextInstalled, nextStorage] = await Promise.all([
      pm.updateSummary(),
      pm.listUpdates(),
      pm.listInstalled(),
      pm.storageSummary(),
    ]);
    summary.value = nextSummary;
    updates.value = nextUpdates.packages;
    installed.value = nextInstalled.packages;
    storage.value = nextStorage;
    selectedUpdates.value = filterSelection(selectedUpdates.value, updates.value);
    selectedInstalled.value = filterSelection(selectedInstalled.value, installed.value);
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : String(error);
  } finally {
    loading.value = false;
  }
}

function filterSelection(selected: ReadonlySet<string>, packages: PmPackageRecord[]): Set<string> {
  const validNames = new Set(packages.map((pkg) => pkg.name));
  return new Set([...selected].filter((name) => validNames.has(name)));
}

function toggleSelection(tab: ActiveTab, name: string) {
  const source = tab === 'updates' ? selectedUpdates : selectedInstalled;
  const next = new Set(source.value);
  if (next.has(name)) {
    next.delete(name);
  } else {
    next.add(name);
  }
  source.value = next;
}

async function refresh() {
  operationBusy.value = true;
  try {
    activeOperation.value = await getPackageManager().startRefresh();
    await loadPackageState();
  } finally {
    operationBusy.value = false;
  }
}

async function runBatchUpdate() {
  const packages = selectedPackageNames(updates.value, selectedUpdates.value);
  if (packages.length === 0) return;
  await startOperation('update', packages);
}

async function runBatchRemove() {
  const packages = selectedPackageNames(installed.value, selectedInstalled.value);
  if (packages.length === 0) return;
  await startOperation('remove', packages);
}

async function handleRowAction(operation: RowOperation, name: string) {
  if (operation === 'open') {
    await router.push(`/app/${name}`);
    return;
  }
  await startOperation(operation, [name]);
}

async function startOperation(operation: Extract<PackageOperationKind, 'update' | 'remove'>, packages: string[]) {
  operationBusy.value = true;
  try {
    const pm = getPackageManager();
    activeOperation.value = operation === 'update'
      ? await pm.startUpdate(packages)
      : await pm.startRemove(packages);
  } finally {
    operationBusy.value = false;
  }
}

async function closeOperationDialog() {
  activeOperation.value = null;
  await loadPackageState();
}
</script>

<style scoped>
.updates-page {
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding: 82px 20px 24px;
  color: var(--color-text);
}

.hero-grid {
  display: grid;
  grid-template-columns: minmax(0, 1.8fr) minmax(320px, 0.9fr);
  gap: 20px;
}

.update-status {
  min-height: 190px;
}

.management-card {
  overflow: hidden;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  background: var(--color-surface);
  box-shadow: var(--shadow-card);
}

.management-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 20px;
  padding: 24px 28px 18px;
  border-bottom: 1px solid var(--color-border);
}

.eyebrow {
  margin: 0 0 6px;
  color: var(--color-text-subtle);
  font-size: 13px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

h1 {
  margin: 0;
  font-size: 32px;
  line-height: 1.15;
}

.intro {
  margin: 8px 0 0;
  color: var(--color-text-muted);
  font-size: 15px;
}

.refresh,
.primary,
.danger {
  min-width: 128px;
  height: 36px;
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  color: var(--color-text);
  cursor: pointer;
}

.refresh {
  background: var(--color-button-grey);
}

.primary {
  background: var(--color-button-green);
}

.danger {
  background: var(--color-button-red);
}

.refresh:disabled,
.primary:disabled,
.danger:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.error {
  margin: 18px 28px 0;
  padding: 12px 14px;
  border: 1px solid var(--color-danger-border);
  border-radius: var(--radius-control);
  background: var(--color-danger);
  color: var(--color-text);
}

.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 18px;
  padding: 18px 28px;
}

.tabs {
  display: inline-flex;
  padding: 3px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: var(--color-surface-raised);
}

.tabs button {
  min-width: 132px;
  height: 34px;
  border: 0;
  border-radius: calc(var(--radius-control) - 2px);
  background: transparent;
  color: var(--color-text-muted);
  cursor: pointer;
}

.tabs button.active {
  background: var(--color-info);
  color: var(--color-text);
}

.tabs span {
  margin-left: 6px;
  color: var(--color-text-subtle);
}

.batch-actions {
  display: flex;
  gap: 10px;
}

.package-list {
  padding: 0 28px 12px;
}

.list-state {
  margin: 0 28px 28px;
  padding: 36px;
  border: 1px dashed var(--color-border);
  border-radius: var(--radius-card);
  color: var(--color-text-muted);
  text-align: center;
}

@media (max-width: 900px) {
  .hero-grid,
  .management-header,
  .toolbar {
    grid-template-columns: 1fr;
    flex-direction: column;
    align-items: stretch;
  }

  .toolbar {
    align-items: stretch;
  }

  .tabs,
  .tabs button,
  .batch-actions button {
    width: 100%;
  }
}
</style>
