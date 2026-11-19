type Locales<T> = Record<string, T>;
type Caption = { title: string; subtitle?: string };
type AppText = { name: string; summary?: string; description_markdown?: string };
type Target = { app?: string; collection?: string; url?: string };
type App = {
  id: string;
  default_locale: string;
  locales: Locales<AppText>;
  icon?: string | null;
  categories: string[];
};
type Collection = { id: string; apps: string[]; locales: Locales<Caption> };
type Index = { default_locale: string; homepage: string; apps: App[] };
type Homepage = {
  hero: { target: Target; locales: Locales<Caption>; image: string } | null;
  essentials: string | null;
  guides: {
    more_url?: string;
    items: { id: string; date: string; url: string; locales: Locales<Caption> }[];
  };
};
type Installer = { kind: string; package?: { names: string[] }; acquisition?: { landing_url?: string } };
type Disclosures = {
  availability?: string;
  runtime?: string;
  service?: string;
  support?: string;
  telemetry?: string;
  notes?: string[];
  reviewed_at?: string;
};
type Detail = App & {
  external?: boolean;
  disclosures: Disclosures;
  installers: Installer[];
  publisher?: { name: string };
  links?: { homepage?: string };
  media: { icon?: string; hero?: string; screenshots?: { path: string; alt: Locales<string> }[] };
};

function localized<T>(values: Locales<T>, locale: string, fallback: string): T {
  const language = locale.split('-')[0];
  const key = [
    locale,
    language,
    ...Object.keys(values).filter((k) => k.startsWith(language + '-')),
    fallback,
  ].find((k) => Object.prototype.hasOwnProperty.call(values, k));
  const value = key ? values[key] : Object.values(values)[0];
  if (value === undefined) throw new Error('ASMR record has no translations');
  return value;
}

function appPath(id: string): string {
  if (!/^[a-z0-9][a-z0-9+._-]*$/.test(id)) throw new Error('Invalid ASMR app ID');
  return 'apps/' + id + '.json';
}

export function catalogRoot(address: string): URL {
  const root = new URL(address);
  if (!['http:', 'https:'].includes(root.protocol) || root.search || root.hash) {
    throw new Error('ASMR requires an HTTP(S) directory URL');
  }
  if (!root.pathname.endsWith('/')) root.pathname += '/';
  return root;
}

function catalogUrl(root: URL, path: string): string {
  const url = new URL(path, root);
  if (!url.href.startsWith(root.href) || !path || /^[/:\\]/.test(path)) {
    throw new Error('Invalid ASMR catalog path: ' + path);
  }
  return url.href;
}

async function httpJson(url: string): Promise<unknown> {
  const response = await fetch(url);
  if (!response.ok) throw new Error('ASMR ' + response.status + ': ' + url);
  return response.json();
}

export function createAsmrCatalog(address: string, loadJson = httpJson) {
  const root = catalogRoot(address);
  async function read<T>(path: string): Promise<T> {
    const value = await loadJson(catalogUrl(root, path));
    if (!value || typeof value !== 'object' || !('schema_version' in value) || value.schema_version !== 1) {
      throw new Error('Unsupported ASMR schema in ' + path);
    }
    return value as T;
  }

  function card(app: App, locale: string) {
    const text = localized(app.locales, locale, app.default_locale);
    return {
      id: app.id,
      name: text.name,
      summary: text.summary,
      icon: app.icon ? catalogUrl(root, app.icon) : undefined,
    };
  }

  async function detail(id: string, locale: string) {
    const app = await read<Detail>(appPath(id));
    if (app.id !== id) throw new Error('Unexpected ASMR app ID: ' + app.id);
    const text = localized(app.locales, locale, app.default_locale);
    const packageNames = app.external
      ? []
      : app.installers
        .filter((installer) => installer.kind === 'aosc-repository')
        .flatMap((installer) => installer.package?.names ?? []);
    const external = app.installers.find((i) => ['browser-assisted-deb', 'external-link'].includes(i.kind));
    return {
      ...card({ ...app, icon: app.media.icon }, locale),
      description: text.description_markdown,
      hero: app.media.hero ? catalogUrl(root, app.media.hero) : undefined,
      screenshots: (app.media.screenshots || []).map((s) => ({
        url: catalogUrl(root, s.path),
        alt: localized(s.alt, locale, app.default_locale),
      })),
      disclosures: app.disclosures,
      publisher: app.publisher?.name,
      homepage: app.links?.homepage,
      packageNames: [...new Set(packageNames)],
      downloadUrl: external?.acquisition?.landing_url,
    };
  }

  async function homepage(locale: string) {
    const index = await read<Index>('index.json');
    const home = await read<Homepage>(index.homepage);
    const hero = home.hero && {
      ...localized(home.hero.locales, locale, index.default_locale),
      target: home.hero.target,
      image: catalogUrl(root, home.hero.image),
    };
    const collectionId = hero?.target.collection;
    const collections =
      home.essentials || collectionId
        ? (await read<{ collections: Collection[] }>('collections.json')).collections
        : [];
    const apps = new Map(index.apps.map((app) => [app.id, app]));
    async function collection(id?: string | null) {
      if (!id) return null;
      const entry = collections.find((c) => c.id === id);
      if (!entry) throw new Error('Unknown ASMR collection: ' + id);
      return {
        id,
        title: localized(entry.locales, locale, index.default_locale).title,
        apps: await Promise.all(
          entry.apps.map((appId) => {
            const app = apps.get(appId);
            return app ? card(app, locale) : detail(appId, locale);
          }),
        ),
      };
    }
    return {
      hero,
      essentials: await collection(home.essentials),
      heroCollection: collectionId !== home.essentials ? await collection(collectionId) : null,
      guides: {
        more_url: home.guides.more_url,
        items: home.guides.items.map(({ locales, ...item }) => ({
          ...item,
          ...localized(locales, locale, index.default_locale),
        })),
      },
    };
  }

  async function cards(locale: string) {
    const index = await read<Index>('index.json');
    return index.apps.map((app) => ({ ...card(app, locale), categories: app.categories }));
  }
  return { detail, homepage, cards };
}

export type CatalogDetail = Awaited<ReturnType<ReturnType<typeof createAsmrCatalog>['detail']>>;
