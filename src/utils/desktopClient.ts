import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';
import { shouldUseStubBackend } from './pmClient';
import { externalPageUrl, launchDesktopApplication } from './desktopActions';

export async function launchCatalogApp(catalogName: string): Promise<void> {
  await launchDesktopApplication(catalogName, shouldUseStubBackend(), invoke);
}

export async function openExternalPage(value: string): Promise<void> {
  const url = externalPageUrl(value);
  if (shouldUseStubBackend()) window.open(url, '_blank', 'noopener,noreferrer');
  else await openUrl(url);
}
