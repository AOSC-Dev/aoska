<template>
  <div class="home-layout">
    <WelcomeCard
      v-if="homepage?.hero"
      class="welcome"
      :image="homepage.hero.image"
      :title="homepage.hero.title"
      :subtitle="homepage.hero.subtitle"
      @click="activateHero"
    />
    <UpdateCard
      class="update"
      :class="{ wide: !homepage?.hero }"
      :is-loading="loading"
      :error="updateError"
      :update="update"
      :update-security="updateSecurity"
      :security-classification-available="securityClassificationAvailable"
      @action="router.push('/updates')"
    />

    <p v-if="catalogError" class="catalog-state" role="alert">
      {{ catalogError }}
      <button type="button" @click="refresh++">{{ $t('home.retry') }}</button>
    </p>
    <p v-else-if="!homepage" class="catalog-state" role="status">{{ $t('home.loading') }}</p>
    <p v-if="linkError" class="catalog-state" role="alert">{{ linkError }}</p>

    <div v-if="collections.length" class="app-content">
      <section
        v-for="collection in collections"
        :id="'collection-' + collection.id"
        :key="collection.id"
        tabindex="-1"
      >
        <h2>{{ collection.title }}</h2>
        <div class="apps">
          <AppCard
            v-for="app in collection.apps"
            :key="app.id"
            :name="app.name"
            :intro="app.summary"
            :image="app.icon"
            @click="openApp(app.id)"
          />
        </div>
      </section>
    </div>

    <section v-if="homepage && (homepage.guides.items.length || homepage.guides.more_url)" class="tip-content">
      <header>
        <h2>{{ $t('home.tipTitle') }}</h2>
        <button v-if="homepage.guides.more_url" type="button" @click="openLink(homepage.guides.more_url)">
          {{ $t('home.moreGuides') }}
        </button>
      </header>
      <div class="tips">
        <TipCard
          v-for="guide in homepage.guides.items"
          :key="guide.id"
          :date="guide.date"
          :title="guide.title"
          :intro="guide.subtitle"
          @click="openLink(guide.url)"
        />
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onBeforeMount, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import router from '../../router';
import { getCatalogHomepage } from '../../utils/asmrClient';
import { openExternalPage } from '../../utils/desktopClient';
import { getPackageManager } from '../../utils/pmClient';
import AppCard from '../share/AppCard.vue';
import TipCard from '../share/TipCard.vue';
import UpdateCard from '../share/UpdateCard.vue';
import WelcomeCard from '../share/WelcomeCard.vue';

const { locale } = useI18n();
const homepage = ref<Awaited<ReturnType<typeof getCatalogHomepage>> | null>(null);
const catalogError = ref('');
const linkError = ref('');
const refresh = ref(0);
const showHeroCollection = ref(false);
const collections = computed(() => {
  const essentials = homepage.value?.essentials;
  const result = essentials ? [essentials] : [];
  const featured = homepage.value?.heroCollection;
  if (showHeroCollection.value && featured && featured.id !== essentials?.id) result.push(featured);
  return result;
});

watch([locale, refresh], async ([language], _, onCleanup) => {
  let active = true;
  onCleanup(() => { active = false; });
  catalogError.value = '';
  try {
    const content = await getCatalogHomepage(language);
    if (active) homepage.value = content;
  } catch (cause) {
    if (active) catalogError.value = String(cause);
  }
}, { immediate: true });

function openApp(id: string) {
  return router.push('/app/' + encodeURIComponent(id));
}

async function openLink(url: string) {
  linkError.value = '';
  try {
    await openExternalPage(url);
  } catch (cause) {
    linkError.value = String(cause);
  }
}

async function activateHero() {
  const target = homepage.value?.hero?.target;
  if (target?.app) {
    await openApp(target.app);
  } else if (target?.url) {
    await openLink(target.url);
  } else if (target?.collection) {
    showHeroCollection.value = true;
    await nextTick();
    const section = document.getElementById('collection-' + target.collection);
    section?.focus({ preventScroll: true });
    section?.scrollIntoView({ block: 'start' });
  }
}

const loading = ref(true);
const update = ref(0);
const updateSecurity = ref(0);
const securityClassificationAvailable = ref(true);
const updateError = ref('');

onBeforeMount(async () => {
  try {
    const summary = await getPackageManager().updateSummary();
    securityClassificationAvailable.value = summary.securityClassificationAvailable;
    updateSecurity.value = summary.security ?? 0;
    update.value = Math.max(0, summary.total - updateSecurity.value);
  } catch (cause) {
    updateError.value = String(cause);
  } finally {
    loading.value = false;
  }
});
</script>

<style scoped>
.home-layout {
  display: grid;
  grid-template-columns: minmax(0, 2fr) minmax(320px, 1fr);
  gap: 20px;
  padding: 62px 20px 20px;
  color: var(--color-text);
}

.welcome, .update {
  height: 200px;
}

.update, .tip-content {
  grid-column: 2;
}

.update.wide {
  grid-column: 1 / -1;
}

.app-content {
  grid-column: 1;
}

.app-content section {
  scroll-margin-top: 62px;
}

.app-content section + section {
  margin-top: 24px;
}

.catalog-state {
  grid-column: 1 / -1;
}

h2 {
  margin-bottom: 10px;
  font-size: 28px;
  font-weight: 500;
}

.apps {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(min(300px, 100%), 1fr));
  gap: 15px;
}

.tip-content header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 8px;
}

.tip-content header button {
  color: var(--color-accent);
  background: transparent;
  border: 0;
  cursor: pointer;
}

.tips {
  display: grid;
  gap: 12px;
}

@media (max-width: 760px) {
  .home-layout {
    grid-template-columns: minmax(0, 1fr);
  }

  .update, .tip-content {
    grid-column: 1;
  }
}
</style>
