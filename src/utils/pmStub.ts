import type {
  PackageManagerClient,
  PackageOperationKind,
  PackagePrimaryAction,
  PmCancelResult,
  PmCapabilities,
  PmOperationLogs,
  PmOperationStart,
  PmOperationState,
  PmOperationStatus,
  PmPackageList,
  PmPackageRecord,
  PmPackageState,
  PmStorageSummary,
  PmTumUpdateInfo,
  PmUpdateSummary,
} from '../types/pm';

interface StubOperation {
  unit: string;
  operation: PackageOperationKind;
  packages: string[];
  state: PmOperationState;
  startedAt: string;
  finishedAt?: string;
  logs: string[];
}

const STUB_NOW = '2026-06-12T08:00:00.000Z';

const stubPackages: PmPackageRecord[] = [
  {
    name: 'wechat',
    displayName: '微信',
    packageName: 'wechat',
    architecture: 'amd64',
    installed: true,
    upgradable: true,
    openable: true,
    aoskaManageable: true,
    currentVersion: '4.0.1.11',
    newVersion: '4.0.1.12',
    installedSize: 711_190_000,
    downloadSize: 126_500_000,
    source: '官方安装包',
    homepage: 'https://wx.qq.com',
    isSecurity: true,
    isSystem: false,
  },
  {
    name: 'firefox',
    displayName: 'Mozilla Firefox',
    packageName: 'firefox',
    architecture: 'amd64',
    installed: false,
    upgradable: false,
    openable: false,
    aoskaManageable: true,
    newVersion: '131.0.3',
    installedSize: 85_000_000,
    downloadSize: 88_000_000,
    source: '安同 OS 软件仓库',
    homepage: 'https://www.mozilla.org/firefox/',
    isSecurity: false,
    isSystem: false,
  },
  {
    name: 'steam',
    displayName: 'Steam',
    packageName: 'steam',
    architecture: 'amd64',
    installed: true,
    upgradable: false,
    openable: true,
    aoskaManageable: true,
    currentVersion: '1.0.0.78',
    installedSize: 120_000_000,
    source: '安同 OS 软件仓库',
    homepage: 'https://store.steampowered.com/',
    isSecurity: false,
    isSystem: false,
  },
  {
    name: 'aosc-os-base',
    displayName: 'AOSC OS Base',
    packageName: 'aosc-os-base',
    architecture: 'all',
    installed: true,
    upgradable: true,
    openable: false,
    aoskaManageable: false,
    currentVersion: '20260601',
    newVersion: '20260612',
    installedSize: 512_000_000,
    downloadSize: 64_000_000,
    source: '安同 OS 软件仓库',
    isSecurity: false,
    isSystem: true,
  },
];

const stubCapabilities = [
  'stub.backend.v1',
  'query.installed.v1',
  'query.upgradable.v1',
  'query.package-detail.v1',
  'storage.summary.v1',
  'run.upgrade.v1',
  'run.upgrade.selected.v1',
  'run.install.v1',
  'run.remove.v1',
  'run.refresh.v1',
  'unit.status.v1',
  'unit.result.v1',
  'unit.logs.v1',
  'unit.cancel.v1',
];

function clone<T>(value: T): T {
  return structuredClone(value);
}

function sortedPackages(packages: PmPackageRecord[]): PmPackageRecord[] {
  return [...packages].sort((a, b) => a.displayName.localeCompare(b.displayName));
}

function packageAction(pkg: PmPackageRecord): PackagePrimaryAction {
  if (pkg.requiresManualAction) return 'manual';
  if (!pkg.installed) return 'install';
  if (pkg.upgradable) return 'update';
  if (pkg.openable) return 'open';
  return 'remove';
}

function packageSecondaryActions(pkg: PmPackageRecord): PackagePrimaryAction[] {
  const actions: PackagePrimaryAction[] = [];
  if (pkg.homepage) actions.push('download');
  if (pkg.installed && pkg.openable) actions.push('open');
  if (pkg.installed) actions.push('remove');
  return actions.filter((action, index, all) => all.indexOf(action) === index && action !== packageAction(pkg));
}

function requirePackages(packages: string[], operation: PackageOperationKind): string[] {
  const filtered = packages.map((item) => item.trim()).filter(Boolean);
  if (operation !== 'refresh' && filtered.length === 0) {
    throw new Error(`${operation} requires at least one package`);
  }
  return filtered;
}

export function createStubPackageManager(): PackageManagerClient {
  const operations = new Map<string, StubOperation>();
  let operationId = 1;

  function makeOperation(operation: PackageOperationKind, requested: string[]): PmOperationStart {
    const packages = requirePackages(requested, operation);
    const unit = `aoska-stub-${operation}-${operationId++}.service`;
    const startedAt = new Date(STUB_NOW).toISOString();
    const logs = [
      `[stub] scheduled ${operation}`,
      packages.length > 0 ? `[stub] packages: ${packages.join(', ')}` : '[stub] refresh package metadata',
      '[stub] operation is safe; no real package manager was touched',
    ];
    operations.set(unit, {
      unit,
      operation,
      packages,
      state: 'running',
      startedAt,
      logs,
    });
    return { unit, operation, packages: clone(packages), startedAt };
  }

  function operation(unit: string): StubOperation {
    const found = operations.get(unit);
    if (!found) throw new Error(`unknown stub operation: ${unit}`);
    return found;
  }

  return {
    async capabilities(): Promise<PmCapabilities> {
      return { kind: 'stub', capabilities: clone(stubCapabilities) };
    },

    async updateSummary(): Promise<PmUpdateSummary> {
      const updates = stubPackages.filter((pkg) => pkg.upgradable);
      const security = updates.filter((pkg) => pkg.isSecurity).length;
      const system = updates.filter((pkg) => pkg.isSystem).length;
      return {
        total: updates.length,
        security,
        securityClassificationAvailable: true,
        system,
        software: updates.length - system,
        lastCheckedAt: STUB_NOW,
        stale: false,
      };
    },

    async listUpdates(): Promise<PmPackageList> {
      return { packages: clone(sortedPackages(stubPackages.filter((pkg) => pkg.upgradable))) };
    },

    async listInstalled(): Promise<PmPackageList> {
      return { packages: clone(sortedPackages(stubPackages.filter((pkg) => pkg.installed))) };
    },

    async packageState(name: string): Promise<PmPackageState> {
      const pkg = stubPackages.find((item) => item.name === name || item.packageName === name);
      if (!pkg) {
        return {
          name,
          displayName: name,
          packageName: name,
          installed: false,
          upgradable: false,
          openable: false,
          aoskaManageable: false,
          action: {
            primary: 'manual',
            secondary: [],
            disabledReason: 'Package is not present in the stub package-manager dataset.',
          },
        };
      }
      return {
        ...clone(pkg),
        action: {
          primary: packageAction(pkg),
          secondary: packageSecondaryActions(pkg),
        },
      };
    },

    async tumUpdates(): Promise<PmTumUpdateInfo[]> {
      return [
        {
          manifest_name: 'security',
          name: {
            default: '安全更新',
            zh_CN: '安全更新',
            en: 'Security updates',
          },
          is_security: true,
          package_count: 1,
          package_names: ['wechat'],
          caution: {
            default: '建议及时保存工作并更新。',
            zh_CN: '建议及时保存工作并更新。',
            en: 'Save your work and update soon.',
          },
        },
      ];
    },

    async storageSummary(): Promise<PmStorageSummary> {
      const totalBytes = 32 * 1024 ** 3;
      const aoskaBytes = 6.16 * 1024 ** 3;
      const omaBytes = 4.4 * 1024 ** 3;
      const unmanagedBytes = 2.2 * 1024 ** 3;
      return {
        totalUsedBytes: Math.round(aoskaBytes + omaBytes + unmanagedBytes),
        totalBytes,
        estimated: true,
        segments: [
          { id: 'aoska', label: 'Aoska 可管理的软件', bytes: Math.round(aoskaBytes), color: '#009688' },
          { id: 'oma', label: '其他 oma 软件', bytes: Math.round(omaBytes), color: '#1e88e5' },
          { id: 'unmanaged', label: '不由 oma 管理的空间', bytes: Math.round(unmanagedBytes), color: '#fb8c00' },
          { id: 'free', label: '可用空间', bytes: Math.round(totalBytes - aoskaBytes - omaBytes - unmanagedBytes), color: '#5f6368' },
        ],
      };
    },

    async startUpdate(packages: string[]): Promise<PmOperationStart> {
      return makeOperation('update', packages.length > 0 ? packages : stubPackages.filter((pkg) => pkg.upgradable).map((pkg) => pkg.name));
    },

    async startInstall(packages: string[]): Promise<PmOperationStart> {
      return makeOperation('install', packages);
    },

    async startRemove(packages: string[]): Promise<PmOperationStart> {
      return makeOperation('remove', packages);
    },

    async startRefresh(): Promise<PmOperationStart> {
      return makeOperation('refresh', ['metadata']);
    },

    async operationStatus(unit: string): Promise<PmOperationStatus> {
      const current = operation(unit);
      return {
        unit,
        operation: current.operation,
        state: current.state,
        startedAt: current.startedAt,
        finishedAt: current.finishedAt,
        message: current.state === 'running' ? 'Stub operation is running.' : 'Stub operation is terminal.',
      };
    },

    async operationLogs(unit: string): Promise<PmOperationLogs> {
      const current = operation(unit);
      return { unit, lines: clone(current.logs) };
    },

    async cancelOperation(unit: string): Promise<PmCancelResult> {
      const current = operation(unit);
      if (['succeeded', 'failed', 'cancelled'].includes(current.state)) {
        return { unit, state: 'already_terminal' };
      }
      current.state = 'cancelled';
      current.finishedAt = new Date(STUB_NOW).toISOString();
      current.logs.push('[stub] operation cancelled');
      return { unit, state: 'cancelled' };
    },
  };
}

export const stubPackageManager = createStubPackageManager();
