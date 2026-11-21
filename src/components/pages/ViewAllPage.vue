<template>
  <section class="view-all">
    <header class="page-header">
      <h1>{{ title }}</h1>
      <p v-if="cards && !error">{{ $t('pages.resultCount', { count: packages.length }) }}</p>
    </header>
    <p v-if="error" role="alert">
      {{ error }} <button type="button" @click="refresh++">{{ $t('home.retry') }}</button>
    </p>
    <p v-else-if="!cards" role="status">{{ $t('home.loading') }}</p>
    <p v-else-if="!packages.length" class="empty-state">
      {{ searchQuery ? $t('pages.emptySearch') : $t('pages.emptyCatalog') }}
    </p>
    <div v-else class="results">
      <AppCard
        v-for="app in packages"
        :key="app.id"
        :name="app.name"
        :intro="app.summary"
        :image="app.icon"
        @click="router.push('/app/' + encodeURIComponent(app.id))"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute, useRouter } from 'vue-router';
import AppCard from '../share/AppCard.vue';
import { getCatalogCards } from '../../utils/asmrClient';
import { normalizeSearchQuery } from '../../utils/catalogSearch';

const props = defineProps<{ category?: string }>();
const route = useRoute();
const router = useRouter();
const { locale, t, te } = useI18n();
const cards = ref<Awaited<ReturnType<typeof getCatalogCards>> | null>(null);
const error = ref('');
const refresh = ref(0);
const searchQuery = computed(() => typeof route.query.q === 'string' ? route.query.q.trim() : '');
const title = computed(() => {
  if (props.category) {
    const key = props.category === 'office' ? 'app.working' : 'app.' + props.category;
    return te(key) ? t(key) : props.category;
  }
  return searchQuery.value ? t('pages.searchResults') + ' · ' + searchQuery.value : t('pages.allApps');
});
const packages = computed(() => {
  const query = normalizeSearchQuery(searchQuery.value);
  return (cards.value ?? []).filter(app =>
    (!props.category || app.categories.includes(props.category)) &&
    (!query || [app.id, app.name, app.summary, ...app.searchTerms].some(field => field?.toLocaleLowerCase().includes(query)))
  );
});

watch([locale, refresh], async ([language], _, onCleanup) => {
  let active = true;
  onCleanup(() => { active = false; });
  cards.value = null;
  error.value = '';
  try {
    const data = await getCatalogCards(language);
    if (active) cards.value = data;
  } catch (cause) {
    if (active) error.value = String(cause);
  }
}, { immediate: true });
</script>

<style scoped>
.view-all { margin: 0; padding: 62px 20px 20px; color: var(--color-text); }
.page-header { display: flex; align-items: baseline; justify-content: space-between; gap: 18px; margin-bottom: 14px; }
.page-header h1 { font-size: 30px; }
.page-header p, .empty-state { color: var(--color-text-muted); }
.results { display: grid; grid-template-columns: repeat(auto-fill, minmax(min(100%, 320px), 1fr)); gap: 8px; }
</style>
