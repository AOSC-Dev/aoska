<template>
  <section class="settings">
    <h1>{{ $t('settings.title') }}</h1>
    <div v-if="loadError" role="alert">
      {{ $t('settings.loadError', { reason: loadError }) }}
      <button type="button" @click="load">{{ $t('home.retry') }}</button>
    </div>
    <p v-else-if="!settings" role="status">{{ $t('settings.loading') }}</p>
    <form v-else @submit.prevent="save" @input="clearFeedback">
      <fieldset :disabled="saving">
        <legend>{{ $t('settings.source') }}</legend>
        <label for="catalog-channel">{{ $t('settings.channel') }}</label>
        <select id="catalog-channel" v-model="channel">
          <option value="auto">{{ $t('settings.auto', { channel: $t('settings.' + settings.defaultChannel) }) }}</option>
          <option value="dev">{{ $t('settings.dev') }}</option>
          <option value="stable">{{ $t('settings.stable') }}</option>
          <option value="custom">{{ $t('settings.custom') }}</option>
        </select>
        <p v-if="channel === 'stable' || (channel === 'auto' && settings.defaultChannel === 'stable')" class="hint">
          {{ $t('settings.stablePending') }}
        </p>
        <template v-if="channel === 'custom'">
          <label for="catalog-url">{{ $t('settings.url') }}</label>
          <input
            id="catalog-url"
            v-model.trim="customUrl"
            type="url"
            aria-describedby="url-hint"
            placeholder="http://127.0.0.1:8765/"
            spellcheck="false"
            required
          />
          <p id="url-hint" class="hint">{{ $t('settings.urlHint') }}</p>
        </template>
        <dl>
          <dt>{{ $t('settings.effective') }}</dt>
          <dd><code>{{ settings.effectiveUrl }}</code></dd>
        </dl>
        <p v-if="settings.overridden" class="hint">{{ $t('settings.override') }}</p>
        <div class="actions">
          <button type="submit" :disabled="!changed">{{ $t(saving ? 'settings.saving' : 'settings.save') }}</button>
          <span v-if="saved" role="status">{{ $t('settings.saved') }}</span>
        </div>
        <p v-if="saveError" role="alert">{{ $t('settings.saveError', { reason: saveError }) }}</p>
      </fieldset>
    </form>
  </section>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { getCatalogSettings, saveCatalogSettings, type CatalogSelection, type CatalogSettings } from '../../utils/catalogSettings';

const settings = ref<CatalogSettings | null>(null);
const channel = ref<CatalogSelection['channel']>('auto');
const customUrl = ref('');
const loadError = ref('');
const saveError = ref('');
const saved = ref(false);
const saving = ref(false);

const changed = computed(() => {
  const current = settings.value?.selection;
  return current && (channel.value !== current.channel ||
    (channel.value === 'custom' && customUrl.value !== current.customUrl));
});

function display(result: CatalogSettings) {
  settings.value = result;
  channel.value = result.selection.channel;
  customUrl.value = result.selection.customUrl ?? '';
}

function clearFeedback() {
  saved.value = false;
  saveError.value = '';
}

async function load() {
  loadError.value = '';
  try {
    display(await getCatalogSettings());
  } catch (error) {
    loadError.value = String(error);
  }
}

async function save() {
  if (saving.value || !changed.value) return;
  clearFeedback();
  saving.value = true;
  try {
    const selection = channel.value === 'custom' ? { channel: channel.value, customUrl: customUrl.value } : { channel: channel.value };
    display(await saveCatalogSettings(selection));
    saved.value = true;
  } catch (error) {
    saveError.value = String(error);
  } finally {
    saving.value = false;
  }
}

onMounted(load);
</script>

<style scoped>
.settings {
  max-width: 760px;
  margin: 0 auto;
  padding: 64px 20px 32px;
  color: var(--color-text);
}

h1 { margin-bottom: 16px; }
fieldset {
  min-width: 0;
  padding: 16px 20px 20px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
}
legend { padding: 0 6px; font-size: 18px; font-weight: 600; }
label, dt { display: block; margin: 12px 0 5px; font-weight: 600; }
select, input, button {
  padding: 6px 10px;
  color: var(--color-text);
  background: var(--color-surface-raised);
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  font: inherit;
}
select, input { max-width: 100%; }
input { width: 100%; }
button { cursor: pointer; }
button:disabled { opacity: 0.55; cursor: default; }
select:focus-visible { outline: 2px solid var(--color-focus); }
.hint { margin-top: 6px; color: var(--color-text-muted); }
dd { overflow-wrap: anywhere; }
.actions { display: flex; align-items: center; gap: 12px; margin-top: 20px; }
[role="alert"] { margin-top: 12px; }
</style>
