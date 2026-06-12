<template>
  <div class="view-all">
    <div class="ranking">
      <span>{{ $t("pages.ranking") }}</span>
      <span>{{ $t("pages.name") }}</span>
      <span>{{ $t("pages.download") }}</span>
      <span>{{ $t("pages.update") }}</span>
    </div>
    <AppBanner
      v-for="(app, index) in packages"
      :key="index"
      :name="app.name"
      :intro="app.intro"
      :icon="resolveAssetPath(app.icon, app.name)"
      @click="showDetail(app.name)"
    ></AppBanner>
  </div>
</template>

<script setup lang='ts'>
import AppBanner from '../share/AppBanner.vue';
import router from '../../router';
import { onMounted, ref } from 'vue';
import { fetchIndex } from '../../utils/wrapper';
import { PackageBrief } from '../../types/packages';
import { resolveAssetPath } from "../../utils/url";

const packages = ref<PackageBrief[] | null>(null);

onMounted(async () => {
  const index = await fetchIndex();
  packages.value = index.packages.flatMap(categoryIndex => categoryIndex.packages);
});

// 跳转到应用详情
const showDetail = (name: string) => {
  router.push(`/app/${name}`);
};
</script>

<style scoped>
.view-all {
  margin: 0;
  padding: 62px 20px 20px;
  color: var(--color-text);
}

.ranking {
  height: 40px;
  font-size: 18px;
  line-height: 40px;
  border: 1px solid var(--color-border);
  background-color: var(--color-info);
  border-radius: var(--radius-card) var(--radius-card) 0 0;
}

.ranking span {
  margin-left: 10px;
}
</style>
