const assert = require('node:assert/strict');
const { execFileSync } = require('node:child_process');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');

const repo = path.resolve(__dirname, '..');
async function runFrontendTests(outDir) {
  const { createTauriPackageManager } = require(path.join(outDir, 'utils', 'pmTauri.js'));
  const { observeOperation, loadPackageInventory, createPackageOperationFlow } = require(path.join(outDir, 'utils', 'pmOperations.js'));
  const { createStubPackageManager } = require(path.join(outDir, 'utils', 'pmStub.js'));
  const { actionLabel, isDestructiveAction, operationHeading } = require(path.join(outDir, 'utils', 'pmPresentation.js'));
  const { selectedPackageNames, batchActionState } = require(path.join(outDir, 'utils', 'pmSelection.js'));
  const { flattenCatalogPackages, searchCatalogPackages } = require(path.join(outDir, 'utils', 'catalogSearch.js'));

  const catalogIndex = {
    version: 1,
    generated_at: '2026-06-12T00:00:00.000Z',
    packages: [
      {
        category: 'working',
        packages: [
          { name: 'wechat', intro: '用户超十亿的即时聊天软件', icon: 'wechat.png' },
          { name: 'firefox', intro: '快速、安全的网页浏览器', icon: 'firefox.png' },
        ],
      },
      {
        category: 'games',
        packages: [
          { name: 'steam', intro: '数字游戏分发平台', icon: 'steam.png' },
        ],
      },
    ],
  };

  const calls = [];
  const id = '87025131895291905';
  const adapter = createTauriPackageManager(async (command, args) => {
    calls.push({ command, args });
    if (command === 'pm_package_state') return { packages: [{ name: 'firefox', version: '160', installed: false, upgradable: false, openable: false, aoska_manageable: true }] };
    if (command === 'pm_start_remove') return { id, operation: 'remove', packages: args.packages, started_at: '2026-11-17T00:00:00Z' };
    if (command === 'pm_operation_status') return { id, operation: 'remove', state: 'succeeded' };
    throw new Error('unsupported test command: ' + command);
  });
  const unknownPackage = await createTauriPackageManager(async () => ({ packages: [{ name: 'missing', installed: false, upgradable: false, aoskaManageable: false, requiresManualAction: true }] })).packageState('missing');
  assert.equal(unknownPackage.action.primary, 'manual');
  assert.ok(unknownPackage.action.disabledReason);
  const multiarch = await createTauriPackageManager(async () => ({ packages: [{ name: 'libc:arm64', name_without_arch: 'libc', displayName: 'libc', installed: true, upgradable: true }] })).listUpdates();
  assert.equal(multiarch.packages[0].name, 'libc:arm64');
  assert.deepEqual(selectedPackageNames(multiarch.packages, new Set(['libc:arm64'])), ['libc:arm64']);
  assert.equal(operationHeading('install', 'unknown', 'zh'), '无法确认操作状态');
  const candidate = await adapter.packageState('firefox');
  assert.equal(candidate.installed, false);
  assert.equal(candidate.openable, false);
  assert.equal(candidate.action.primary, 'install');
  const sparsePackage = await createTauriPackageManager(async () => ({ packages: [{
    name: 'firefox', displayName: 'Firefox', packageName: 'firefox', currentVersion: null,
    newVersion: '160.0', installed: false, upgradable: false, openable: false, aoskaManageable: true,
  }] })).packageState('firefox');
  assert.equal(sparsePackage.currentVersion, undefined);
  assert.equal(sparsePackage.source, undefined);
  assert.equal(sparsePackage.homepage, undefined);
  assert.equal(sparsePackage.architecture, undefined);
  assert.equal(sparsePackage.currentVersion ?? 'catalog-version', 'catalog-version');
  assert.equal(sparsePackage.source ?? '安同 OS 软件仓库', '安同 OS 软件仓库');
  assert.equal(sparsePackage.homepage ?? 'https://example.org', 'https://example.org');
  assert.equal(sparsePackage.newVersion, '160.0');
  const sparsePlan = await createTauriPackageManager(async () => ({
    install: [{ name: 'firefox', currentVersion: null, newVersion: '160.0' }],
    remove: [{ name: 'old-package' }], totalDownloadSize: 0, diskSizeDelta: 0,
  })).planInstall(['firefox']);
  assert.equal(sparsePlan.install[0].currentVersion, undefined);
  assert.equal(sparsePlan.install[0].newVersion, '160.0');
  assert.equal(sparsePlan.remove[0].currentVersion, undefined);
  assert.equal(sparsePlan.remove[0].newVersion, undefined);
  const started = await adapter.startRemove(['firefox']);
  assert.equal(started.id, id);
  assert.deepEqual(calls.at(-1), { command: 'pm_start_remove', args: { packages: ['firefox'] } });
  await adapter.operationStatus(id);
  assert.deepEqual(calls.at(-1).args, { id });
  await assert.rejects(createTauriPackageManager(async () => ({})).startInstall(['firefox']), /id/i);
  await assert.rejects(createTauriPackageManager(async () => { throw new Error('denied'); }).startInstall(['firefox']), /denied/);

  async function checksCatalogInventoryResponse() {
    const empty = createTauriPackageManager(async () => ({ packages: [] }));
    assert.deepEqual((await empty.listCatalogInstalled()).packages, []);

    for (const response of [{}, { packages: [null] }, { packages: [{ installed: true }] }, { packages: [{ name: 'firefox' }] }]) {
      const invalidCatalog = createTauriPackageManager(async command => {
        assert.equal(command, 'pm_list_catalog_installed');
        return response;
      });
      await assert.rejects(invalidCatalog.listCatalogInstalled(), /catalog|package|installed|name/i);
    }
    const validCatalog = createTauriPackageManager(async () => ({ packages: [{
      name: 'firefox:amd64', installed: true, catalogName: 'firefox', icon: 'https://example.org/firefox.png',
    }] }));
    assert.equal((await validCatalog.listCatalogInstalled()).packages[0].name, 'firefox:amd64');
  }
  await checksCatalogInventoryResponse();

  const inventory = await loadPackageInventory({
    capabilities: async () => ({ kind: 'tauri', capabilities: ['query.upgradable.v1'] }),
    updateSummary: async () => ({ total: 1 }),
    listUpdates: async () => ({ packages: [{ name: 'firefox' }] }),
    listInstalled: async () => { throw new Error('unsupported installed query'); },
    storageSummary: async () => { throw new Error('unsupported storage query'); },
  });
  assert.equal(inventory.installedScope, 'unavailable');
  assert.equal(inventory.updates.length, 1);
  assert.deepEqual(inventory.installed, []);
  assert.equal(inventory.storage, null);

  let polls = 0;
  let finished = 0;
  let errors = 0;
  let next;
  let cleared = 0;
  const scheduler = { schedule: callback => { next = callback; return 1; }, clear: () => { cleared++; } };
  const stop = observeOperation({
    operationStatus: async () => {
      polls++;
      if (polls === 1) throw new Error('temporary disconnect');
      return { id, operation: 'refresh', state: polls === 2 ? 'running' : 'succeeded' };
    },
    operationLogs: async () => ({ id, lines: ['metadata refreshed'] }),
  }, id, { update: () => {}, error: () => { errors++; }, finished: () => { finished++; } }, scheduler);
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(errors, 1);
  await next();
  assert.equal(finished, 0);
  await next();
  assert.equal(finished, 1);
  stop();
  assert.ok(cleared > 0);
  let resolvePending;
  let updatesAfterStop = 0;
  const stopPending = observeOperation({
    operationStatus: () => new Promise(resolve => { resolvePending = resolve; }),
    operationLogs: async () => ({ id, lines: [] }),
  }, id, { update: () => { updatesAfterStop++; }, error: () => {}, finished: () => { updatesAfterStop++; } }, scheduler);
  stopPending();
  resolvePending({ id, operation: 'refresh', state: 'failed', message: 'permission denied' });
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(updatesAfterStop, 0);
  let failure;
  observeOperation({
    operationStatus: async () => ({ id, operation: 'install', state: 'failed', message: 'permission denied' }),
    operationLogs: async () => ({ id, lines: [] }),
  }, id, { update: () => {}, error: () => {}, finished: status => { failure = status.message; } }, scheduler);
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(failure, 'permission denied');

  let unknownFinished = 0;
  let unknownState;
  const stopUnknown = observeOperation({
    operationStatus: async () => ({ id, operation: 'refresh', state: 'unknown', message: 'Connection lost; task may still be running.' }),
    operationLogs: async () => ({ id, lines: [] }),
  }, id, { update: status => { unknownState = status; }, error: () => {}, finished: () => { unknownFinished++; } }, scheduler);
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(unknownFinished, 0);
  assert.equal(unknownState.state, 'unknown');
  assert.match(unknownState.message, /may still be running/);
  stopUnknown();

  let starts = 0;
  let reloads = 0;
  const state = { plan: null, activeOperation: null, busy: false, submitting: false, error: '' };
  const selectedPlan = { operation: 'install', packages: ['firefox'], install: [{ name: 'firefox' }, { name: 'dependency' }], remove: [], totalDownloadSize: 10, diskSizeDelta: 20 };
  const flow = createPackageOperationFlow({
    planInstall: async () => selectedPlan,
    startInstall: async packages => { starts++; return { id, operation: 'install', packages }; },
  }, state, async () => { reloads++; });
  await flow.prepare('install', ['firefox']);
  assert.equal(starts, 0);
  assert.equal(state.plan.install.length, 2);
  flow.close();
  assert.equal(starts, 0);
  await flow.prepare('install', ['firefox']);
  await flow.confirm();
  assert.equal(starts, 1);
  assert.equal(state.busy, true);
  flow.close();
  assert.equal(state.activeOperation.id, id);
  await flow.confirm();
  assert.equal(starts, 1);
  await flow.finished();
  assert.equal(reloads, 1);
  assert.equal(state.busy, false);
  const denied = createPackageOperationFlow({
    planInstall: async () => selectedPlan,
    startInstall: async () => { throw new Error('authorization denied'); },
  }, state, async () => {});
  await denied.prepare('install', ['firefox']);
  await denied.confirm();
  assert.match(state.error, /authorization denied/);
  assert.equal(state.busy, false);
  assert.equal(state.submitting, false);
  assert.equal(state.plan, selectedPlan);
  denied.close();
  const rejected = createPackageOperationFlow({ planRemove: async () => { throw new Error('plan rejected'); } }, state, async () => {});
  await rejected.prepare('remove', ['firefox']);
  assert.equal(state.plan, null);
  assert.match(state.error, /plan rejected/);
  assert.equal(state.submitting, false);

  const pm = createStubPackageManager();

  const capabilities = await pm.capabilities();
  assert.equal(capabilities.kind, 'stub');
  assert.ok(capabilities.capabilities.includes('stub.backend.v1'));
  assert.ok(capabilities.capabilities.includes('run.install.v1'));
  assert.ok(capabilities.capabilities.includes('operation.logs.v1'));

  const updates = await pm.listUpdates();
  const summary = await pm.updateSummary();
  assert.equal(summary.total, updates.packages.length);
  assert.equal(summary.security, 1);
  assert.equal(summary.securityClassificationAvailable, true);

  const wechat = await pm.packageState('wechat');
  assert.equal(wechat.name, 'wechat');
  assert.equal(wechat.installed, true);
  assert.equal(wechat.upgradable, true);
  assert.equal(wechat.action.primary, 'update');

  const firefox = await pm.packageState('firefox');
  assert.equal(firefox.name, 'firefox');
  assert.equal(firefox.installed, false);
  assert.equal(firefox.action.primary, 'install');
  assert.equal(actionLabel('install', 'zh'), '安装');
  assert.equal(actionLabel('remove', 'zh'), '卸载');
  assert.equal(isDestructiveAction('remove'), true);
  assert.equal(operationHeading('install', 'running', 'zh'), '正在安装');

  const installStart = await pm.startInstall(['firefox']);
  assert.match(installStart.id, /^aoska-stub-install-/);

  const storage = await pm.storageSummary();
  assert.ok(storage.totalUsedBytes > 0);
  assert.ok(storage.segments.some((segment) => segment.id === 'aoska'));

  const selection = new Set(['wechat']);
  assert.deepEqual(selectedPackageNames(updates.packages, selection), ['wechat']);
  assert.deepEqual(batchActionState('update', updates.packages, new Set()), { enabled: false, count: 0 });
  assert.deepEqual(batchActionState('update', updates.packages, selection), { enabled: true, count: 1 });

  assert.deepEqual(flattenCatalogPackages(catalogIndex).map((pkg) => pkg.name), ['wechat', 'firefox', 'steam']);
  assert.deepEqual(searchCatalogPackages(catalogIndex, '').map((pkg) => pkg.name), ['wechat', 'firefox', 'steam']);
  assert.deepEqual(searchCatalogPackages(catalogIndex, 'FIRE').map((pkg) => pkg.name), ['firefox']);
  assert.deepEqual(searchCatalogPackages(catalogIndex, '聊天').map((pkg) => pkg.name), ['wechat']);
  assert.deepEqual(searchCatalogPackages(catalogIndex, 'not-present').map((pkg) => pkg.name), []);

  const updateStart = await pm.startUpdate(['wechat']);
  assert.match(updateStart.id, /^aoska-stub-update-/);
  const updateLogs = await pm.operationLogs(updateStart.id);
  assert.ok(updateLogs.lines.some((line) => line.includes('wechat')));
  const updateCancel = await pm.cancelOperation(updateStart.id);
  assert.ok(['already_terminal', 'cancelled'].includes(updateCancel.state));

  const { externalPageUrl, launchDesktopApplication } = require(path.join(outDir, 'utils', 'desktopActions.js'));
  assert.equal(externalPageUrl('https://example.org/path'), 'https://example.org/path');
  for (const url of ['file:///etc/passwd', 'javascript:alert(1)', 'data:text/plain,test']) assert.throws(() => externalPageUrl(url));
  const launchCalls = [];
  await launchDesktopApplication('firefox', false, async (...args) => { launchCalls.push(args); });
  assert.deepEqual(launchCalls, [['app_launch', {catalogName:'firefox'}]]);
  await assert.rejects(launchDesktopApplication('firefox', true, async () => { throw new Error('must not invoke'); }), /浏览器预览/);
  const catalogPm = { ...pm, capabilities: async () => ({kind:'tauri',capabilities:['query.catalog-installed.v1']}), listInstalled: async () => { throw new Error('must not enumerate system'); }, listCatalogInstalled: async () => ({packages:[{name:'firefox:amd64',installed:true},{name:'firefox:amd64',installed:true},{name:'absent',installed:false}]}) };
  const catalogInventory = await loadPackageInventory(catalogPm);
  assert.equal(catalogInventory.installedScope, 'catalog');
  assert.equal(catalogInventory.installed.length, 1);
  const failedCatalog = await loadPackageInventory({...catalogPm,listCatalogInstalled:async()=>{throw new Error('catalog unavailable');}});
  assert.match(failedCatalog.installedError, /catalog unavailable/);
  assert.ok(failedCatalog.updates.length > 0);

  async function fallsBackToSystemInventoryOnlyWhenAdvertised() {
    let systemCalls = 0;
    const systemInventory = await loadPackageInventory({
      ...pm,
      capabilities: async () => ({ kind: 'tauri', capabilities: ['query.installed.v1'] }),
      listCatalogInstalled: async () => { throw new Error('catalog capability not advertised'); },
      listInstalled: async () => { systemCalls++; return { packages: [{ name: 'system-tool', installed: true }] }; },
    });
    assert.equal(systemCalls, 1);
    assert.equal(systemInventory.installedScope, 'system');
    assert.deepEqual(systemInventory.installed.map(pkg => pkg.name), ['system-tool']);
    assert.equal(systemInventory.installedError, undefined);
  }
  await fallsBackToSystemInventoryOnlyWhenAdvertised();

  // Drive the same mock used by the browser without sleeping or touching a package manager.
  let clock = Date.parse('2026-11-19T00:00:00Z');
  const mock = createStubPackageManager({ now: () => clock, durationMs: 1000 });
  const catalogNames = async () => (await loadPackageInventory(mock)).installed.map(pkg => pkg.name);
  const remove = await mock.startRemove(['steam']);
  assert.equal((await mock.operationStatus(remove.id)).state, 'running');
  assert.ok((await catalogNames()).includes('steam'));
  clock += 1000;
  assert.equal((await mock.operationStatus(remove.id)).state, 'succeeded');
  assert.ok(!(await catalogNames()).includes('steam'));
  const install = await mock.startInstall(['steam']);
  clock += 1000;
  assert.equal((await mock.operationStatus(install.id)).state, 'succeeded');
  assert.equal((await mock.packageState('steam')).action.primary, 'open');
  const cancelled = await mock.startUpdate(['wechat']);
  await mock.cancelOperation(cancelled.id);
  clock += 1000;
  assert.equal((await mock.operationStatus(cancelled.id)).state, 'cancelled');
  assert.equal((await mock.updateSummary()).security, 1);
  const upgrade = await mock.startUpdate(['wechat']);
  clock += 1000;
  assert.equal((await mock.operationStatus(upgrade.id)).state, 'succeeded');
  assert.equal((await mock.packageState('wechat')).currentVersion, '4.0.1.12');
  assert.deepEqual(await mock.tumUpdates(), []);
  assert.equal((await createStubPackageManager().updateSummary()).security, 1);
  const refresh = await mock.startRefresh();
  clock += 1000;
  assert.equal((await mock.operationStatus(refresh.id)).state, 'succeeded');
  assert.equal((await mock.updateSummary()).lastCheckedAt, new Date(clock).toISOString());
  await assert.rejects(mock.planRemove([]));
  await assert.rejects(mock.startInstall(['not-in-fixture']));
  console.log('frontend stub backend tests passed');
}

async function runTests(suite = 'core') {
  const outDir = fs.mkdtempSync(path.join(os.tmpdir(), 'aoska-core-tests-'));
  try {
    execFileSync(path.join(repo, 'node_modules', '.bin', 'tsc'),
      ['-p', 'tsconfig.frontend-test.json', '--outDir', outDir],
      { cwd: repo, stdio: 'inherit' });
    if (suite !== 'activity') await runFrontendTests(outDir);
    await require('./test-update-activity.cjs').runUpdateActivityTests(outDir);
  } finally {
    fs.rmSync(outDir, { recursive: true, force: true });
  }
}

module.exports = { runTests };
if (require.main === module) {
  runTests().catch(error => {
    console.error(error);
    process.exitCode = 1;
  });
}
