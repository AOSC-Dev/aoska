const assert = require('node:assert/strict');
const path = require('node:path');

exports.runCatalogTests = async function runCatalogTests(outDir) {
  const { createAsmrCatalog } = require(path.join(outDir, 'utils/asmr.js'));
  const { browserCatalogPreferences, channelUrls } = require(path.join(outDir, 'utils/catalogPreferences.js'));
  const saved = new Map();
  const storage = { getItem: key => saved.get(key) ?? null, setItem: (key, value) => saved.set(key, value) };
  const dev = browserCatalogPreferences(storage, 'dev');
  const stable = browserCatalogPreferences(storage, 'stable');
  assert.equal(dev.get().effectiveUrl, channelUrls.dev);
  assert.equal(stable.get().effectiveUrl, channelUrls.stable);
  assert.equal(dev.save({ channel: 'stable' }).effectiveUrl, channelUrls.stable);
  assert.equal(stable.save({ channel: 'dev' }).effectiveUrl, channelUrls.dev);
  const root = 'http://localhost:9000/examples/dist/v1/';
  assert.equal(dev.save({ channel: 'custom', customUrl: root.slice(0, -1) }).effectiveUrl, root);
  assert.equal(stable.get().effectiveUrl, root);
  const forced = browserCatalogPreferences(storage, 'dev', 'https://example.org/local');
  assert.equal(forced.get().effectiveUrl, 'https://example.org/local/');
  assert.equal(forced.get().overridden, true);
  for (const url of ['', 'file:///tmp/v1', 'https://example.org/?key=secret', 'http://user:pass@localhost/']) {
    assert.throws(() => dev.save({ channel: 'custom', customUrl: url }));
    assert.equal(dev.get().effectiveUrl, root);
  }
  assert.equal(forced.save({ channel: 'auto' }).overridden, true);
  assert.equal(stable.get().effectiveUrl, channelUrls.stable);
  const caption = { en: { title: 'Guide' }, 'zh-CN': { title: '指南' } };
  const card = {
    id: 'editor',
    default_locale: 'en',
    locales: { en: { name: 'Editor' } },
    icon: null,
    categories: ['office'],
    aliases: ['Code Writer'],
    keywords: ['编程'],
  };
  const files = {
    'index.json': { default_locale: 'zh-CN', homepage: 'editorial/home.json', apps: [card] },
    'editorial/home.json': {
      hero: { target: { app: 'editor' }, image: 'banners/feature.svg', locales: caption },
      essentials: 'essentials',
      guides: {
        items: [{ id: 'intro', date: '2026-11-20', url: 'https://wiki.aosc.io/', locales: caption }],
      },
    },
    'collections.json': {
      collections: [{ id: 'essentials', locales: caption, apps: ['unlisted', 'editor'] }],
    },
    'apps/editor.json': {
      ...card,
      media: { screenshots: [{ path: 'apps/editor/shot.png', alt: { en: 'Window' } }] },
      disclosures: { telemetry: 'unknown' },
      installers: [{ kind: 'aosc-repository', package: { names: ['editor-bin'] } }],
    },
    'apps/unlisted.json': {
      ...card,
      id: 'unlisted',
      external: true,
      media: {},
      disclosures: { availability: 'unverified' },
      installers: [
        {
          kind: 'browser-assisted-deb',
          acquisition: { landing_url: 'https://example.org/download' },
          artifact: { expected_package_names: ['unlisted-deb'] },
        },
      ],
    },
  };
  const requests = [];
  const catalog = createAsmrCatalog(root.slice(0, -1), async (url) => {
    requests.push(url);
    assert.ok(url.startsWith(root));
    const file = files[url.slice(root.length)];
    if (!file) throw new Error('404: ' + url);
    return { schema_version: 1, ...file };
  });
  const home = await catalog.homepage('zh');
  assert.deepEqual(
    requests,
    ['index.json', 'editorial/home.json', 'collections.json', 'apps/unlisted.json'].map((p) => root + p),
  );
  assert.equal(home.hero.image, root + 'banners/feature.svg');
  assert.equal(home.guides.items[0].title, '指南');
  assert.deepEqual(
    home.essentials.apps.map((app) => app.id),
    ['unlisted', 'editor'],
  );
  const cards = await catalog.cards('zh');
  assert.ok(cards[0].searchTerms.includes('Code Writer'));
  assert.ok(cards[0].searchTerms.includes('编程'));
  const detail = await catalog.detail('editor', 'en-US');
  assert.deepEqual(detail.sourceKinds, ['aosc-repository']);
  assert.equal(detail.name, 'Editor');
  assert.equal(detail.disclosures.telemetry, 'unknown');
  assert.deepEqual(detail.screenshots, [{ url: root + 'apps/editor/shot.png', alt: 'Window' }]);
  assert.deepEqual(detail.packageNames, ['editor-bin']);
  const external = await catalog.detail('unlisted', 'zh');
  assert.deepEqual(external.packageNames, []);
  assert.equal(external.downloadUrl, 'https://example.org/download');
  await assert.rejects(catalog.detail('../index', 'en'), /Invalid ASMR app ID/);
  await assert.rejects(catalog.detail('missing', 'en'), /404/);
  files['editorial/home.json'] = { hero: null, essentials: null, guides: { items: [] } };
  const empty = await catalog.homepage('zh');
  assert.equal(empty.hero, null);
  assert.equal(empty.essentials, null);
  assert.deepEqual(empty.guides.items, []);
  files['index.json'].schema_version = 2;
  await assert.rejects(catalog.homepage('en'), /Unsupported ASMR schema/);
  console.log('ASMR catalog mock tests passed');
};
