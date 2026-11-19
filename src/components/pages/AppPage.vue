<template>
  <AppHeader
    :app-name="packageDetail?.title"
    :app-intro="packageDetail?.intro"
    :app-icon="resolveAssetPath(packageDetail?.icon || '', packageName)"
    :app-banner-img="resolveAssetPath(packageDetail?.banner || '', packageName)"
    :package-state="packageState"
    :disabled="operationBusy"
    @action="handlePackageAction"
  />
  <p v-if="errorMessage || operationError" role="alert" class="package-error">{{ errorMessage || operationError }}</p>
  <p v-if="catalogError" class="package-error" role="alert">{{ catalogError }} <button type="button" @click="loadCatalog">重试目录</button></p>
  <button v-if="errorMessage" type="button" @click="loadPackageState">重试软件包状态</button>
  <div class="down-content">
    <div class="image-container">
      <ImageCarousel
        v-if="packageDetail?.screenshot.length"
        :images="resolveAssetPath(packageDetail?.screenshot || [], packageName)"
        :autoplay="false"
        :interval="4000"
      />
      <p v-if="!packageDetail?.screenshot.length">暂无应用截图</p>
    </div>
    <div class="down-right">
      <div class="flags">
        <template
          v-for="(v, k) in packageDetail?.package_flags"
          :key="k"
        >
          <SoftwareFlags
            v-if="v"
            :type="k" />
        </template>
      </div>
      <div class="info">
        <table>
          <tbody>
            <tr>
              <td class="l1">发行方：</td>
              <td class="l2">{{ packageDetail?.package_info.publisher }}</td>
              <td class="l3">软件版本：</td>
              <td class="l4">{{ packageState?.currentVersion ?? packageDetail?.package_info.version }}</td>
            </tr>
            <tr>
              <td></td>
              <td></td>
              <td class="l3">更新日期：</td>
              <td>{{ packageDetail?.package_info.update_date }}</td>
            </tr>
            <tr>
              <td class="l1">来源：</td>
              <td>{{ packageState?.source ?? packageDetail?.package_info.source }}</td>
              <td class="l3">安装大小：</td>
              <td>{{ installSizeFormatted }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <div class="btns">
        <button class="main-page" type="button" @click="openHomepage">官方网站</button>
        <button class="report-bug" type="button" @click="reportIssue">报告使用问题</button>
      </div>
    </div>
  </div>
  <OperationDialog :operation="activeOperation" :plan="plan" :submitting="submitting" :error="operationError" @confirm="confirm" @finished="finished" @close="close" />
</template>

<script setup lang="ts">
import { filesize } from 'filesize';
import SoftwareFlags from "../share/SoftwareFlags.vue";
import ImageCarousel from "../share/ImageCarousel.vue";
import { computed, ref, watch } from 'vue';
import AppHeader from "./AppPage/AppHeader.vue";
import { useRoute } from "vue-router";
import { fetchDetail } from "../../utils/wrapper";
import { getPackageManager } from '../../utils/pmClient';
import type { PackagePrimaryAction, PmPackageState, PmCapabilities } from '../../types/pm';
import { PackageDetail } from "../../types/packages";
import { resolveAssetPath } from '../../utils/url';
import OperationDialog from '../share/OperationDialog.vue';
import { launchCatalogApp, openExternalPage } from '../../utils/desktopClient';
import { usePackageOperation } from '../../utils/usePackageOperation';

const route = useRoute();
const packageName = ref<string>(route.params.pkgName as string);
const packageDetail = ref<PackageDetail | null>(null);
const packageState = ref<PmPackageState | null>(null);
const errorMessage = ref('');
const catalogError = ref('');
const capabilities = ref<PmCapabilities | null>(null);
const { plan, activeOperation, submitting, locked: operationBusy, error: operationError, prepare, confirm, finished, close } = usePackageOperation(loadPackageState);

async function loadPackageState() {
  errorMessage.value = '';
  const name = packageName.value;
  try {
    const pm = getPackageManager();
    capabilities.value = await pm.capabilities();
    const nextState = await pm.packageState(name);
    if (name === packageName.value) packageState.value = nextState;
  } catch (cause) {
    if (name === packageName.value) { errorMessage.value = String(cause); packageState.value = null; }
  }
}

async function loadCatalog() {
  const name = packageName.value;
  catalogError.value = '';
  try { const detail = await fetchDetail(name); if (name === packageName.value) packageDetail.value = detail; }
  catch (cause) { if (name === packageName.value) catalogError.value = String(cause); }
}
watch(() => route.params.pkgName, async value => {
  packageName.value = String(value);
  packageDetail.value = null;
  packageState.value = null;
  await Promise.all([loadCatalog(), loadPackageState()]);
}, { immediate: true });

const installSizeFormatted = computed(() => {
  return filesize(packageState.value?.installedSize ?? packageDetail.value?.package_info?.install_size ?? 0);
});

async function handlePackageAction(action: PackagePrimaryAction) {
  if (action === 'download') { await openHomepage(); return; }
  if (action === 'open') {
    try { await launchCatalogApp(packageName.value); } catch (cause) { errorMessage.value = String(cause); }
    return;
  }
  if (!['install', 'update', 'remove'].includes(action) || operationBusy.value) return;
  const kind = action as 'install' | 'update' | 'remove';
  const capability = kind === 'update' ? 'upgrade.selected' : kind;
  if (!capabilities.value?.capabilities.includes(`run.${capability}.v1`) || !capabilities.value.capabilities.includes(`plan.${capability}.v1`)) {
    errorMessage.value = '当前包管理服务不支持此操作。';
    return;
  }
  await prepare(kind, [packageName.value]);
}

async function openHomepage() {
  const homepage = packageDetail.value?.package_info.homepage ?? packageState.value?.homepage;
  try { if (homepage) await openExternalPage(homepage); } catch (cause) { errorMessage.value = String(cause); }
}
async function reportIssue() {
  try { await openExternalPage(`https://github.com/AOSC-Dev/aoska/issues/new?title=${encodeURIComponent('应用使用问题：' + packageName.value)}`); }
  catch (cause) { errorMessage.value = String(cause); }
}
</script>

<style scoped>
.package-error { margin: 12px 20px; padding: 12px; background: var(--color-danger); color: var(--color-text); overflow-wrap: anywhere; }
.down-content {
  display: grid;
  grid-template-columns: minmax(0, 2fr) minmax(360px, 520px);
  gap: 20px;
  padding: 10px 20px 20px;
  color: var(--color-text);
}

.image-container {
  width: 100%;
  height: calc(100vh - 292px);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  background: #111;
}

.image-container img {
  width: auto;
  height: 100%;
  margin-right: 10px;
}

.down-right {
  width: 100%;
  height: calc(100vh - 292px);
  display: flex;
  flex-direction: column;
}

.flags{
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.info {
  width: 100%;
  margin-top: auto;
  padding: 10px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  background-color: var(--color-surface-raised);
  color: var(--color-text);
}

table {
  width: 100%;
}

.l1 {
  width: 65px;
  font-weight: bold;
}

.l3 {
  width: 85px;
  font-weight: bold;
}

.l2 {
  width: calc(50% - 65px);
}

.l4 {
  width: calc(50% - 85px);
}

.btns {
  display: flex;
  gap: 10px;
  margin-top: 10px;
}

.main-page,
.report-bug {
  flex: 1;
  height: 36px;
  border-radius: var(--radius-control);
  color: var(--color-text);
  font-size: 18px;
  line-height: 34px;
  text-align: center;
  cursor: pointer;
}

.main-page {
  background-color: var(--color-button-grey);
  border: 1px solid var(--color-border-strong);
}

.report-bug {
  background-color: var(--color-button-yellow);
  border: 1px solid var(--color-border-strong);
}
</style>
