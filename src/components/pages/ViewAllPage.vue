<template>
  <div class="view-all">
    <header class="page-header">
      <div>
        <p class="eyebrow">{{ searchQuery ? $t('pages.searchResults') : $t('pages.allApps') }}</p>
        <h1>{{ searchQuery ? `“${searchQuery}”` : $t('app.view-all') }}</h1>
      </div>
      <p class="count">{{ $t('pages.resultCount', { count: packages.length }) }}</p>
    </header>
    <div class="ranking">
      <span>{{ $t("pages.ranking") }}</span>
      <span>{{ $t("pages.name") }}</span>
      <span>{{ $t("pages.download") }}</span>
      <span>{{ $t("pages.update") }}</span>
    </div>
    <div v-if="!loading && packages.length === 0" class="empty-state">
      {{ searchQuery ? $t('pages.emptySearch') : $t('pages.emptyCatalog') }}
    </div>
    <template v-else>
      <AppBanner
        v-for="(app, index) in packages"
        :key="index"
        :name="app.name"
        :intro="app.intro"
        :icon="resolveAssetPath(app.icon, app.name)"
        @click="showDetail(app.name)"
      ></AppBanner>
    </template>
  </div>
</template>

<script setup lang='ts'>
import AppBanner from '../share/AppBanner.vue';
import router from '../../router';
import { computed, onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import { fetchIndex } from '../../utils/wrapper';
import { resolveAssetPath } from "../../utils/url";
import type { Index } from '../../types/home';
import { searchCatalogPackages } from '../../utils/catalogSearch';

const route = useRoute();
const index = ref<Index | null>(null);
const loading = ref(true);
const searchQuery = computed(() => typeof route.query.q === 'string' ? route.query.q.trim() : '');
const packages = computed(() => index.value ? searchCatalogPackages(index.value, searchQuery.value) : []);

onMounted(async () => {
  index.value = await fetchIndex();
  loading.value = false;
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

.page-header {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: 18px;
  margin-bottom: 14px;
  padding: 18px 20px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  background-color: var(--color-surface);
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
  font-size: 30px;
  line-height: 1.1;
}

.count {
  margin: 0;
  color: var(--color-text-muted);
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

.empty-state {
  padding: 46px 20px;
  border: 1px dashed var(--color-border);
  border-top: 0;
  border-radius: 0 0 var(--radius-card) var(--radius-card);
  background-color: var(--color-surface);
  color: var(--color-text-muted);
  text-align: center;
}
</style>
