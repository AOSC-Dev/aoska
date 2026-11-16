import type {
  PackageManagerClient,
  PackageOperationKind,
  PmCancelResult,
  PmCapabilities,
  PmOperationLogs,
  PmOperationPlan,
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

function optionalStringField(value: RawRecord, keys: string[], fallback?: string): string | undefined {
  for (const key of keys) {
    const found = value[key];
    if (typeof found === 'string' && found.length > 0) return found;
  }
  return fallback;
}

function stringField(value: RawRecord, keys: string[], fallback = ''): string {
  return optionalStringField(value, keys) ?? fallback;
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
  const name = stringField(raw, ['name', 'package_name', 'packageName', 'package', 'name_without_arch'], defaults.name ?? 'unknown');
  const currentVersion = optionalStringField(raw, ['current_version', 'currentVersion', 'old_version'], defaults.currentVersion);
  const newVersion = optionalStringField(raw, ['new_version', 'newVersion'], defaults.newVersion);
  const installed = boolField(raw, ['installed'], defaults.installed ?? Boolean(currentVersion));
  const upgradable = boolField(raw, ['upgradable'], defaults.upgradable ?? (installed && Boolean(newVersion)));
  return {
    name,
    displayName: stringField(raw, ['title', 'display_name', 'displayName', 'name'], defaults.displayName ?? name),
    packageName: stringField(raw, ['package_name', 'packageName', 'name'], defaults.packageName ?? name),
    architecture: optionalStringField(raw, ['architecture', 'arch'], defaults.architecture),
    installed,
    upgradable,
    openable: boolField(raw, ['openable'], false),
    aoskaManageable: boolField(raw, ['aoska_manageable', 'aoskaManageable'], defaults.aoskaManageable ?? true),
    currentVersion,
    newVersion,
    installedSize: numberField(raw, ['installed_size', 'installedSize', 'new_size', 'size']) ?? defaults.installedSize,
    downloadSize: numberField(raw, ['download_size', 'downloadSize']) ?? defaults.downloadSize,
    source: optionalStringField(raw, ['source'], defaults.source),
    homepage: optionalStringField(raw, ['homepage'], defaults.homepage),
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
  if (pkg.requiresManualAction || !pkg.aoskaManageable) {
    return { primary: 'manual', secondary: [], disabledReason: '当前软件包无法通过软件中心管理，请参考官方网站手动处理。' };
  }
  if (!pkg.installed) return { primary: 'install', secondary: pkg.homepage ? ['download'] : [] };
  if (pkg.upgradable) return { primary: 'update', secondary: ['open', 'remove'].filter((action) => action !== 'open' || pkg.openable) as PmPackageState['action']['secondary'] };
  if (pkg.openable) return { primary: 'open', secondary: ['remove'] };
  return { primary: 'remove', secondary: [] };
}

function operationFromRaw(raw: unknown, operation: PackageOperationKind, packages: string[]): PmOperationStart {
  if (!isRecord(raw) || typeof raw.id !== 'string' || !raw.id) {
    throw new Error('Package operation response is missing its string id');
  }
  return {
    id: raw.id,
    operation,
    packages,
    startedAt: stringField(raw, ['started_at', 'startedAt'], new Date().toISOString()),
    raw,
  };
}

function planFromRaw(raw: RawRecord, operation: PmOperationPlan['operation'], packages: string[]): PmOperationPlan {
  const entries = (value: unknown) => {
    if (!Array.isArray(value)) throw new Error('Invalid package operation plan');
    return value.map((item: RawRecord) => ({
      name: stringField(item, ['name']),
      currentVersion: optionalStringField(item, ['current_version', 'currentVersion']),
      newVersion: optionalStringField(item, ['new_version', 'newVersion']),
      downloadSize: numberField(item, ['download_size', 'downloadSize']),
      installedSize: numberField(item, ['installed_size', 'installedSize']),
    }));
  };
  return { operation, packages, install: entries(raw.install), remove: entries(raw.remove),
    totalDownloadSize: numberField(raw, ['total_download_size', 'totalDownloadSize']) ?? 0,
    diskSizeDelta: numberField(raw, ['disk_size_delta', 'diskSizeDelta']) ?? 0 };
}

export function createTauriPackageManager(invokeCommand: InvokeFunction): PackageManagerClient {
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
      return invokeCommand<PmTumUpdateInfo[]>('pm_tum_updates');
    },

    async storageSummary(): Promise<PmStorageSummary | null> {
      return null;
    },

    async planUpdate(packages: string[]): Promise<PmOperationPlan> {
      return planFromRaw(await invokeCommand<RawRecord>('pm_plan_update', { packages }), 'update', packages);
    },
    async planInstall(packages: string[]): Promise<PmOperationPlan> {
      return planFromRaw(await invokeCommand<RawRecord>('pm_plan_install', { packages }), 'install', packages);
    },
    async planRemove(packages: string[]): Promise<PmOperationPlan> {
      return planFromRaw(await invokeCommand<RawRecord>('pm_plan_remove', { packages }), 'remove', packages);
    },

    async startUpdate(packages: string[]): Promise<PmOperationStart> {
      const result = await invokeCommand<unknown>('pm_start_update', { packages });
      return operationFromRaw(result, 'update', packages);
    },

    async startInstall(packages: string[]): Promise<PmOperationStart> {
      const result = await invokeCommand<unknown>('pm_start_install', { packages });
      return operationFromRaw(result, 'install', packages);
    },

    async startRemove(packages: string[]): Promise<PmOperationStart> {
      const result = await invokeCommand<unknown>('pm_start_remove', { packages });
      return operationFromRaw(result, 'remove', packages);
    },

    async startRefresh(): Promise<PmOperationStart> {
      const result = await invokeCommand<unknown>('pm_start_refresh', {});
      return operationFromRaw(result, 'refresh', []);
    },

    async operationStatus(id: string): Promise<PmOperationStatus> {
      const result = await invokeCommand<RawRecord>('pm_operation_status', { id });
      return {
        id,
        operation: stringField(result, ['operation'], 'update') as PackageOperationKind,
        state: stringField(result, ['state'], 'unknown') as PmOperationStatus['state'],
        message: optionalStringField(result, ['message'], undefined),
        progress: numberField(result, ['progress']),
        raw: result,
      };
    },

    async operationLogs(id: string): Promise<PmOperationLogs> {
      const result = await invokeCommand<unknown>('pm_operation_logs', { id });
      const lines = isRecord(result) && Array.isArray(result['lines'])
        ? result['lines'].filter((line): line is string => typeof line === 'string')
        : [];
      return { id, lines, raw: result };
    },

    async cancelOperation(id: string): Promise<PmCancelResult> {
      const result = await invokeCommand<RawRecord>('pm_cancel_operation', { id });
      const state = stringField(result, ['state']);
      if (state !== 'already_terminal' && state !== 'cancelled') throw new Error('Cancellation was not confirmed');
      return { id, state, raw: result };
    },
  };
}
