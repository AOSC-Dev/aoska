# AOSKA Design Frontend Stub Backend Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the DESIGN.md frontend surface against a deterministic stub package-manager backend so UI flows can be developed and tested without mutating the real system.

**Architecture:** Keep catalog metadata and package-manager state separate. Add a typed package-manager client with a Tauri adapter and a stub adapter; Vue pages consume the typed client rather than raw `invoke()` names. Implement the dark visual shell and operation flows in small slices that are typechecked, built, and committed independently.

**Tech Stack:** Vue 3, TypeScript, Vite, Tauri 2 command wrappers, Node built-in assertions for stub tests, existing CSS variables/components.

---

## File map

- Modify `aoska/DESIGN.md`: remove stale `common/oma.rs` reference and mark stub frontend development as the current implementation mode.
- Create `aoska/src/types/pm.ts`: typed package-manager state, package records, operation and storage DTOs used by frontend.
- Create `aoska/src/utils/pmStub.ts`: deterministic in-memory package-manager stub for browser/Vite development and tests.
- Create `aoska/src/utils/pmClient.ts`: package-manager facade selecting stub outside Tauri and Tauri `pm_*` commands inside Tauri.
- Modify `aoska/src/utils/wrapper.ts`: keep catalog helpers and route update helpers through the typed package-manager facade.
- Create `aoska/scripts/test-frontend-stub.cjs`: compile selected TypeScript modules to a temporary directory and run stub behavior assertions with Node.
- Create `aoska/tsconfig.frontend-test.json`: emit CommonJS for pure TypeScript stub modules only.
- Modify `aoska/package.json`: add `test:frontend:stub` script.
- Modify `aoska/src/assets/styles/variables.css`, `aoska/index.html`, `aoska/src/App.vue`: dark design tokens and shell.
- Modify shared components: `NavBar.vue`, `CustomButton.vue`, `AppCard.vue`, `AppBanner.vue`, `TipCard.vue`, `WelcomeCard.vue`, `UpdateCard.vue`, `SoftwareFlags.vue`, `ImageCarousel.vue`.
- Modify pages: `HomePage.vue`, `AppPage.vue`, `AppPage/AppHeader.vue`, `UpdatePage.vue`, `Updates/UpdateItem.vue`.
- Create `aoska/src/components/share/OperationDialog.vue`: confirmation/progress/log/result dialog backed by stub/Tauri operations.
- Create `aoska/src/components/pages/Updates/PackageOperationRow.vue`: installed/upgradable row with checkbox and per-row actions.
- Create `aoska/src/components/pages/Updates/StorageSummary.vue`: capability-gated storage summary; visible in stub, hidden if real backend lacks capability.
- Modify `aoska/src/locales/zh.json` and `aoska/src/locales/en.json`: design-state copy and operation labels.

## Task 1: Governance/design cleanup and plan

**Files:**
- Modify: `aoska/DESIGN.md`
- Create: `aoska/docs/superpowers/plans/2026-06-12-design-frontend-stub-backend.md`

- [ ] **Step 1: Update DESIGN.md evidence and implementation constraints**

Replace the stale `src-tauri/src/common/oma.rs` reference with `src-tauri/src/common/omactl.rs` and `src-tauri/src/common/omactl_types.rs`. Add a short implementation note that frontend work should use a stub package-manager backend for Vite/browser development and only use real `pm_*` commands under Tauri.

- [ ] **Step 2: Verify docs-only changes**

Run: `git -C aoska diff --check`
Expected: exit 0.

- [ ] **Step 3: Commit**

Run:
```bash
git -C aoska add DESIGN.md docs/superpowers/plans/2026-06-12-design-frontend-stub-backend.md
git -C aoska commit --no-gpg-sign -m "docs: plan design frontend implementation" -m "AI: Codex"
```

## Task 2: Typed package-manager stub backend

**Files:**
- Create: `aoska/src/types/pm.ts`
- Create: `aoska/src/utils/pmStub.ts`
- Create: `aoska/src/utils/pmClient.ts`
- Modify: `aoska/src/utils/wrapper.ts`
- Create: `aoska/scripts/test-frontend-stub.cjs`
- Create: `aoska/tsconfig.frontend-test.json`
- Modify: `aoska/package.json`

- [ ] **Step 1: Write stub behavior test before production code**

Create `scripts/test-frontend-stub.cjs` so it compiles `src/types/pm.ts` and `src/utils/pmStub.ts`, then asserts:
- capabilities include `stub.backend.v1` and operation capabilities;
- update summary total equals stub upgradable package count;
- `getPackageState('wechat')` is installed and upgradable;
- `startUpdate(['wechat'])` returns a unit and appends operation logs;
- `cancelOperation(unit)` returns an already-terminal or cancelled state safely.

- [ ] **Step 2: Run test and confirm RED**

Run: `COREPACK_HOME=/tmp/corepack-cache corepack pnpm run test:frontend:stub`
Expected: fails because script/types/stub are missing.

- [ ] **Step 3: Implement minimal typed stub and client facade**

Add DTOs, deterministic stub data, package-manager facade, and `package.json` script. Do not call real Tauri commands when `window.__TAURI_INTERNALS__` is absent or `VITE_AOSKA_PM_BACKEND=stub`.

- [ ] **Step 4: Verify GREEN**

Run:
```bash
COREPACK_HOME=/tmp/corepack-cache corepack pnpm run test:frontend:stub
COREPACK_HOME=/tmp/corepack-cache corepack pnpm exec vue-tsc --noEmit
COREPACK_HOME=/tmp/corepack-cache corepack pnpm exec vite build
```
Expected: all pass.

- [ ] **Step 5: Commit**

Run:
```bash
git -C aoska add package.json tsconfig.frontend-test.json scripts/test-frontend-stub.cjs src/types/pm.ts src/utils/pmStub.ts src/utils/pmClient.ts src/utils/wrapper.ts
git -C aoska commit --no-gpg-sign -m "feat: add frontend package manager stub" -m "AI: Codex"
```

## Task 3: Dark design shell and core components

**Files:**
- Modify: `aoska/src/assets/styles/variables.css`
- Modify: `aoska/index.html`, `aoska/src/App.vue`
- Modify: `aoska/src/components/share/NavBar.vue`, `CustomButton.vue`, `AppCard.vue`, `AppBanner.vue`, `TipCard.vue`, `WelcomeCard.vue`, `UpdateCard.vue`, `SoftwareFlags.vue`, `ImageCarousel.vue`
- Modify: `aoska/src/components/pages/HomePage.vue`
- Modify: `aoska/src/locales/zh.json`, `aoska/src/locales/en.json`

- [ ] **Step 1: Add visual-state test using existing build/typecheck**

Before editing visual components, run:
```bash
COREPACK_HOME=/tmp/corepack-cache corepack pnpm exec vue-tsc --noEmit
```
Expected: pass; this is the locked baseline.

- [ ] **Step 2: Implement dark tokens and component restyle**

Use CSS custom properties for surfaces, text, borders, focus, success, info, security, warning, and accent. Keep component APIs stable. Ensure update card has checking/no-update/software/security/unknown states.

- [ ] **Step 3: Verify**

Run:
```bash
COREPACK_HOME=/tmp/corepack-cache corepack pnpm run test:frontend:stub
COREPACK_HOME=/tmp/corepack-cache corepack pnpm exec vue-tsc --noEmit
COREPACK_HOME=/tmp/corepack-cache corepack pnpm exec vite build
```
Expected: all pass.

- [ ] **Step 4: Commit**

Run:
```bash
git -C aoska add index.html src/App.vue src/assets/styles/variables.css src/components/share src/components/pages/HomePage.vue src/locales/zh.json src/locales/en.json
git -C aoska commit --no-gpg-sign -m "feat: apply aoska dark frontend shell" -m "AI: Codex"
```

## Task 4: App detail action states and operation dialog

**Files:**
- Create: `aoska/src/components/share/OperationDialog.vue`
- Modify: `aoska/src/components/pages/AppPage.vue`
- Modify: `aoska/src/components/pages/AppPage/AppHeader.vue`
- Modify: `aoska/src/utils/pmClient.ts`, `aoska/src/utils/pmStub.ts` if operation helpers need refinement
- Modify: `aoska/src/locales/zh.json`, `aoska/src/locales/en.json`

- [ ] **Step 1: Extend stub test for app action states**

Add assertions to `scripts/test-frontend-stub.cjs` that `getPackageState('firefox')` is installable/openable/upgradable according to stub data and that `startInstall(['firefox'])` returns operation metadata.

- [ ] **Step 2: Run test and confirm RED**

Run: `COREPACK_HOME=/tmp/corepack-cache corepack pnpm run test:frontend:stub`
Expected: fails until operation/state helper behavior exists.

- [ ] **Step 3: Implement app detail actions**

App detail should show action buttons based on package state: install, open, update, remove, official website/download page. Mutating actions open a confirmation/progress dialog and call stub/Tauri operation methods.

- [ ] **Step 4: Verify GREEN**

Run:
```bash
COREPACK_HOME=/tmp/corepack-cache corepack pnpm run test:frontend:stub
COREPACK_HOME=/tmp/corepack-cache corepack pnpm exec vue-tsc --noEmit
COREPACK_HOME=/tmp/corepack-cache corepack pnpm exec vite build
```
Expected: all pass.

- [ ] **Step 5: Commit**

Run:
```bash
git -C aoska add scripts/test-frontend-stub.cjs src/components/share/OperationDialog.vue src/components/pages/AppPage.vue src/components/pages/AppPage/AppHeader.vue src/utils/pmClient.ts src/utils/pmStub.ts src/locales/zh.json src/locales/en.json
git -C aoska commit --no-gpg-sign -m "feat: add app package operation UI" -m "AI: Codex"
```

## Task 5: Update and uninstall page

**Files:**
- Create: `aoska/src/components/pages/Updates/PackageOperationRow.vue`
- Create: `aoska/src/components/pages/Updates/StorageSummary.vue`
- Modify: `aoska/src/components/pages/UpdatePage.vue`
- Modify: `aoska/src/components/pages/Updates/UpdateItem.vue`
- Modify: `aoska/src/utils/pmClient.ts`, `aoska/src/utils/pmStub.ts`
- Modify: `aoska/src/locales/zh.json`, `aoska/src/locales/en.json`

- [ ] **Step 1: Extend stub test for update page data**

Add assertions that storage summary totals are consistent, installed list has selectable packages, and selected update/remove operations reject empty selections but accept non-empty selections.

- [ ] **Step 2: Run test and confirm RED**

Run: `COREPACK_HOME=/tmp/corepack-cache corepack pnpm run test:frontend:stub`
Expected: fails until storage/list helpers exist.

- [ ] **Step 3: Implement update/uninstall page**

Render storage summary when capability is present, tabs/counts for installed and upgradable packages, checkboxes, per-row update/remove/open actions, disabled batch buttons when no selection, and confirmation dialog for batch operations. If storage capability is absent, hide storage summary.

- [ ] **Step 4: Verify GREEN**

Run:
```bash
COREPACK_HOME=/tmp/corepack-cache corepack pnpm run test:frontend:stub
COREPACK_HOME=/tmp/corepack-cache corepack pnpm exec vue-tsc --noEmit
COREPACK_HOME=/tmp/corepack-cache corepack pnpm exec vite build
```
Expected: all pass.

- [ ] **Step 5: Commit**

Run:
```bash
git -C aoska add scripts/test-frontend-stub.cjs src/components/pages/UpdatePage.vue src/components/pages/Updates src/utils/pmClient.ts src/utils/pmStub.ts src/locales/zh.json src/locales/en.json
git -C aoska commit --no-gpg-sign -m "feat: add update management frontend" -m "AI: Codex"
```

## Task 6: Browser visual verification and final review

**Files:**
- Optional create: `.omx/artifacts/visual-ralph/aoska-design-frontend/`
- No production changes unless browser verification finds a bug.

- [ ] **Step 1: Run final static verification**

Run:
```bash
COREPACK_HOME=/tmp/corepack-cache corepack pnpm run test:frontend:stub
COREPACK_HOME=/tmp/corepack-cache corepack pnpm exec vue-tsc --noEmit
COREPACK_HOME=/tmp/corepack-cache corepack pnpm exec vite build
```
Expected: all pass.

- [ ] **Step 2: Start Vite preview/dev server and capture browser evidence**

Run Vite in stub mode and inspect `/home`, `/app/wechat`, and `/updates` with Codex browser/in-app browser. Capture screenshots or DOM evidence for those routes.

- [ ] **Step 3: Fix only verified issues**

If browser evidence shows broken layout or runtime errors, write a targeted test or typecheck guard where possible, fix, rerun static verification, then commit with a `fix:` conventional commit and `AI: Codex` body.

- [ ] **Step 4: Final status**

Run `git -C aoska status --short` and report changed features, commits, verification evidence, and remaining design gaps.
