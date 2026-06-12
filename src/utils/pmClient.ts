import { invoke } from '@tauri-apps/api/core';
import { stubPackageManager } from './pmStub';
import type {
  PackageManagerClient,
  PackageOperationKind,
  PmCancelResult,
  PmCapabilities,
  PmOperationLogs,
  PmOperationStart,
  PmOperationStatus,
  PmPackageList,
  PmPackageRecord,
  PmPackageState,
  PmStorageSummary,
  PmTumUpdateInfo,
  PmUpdateSummary,
} from '../types/pm';

type InvokeFunction = <T>(command: string, args?: Record<string, unknown>) => Promise<T>;

type RawRecord = Record<string, unknown>;

function isRecord(value: unknown): value is RawRecord {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function stringField(value: RawRecord, keys: string[], fallback = ''): string {
  for (const key of keys) {
    const found = value[key];
    if (typeof found === 'string' && found.length > 0) return found;
  }
  return fallback;
}

function numberField(value: RawRecord, keys: string[]): number | undefined {
  for (const key of keys) {
    const found = value[key];
    if (typeof found === 'number') return found;
    if (typeof found === 'string') {
      const parsed = Number(found);
      if (Number.isFinite(parsed)) return parsed;
    }
  }
  return undefined;
}

function boolField(value: RawRecord, keys: string[], fallback = false): boolean {
  for (const key of keys) {
    const found = value[key];
    if (typeof found === 'boolean') return found;
  }
  return fallback;
}

function packageArray(raw: unknown): RawRecord[] {
  if (Array.isArray(raw)) return raw.filter(isRecord);
  if (!isRecord(raw)) return [];
  for (const key of ['packages', 'items', 'updates', 'installed']) {
    const value = raw[key];
    if (Array.isArray(value)) return value.filter(isRecord);
  }
  return [];
}

function normalizePackage(raw: RawRecord, defaults: Partial<PmPackageRecord>): PmPackageRecord {
  const name = stringField(raw, ['name_without_arch', 'name', 'package', 'packageName'], defaults.name ?? 'unknown');
  const currentVersion = stringField(raw, ['current_version', 'currentVersion', 'version', 'old_version'], defaults.currentVersion);
  const newVersion = stringField(raw, ['new_version', 'newVersion'], defaults.newVersion);
  const installed = defaults.installed ?? Boolean(currentVersion);
  const upgradable = defaults.upgradable ?? Boolean(newVersion);
  return {
    name,
    displayName: stringField(raw, ['title', 'display_name', 'displayName', 'name'], defaults.displayName ?? name),
    packageName: stringField(raw, ['package_name', 'packageName', 'name'], defaults.packageName ?? name),
    architecture: stringField(raw, ['architecture', 'arch'], defaults.architecture),
    installed,
    upgradable,
    openable: defaults.openable ?? installed,
    aoskaManageable: defaults.aoskaManageable ?? true,
    currentVersion,
    newVersion,
    installedSize: numberField(raw, ['installed_size', 'installedSize', 'new_size', 'size']) ?? defaults.installedSize,
    downloadSize: numberField(raw, ['download_size', 'downloadSize']) ?? defaults.downloadSize,
    source: stringField(raw, ['source'], defaults.source),
    homepage: stringField(raw, ['homepage'], defaults.homepage),
    isSecurity: boolField(raw, ['is_security', 'isSecurity'], defaults.isSecurity),
    isSystem: boolField(raw, ['is_system', 'isSystem'], defaults.isSystem),
    requiresManualAction: boolField(raw, ['requires_manual_action', 'requiresManualAction'], defaults.requiresManualAction),
  };
}

function normalizeList(raw: unknown, defaults: Partial<PmPackageRecord>): PmPackageList {
  return {
    packages: packageArray(raw).map((pkg) => normalizePackage(pkg, defaults)),
    raw,
  };
}

function actionForPackage(pkg: PmPackageRecord): PmPackageState['action'] {
  if (pkg.requiresManualAction) return { primary: 'manual', secondary: [] };
  if (!pkg.installed) return { primary: 'install', secondary: pkg.homepage ? ['download'] : [] };
  if (pkg.upgradable) return { primary: 'update', secondary: ['open', 'remove'].filter((action) => action !== 'open' || pkg.openable) as PmPackageState['action']['secondary'] };
  if (pkg.openable) return { primary: 'open', secondary: ['remove'] };
  return { primary: 'remove', secondary: [] };
}

function operationFromRaw(raw: unknown, operation: PackageOperationKind, packages: string[]): PmOperationStart {
  if (isRecord(raw)) {
    return {
      unit: stringField(raw, ['unit'], `unknown-${operation}.service`),
      operation: stringField(raw, ['operation'], operation) as PackageOperationKind,
      packages,
      startedAt: new Date().toISOString(),
      raw,
    };
  }
  return { unit: `unknown-${operation}.service`, operation, packages, startedAt: new Date().toISOString(), raw };
}

export function shouldUseStubBackend(): boolean {
  if (import.meta.env.VITE_AOSKA_PM_BACKEND === 'stub') return true;
  if (typeof window === 'undefined') return true;
  return !('__TAURI_INTERNALS__' in window);
}

export function createTauriPackageManager(invokeCommand: InvokeFunction = invoke): PackageManagerClient {
  return {
    async capabilities(): Promise<PmCapabilities> {
      const result = await invokeCommand<{ capabilities: string[]; raw?: unknown }>('pm_capabilities');
      return { kind: 'tauri', capabilities: result.capabilities, raw: result.raw };
    },

    async updateSummary(): Promise<PmUpdateSummary> {
      const result = await invokeCommand<RawRecord>('pm_update_summary');
      return {
        total: numberField(result, ['total']) ?? 0,
        security: numberField(result, ['security']) ?? null,
        securityClassificationAvailable: boolField(result, ['security_classification_available', 'securityClassificationAvailable']),
        system: numberField(result, ['system']) ?? 0,
        software: numberField(result, ['software']) ?? numberField(result, ['total']) ?? 0,
        stale: boolField(result, ['stale']),
        raw: result['raw'] ?? result,
      };
    },

    async listUpdates(): Promise<PmPackageList> {
      const result = await invokeCommand<unknown>('pm_list_updates');
      return normalizeList(result, { installed: true, upgradable: true });
    },

    async listInstalled(): Promise<PmPackageList> {
      const result = await invokeCommand<unknown>('pm_list_installed');
      return normalizeList(result, { installed: true, upgradable: false });
    },

    async packageState(name: string): Promise<PmPackageState> {
      const result = await invokeCommand<unknown>('pm_package_state', { packages: [name] });
      const [pkg] = normalizeList(result, { name, packageName: name }).packages;
      const state = pkg ?? normalizePackage({ name }, { name, packageName: name, installed: false, upgradable: false, aoskaManageable: false });
      return { ...state, action: actionForPackage(state), raw: result };
    },

    async tumUpdates(): Promise<PmTumUpdateInfo[] | null> {
      return null;
    },

    async storageSummary(): Promise<PmStorageSummary | null> {
      return null;
    },

    async startUpdate(packages: string[]): Promise<PmOperationStart> {
      const result = await invokeCommand<unknown>('pm_start_update', { packages, assumeYes: true });
      return operationFromRaw(result, 'update', packages);
    },

    async startInstall(packages: string[]): Promise<PmOperationStart> {
      const result = await invokeCommand<unknown>('pm_start_install', { packages, assumeYes: true });
      return operationFromRaw(result, 'install', packages);
    },

    async startRemove(packages: string[]): Promise<PmOperationStart> {
      const result = await invokeCommand<unknown>('pm_start_remove', { packages, removeConfig: true, assumeYes: true });
      return operationFromRaw(result, 'remove', packages);
    },

    async startRefresh(): Promise<PmOperationStart> {
      const result = await invokeCommand<unknown>('pm_start_refresh', { purpose: 'check-updates' });
      return operationFromRaw(result, 'refresh', []);
    },

    async operationStatus(unit: string): Promise<PmOperationStatus> {
      const result = await invokeCommand<RawRecord>('pm_operation_status', { unit });
      return {
        unit,
        operation: stringField(result, ['operation'], 'update') as PackageOperationKind,
        state: stringField(result, ['state'], 'unknown') as PmOperationStatus['state'],
        message: stringField(result, ['message'], undefined),
        raw: result,
      };
    },

    async operationLogs(unit: string): Promise<PmOperationLogs> {
      const result = await invokeCommand<unknown>('pm_operation_logs', { unit });
      const lines = isRecord(result) && Array.isArray(result['lines'])
        ? result['lines'].filter((line): line is string => typeof line === 'string')
        : [];
      return { unit, lines, raw: result };
    },

    async cancelOperation(unit: string): Promise<PmCancelResult> {
      const result = await invokeCommand<RawRecord>('pm_cancel_operation', { unit });
      const state = stringField(result, ['state'], 'cancelled');
      return { unit, state: state === 'already_terminal' ? 'already_terminal' : 'cancelled', raw: result };
    },
  };
}

let packageManager: PackageManagerClient | null = null;

export function getPackageManager(): PackageManagerClient {
  if (packageManager) return packageManager;
  packageManager = shouldUseStubBackend() ? stubPackageManager : createTauriPackageManager();
  return packageManager;
}

export function resetPackageManagerForTests(client: PackageManagerClient | null): void {
  packageManager = client;
}
