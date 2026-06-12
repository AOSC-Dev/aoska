import { invoke } from '@tauri-apps/api/core';
import { InstallOperation, type OmaOperation, type TumUpdateInfo } from '../types/oma';
import type { CategoryIndex, Index, RecommendIndex } from '../types/home';
import type { PackageDetail } from '../types/packages';
import type { PmPackageRecord } from '../types/pm';
import { getPackageManager, shouldUseStubBackend } from './pmClient';
import { stubByCategory, stubDetail, stubIndex, stubRecommend } from './catalogStub';

function pmPackagesToOmaOperation(packages: PmPackageRecord[]): OmaOperation {
  return {
    install: packages.map((pkg, index) => ({
      name: pkg.packageName,
      name_without_arch: pkg.name,
      old_version: pkg.currentVersion,
      new_version: pkg.newVersion ?? pkg.currentVersion ?? '',
      old_size: pkg.installedSize,
      new_size: pkg.installedSize ?? 0,
      pkg_urls: [],
      arch: pkg.architecture ?? '',
      download_size: pkg.downloadSize ?? 0,
      op: pkg.installed ? InstallOperation.Upgrade : InstallOperation.Install,
      automatic: false,
      index,
    })),
    remove: [],
    disk_size_delta: 0,
    autoremovable: [0, 0],
    total_download_size: packages.reduce((sum, pkg) => sum + (pkg.downloadSize ?? 0), 0),
    suggest: [],
    recommend: [],
  };
}

export async function fetchUpdateDetail(): Promise<OmaOperation> {
  const updates = await getPackageManager().listUpdates();
  return pmPackagesToOmaOperation(updates.packages);
}

export async function fetchUpdateCount(): Promise<number> {
  return (await getPackageManager().updateSummary()).total;
}

export async function fetchTumUpdate(): Promise<TumUpdateInfo[] | null> {
  try {
    return await getPackageManager().tumUpdates();
  } catch (error) {
    if (String(error).includes('TUM_UNAVAILABLE')) {
      return null;
    }
    throw error;
  }
}

export async function fetchRecommend(): Promise<RecommendIndex> {
  if (shouldUseStubBackend()) return stubRecommend();
  return invoke<RecommendIndex>('fetch_recommend');
}

export async function fetchIndex(): Promise<Index> {
  if (shouldUseStubBackend()) return stubIndex();
  return invoke<Index>('fetch_index');
}

export async function fetchDetail(pkgName: string): Promise<PackageDetail> {
  if (shouldUseStubBackend()) return stubDetail(pkgName);
  return invoke<PackageDetail>('fetch_detail', { pkgName });
}

export async function fetchByCategory(category: string): Promise<CategoryIndex> {
  if (shouldUseStubBackend()) return stubByCategory(category);
  return invoke<CategoryIndex>('fetch_by_category', { category });
}

export async function getEndpointBaseUrl(): Promise<string> {
  if (shouldUseStubBackend()) return typeof window === 'undefined' ? '/' : window.location.origin;
  return invoke<string>('get_endpoint_base_url');
}
