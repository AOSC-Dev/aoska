import type { PackageOperationKind, PackagePrimaryAction, PmOperationState } from '../types/pm';

type Locale = 'zh' | 'en';

const actionLabels: Record<Locale, Record<PackagePrimaryAction, string>> = {
  zh: {
    install: '安装',
    open: '打开',
    update: '更新',
    remove: '卸载',
    download: '官方网站',
    manual: '手动操作',
    none: '不可用',
  },
  en: {
    install: 'Install',
    open: 'Open',
    update: 'Update',
    remove: 'Remove',
    download: 'Website',
    manual: 'Manual action',
    none: 'Unavailable',
  },
};

const operationLabels: Record<Locale, Record<PackageOperationKind, string>> = {
  zh: {
    install: '安装',
    remove: '卸载',
    update: '更新',
    refresh: '刷新',
  },
  en: {
    install: 'Installing',
    remove: 'Removing',
    update: 'Updating',
    refresh: 'Refreshing',
  },
};

export function normalizeLocale(locale: string): Locale {
  return locale.toLowerCase().startsWith('zh') ? 'zh' : 'en';
}

export function actionLabel(action: PackagePrimaryAction, locale = navigator.language): string {
  return actionLabels[normalizeLocale(locale)][action];
}

export function isDestructiveAction(action: PackagePrimaryAction): boolean {
  return action === 'remove';
}

export function operationHeading(
  operation: PackageOperationKind,
  state: PmOperationState,
  locale = navigator.language,
): string {
  const normalized = normalizeLocale(locale);
  if (normalized === 'en') {
    if (state === 'succeeded') return `${operationLabels.en[operation]} complete`;
    if (state === 'failed') return `${operationLabels.en[operation]} failed`;
    if (state === 'cancelled') return `${operationLabels.en[operation]} cancelled`;
    return operationLabels.en[operation];
  }
  if (state === 'succeeded') return `${operationLabels.zh[operation]}完成`;
  if (state === 'failed') return `${operationLabels.zh[operation]}失败`;
  if (state === 'cancelled') return `${operationLabels.zh[operation]}已取消`;
  return `正在${operationLabels.zh[operation]}`;
}
