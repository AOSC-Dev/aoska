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

On Linux, Aoska detects Wayland sessions with the NVIDIA kernel module loaded
and sets `__NV_DISABLE_EXPLICIT_SYNC=1` before GTK initializes. This avoids the
surface acquire-point violation described in [WebKit #280210](https://bugs.webkit.org/show_bug.cgi?id=280210)
while allowing WebKit to select its DMA-BUF renderer. Existing values of that
variable take precedence. To test the native explicit-sync path, launch with
`__NV_DISABLE_EXPLICIT_SYNC=0 aoska`. The workaround affects only the Aoska
process and its children.

### ASMR catalog

Aoska reads a built ASMR v1 directory over HTTP(S). To preview the ASMR examples,
serve the generated directory from a second terminal. The helper binds to loopback
and allows the browser preview to read JSON from a different port.

```sh
python3 scripts/serve-asmr.py ../aosc-os-asmr/examples/dist/v1
```

If the example output is missing, build it into a temporary directory and serve it:

```sh
catalog_dir=$(mktemp -d)
python3 ../aosc-os-asmr/tools/asmr.py build --root ../aosc-os-asmr/examples --output "$catalog_dir/v1"
python3 scripts/serve-asmr.py "$catalog_dir/v1"
```

Open **Settings** to select a catalog. **Automatic** follows the build channel:

| Build | Default catalog |
| --- | --- |
| Tauri debug build or Vite dev server | Dev · `https://aosc-dev.github.io/aosc-os-asmr/v1/` (GitHub Pages) |
| Tauri release build or Vite production build | Stable · `https://repo.aosc.io/asmr/v1/` (planned AOSC mirror path) |

Select **Custom** and enter `http://127.0.0.1:8765/` to use the local example
server above. Save, then open Home to load the selected catalog. Desktop
preferences persist in the user’s Aoska configuration directory. Browser
preferences persist for the preview’s origin. Selecting **Automatic** restores
the build default. Stable publication is pending provisioning of the mirror path.

An explicit environment URL takes precedence over the saved preference. Desktop
reads `AOSKA_ASMR_URL` at launch; the browser preview reads
`VITE_AOSKA_ASMR_URL` at Vite startup or build time. Use the directory
containing `index.json`, for example:

```sh
AOSKA_ASMR_URL=http://127.0.0.1:8765/ pnpm tauri dev
VITE_AOSKA_ASMR_URL=http://127.0.0.1:8765/ pnpm dev
```

The homepage follows the pointer in `index.json`;
collection cards use app IDs, and details load `apps/<id>.json` on demand.
For an end-to-end preview, open the repository app and the assisted-download app
from Essentials. Only the repository app has a package name for amo. In the
browser preview its package operations use the local stub. A Tauri build can
also use that stub by setting `VITE_AOSKA_PM_BACKEND=stub` at build time.

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
session. Aoska resolves the selected repository package to `package-name.desktop`
unless the index supplies an explicit `desktop_id`. Website and download links use the system URL opener and accept HTTP(S) URLs.

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
