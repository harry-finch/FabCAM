# Building FabCAM from source

FabCAM is the public product and package name. The bundle identifier
`org.opcenter.ivacam`, internal executable `ivac-desktop` and application-data
paths remain unchanged. Earlier ivaCAM beta installations are test installs;
see [REL1-A transition and development signing](releases/v1-stabilization-campaign/REL1_A.md).
Upstream attribution is retained; this supersedes the packaging-name retention in ADR-0012.

Targeted at users who'd rather not run a prebuilt binary they didn't
compile themselves. The project is a Cargo workspace plus a Vite frontend;
the desktop bundle is produced by Tauri 2.

## 1. Prerequisites

| Tool        | Version          | Notes                                          |
|-------------|------------------|------------------------------------------------|
| Rust        | pinned in [`rust-toolchain.toml`](../rust-toolchain.toml) | rustup picks up the pin automatically the first time you run cargo in this repo. |
| Node.js     | 20.19+ or 22.12+ | Matches the installed Vite 8 engine requirement; Node 24 was used for local checks. |
| pnpm        | ≥ 10             | Required. The frontend pulls in the local `ivac-wasm` package via pnpm's `link:` protocol, which npm can't resolve; lockfile is `frontend/pnpm-lock.yaml`. |
| Tauri CLI   | 2.x              | Install via `cargo install tauri-cli --version "^2" --locked` once you're set up for desktop builds. |
| wasm-pack   | ≥ 0.14           | Install via `cargo install wasm-pack --locked` for the WASM crate. |
| wasm-bindgen-cli | = `wasm-bindgen` crate ver | Optional but recommended. With it (+ `binaryen`) on `PATH`, the wasm build runs `wasm-pack --mode no-install` — no GitHub downloads at build time (deterministic, offline-capable, dodges the flaky-download race). Must match the crate **exactly**: `cargo install wasm-bindgen-cli --version <X> --locked` (current `<X>` = the `wasm-bindgen` version in `Cargo.lock`). Without it, wasm-pack falls back to downloading its own copy. |
| binaryen (`wasm-opt`) | matches above | Pairs with `wasm-bindgen-cli` for the offline wasm build. `apt install binaryen` / `brew install binaryen`. |
| cargo-deny  | ≥ 0.19           | Optional but recommended; CI runs it. `cargo install cargo-deny --locked`. |
| sccache     | ≥ 0.15           | Required — `.cargo/config.toml` sets it as `rustc-wrapper` (compile cache; big win after `cargo clean` / across worktrees). `cargo install sccache --locked`, or delete the `[build]` block from `.cargo/config.toml` to opt out. |

> **Reproducible tool setup (release gate).** CI uses Node 24 (`.nvmrc`) and pnpm 10;
> the release gate was last validated with pnpm 10.34.7. Node ≥ 25 no longer bundles
> Corepack, so install pnpm locally: `npm install --prefix ~/.local/pnpm-10 pnpm@10.34.7`
> and put `~/.local/pnpm-10/node_modules/.bin` on `PATH`. `scripts/pre-release.sh` runs the
> UI smoke gate only if `chromium`, `chromium-browser` or `google-chrome` is on `PATH`
> (on macOS, use an `exec` wrapper script named `google-chrome` on `PATH`, e.g.
> `printf '#!/bin/sh\nexec "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" "$@"\n'`;
> a plain symlink makes Chrome abort at launch).

### Linux (Debian/Ubuntu)

```sh
sudo apt-get install -y \
    build-essential pkg-config libssl-dev \
    libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev \
    librsvg2-dev libfreetype6-dev
```

`webkit2gtk-4.1` is the version Tauri 2 expects; on Ubuntu 22.04 you may
need the `webkit2gtk-4.0-dev` package instead. Either way, the Tauri 2
release notes are authoritative if your distro lags.

### Linux (Fedora/RHEL)

```sh
sudo dnf install -y \
    @development-tools openssl-devel \
    gtk3-devel webkit2gtk4.1-devel libappindicator-gtk3-devel \
    librsvg2-devel freetype-devel
```

### macOS

```sh
xcode-select --install   # CommandLine Tools
```

That's it — the frameworks Tauri needs (WebKit, security, etc.) come from
the system. V1 uses ad-hoc community signing; Developer ID/notarization is
optional post-V1 work and no Apple credentials are configured for it now.

### Windows

- Visual Studio 2022 Build Tools with the **C++ workload** (provides
  `link.exe`, MSVC, the Windows SDK).
- [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/)
  — usually preinstalled on Windows 11; install on Windows 10 if missing.
- For installer signing (optional): the WiX 3.x toolset.

See [`BUILDING_WINDOWS.md`](./BUILDING_WINDOWS.md) for the full
Windows walkthrough (prerequisites, concrete build steps, artifact
paths, and the Windows-VM route for Linux developers).

## 2. Clone

The repo expects a few sibling reference checkouts under `refs/`. Pull
them yourself; CI does the same.

```sh
git clone https://github.com/harry-finch/FabCAM.git
cd FabCAM

git clone https://github.com/multigcs/viaconstructor refs/viaconstructor
git clone https://github.com/IxMilia/dxf-rs           refs/dxf-rs
```

These are read-only references — the Rust port reimplements the parts it
needs. The viaConstructor checkout supplies DXF fixtures under
`refs/viaconstructor/tests/data/` that the workspace smoke test exercises.

## 3. Build

### Headless workspace (no UI)

```sh
cargo build --locked
cargo test --locked --tests   # full Rust unit + integration suite
```

### Feature matrix: `cps` (Autodesk-`.cps` post runtime)

The `.cps` post runtime (`crates/ivac-cps`, a JS engine + runtime
prelude) is behind a cargo feature so size-sensitive builds can drop it.

| transport | `cps` | why |
| --- | --- | --- |
| `ivac-cli` | **on** by default | desktop-class; no size pressure |
| `ivac-server` | **on** by default | serves `/posts`, `/posts/inspect` |
| `ivac-tauri` | **on** by default | desktop app |
| `ivac-wasm` | **off** by default | +1.5 MiB gzipped more than doubles the browser bundle |

```sh
cargo build -p ivac-core --features cps      # opt in explicitly
cargo test  -p ivac-core                     # feature-off matrix
cargo test  -p ivac-core --features cps      # feature-on matrix
scripts/wasm-size-guard.sh                   # measure the wasm delta
```

Every transport advertises its build through `/version` →
`capabilities`: `post-cps` is present exactly when the runtime is
compiled in, and the frontend hides the `.cps` option when it isn't. So
a wasm-only (browser) session shows the built-in dialects only, while
the desktop app and server offer `.cps` posts.

### Web frontend (browser)

```sh
cd frontend
pnpm install                 # pnpm required (link: dep, see prerequisites)
pnpm dev                     # http://localhost:5173, hot reload
pnpm build                   # static bundle to frontend/dist/
```

The full stack on top of `ivac-server` (Rust HTTP):

```sh
# in another shell
cargo run -p ivac-server     # listens on 127.0.0.1:8766
```

`vite.config.ts` already proxies `/api/*` to that port, so the dev
server's "Open file" / Generate UI just works.

### Browser-only (WASM, no backend)

```sh
wasm-pack build crates/ivac-wasm --target web --release
# pkg/ is published as a local dep — pnpm picks it up via
#   "ivac-wasm": "link:../crates/ivac-wasm/pkg"
cd frontend && pnpm install && pnpm dev
# visit http://localhost:5173/?api=wasm
```

The browser downloads the ivac-wasm pkg lazily and runs the entire
CAM pipeline client-side — no Rust server, no Python, no anything.

> **Pkg freshness is automatic.** `pnpm dev` / `pnpm build` (and thus
> every `cargo tauri build/dev`, which drives `pnpm build`) run
> `frontend/scripts/ensure-wasm-fresh.mjs` first: it rebuilds the pkg via
> `wasm-pack` only when the compiled wasm is older than the
> `ivac-wasm`/`ivac-core` sources, and is a fast no-op otherwise. So you
> normally don't run `wasm-pack` by hand — the explicit command above is
> just for a clean first build or to refresh the pkg in isolation. If
> `wasm-pack` isn't installed but a pkg already exists, the guard warns
> and proceeds (a frontend-only dev can keep working); it only errors when
> the pkg is missing entirely.

#### Install-free trial channel (OS-agnostic)

This is the lowest-friction way to let someone *try* FabCAM:
ship a static bundle to any CDN / static host and hand out a URL — it
runs in the browser on any OS, nothing to install, and the user's
geometry never leaves their machine.

```sh
wasm-pack build crates/ivac-wasm --target web --release
cd frontend && pnpm install && pnpm build   # → frontend/dist/
# host frontend/dist/ on GitHub Pages / S3+CloudFront / Netlify / …
# share:  https://your.site/
```

A **production build defaults to the in-browser wasm engine** when no
backend is configured, so the bare URL just works for a static deploy.
`?api=wasm` forces it explicitly (handy in dev); set
`VITE_IVAC_API=https://your-server` at build time to point at a
`ivac-server` instead. The wasm chunk stays lazy — only fetched when the
wasm transport is actually selected. No CORS, no API server, no
TLS-on-API, no auth to manage, because there is no backend.

Deploy the built `dist/` behind a normal static host — never expose the
Vite **dev** server publicly (its `server.fs.allow` opens the repo for
local convenience).

Copy-paste server configs (with the `.wasm` MIME type, immutable asset
caching, and an `index.html` fallback already set) live in
[`deploy/`](../deploy/): [`Caddyfile`](../deploy/Caddyfile),
[`nginx.conf`](../deploy/nginx.conf), and a [`README`](../deploy/README.md)
covering the hard requirements (serve `.wasm` as `application/wasm`, serve
from the domain root, no COOP/COEP needed, HTTPS optional).

Known limitations of this mode (track before leaning on it as the
headline trial path):

- **Generate runs in a Web Worker** (`ivac-5ue0`) — the CAM
  pipeline runs off the UI thread, so a heavy generate doesn't freeze
  the tab, and aborting a run cancels it for real (the worker
  is terminated and respawned). Falls back to the main-thread client
  where module workers aren't available. The 3D **sim** still runs on
  the main thread (its per-frame heightfield would need transferring
  every frame). To keep it smooth there, `?api=wasm` mode auto-caps the
  sim heightfield to a coarser grid (`WASM_TRIAL_SIM_CELL_CAP`, ~250k
  cells) — a lower-res carve *preview*, deliberately traded for
  zero-latency scrubbing (`ivac-5v1b`). Server / Tauri keep
  full sim fidelity.
- **Touch is supported** (`ivac-bwt7`) — the 2D canvas does
  pinch-zoom, two-finger pan, tap- and box-select, and a long-press
  opens the context menu; the 3D view uses OrbitControls (one-finger
  rotate, two-finger pan/zoom) with the same long-press menu. A
  touch-only ⧉ toggle enables keyboardless multi-select. Mouse +
  keyboard is unchanged.
- **wasm32 limits** — single-threaded, ~2–4 GB address space, plus a
  one-time module download; large jobs are slower than the native
  `ivac-server` and may hit memory ceilings sooner.

### Desktop bundle (Tauri)

FabCAM A1 desktop/browser icons use the existing resource paths and stable Tauri
identity. Refresh generated copies from the repository root with
`python3 assets/branding/fabcam/tools/build_assets.py --integrate` (Pillow and
Inkscape; no font required). Masters and native-format limitations are documented
in [the brand specification](../assets/branding/fabcam/README.md). This desktop
command does not regenerate Android launcher assets or change application IDs.
An icon file passing validation does not establish package/runtime compatibility;
build and inspect on each target OS separately.

```sh
cd crates/ivac-tauri
cargo tauri build            # produces a release bundle for your platform
```

> **Moved the checkout since your last build?** Physically relocating the
> repo (e.g. renaming a parent directory) bakes the old absolute path into
> the Tauri crates' build-script output, so `cargo tauri build` — and the
> [Android build](#android-tauri-mobile) below — die with `failed to read
> plugin permissions: …/out/permissions/…/app_hide.toml: No such file or
> directory`. Repair it surgically and re-run:
>
> ```sh
> scripts/fix-stale-target-paths.sh   # add --check to detect only; a no-op when nothing is stale
> ```
>
> It drops only the `tauri` / plugin / `ivac-tauri` fingerprints + build-script
> output, so cargo regenerates the permission ACL at the live path on the next
> build. The `webkit2gtk` / `wry` native bindings are left compiled, so the slow
> C-FFI stack is **not** rebuilt.

Outputs land under `target/release/bundle/`:

| Platform | Artifact                                                   |
|----------|------------------------------------------------------------|
| Linux    | `bundle/deb/*.deb`, `bundle/rpm/*.rpm`, `bundle/appimage/*.AppImage` |
| macOS    | `bundle/dmg/FabCAM_*.dmg`, `bundle/macos/FabCAM.app`   |
| Windows  | `bundle/msi/ivaCAM.msi`                            |

After bundling the AppImage, run

```sh
scripts/strip-appimage-media.sh
```

It removes the GStreamer *core libraries* that linuxdeploy bundles as
WebKit transitive deps and re-packs the AppImage. The app uses no
media (the webview's media backend is disabled at runtime), and the
bundled copies actively cause harm: they shadow the host's GStreamer,
so the host's WebKit loads OUR core against the HOST's plugins —
absent or version-mismatched plugins then print `GStreamer element
appsink not found. Please install it.` on every launch. With the
bundled core stripped, the loader falls back to the host's own
GStreamer (a hard dependency of every webkit2gtk package), keeping
the stack version-consistent and the AppImage smaller. Hosts that
genuinely lack the GStreamer plugin packages may still print the
warning — accepted: it's harmless stderr noise, and the alternative
(`bundleMediaFramework: true`) costs +14 MB for a media stack the app
never uses.

### Android (Tauri mobile)

The inherited Android implementation runs as a native Tauri 2 app: the Svelte UI loads from
the embedded `dist/` in the System WebView and calls the **native**
`ivac-core` commands over Tauri IPC (not the wasm path — `ivac-core` is
pure Rust with zero `*-sys`/C deps, so it cross-compiles to the Android
ABIs with just `rustup target add` + the NDK linker).

**One-time toolchain setup.** You need the Android SDK + NDK and JDK 21
on top of the desktop prerequisites:

```sh
# 1. SDK via cmdline-tools (sdkmanager). Point ANDROID_HOME wherever you
#    keep it; ~/Android/Sdk matches Android Studio's default.
export ANDROID_HOME="$HOME/Android/Sdk"
sdkmanager --install \
    "platform-tools" \
    "platforms;android-34" \
    "build-tools;34.0.0" \
    "ndk;27.2.12479018" \
    "cmake;3.22.1"
sdkmanager --licenses        # accept

# 2. Env the Tauri/Gradle build reads:
export NDK_HOME="$ANDROID_HOME/ndk/27.2.12479018"
export JAVA_HOME="/usr/lib/jvm/java-21-openjdk-amd64"   # JDK 21 (Gradle needs it)

# 3. Rust targets for the four Android ABIs:
rustup target add \
    aarch64-linux-android armv7-linux-androideabi \
    i686-linux-android x86_64-linux-android
```

**Scaffold + build.** From the Tauri crate:

```sh
cd crates/ivac-tauri
cargo tauri android init          # scaffolds gen/android (gradle, manifest, MainActivity)

# Debug APK for a 64-bit ARM device/emulator:
cargo tauri android build --debug --apk --target aarch64
# → gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk
#   (contains lib/arm64-v8a/libivac_tauri_lib.so — the native core)

# All four ABIs as separate per-ABI APKs (what we ship to testers):
cargo tauri android build --debug --apk --split-per-abi \
    --target aarch64 armv7 i686 x86_64
# → gen/android/app/build/outputs/apk/<abi>/debug/app-<abi>-debug.apk
#   for abi in arm64-v8a | armeabi-v7a | x86 | x86_64
# (Omit --split-per-abi for a single fat "universal" APK carrying all four.)

# Or live-reload onto a connected device / running emulator:
cargo tauri android dev
```

> After a version bump, delete `gen/android/app/tauri.properties` before
> building so the APK picks up the new version (see Versioning above).

**Staging for testers.** The build tools each drop output in their own
tool-specific tree (the APK path above; AppImages under
`target/release/bundle/appimage/`). Do **not** hand those out or copy
them by hand — names and locations drift. Instead run, from the repo
root, after a build:

```sh
scripts/stage-artifacts.sh android     # or: appimage | all
```

It copies the freshest build into **`dist-test/`** (the one
git-ignored directory testers are handed) under a version-stamped
canonical name read from `tauri.conf.json`:
`ivaCAM-<ver>-debug-arm64.apk` / `ivaCAM-<ver>-amd64.AppImage`.
`dist-test/` is the single source of truth for "shippable package" —
nothing else should accumulate loose APKs/AppImages.

`gen/android/` is **git-ignored** (`.gitignore` → `crates/ivac-tauri/gen`)
while the port settles — `cargo tauri android init` regenerates it
deterministically, so you re-scaffold rather than clone it. Once the
Android build is validated on-device we'll commit it (Tauri's
recommendation) so the build is reproducible without re-running `init`.

Notes / gotchas:

- **versionName ≥ 0.0.1.** Android's manifest merger rejects `0.0.0`. The
  APK's version comes from the git-ignored `gen/android/app/tauri.properties`
  (`tauri.android.versionName` / `versionCode`). `android build` writes that
  file from `tauri.conf.json`'s `version` **only when it's absent** — it does
  *not* overwrite an existing one, and `android init` leaves it untouched
  entirely. So after a version bump you must **delete `tauri.properties`**;
  the next `android build` regenerates it at the new version (verified:
  `0.3.0` → `versionName=0.3.0`, `versionCode=3000`). Skip the delete and the
  APK silently keeps the old version. See [Versioning](#versioning) below.
- **JDK 21** is required by the Gradle plugin; JDK 17 may work but is
  untested here. Expect deprecation warnings (source/target 8) — benign.
- **`beforeBuildCommand` (`pnpm build`) still runs**, so the static
  `dist/` is embedded in the APK. `ensure-wasm-fresh` also runs and
  bundles the wasm pkg even though the native path doesn't use it on
  Android — harmless, trimmed later.
- **Desktop-only plugins** (`window-state`) and `tauri.conf.json`'s
  `app.windows` are `#[cfg(desktop)]`-gated / ignored on mobile.
- **WebGL/Three.js 3D-sim performance** in the Android System WebView is
  the open risk — validate early; it may force a lower sim fidelity on
  mobile.

This is **build-verified, not yet runtime-verified** — the APK assembles
and links the native core, but on-device IPC / UI / file access (SAF)
validation is still pending hardware.

### Versioning

The project version has **one source of truth**:
`[workspace.package].version` in the root `Cargo.toml`. Do not hand-edit
versions anywhere else — run `scripts/bump-version.sh`, which writes that
one value and propagates it:

| Consumer                         | How it gets the version                                  |
|----------------------------------|----------------------------------------------------------|
| All Rust crates                  | inherit via `version.workspace = true` (resolved by cargo) |
| `schema/openapi.yaml` `info.version` | rewritten by `cargo xtask schema`; the `schema-check` CI guard fails on drift |
| `tauri.conf.json` `version`       | explicit copy the bump script writes — Tauri **can't** resolve `version.workspace`, so it needs a literal; `cargo xtask version-check` (CI) fails on drift |
| `frontend/package.json`          | no `version` field (private, unpublished package)        |
| Android APK (`tauri.properties`) | written from `tauri.conf.json` by `android build` only when absent — **delete it after a bump** so it regenerates (see Android note above) |

To cut a release, bump the single source and regenerate the derived
files with one command:

```sh
scripts/bump-version.sh 0.3.0   # edits Cargo.toml + tauri.conf.json,
                                # refreshes Cargo.lock, regenerates
                                # openapi.yaml + generated.ts, runs the guards
git diff                        # review
git commit -am "chore: release v0.3.0"
git tag v0.3.0 && git push --follow-tags
```

Only after separate publication authorization, a tag push triggers
`release-desktop.yml` for qualified desktop **artifacts only**, not a draft or
public release. SEC1 manual promotion remains disabled pending approval-protection
acceptance. The commands above are historical versioning examples, not authorization.
For the separately scoped Android APKs, delete
`crates/ivac-tauri/gen/android/app/tauri.properties` (so the version
refreshes), build them locally, and attach them to the draft out-of-band
(see the Android section above).

## 4. Verify

```sh
# Workspace tests must be green:
cargo test --workspace

# Frontend type check (svelte-check) must be clean:
cd frontend && pnpm check

# Smoke test the binary you just built:
./target/release/ivac --help
./target/release/ivac generate --request tests/fixtures/cli-v9/profile.request.json > out.json
```

For Tauri builds, launching the bundled app should open a window titled
*FabCAM* with the same UI you saw under `pnpm dev`.

### Explicit CLI generation (V1.1)

`ivac generate --request request.json` accepts a serialized v9 `PipelineRequest`,
not the frontend saved-project envelope. See [CLI request contract](CLI_GENERATE.md)
for reviewed geometry templates, input refusals, response fields and security boundaries.
The example above is a software-only fixture, not workshop-qualified machining data.

## 5. Troubleshooting

- **rustc version mismatch** — delete `target/` and let `rustup` re-install
  the pinned channel. Don't override the `rust-toolchain.toml`.
- **`webkit2gtk-4.1` not found on Linux** — your distro still ships the
  4.0 ABI. Install `libwebkit2gtk-4.0-dev` and re-run; Tauri 2 falls back.
- **WebView2 missing on Windows** — install the runtime from Microsoft;
  the bundle won't embed it for you in dev builds.
- **macOS first-launch warnings** — retain valid ad-hoc signing; follow the
  [community distribution plan](releases/v1-stabilization-campaign/REL1_B_PLAN.md)
  for Apple’s per-app Open Anyway procedure. Never disable Gatekeeper or remove
  quarantine. Investigate damaged bundles or checksum failures rather than bypassing them.

If something else trips you up that isn't listed, please open an issue
with the exact platform, toolchain versions (`rustc -V`, `node -v`,
`cargo tauri info`) and the failing command.


## V1 desktop test candidates

The manual `Desktop test candidates` workflow prepares macOS arm64 DMG, Windows x64
NSIS and Ubuntu amd64 DEB artifacts without publishing a Release. See the
[candidate preparation/protocol](releases/DESKTOP_CANDIDATES.md) for exact commands,
source/checksum manifests and the distinction between builder OS and tested runtime.

### V1 packaging identity and macOS signature audit

See [Phase 0](releases/v1-stabilization-campaign/PHASE0_AUDIT.md) for the productName migration boundary, actual failed signature diagnostics and secure signing/notarization prerequisites. Existing unsigned test builds are preserved; distribution is not qualified.

### REL1-B1 community macOS builds — approved V1 policy

Keep the existing `tauri.dev-signing.conf.json` ad-hoc overlay for development and
community packaging (`tauri build --config tauri.dev-signing.conf.json --bundles app,dmg -- --locked`
from crates/ivac-tauri with the documented toolchain). Validate DMG, assembled/installed
bundle signature and identity; generate SHA-256 after final packaging. Preserve data
and org.opcenter.ivacam. Clearly label the app “not notarized by Apple”.

[REL1-B1 acceptance and first-launch instructions](releases/v1-stabilization-campaign/REL1_B_PLAN.md)
require a fresh browser download and clean macOS installation/runtime test. Existing
local launch does not satisfy this gate. Developer ID/notarization/stapling are optional
REL1-B2 post-V1 work. No Apple credentials, publication or workflow dispatch authorized
by this planning update; release artifacts and checksums are published only after approval.

After building a community DMG, verify it before collection:

```sh
python3 scripts/verify-macos-community.py target/release/bundle/dmg/FabCAM_0.9.0-beta.1_aarch64.dmg --version 0.9.0-beta.1 --arch arm64 --output dist-test/macos-community-verification.json
```

Use the actual version/path and a new evidence destination. Universal packages require
`--arch arm64,x86_64`. The verifier needs macOS disk-image mounting permissions and
never changes quarantine or signs the mounted app. Existing evidence is not overwritten.
Local structural verification does not qualify clean installation; use the
[REL1-B1 clean-host protocol](releases/v1-stabilization-campaign/REL1_B1_CLEAN_INSTALL.md).


## Transactional program export (NUMSAFE3)

`ivac stream-gcode request.json --output job.ngc` stages output beside its
final destination and replaces a regular, non-read-only file only after generation
and flushing succeed. Failed generation preserves the previous file. Desktop
**Save program** uses the same native file utility after existing qualification
checks. Directory, symlink and read-only destinations are refused.

For `--two-sided --output job.ngc`, **both** `job.front.ngc` and `job.back.ngc`
must be new names; choose a fresh base for each attempt. Both programs are
prepared before publication. Publication is atomic per file where supported,
not for the pair: a second publication error can leave the new front file and
is explicitly reported as a failure. Existing pairs are never overwritten.
Filesystems without the required rename/hard-link capability fail closed.

These guarantees do not apply to bytes already sent to stdout/HTTP, browser
filesystem handling, power loss or forced process termination. CLI streaming
retains its documented lack of preview-derived warnings. See the
[NUMSAFE3 report](releases/NUMSAFE3_RESOURCE_EXPORT_RESULTS_2026_10_20.md) for
software evidence, operating-system limits and remaining qualification gates.

SEC1 publication reporting: native program export returns a published path and an
optional cleanup warning. Failure to remove a staging name after final publication
must not be reported as an unpublished file. CLI pair export continues after a
first-side cleanup warning; any second-side publication failure is explicitly
partial, preserves existing destinations and never triggers destructive rollback.

### SEC1 strict software qualification

`scripts/pre-release.sh` is mandatory/fail-closed: absent wasm-pack, cargo-deny or
Chromium fails, generator failures propagate, version consistency is checked, and
pedantic Clippy includes all features. Its green result qualifies software only.
WASM sizes use portable byte counts and deterministic gzip; no macOS stat wrapper
is required. Optional external FANUC oracles and ignored bounded TEST1 worker
entrypoints remain explicitly separate from mandatory gate success; TEST1 is run
through its dedicated watchdog workflow.

CI is reusable and records every mandatory job result plus the exact source SHA
and workflow run ID. Both candidate and tag builds depend on this qualification;
only artifacts are uploaded. The V1 artifact matrix is macOS arm64 DMG, Windows
x64 NSIS and Ubuntu amd64 DEB. Older universal macOS and additional packaging
formats are not inferred qualified by this V1 path.

`promote-desktop-draft.yml` is deliberately disabled (`if: false`) and contains no
release command. Its prepared verifier selects an explicit run, validates protected
reviewer configuration, source/version/run identity and checksums, and only copies
verified installer bytes. Activation and the actual draft-publication step require
a separate human checkpoint proving the protected environment works (required
reviewers, self-review prevention and no administrator bypass). No environment or
GitHub setting was configured by SEC1. Merely naming an environment is insufficient.
