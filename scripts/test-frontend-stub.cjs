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
