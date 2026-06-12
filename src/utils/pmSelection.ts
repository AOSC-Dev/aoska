import type { PackageOperationKind, PmPackageRecord } from '../types/pm';

export interface BatchActionState {
  enabled: boolean;
  count: number;
}

export function selectedPackageNames(
  packages: PmPackageRecord[],
  selected: ReadonlySet<string>,
): string[] {
  return packages
    .filter((pkg) => selected.has(pkg.name))
    .map((pkg) => pkg.name);
}

export function batchActionState(
  _operation: Extract<PackageOperationKind, 'update' | 'remove'>,
  packages: PmPackageRecord[],
  selected: ReadonlySet<string>,
): BatchActionState {
  const count = selectedPackageNames(packages, selected).length;
  return { enabled: count > 0, count };
}
