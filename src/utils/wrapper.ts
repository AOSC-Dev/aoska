import { InstallOperation, type OmaOperation, type TumUpdateInfo } from '../types/oma';
import type { PmPackageRecord } from '../types/pm';
import { getPackageManager } from './pmClient';

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
