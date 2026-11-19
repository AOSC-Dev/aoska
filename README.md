# aoska

AOSC OS software store built with Tauri 2, Vue 3, and TypeScript. ASMR provides
catalog metadata; amo provides package state and operations over the system D-Bus.

## AOSC OS development

Install the development tools and desktop integration dependencies:

```sh
sudo oma install git nodejs pnpm rustc cargo make pkg-config webkit2gtk amo debconf-kde
```

`pkg-config` provides the compiler flags for GTK/WebKit. Run aoska in a graphical
user session with a Polkit authentication agent so amo can request authorization
for package changes. The KDE debconf helper handles package configuration prompts.

Check that amo is running and exposes its interface:

```sh
systemctl status amo.service
busctl --system introspect io.aosc.Amo /io/aosc/Amo
```

The integration targets [amo 0.6.1](https://github.com/AOSC-Dev/amo/tree/v0.6.1),
interface `io.aosc.Amo1`. The consumer contract and version-specific limitations
are recorded in [RFC-0002](docs/rfc/RFC-0002.md).

```sh
pnpm install --frozen-lockfile
pnpm tauri dev
```

For browser-only frontend work, `pnpm dev` uses deterministic catalog and
package-manager stubs. Tauri uses the real amo backend; installing, updating, or
removing packages there changes the test system after confirmation and authorization.

## Checks and builds

```sh
pnpm run test:core
pnpm exec eslint src
pnpm build
cargo test --locked --manifest-path src-tauri/Cargo.toml --lib
pnpm tauri build
```

The Linux D-Bus regression test starts a private `dbus-daemon` and does not use the
system package manager. The opt-in test in `src-tauri/tests/amo_live.rs` targets a
real amo service: it refreshes metadata and, when `AOSKA_TEST_PACKAGE` is set,
installs and removes that initially absent package after checking the plan contains
no other changes. Run it explicitly from an authorized graphical session:

```sh
AOSKA_TEST_PACKAGE=sl cargo test --locked --manifest-path src-tauri/Cargo.toml --test amo_live -- --ignored --nocapture
```

To build the native program without downloading bundle helpers:

```sh
pnpm exec tauri build --debug --no-bundle --ci -- --locked
```

For a release binary without distro packaging, run `pnpm build`, then
`cargo build --locked --release --manifest-path src-tauri/Cargo.toml --bin aoska`.

## Current package-manager capabilities

amo supports package lookup, update lists, transaction previews, install/remove,
selected or full upgrades, and metadata refresh. aoska follows each request until
its matching `ResultReport` signal arrives. A successful method return only means
the daemon accepted the request.

amo 0.6.1 has no full installed-package inventory, storage summary, cancellation,
or operation-history API. Those controls are unavailable with this backend.
The installed-app tab queries only applications in the ASMR catalog and is labeled
with that scope; it does not enumerate every system package. Installed apps can be
removed individually or in a confirmed batch. Unknown package sizes remain unknown.

Application launching uses GIO and registered visible Desktop Entries in the user
session. Catalog `desktop_id` is optional (in the index or package information);
otherwise aoska checks `package-name.desktop`. It never treats installation alone
as proof that an application can be opened. Website and download links use the
system URL opener and accept HTTP(S) URLs.

Update cards retain successful checks/updates made through aoska, show a reminder
after 14 days, and distinguish completion, failure, and unknown operation state.
The saved times describe aoska activity, not the system's global update history.
An empty TUM list cannot distinguish unavailable security metadata from no matching
security updates, so aoska keeps the classification unknown in that case.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).
