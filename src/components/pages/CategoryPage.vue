<template>
  <div class="working">
    <div class="ranking">
      <span>{{ $t("pages.ranking") }}</span>
      <span>{{ $t("pages.name") }}</span>
      <span>{{ $t("pages.download") }}</span>
      <span>{{ $t("pages.update") }}</span>
    </div>
    <AppBanner
      v-for="(app, index) in categoryIndex?.packages"
      :key="index"
      :name="app.name"
      :intro="app.intro"
      :icon="resolveAssetPath(app.icon, app.name)"
      @click="showDetail(app.name)"
    ></AppBanner>
  </div>
</template>

<script setup lang='ts'>
import { ref, onMounted } from 'vue';

import AppBanner from '../share/AppBanner.vue';
import router from '../../router';
import { onBeforeRouteUpdate, useRoute } from 'vue-router';
import { CategoryIndex } from '../../types/home';
import { fetchByCategory } from '../../utils/wrapper';
import { resolveAssetPath } from '../../utils/url';

const route = useRoute();
const categoryIndex = ref<CategoryIndex | null>(null);

const fetchCategory = async (categoryName: string) => {
  categoryIndex.value = await fetchByCategory(categoryName as string);
};

onMounted(async () => {
  await fetchCategory(route.params.categoryName as string);
});

onBeforeRouteUpdate(async (to, _, next) => {
  await fetchCategory(to.params.categoryName as string);
  next();
});

// 跳转到应用详情
const showDetail = (name: string) => {
  router.push(`/app/${name}`);
};
</script>

<style scoped>
.working {
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
