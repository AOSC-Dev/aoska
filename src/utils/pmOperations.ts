import type { PackageManagerClient, PackageOperationKind, PmOperationPlan, PmOperationStart, PmOperationStatus } from '../types/pm';

export function isTerminalOperation(state: string): boolean {
  return ['succeeded', 'failed', 'cancelled', 'already_terminal'].includes(state);
}

export type InstalledScope = 'catalog' | 'system' | 'unavailable';

async function loadInstalledApps(pm: PackageManagerClient, scope: InstalledScope) {
  if (scope === 'unavailable') return { installed: [] };
  try {
    const list = scope === 'catalog' ? await pm.listCatalogInstalled() : await pm.listInstalled();
    const unique = new Map(list.packages.filter(pkg => pkg.installed).map(pkg => [pkg.name, pkg]));
    return { installed: [...unique.values()] };
  } catch (error) {
    // Updates remain usable while the installed tab displays the query error.
    return { installed: [], installedError: String(error) };
  }
}

export async function loadPackageInventory(pm: PackageManagerClient) {
  const capabilities = await pm.capabilities();
  const supports = (name: string) => capabilities.capabilities.includes(name);
  let installedScope: InstalledScope = 'unavailable';
  if (supports('query.catalog-installed.v1')) installedScope = 'catalog';
  else if (supports('query.installed.v1')) installedScope = 'system';

  const [summary, updates, inventory, storage] = await Promise.all([
    pm.updateSummary(),
    pm.listUpdates(),
    loadInstalledApps(pm, installedScope),
    supports('storage.summary.v1') ? pm.storageSummary() : null,
  ]);
  return { capabilities, summary, updates: updates.packages, installedScope, ...inventory, storage };
}

interface OperationObserver {
  update(status: PmOperationStatus, lines: string[]): void;
  error(error: unknown): void;
  finished(status: PmOperationStatus): void;
}

interface Scheduler {
  schedule(callback: () => Promise<void>): ReturnType<typeof setTimeout>;
  clear(timer: ReturnType<typeof setTimeout>): void;
}

export function observeOperation(
  pm: Pick<PackageManagerClient, 'operationStatus' | 'operationLogs'>,
  id: string,
  observer: OperationObserver,
  scheduler: Scheduler = { schedule: callback => setTimeout(callback, 1000), clear: clearTimeout },
): () => void {
  let stopped = false;
  let timer: ReturnType<typeof setTimeout> | undefined;
  async function poll() {
    try {
      const status = await pm.operationStatus(id);
      const logs = await pm.operationLogs(id);
      if (stopped) return;
      observer.update(status, logs.lines);
      if (isTerminalOperation(status.state)) {
        stopped = true;
        observer.finished(status);
        return;
      }
    } catch (error) {
      if (!stopped) observer.error(error);
    }
    if (!stopped) timer = scheduler.schedule(poll);
  }
  void poll();
  return () => {
    stopped = true;
    if (timer !== undefined) scheduler.clear(timer);
  };
}

export interface PackageOperationFlowState {
  plan: PmOperationPlan | null;
  activeOperation: PmOperationStart | null;
  busy: boolean;
  submitting: boolean;
  error: string;
}

export function createPackageOperationFlow(pm: PackageManagerClient, state: PackageOperationFlowState, onFinished: () => Promise<void>) {
  const locked = () => state.busy || state.submitting;
  async function prepare(operation: Exclude<PackageOperationKind, 'refresh'>, packages: string[]) {
    if (locked()) return;
    state.error = '';
    state.plan = null;
    state.submitting = true;
    try {
      state.plan = await (operation === 'install' ? pm.planInstall(packages)
        : operation === 'remove' ? pm.planRemove(packages) : pm.planUpdate(packages));
    } catch (cause) {
      state.error = String(cause);
    } finally {
      state.submitting = false;
    }
  }
  async function confirm() {
    if (!state.plan || locked()) return;
    const selected = state.plan;
    state.submitting = true;
    state.error = '';
    try {
      state.activeOperation = await (selected.operation === 'install' ? pm.startInstall(selected.packages)
        : selected.operation === 'remove' ? pm.startRemove(selected.packages) : pm.startUpdate(selected.packages));
      state.busy = true;
      state.plan = null;
    } catch (cause) {
      state.error = String(cause);
    } finally {
      state.submitting = false;
    }
  }
  async function refresh() {
    if (locked()) return;
    state.submitting = true;
    state.error = '';
    try {
      state.activeOperation = await pm.startRefresh();
      state.busy = true;
    } catch (cause) {
      state.error = String(cause);
    } finally {
      state.submitting = false;
    }
  }
  async function finished() {
    try { await onFinished(); } catch (cause) { state.error = String(cause); }
    finally { state.busy = false; }
  }
  function close() {
    if (locked()) return;
    state.activeOperation = null;
    state.plan = null;
  }
  return { prepare, confirm, refresh, finished, close };
}
