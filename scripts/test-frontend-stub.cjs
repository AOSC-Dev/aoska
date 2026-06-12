const assert = require('node:assert/strict');
const { execFileSync } = require('node:child_process');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');

const repo = path.resolve(__dirname, '..');
const outDir = path.join(os.tmpdir(), 'aoska-frontend-stub-tests');
fs.rmSync(outDir, { recursive: true, force: true });

const tsc = path.join(repo, 'node_modules', '.bin', 'tsc');
execFileSync(tsc, ['-p', 'tsconfig.frontend-test.json', '--outDir', outDir], {
  cwd: repo,
  stdio: 'inherit',
});

const { createStubPackageManager } = require(path.join(outDir, 'utils', 'pmStub.js'));
const { actionLabel, isDestructiveAction, operationHeading } = require(path.join(outDir, 'utils', 'pmPresentation.js'));
const { selectedPackageNames, batchActionState } = require(path.join(outDir, 'utils', 'pmSelection.js'));

async function main() {
  const pm = createStubPackageManager();

  const capabilities = await pm.capabilities();
  assert.equal(capabilities.kind, 'stub');
  assert.ok(capabilities.capabilities.includes('stub.backend.v1'));
  assert.ok(capabilities.capabilities.includes('run.install.v1'));
  assert.ok(capabilities.capabilities.includes('unit.logs.v1'));

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
  assert.match(installStart.unit, /^aoska-stub-install-/);

  const storage = await pm.storageSummary();
  assert.ok(storage.totalUsedBytes > 0);
  assert.ok(storage.segments.some((segment) => segment.id === 'aoska'));

  const selection = new Set(['wechat']);
  assert.deepEqual(selectedPackageNames(updates.packages, selection), ['wechat']);
  assert.deepEqual(batchActionState('update', updates.packages, new Set()), { enabled: false, count: 0 });
  assert.deepEqual(batchActionState('update', updates.packages, selection), { enabled: true, count: 1 });

  const updateStart = await pm.startUpdate(['wechat']);
  assert.match(updateStart.unit, /^aoska-stub-update-/);
  const updateLogs = await pm.operationLogs(updateStart.unit);
  assert.ok(updateLogs.lines.some((line) => line.includes('wechat')));
  const updateCancel = await pm.cancelOperation(updateStart.unit);
  assert.ok(['already_terminal', 'cancelled'].includes(updateCancel.state));

  console.log('frontend stub backend tests passed');
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
