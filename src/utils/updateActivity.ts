import type { PackageManagerClient, PmOperationStart, PmOperationStatus } from '../types/pm';

export interface UpdateActivityState {
  lastCheckedAt?: string;
  lastUpdatedAt?: string;
  current?: PmOperationStart;
  result?: PmOperationStatus;
  uncertain?: PmOperationStatus;
}
type Store = Pick<Storage, 'getItem' | 'setItem'>;
const key = 'aoska.update-times.v1';
export function isStale(time: string | undefined, now = Date.now()): boolean {
  return !time || !Number.isFinite(Date.parse(time)) || now - Date.parse(time) >= 14 * 86400000;
}
export function formatActivityDate(time: string, now = Date.now(), locale = 'zh-CN'): string {
  const days = Math.max(0, Math.floor((now - Date.parse(time)) / 86400000));
  if (days < 10) return days === 0 ? '今天' : `${days} 天前`;
  const date = new Date(time);
  return date.toLocaleDateString(locale, { year: date.getFullYear() === new Date(now).getFullYear() ? undefined : 'numeric', month: 'long', day: 'numeric' });
}
export function createUpdateActivity(storage?: Store, clock = Date.now) {
  let value: UpdateActivityState = {};
  try {
    const saved = JSON.parse(storage?.getItem(key) ?? '{}');
    for (const field of ['lastCheckedAt', 'lastUpdatedAt'] as const) {
      if (typeof saved?.[field] === 'string' && Number.isFinite(Date.parse(saved[field]))) value[field] = saved[field];
    }
  } catch { /* Storage is optional. */ }
  const listeners = new Set<() => void>();
  function notify() { listeners.forEach(listener => listener()); }
  return {
    get: () => ({ ...value }),
    clock,
    subscribe(listener: () => void) {
      listeners.add(listener);
      return () => { listeners.delete(listener); };
    },
    dismiss() {
      value = { ...value, result: undefined };
      notify();
    },
    started(operation: PmOperationStart) {
      value.current = operation;
      value.result = undefined;
      value.uncertain = undefined;
      notify();
    },
    observed(status: PmOperationStatus) {
      if (!value.current || value.current.id !== status.id) return;
      if (status.state === 'unknown') {
        value.uncertain = status;
        notify();
        return;
      }
      if (value.uncertain) {
        value.uncertain = undefined;
        notify();
      }
      if (!['succeeded', 'failed', 'cancelled'].includes(status.state)) return;
      if (status.state === 'succeeded') {
        const time = new Date(clock()).toISOString();
        if (status.operation === 'refresh') value.lastCheckedAt = time;
        if (status.operation === 'update') value.lastUpdatedAt = time;
        try {
          storage?.setItem(key, JSON.stringify({
            lastCheckedAt: value.lastCheckedAt,
            lastUpdatedAt: value.lastUpdatedAt,
          }));
        } catch { /* Storage is optional. */ }
      }
      value.result = { ...status, finishedAt: status.finishedAt ?? new Date(clock()).toISOString() };
      value.current = undefined;
      notify();
    },
  };
}
let browserStore: Store | undefined;
try {
  if (typeof window !== 'undefined') browserStore = window.localStorage;
} catch { /* Storage is optional. */ }
export const updateActivity = createUpdateActivity(browserStore);
export function trackPackageManager(pm: PackageManagerClient, activity = updateActivity): PackageManagerClient {
  const started = async (call: () => Promise<PmOperationStart>) => {
    const operation = await call();
    activity.started(operation);
    return operation;
  };
  return {
    ...pm,
    async updateSummary() {
      const summary = await pm.updateSummary();
      const time = activity.get().lastCheckedAt;
      return { ...summary, lastCheckedAt: time, stale: isStale(time, activity.clock()) };
    },
    startRefresh: () => started(() => pm.startRefresh()),
    startUpdate: packages => started(() => pm.startUpdate(packages)),
    startInstall: packages => started(() => pm.startInstall(packages)),
    startRemove: packages => started(() => pm.startRemove(packages)),
    async operationStatus(id) {
      const status = await pm.operationStatus(id);
      activity.observed(status);
      return status;
    },
  };
}

export function activityPresentation(input: { activity: UpdateActivityState; loading?: boolean; error?: string; security: number; updates: number; stale: boolean }) {
  const { activity } = input;
  if (activity.uncertain) return { state: 'unknown', tone: 'warning-update' };
  if (activity.current?.operation === 'refresh') return { state: 'checking', tone: 'checking-update' };
  if (input.error) return { state: 'error', tone: 'warning-update' };
  if (activity.result?.state === 'succeeded') return { state: 'completed', tone: 'no-update' };
  if (activity.result) return { state: 'failed', tone: 'warning-update' };
  if (input.loading) return { state: 'loading', tone: 'checking-update' };
  if (input.security > 0) return { state: 'security', tone: 'security-update' };
  if (input.stale) return { state: 'stale', tone: 'warning-update' };
  if (input.updates > 0) return { state: 'updates', tone: 'system-update' };
  return { state: 'empty', tone: 'no-update' };
}
