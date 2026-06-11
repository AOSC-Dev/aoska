# Design

## Source of truth

- Status: Draft
- Last refreshed: 2026-06-12
- Primary product surfaces:
  - Home / recommendations.
  - Category, view-all, and search-result browsing.
  - App detail page.
  - Updates and uninstall page.
  - Package operation status, progress, logs, and recovery states.
- Evidence reviewed:
  - GitHub issue [#33: 设计稿 202601 添加暗色模式](https://github.com/AOSC-Dev/aoska/issues/33), including PNG, SVG, and Penpot attachments.
  - Current routes in `src/router/index.ts`.
  - Current shared/page components under `src/components/`.
  - Current theme tokens in `src/assets/styles/variables.css`.
  - Current package-management backend in `src-tauri/src/command.rs`, `src-tauri/src/common/oma.rs`, and `src-tauri/src/common/omactl.rs`.
  - Local governance drafts: `docs/rfc/RFC-0001.md` and `gov/adr/ADR-0001-*`.

## Brand

- Personality:
  - Dark, KDE/Plasma-adjacent, system-integrated, utility-first.
  - Calm and trustworthy for normal package-management operations.
  - Friendly, lightly playful Chinese microcopy is acceptable, especially in update status cards, but must not hide the actual system state.
- Trust signals:
  - Explicit compatibility, origin, support, telemetry, and availability flags.
  - Clear install/update/remove confirmation for operations that can modify the system.
  - Stable operation progress and diagnostics instead of raw terminal text.
- Avoid:
  - Purely decorative text that makes the state unclear.
  - Relying on clean marketing screenshots only; real app screenshots may be noisy.
  - Parsing or presenting human-oriented `oma`, `systemctl`, or `journalctl` output as UI state.

## Product goals

- Goals:
  - Present aoska as an AOSC OS software store, not a full package-manager GUI.
  - Let users discover apps through curated catalog metadata, categories, recommendations, and search.
  - Let users install, remove, open, and update apps with clear safety boundaries.
  - Make system/package update status understandable: no updates, software updates, system updates, security updates, stale update state, failure, completion, and manual-action-required.
  - Support dark mode as a first-class visual target.
- Non-goals:
  - Expose every `oma` subcommand.
  - Replace terminal workflows for advanced package-manager tasks.
  - Make aoska responsible for dependency resolution, TUM/security matching, or APT package-management semantics.
- Success signals:
  - Users can tell what is installed, what is updatable, what is safe/official/supported, and what operation is currently running.
  - Package-management UI can be driven by structured data from omactl.
  - Screens remain readable with imperfect app screenshots and banners.

## Personas and jobs

- Primary personas:
  - AOSC OS desktop users who want a graphical software discovery and update experience.
  - AOSC maintainers who need catalog metadata to communicate availability/support caveats.
  - Users installing apps from mixed sources, including AOSC repositories, official installers/download pages, and compatibility layers.
- User jobs:
  - Discover recommended or category-specific software.
  - Search for an app.
  - Inspect app screenshots, description, metadata, support/compatibility flags, and source.
  - Install, open, update, or remove an app.
  - Review all installed/updatable software and batch update/remove selections.
  - Understand when updates require security attention or manual terminal action.
- Key contexts of use:
  - Desktop GUI under Tauri.
  - System package state may change outside aoska through `oma` or other package tools.
  - Network, mirror, permission, and package-manager lock failures are expected operational states.

## Information architecture

- Primary navigation:
  - `主页`
  - `办公`
  - `游戏`
  - `影音`
  - `创作`
  - `搜索结果`
  - `更新与卸载`
- Core routes/screens:
  - Home: banner, recommended apps, tips/guides, update status card.
  - Category/view-all/search results: app cards/lists filtered by catalog metadata and search.
  - App detail: hero, screenshots, metadata, flags, actions.
  - Updates and uninstall: storage summary, update health card, installed/updatable list, selected batch actions.
  - Operation status: checking, downloading, running, completed, failed, cancelled, manual action required.
- Content hierarchy:
  1. System/app state and primary action.
  2. App identity: icon, name, short description.
  3. Safety/support flags.
  4. Details: source, publisher, version, update date, install size, screenshots.
  5. Secondary links and diagnostics.

## Design principles

- Principle 1: Separate catalog metadata from package-manager state.
  - Catalog metadata owns app descriptions, screenshots, banners, categories, source labels, flags, and links.
  - omactl/oma-derived state owns installed state, upgradable state, operation plans, unit status, logs, results, TUM/security classification, and storage/package counts.
- Principle 2: Prefer structured state over text parsing.
  - UI states should be derived from typed backend data, not terminal output.
- Principle 3: Make risk visible before action.
  - Non-official, non-native, telemetry, service-limited, unsupported, and manual-operation-required states must be visible near install/update actions.
- Principle 4: Dark mode must remain readable with arbitrary media.
  - Use overlays, gradients, shadows, and fallback layouts for banners/screenshots.
- Tradeoffs:
  - Playful copy can improve warmth, but every card still needs a clear machine state and actionable button.
  - App-store simplicity takes priority over exposing all `oma` power-user features.

## Visual language

- Color:
  - Default surface is dark charcoal/near-black.
  - Cards use slightly lighter dark gray.
  - Positive/success uses muted green.
  - Informational/update states use blue or teal.
  - Security/danger states use dark red.
  - Warning/manual/failure states use brown/orange.
  - The update checking progress bar may use AOSC logo-inspired colors; avoid noisy multi-color motion if it hurts readability.
- Typography:
  - Large Chinese display headings for hero/status cards.
  - Smaller explanatory copy under each status.
  - App cards should prioritize name and one-line summary.
- Spacing/layout rhythm:
  - Desktop-first layout.
  - Main content often uses approximately 2fr/1fr columns: wide primary content left, status/actions right.
  - Buttons should use consistent sizing; design note recommends `min-width: 128px` for general buttons and at least `96px` for compact status-card buttons.
- Shape/radius/elevation:
  - Low-radius rectangular cards.
  - Add visible button borders so selected/unselected rows do not feel misaligned.
  - Use subtle elevation/contrast instead of bright surfaces.
- Motion:
  - Optional hover animation for guide/tip cards where date background expands across the item.
  - Loading/progress animation should be restrained and respect reduced motion.
- Imagery/iconography:
  - App banners and screenshots need dark overlays/gradients for text contrast.
  - Screenshots may be messy; layouts must tolerate non-ideal app media.
  - Flag icons are tracked separately in GitHub issue #32; dark-mode icon color may need explicit assets because Penpot cannot auto-swap colors.

## Components

- Existing components to reuse:
  - `NavBar`
  - `AppBanner`
  - `AppCard`
  - `UpdateCard`
  - `TipCard`
  - `SoftwareFlags`
  - `ImageCarousel`
  - `CustomButton`
- New/changed components:
  - Dark-mode token set.
  - Update-status card variants.
  - Storage usage summary with segmented bar and tooltip.
  - Batch operation toolbar for update/remove selected.
  - Operation confirmation modal.
  - Operation progress/log dialog.
  - App action button group with states: install, open, remove, update, official website, download page.
- Variants and states:
  - App cards: normal, hover, installed, upgradable, selected, disabled.
  - Update cards:
    - Checking updates.
    - No updates.
    - Software updates available.
    - System updates available.
    - Security updates available.
    - Stale update state.
    - Update failed.
    - Update complete.
    - Manual action required.
  - Software flags:
    - 非官方软件包
    - 可用性未验证
    - 已验证可用性
    - 非原生应用程序 / 非原生软件
    - Windows® 应用程序
    - 遥测与跟踪功能
    - 服务可能受限
    - 官方提供支持
- Token/component ownership:
  - Color, spacing, radius, typography, and button sizes should move into shared tokens before broad visual implementation.
  - Do not create a second parallel component system; extend current Vue components first.

## Accessibility

- Target standard:
  - Aim for WCAG AA contrast for primary text and interactive controls.
- Keyboard/focus behavior:
  - Navigation, search, app cards, checkboxes, and action buttons must be keyboard reachable.
  - Focus states must be visible in dark mode.
- Contrast/readability:
  - Text over images requires an overlay or shadow.
  - Do not rely on source images being readable or clean.
  - Links should underline or otherwise become clearly identifiable on hover.
- Screen-reader semantics:
  - Status cards should expose state, counts, and primary action.
  - Flags should be text-backed; icons are not sufficient.
  - Progress bars need accessible labels and values when known.
- Reduced motion and sensory considerations:
  - Hover animations and loading bars should respect reduced-motion preferences.
  - Avoid flashing or rapid color cycling.

## Responsive behavior

- Supported breakpoints/devices:
  - Primary target is desktop Tauri.
  - Minimum usable width should support the current fixed-window design, but future layouts should avoid hard-coded pixel-only assumptions.
- Layout adaptations:
  - Prefer proportional grid columns over fixed right rail where possible.
  - App detail should collapse hero/actions and screenshots/metadata predictably if the window narrows.
  - Update list should keep per-row actions reachable at smaller widths.
- Touch/hover differences:
  - Hover-only affordances must have click/tap equivalents.
  - Guide-card hover animation must not hide essential content.

## Interaction states

- Loading:
  - Checking updates: elapsed time, progress indicator if available, cancel action.
  - Running operation: unit-backed progress/log stream when available.
  - Catalog/network fetch: skeletons or placeholders.
- Empty:
  - No updates: show last check time and `查找更新`.
  - Empty search/category: explain no matching app and offer clear navigation back.
- Error:
  - Update failed: show a concise reason when known.
  - Network, permission, package-manager-busy, unsupported capability, systemd failure, and oma execution failure should map to distinct UI states.
  - Manual terminal action should be explicit when GUI cannot safely continue.
- Success:
  - Update complete: show completion time and acknowledgement.
  - Install/remove/update completion should refresh package state.
- Disabled:
  - Disable actions while another package-changing operation is active.
  - Disable `更新选中` / `卸载选中` until at least one eligible package is selected.
- Offline/slow network:
  - Search/catalog fetch and update checks should distinguish network failure from no updates.
  - Service-limited flags may explain regional/network availability constraints for specific apps.

## Content voice

- Tone:
  - Friendly Chinese copy is encouraged, but should not obscure operational meaning.
  - Example status headings from issue #33: `一切如常！`, `醒醒，有软件更新！`, `有可用的安全补丁！`, `好耶，有系统更新！`, `更新失败了...`, `需要手动操作`.
- Terminology:
  - Use `软件更新` for app/package updates surfaced as software updates.
  - Use `系统更新` when the update is system-level.
  - Use `安全更新` only when TUM/security data confirms security classification.
  - Use `可用性`/`支持`/`来源` consistently in flags and metadata.
- Microcopy rules:
  - Always pair playful headings with concrete counts or reasons.
  - Do not say there are zero security updates when security classification is unavailable.
  - Fuzzy dates:
    - Less than 10 days: show day count, e.g. `3 天前`.
    - 10 days or more: show date.
    - If the year is the current year, omit the year.

## Implementation constraints

- Framework/styling system:
  - Frontend: Vue 3, Vite, TypeScript, vue-router, vue-i18n.
  - App shell: Tauri 2.
  - Styling is currently plain Vue/CSS; token work should extend `src/assets/styles/variables.css`.
- Design-token constraints:
  - Current token file only has a few light background colors.
  - Dark mode requires explicit tokens for surfaces, text, borders, focus, warning, danger, success, info, and accent.
- Performance constraints:
  - Avoid loading all heavy screenshots/media before the detail page needs them.
  - Search and update checks should provide responsive loading states.
- Compatibility constraints:
  - Package-management state should come from omactl, not direct `oma-pm` or `oma-tum` dependencies.
  - Static catalog metadata remains owned by aoska.
  - omactl should provide structured JSON for package-management state, operation starts, unit status, unit results, and unit logs.
- Package-management/API requirements implied by the design:
  - Tauri package-manager commands should use the `pm_*` surface rather than raw `fetch_update_*` or `oma_*` command names.
  - Capability and query command group: `pm_capabilities`, `pm_update_summary`, `pm_list_updates`, `pm_list_installed`, and `pm_package_state`.
  - Planning command group: `pm_plan_update`, `pm_plan_install`, and `pm_plan_remove`.
  - Operation start command group: `pm_start_update`, `pm_start_install`, `pm_start_remove`, and `pm_start_refresh`.
  - Operation observation command group: `pm_operation_status`, `pm_operation_result`, `pm_operation_logs`, `pm_follow_operation_logs`, and `pm_cancel_operation`.
  - Storage command group: `pm_storage_summary`, joining omactl runtime size data with aoska catalog membership.
  - Temporary compatibility wrappers for old `fetch_update_*` and `oma_*` commands may exist only while delegating to the omactl JSON-backed `pm_*` layer and should be removed after frontend migration.
  - Query installed packages with source of management:
    - aoska-manageable software.
    - other oma-managed software.
    - non-oma-managed space.
    - free space.
  - Query upgradable packages with old version, new version, size/download size, package key, and whether the package is eligible for update/remove/open actions.
  - Query app/package detail state for a catalog app:
    - installed/openable.
    - upgradable.
    - installed version.
    - install size.
    - package source.
  - Plan install/remove/update before action where possible.
  - Start install/remove/update for selected package lists.
  - Distinguish scheduling success from operation success.
  - Follow unit logs and expose operation result.
  - Report update check status:
    - checking.
    - no updates.
    - normal updates.
    - security updates.
    - system updates.
    - stale last update.
    - failure reason.
    - manual action required.
  - Preserve `name_without_arch` or an equivalent package key for matching TUM package names to update entries.
- Test/screenshot expectations:
  - Future UI changes should include dark-mode screenshots for home, app detail, update/uninstall, and operation states.
  - Verify text readability over non-ideal screenshots.
  - Verify keyboard navigation for update selection and action buttons.

## Open questions

- [ ] What is the canonical catalog schema for app source labels such as `官方安装包`, `官方下载页面`, `Flathub`, `安同 OS 软件仓库`, and `Snapcraft`?
- [ ] Which source types are installable directly by aoska, and which require a `下载页面` or manual operation?
- [ ] What exact package grouping defines `系统更新` versus ordinary `软件更新`?
- [ ] Which storage numbers can omactl/oma provide reliably, and which are estimated from the system disk?
- [ ] What are the final icon assets for dark mode flags and toolbar controls?
- [ ] Should the update page support uninstalling software that was not originally installed by aoska? If yes, what confirmation copy is required?
- [ ] What is the minimum supported window size?
- [ ] Should `搜索结果` be a real route distinct from category/view-all, or a state of the current listing page?
