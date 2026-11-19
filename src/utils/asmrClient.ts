import { invoke, isTauri } from '@tauri-apps/api/core';
import { createAsmrCatalog } from './asmr';
import { getCatalogSettings } from './catalogSettings';
export type { CatalogDetail } from './asmr';

async function catalog() {
  const { effectiveUrl } = await getCatalogSettings();
  if (isTauri()) {
    return createAsmrCatalog(effectiveUrl, (url) =>
      invoke('fetch_catalog_json', { path: url.slice(effectiveUrl.length) }),
    );
  }
  return createAsmrCatalog(effectiveUrl);
}

export async function getCatalogHomepage(locale = 'zh-CN') {
  return (await catalog()).homepage(locale);
}

export async function getCatalogDetail(id: string, locale = 'zh-CN') {
  return (await catalog()).detail(id, locale);
}

export async function getCatalogCards(locale = 'zh-CN') {
  return (await catalog()).cards(locale);
}
