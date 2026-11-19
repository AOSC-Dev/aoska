import { catalogRoot } from './asmr';

export const channelUrls = {
  dev: 'https://aosc-dev.github.io/aosc-os-asmr/v1/',
  stable: 'https://repo.aosc.io/asmr/v1/',
};

type BuildChannel = keyof typeof channelUrls;
export type CatalogSelection = { channel: 'auto' | BuildChannel | 'custom'; customUrl?: string };
export type CatalogSettings = {
  selection: CatalogSelection;
  defaultChannel: BuildChannel;
  effectiveUrl: string;
  overridden: boolean;
};

function normalizeSelection(value: unknown): CatalogSelection {
  if (value && typeof value === 'object' && 'channel' in value) {
    const { channel } = value;
    if (channel === 'auto' || channel === 'dev' || channel === 'stable') return { channel };
    if (channel === 'custom' && 'customUrl' in value && typeof value.customUrl === 'string') {
      return { channel, customUrl: catalogRoot(value.customUrl.trim()).href };
    }
  }
  throw new Error('Invalid ASMR catalog source');
}

export function browserCatalogPreferences(
  storage: Pick<Storage, 'getItem' | 'setItem'>,
  defaultChannel: BuildChannel,
  override?: string,
) {
  const key = 'aoska.catalog.source';
  function resolve(selection: CatalogSelection): CatalogSettings {
    const channel = selection.channel;
    const configured = override?.trim();
    const url =
      channel === 'custom'
        ? selection.customUrl!
        : channelUrls[channel === 'auto' ? defaultChannel : channel];
    return {
      selection,
      defaultChannel,
      effectiveUrl: catalogRoot(configured || url).href,
      overridden: !!configured,
    };
  }
  return {
    get() {
      const saved = storage.getItem(key);
      return resolve(saved === null ? { channel: 'auto' } : normalizeSelection(JSON.parse(saved)));
    },
    save(value: CatalogSelection) {
      const settings = resolve(normalizeSelection(value));
      storage.setItem(key, JSON.stringify(settings.selection));
      return settings;
    },
  };
}
