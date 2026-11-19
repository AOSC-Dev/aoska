const assert = require('node:assert/strict');
const path = require('node:path');

async function runUpdateActivityTests(out) {
  const {
    createUpdateActivity,
    trackPackageManager,
    isStale,
    formatActivityDate,
    activityPresentation
  } = require(path.join(out, 'utils/updateActivity.js'));

  let now = Date.parse('2026-11-17T00:00:00Z');
  const activity = createUpdateActivity(undefined, () => now);
  let state = 'running';
  const base = {
    updateSummary: async () => ({ total: 1 }),
    startRefresh: async () => ({
      id: '90071992547409999',
      operation: 'refresh',
      packages: [],
      startedAt: new Date(now).toISOString()
    }),
    operationStatus: async () => ({
      id: '90071992547409999',
      operation: 'refresh',
      state
    })
  };
  const pm = trackPackageManager(base, activity);
  await pm.updateSummary();
  assert.equal(activity.get().lastCheckedAt, undefined);
  await pm.startRefresh();
  assert.equal(activity.get().lastCheckedAt, undefined);
  await pm.operationStatus('90071992547409999');
  assert.equal(activity.get().lastCheckedAt, undefined);
  state = 'failed';
  await pm.operationStatus('90071992547409999');
  assert.equal(activity.get().lastCheckedAt, undefined);
  await pm.startRefresh();
  state = 'succeeded';
  await pm.operationStatus('90071992547409999');
  const checked = activity.get().lastCheckedAt;
  assert.ok(checked);
  now += 10000;
  await pm.operationStatus('90071992547409999');
  assert.equal(activity.get().lastCheckedAt, checked);
  assert.equal(isStale(checked, now), false);
  assert.equal(isStale(undefined, now), true);
  assert.equal(isStale(checked, now + 15 * 86400000), true);
  assert.equal(formatActivityDate(checked, now + 3 * 86400000), '3 天前');
  const november = new Date(2026, 10, 17, 12);
  assert.equal(
    formatActivityDate(november.toISOString(), new Date(2026, 10, 27, 12).getTime()),
    november.toLocaleDateString("zh-CN", { month: "long", day: "numeric" }),
  );
  assert.equal(
    formatActivityDate(november.toISOString(), new Date(2027, 0, 10, 12).getTime()),
    november.toLocaleDateString("zh-CN", { year: "numeric", month: "long", day: "numeric" }),
  );
  const restored = createUpdateActivity(
    { getItem: () => JSON.stringify({ lastCheckedAt: checked }), setItem() {} },
    () => now
  );
  assert.equal(restored.get().lastCheckedAt, checked);
  const invalid = createUpdateActivity(
    { getItem: () => '{broken', setItem() {} },
    () => now
  );
  assert.equal(invalid.get().lastCheckedAt, undefined);
  const broken = createUpdateActivity(
    {
      getItem() {
        throw Error('blocked');
      },
      setItem() {
        throw Error('blocked');
      }
    },
    () => now
  );
  assert.equal(broken.get().lastCheckedAt, undefined);
  const presentation = (extra = {}) =>
    activityPresentation({
      activity: {},
      loading: false,
      error: '',
      security: 10,
      updates: 23,
      stale: true,
      ...extra
    });
  assert.equal(presentation().tone, 'security-update');
  assert.equal(
    presentation({ activity: { result: { state: 'succeeded' } } }).tone,
    'no-update'
  );
  assert.equal(
    presentation({ activity: { result: { state: 'failed' } } }).tone,
    'warning-update'
  );
  assert.equal(
    presentation({ activity: { current: { operation: 'refresh' } } }).tone,
    'checking-update'
  );
  assert.equal(presentation({ security: 0 }).tone, 'warning-update');
  await pm.startRefresh();
  state = 'unknown';
  await pm.operationStatus('90071992547409999');
  assert.ok(activity.get().current);
  assert.equal(activity.get().uncertain.state, 'unknown');
  assert.equal(activity.get().lastCheckedAt, checked);
  assert.equal(presentation({ activity: activity.get() }).state, 'unknown');
  assert.equal(
    presentation({ activity: activity.get() }).tone,
    'warning-update'
  );
  function preservesSuccessfulStateWhenStorageWriteFails() {
    let writes = 0;
    const unavailableStorage = createUpdateActivity(
      {
        getItem: () => null,
        setItem() {
          writes++;
          throw new Error('storage full');
        }
      },
      () => now
    );
    unavailableStorage.started({
      id: 'refresh-1',
      operation: 'refresh',
      packages: [],
      startedAt: new Date(now).toISOString()
    });
    unavailableStorage.observed({
      id: 'refresh-1',
      operation: 'refresh',
      state: 'succeeded'
    });
    assert.equal(writes, 1);
    assert.equal(unavailableStorage.get().result.state, 'succeeded');
    assert.equal(
      unavailableStorage.get().lastCheckedAt,
      new Date(now).toISOString()
    );
    assert.equal(unavailableStorage.get().current, undefined);
  }
  preservesSuccessfulStateWhenStorageWriteFails();

  function recordsUpdateTimeOnlyForTheMatchingSuccessfulOperation() {
    const writes = [];
    const updates = createUpdateActivity(
      {
        getItem: () => null,
        setItem: (_, value) => writes.push(JSON.parse(value))
      },
      () => now
    );
    const start = (operation) =>
      updates.started({
        id: operation + '-1',
        operation,
        packages: [],
        startedAt: new Date(now).toISOString()
      });
    start('update');
    updates.observed({
      id: 'different-request',
      operation: 'update',
      state: 'succeeded'
    });
    assert.equal(updates.get().lastUpdatedAt, undefined);
    assert.equal(writes.length, 0);
    for (const state of ['running', 'unknown', 'failed', 'cancelled']) {
      start('update');
      updates.observed({ id: 'update-1', operation: 'update', state });
      assert.equal(updates.get().lastUpdatedAt, undefined);
    }
    for (const operation of ['install', 'remove']) {
      start(operation);
      updates.observed({ id: operation + '-1', operation, state: 'succeeded' });
      assert.equal(updates.get().lastUpdatedAt, undefined);
      assert.equal(updates.get().lastCheckedAt, undefined);
    }
    start('update');
    updates.observed({
      id: 'update-1',
      operation: 'update',
      state: 'succeeded'
    });
    const updatedAt = updates.get().lastUpdatedAt;
    assert.equal(updatedAt, new Date(now).toISOString());
    assert.equal(updates.get().lastCheckedAt, undefined);
    const writesAtCompletion = writes.length;
    now += 1000;
    updates.observed({
      id: 'update-1',
      operation: 'update',
      state: 'succeeded'
    });
    assert.equal(updates.get().lastUpdatedAt, updatedAt);
    assert.equal(writes.length, writesAtCompletion);
  }
  recordsUpdateTimeOnlyForTheMatchingSuccessfulOperation();
  console.log('update activity tests passed');
}

module.exports = { runUpdateActivityTests };
if (require.main === module) {
  require('./test-frontend-stub.cjs')
    .runTests('activity')
    .catch((error) => {
      console.error(error);
      process.exitCode = 1;
    });
}
