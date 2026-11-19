export type PackageOperationKind = 'install' | 'remove' | 'update' | 'refresh';

export type PackagePrimaryAction =
  | 'install'
  | 'open'
  | 'update'
  | 'remove'
  | 'download'
  | 'manual'
  | 'none';

export interface PackageActionState {
  primary: PackagePrimaryAction;
  secondary: PackagePrimaryAction[];
  disabledReason?: string;
}

export interface PmCapabilities {
  kind: 'stub' | 'tauri';
  capabilities: string[];
  raw?: unknown;
}

export interface PmPackageRecord {
  name: string;
  displayName: string;
  catalogName?: string;
  icon?: string;
  desktopId?: string;
  packageName: string;
  architecture?: string;
  installed: boolean;
  upgradable: boolean;
  openable: boolean;
  aoskaManageable: boolean;
  currentVersion?: string;
  newVersion?: string;
  installedSize?: number;
  downloadSize?: number;
  source?: string;
  homepage?: string;
  isSecurity?: boolean;
  isSystem?: boolean;
  requiresManualAction?: boolean;
}

export interface PmPackageList {
  packages: PmPackageRecord[];
  raw?: unknown;
}

export interface PmPackageState extends PmPackageRecord {
  action: PackageActionState;
  raw?: unknown;
}

export interface PmUpdateSummary {
  total: number;
  security: number | null;
  securityClassificationAvailable: boolean;
  system: number;
  software: number;
  lastCheckedAt?: string;
  stale: boolean;
  raw?: unknown;
}

export interface PmTumUpdateInfo {
  manifest_name: string;
  name: Record<string, string>;
  is_security: boolean;
  package_count: number;
  package_names: string[];
  caution?: Record<string, string>;
}

export interface PmOperationStart {
  id: string;
  operation: PackageOperationKind;
  packages: string[];
  startedAt: string;
  raw?: unknown;
}

export type PmOperationState =
  | 'pending'
  | 'running'
  | 'succeeded'
  | 'failed'
  | 'cancelled'
  | 'already_terminal'
  | 'unknown';

export interface PmOperationStatus {
  id: string;
  operation: PackageOperationKind;
  state: PmOperationState;
  startedAt?: string;
  finishedAt?: string;
  message?: string;
  progress?: number;
  raw?: unknown;
}

export interface PmOperationLogs {
  id: string;
  lines: string[];
  raw?: unknown;
}

export interface PmCancelResult {
  id: string;
  state: 'cancelled' | 'already_terminal';
  raw?: unknown;
}

export interface PmStorageSegment {
  id: 'aoska' | 'oma' | 'unmanaged' | 'free';
  label: string;
  bytes: number;
  color: string;
}

export interface PmStorageSummary {
  totalUsedBytes: number;
  totalBytes: number;
  segments: PmStorageSegment[];
  estimated: boolean;
}

export interface PmPlanPackage {
  name: string;
  currentVersion?: string;
  newVersion?: string;
  downloadSize?: number;
  installedSize?: number;
}

export interface PmOperationPlan {
  operation: Exclude<PackageOperationKind, 'refresh'>;
  packages: string[];
  install: PmPlanPackage[];
  remove: PmPlanPackage[];
  totalDownloadSize: number;
  diskSizeDelta: number;
}

export interface PackageManagerClient {
  capabilities(): Promise<PmCapabilities>;
  updateSummary(): Promise<PmUpdateSummary>;
  listUpdates(): Promise<PmPackageList>;
  listCatalogInstalled(): Promise<PmPackageList>;
  listInstalled(): Promise<PmPackageList>;
  packageState(name: string): Promise<PmPackageState>;
  tumUpdates(): Promise<PmTumUpdateInfo[] | null>;
  storageSummary(): Promise<PmStorageSummary | null>;
  planUpdate(packages: string[]): Promise<PmOperationPlan>;
  planInstall(packages: string[]): Promise<PmOperationPlan>;
  planRemove(packages: string[]): Promise<PmOperationPlan>;
  startUpdate(packages: string[]): Promise<PmOperationStart>;
  startInstall(packages: string[]): Promise<PmOperationStart>;
  startRemove(packages: string[]): Promise<PmOperationStart>;
  startRefresh(): Promise<PmOperationStart>;
  operationStatus(id: string): Promise<PmOperationStatus>;
  operationLogs(id: string): Promise<PmOperationLogs>;
  cancelOperation(id: string): Promise<PmCancelResult>;
}
