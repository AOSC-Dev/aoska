import { invoke, isTauri } from '@tauri-apps/api/core';
import { browserCatalogPreferences, type CatalogSelection, type CatalogSettings } from './catalogPreferences';

export type { CatalogSelection, CatalogSettings } from './catalogPreferences';

function browserPreferences() {
  return browserCatalogPreferences(
    window.localStorage,
    import.meta.env.DEV ? 'dev' : 'stable',
    import.meta.env.VITE_AOSKA_ASMR_URL,
  );
}

export async function getCatalogSettings(): Promise<CatalogSettings> {
  return isTauri() ? invoke<CatalogSettings>('get_catalog_settings') : browserPreferences().get();
}

export async function saveCatalogSettings(selection: CatalogSelection): Promise<CatalogSettings> {
  return isTauri()
    ? invoke<CatalogSettings>('set_catalog_settings', { selection })
    : browserPreferences().save(selection);
}
