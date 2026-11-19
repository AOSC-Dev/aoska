<template>
  <section v-if="detail" class="app-detail">
    <AppHeader
      :app-name="detail.name"
      :app-intro="detail.summary"
      :app-icon="detail.icon ?? undefined"
      :app-banner-img="detail.hero ?? undefined"
      :package-state="packageState"
      :disabled="operationBusy"
      @action="handlePackageAction"
    />
    <label v-if="detail.packageNames.length > 1" class="package-picker">
      {{ $t('detail.package') }}
      <select v-model="selectedPackage" :disabled="operationBusy">
        <option v-for="name in detail.packageNames" :key="name">{{ name }}</option>
      </select>
    </label>
    <p v-if="stateError" role="alert" class="package-error">
      {{ stateError }} <button type="button" @click="loadPackageState">{{ $t('detail.retryState') }}</button>
    </p>
    <div class="down-content">
      <div>
        <div v-if="detail.screenshots.length" class="image-container">
          <ImageCarousel :images="detail.screenshots" />
        </div>
        <p v-if="detail.description" class="description">{{ detail.description }}</p>
      </div>
      <aside class="down-right">
        <div class="flags">
          <template v-for="field in disclosureFields" :key="field">
            <SoftwareFlags v-if="detail.disclosures[field]" :field="field" :value="detail.disclosures[field]!" />
          </template>
        </div>
        <p v-for="(note, index) in detail.disclosures.notes" :key="index">{{ note }}</p>
        <dl class="info">
          <template v-if="detail.disclosures.reviewed_at">
            <dt>{{ $t('detail.reviewed') }}</dt><dd>{{ detail.disclosures.reviewed_at }}</dd>
          </template>
          <template v-if="detail.publisher">
            <dt>{{ $t('detail.publisher') }}</dt><dd>{{ detail.publisher }}</dd>
          </template>
          <template v-if="selectedPackage">
            <dt>{{ $t('detail.package') }}</dt><dd>{{ selectedPackage }}</dd>
          </template>
          <template v-if="packageState?.currentVersion || packageState?.newVersion">
            <dt>{{ $t('detail.version') }}</dt><dd>{{ packageState.currentVersion ?? packageState.newVersion }}</dd>
          </template>
          <template v-if="packageState?.installedSize != null">
            <dt>{{ $t('detail.size') }}</dt><dd>{{ filesize(packageState.installedSize) }}</dd>
          </template>
        </dl>
        <div class="btns">
          <button v-if="website" type="button" @click="openPage(website)">{{ $t('detail.website') }}</button>
          <button v-if="detail.downloadUrl" type="button" @click="openPage(detail.downloadUrl)">{{ $t('detail.download') }}</button>
          <button type="button" @click="reportIssue">{{ $t('detail.report') }}</button>
        </div>
      </aside>
    </div>
  </section>
  <p v-else-if="catalogError" role="alert" class="catalog-status package-error">
    {{ catalogError }} <button type="button" @click="loadCatalog">{{ $t('detail.retryCatalog') }}</button>
  </p>
  <p v-else role="status" class="catalog-status">{{ $t('detail.loading') }}</p>
  <p v-if="actionError || operationError" role="alert" class="package-error">{{ actionError || operationError }}</p>
  <OperationDialog :operation="activeOperation" :plan="plan" :submitting="submitting" :error="operationError" @confirm="confirm" @finished="finished" @close="close" />
</template>

<script setup lang="ts">
import { filesize } from 'filesize';
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { getCatalogDetail, type CatalogDetail } from '../../utils/asmrClient';
import { launchCatalogApp, openExternalPage } from '../../utils/desktopClient';
import { getPackageManager } from '../../utils/pmClient';
import { usePackageOperation } from '../../utils/usePackageOperation';
import type { PackagePrimaryAction, PmPackageState, PmCapabilities } from '../../types/pm';
import ImageCarousel from '../share/ImageCarousel.vue';
import OperationDialog from '../share/OperationDialog.vue';
import SoftwareFlags from '../share/SoftwareFlags.vue';
import AppHeader from './AppPage/AppHeader.vue';

const route = useRoute();
const { locale, t } = useI18n();
const appId = computed(() => String(route.params.pkgName));
const detail = ref<CatalogDetail | null>(null);
const selectedPackage = ref('');
const packageState = ref<PmPackageState | null>(null);
const capabilities = ref<PmCapabilities | null>(null);
const catalogError = ref('');
const stateError = ref('');
const actionError = ref('');
const website = computed(() => detail.value?.homepage ?? packageState.value?.homepage);
const disclosureFields = ['availability', 'runtime', 'telemetry', 'service', 'support'] as const;
const { plan, activeOperation, submitting, locked: operationBusy, error: operationError, prepare, confirm, finished, close } = usePackageOperation(loadPackageState);

async function loadCatalog() {
  const id = appId.value;
  const language = locale.value;
  detail.value = null;
  selectedPackage.value = '';
  catalogError.value = '';
  actionError.value = '';
  try {
    const result = await getCatalogDetail(id, language);
    if (id !== appId.value || language !== locale.value) return;
    detail.value = result;
    selectedPackage.value = result.packageNames[0] ?? '';
  } catch (cause) {
    if (id === appId.value && language === locale.value) catalogError.value = String(cause);
  }
}

async function loadPackageState() {
  const name = selectedPackage.value;
  stateError.value = '';
  packageState.value = null;
  capabilities.value = null;
  if (!name) return;
  try {
    const pm = getPackageManager();
    const [available, state] = await Promise.all([pm.capabilities(), pm.packageState(name)]);
    if (name !== selectedPackage.value) return;
    capabilities.value = available;
    packageState.value = state;
  } catch (cause) {
    if (name === selectedPackage.value) stateError.value = String(cause);
  }
}

watch([appId, locale], loadCatalog, { immediate: true });
watch(selectedPackage, loadPackageState);

async function handlePackageAction(action: PackagePrimaryAction) {
  actionError.value = '';
  if (action === 'download' && website.value) return openPage(website.value);
  if (action === 'open' && packageState.value?.openable) {
    try { await launchCatalogApp(appId.value, selectedPackage.value); } catch (cause) { actionError.value = String(cause); }
    return;
  }
  if (operationBusy.value || !packageState.value || !selectedPackage.value) return;
  if (action !== 'install' && action !== 'update' && action !== 'remove') return;
  const capability = action === 'update' ? 'upgrade.selected' : action;
  if (!capabilities.value?.capabilities.includes(`run.${capability}.v1`) || !capabilities.value.capabilities.includes(`plan.${capability}.v1`)) {
    actionError.value = t('detail.unsupported');
    return;
  }
  await prepare(action, [selectedPackage.value]);
}

async function openPage(url: string) {
  actionError.value = '';
  try { await openExternalPage(url); } catch (cause) { actionError.value = String(cause); }
}
function reportIssue() {
  return openPage(`https://github.com/AOSC-Dev/aoska/issues/new?title=${encodeURIComponent('应用使用问题：' + appId.value)}`);
}
</script>

<style scoped>
.catalog-status { margin: 76px 20px 16px; }
.package-error { padding: 12px 20px; background: var(--color-danger); overflow-wrap: anywhere; }
.down-content { display: grid; grid-template-columns: minmax(0, 2fr) minmax(300px, 1fr); gap: 20px; margin: 12px 20px 24px; }
.image-container { width: 100%; aspect-ratio: 16 / 10; border: 1px solid var(--color-border); border-radius: var(--radius-card); }
.description { white-space: pre-wrap; overflow-wrap: anywhere; margin: 16px 0; line-height: 1.6; }
.down-right { display: flex; flex-direction: column; gap: 12px; }
.flags { display: flex; flex-wrap: wrap; gap: 8px; }
.info { display: grid; grid-template-columns: max-content 1fr; gap: 6px 12px; align-items: baseline; }
.info dt { color: var(--color-text-muted); }
.info dd { margin: 0; overflow-wrap: anywhere; }
.btns { display: flex; flex-wrap: wrap; gap: 8px; }
.btns button, .package-picker select { padding: 6px 12px; background: var(--color-button-grey); color: var(--color-text); border: 1px solid var(--color-border); border-radius: var(--radius-control); }
.package-picker { margin: 0 20px; }
@media (max-width: 720px) { .down-content { grid-template-columns: 1fr; } }
</style>
