import { invoke } from '@tauri-apps/api/core';
import type { PackageManagerClient } from '../types/pm';
import { stubPackageManager } from './pmStub';
import { createTauriPackageManager } from './pmTauri';

export function shouldUseStubBackend(): boolean {
  if (import.meta.env.VITE_AOSKA_PM_BACKEND === 'stub') return true;
  if (typeof window === 'undefined') return true;
  return !('__TAURI_INTERNALS__' in window);
}

let packageManager: PackageManagerClient | null = null;

export function getPackageManager(): PackageManagerClient {
  if (packageManager) return packageManager;
  packageManager = shouldUseStubBackend() ? stubPackageManager : createTauriPackageManager(invoke);
  return packageManager;
}

export function resetPackageManagerForTests(client: PackageManagerClient | null): void {
  packageManager = client;
}
