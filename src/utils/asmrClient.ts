import { invoke, isTauri } from '@tauri-apps/api/core';
import { catalogRoot, createAsmrCatalog } from './asmr';
export type { CatalogDetail } from './asmr';

async function catalog() {
  if (isTauri()) {
    const root = catalogRoot(await invoke<string>('get_endpoint_base_url'));
    return createAsmrCatalog(root.href, (url) =>
      invoke('fetch_catalog_json', { path: url.slice(root.href.length) }),
    );
  }
  return createAsmrCatalog(import.meta.env.VITE_AOSKA_ASMR_URL || 'http://127.0.0.1:8765/');
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
