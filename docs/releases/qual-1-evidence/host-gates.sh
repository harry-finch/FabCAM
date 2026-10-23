#!/usr/bin/env bash
# Host-side acceptance only. No installer, hardware or source mutation.
set -euo pipefail
cd "${1:?usage: host-gates.sh CLEAN_CHECKOUT}"
[[ -f scripts/pre-release.sh && -f rust-toolchain.toml ]]
[[ -z "$(git status --porcelain --untracked-files=no)" ]] || { echo 'Use a clean tracked-source checkout.' >&2; exit 1; }
if [[ -x /opt/homebrew/opt/node@24/bin/node ]]; then export PATH="/opt/homebrew/opt/node@24/bin:$PATH"; fi
if [[ -x "$HOME/.local/pnpm-10/node_modules/.bin/pnpm" ]]; then export PATH="$HOME/.local/pnpm-10/node_modules/.bin:$PATH"; fi
[[ "$(node -p 'process.versions.node.split(".")[0]')" = 24 ]] || { echo 'Node 24 required' >&2; exit 1; }
[[ "$(pnpm --version)" = 10.34.7 ]] || { echo 'pnpm 10.34.7 required' >&2; exit 1; }
qual1_bin="$(mktemp -d)"
trap 'rm -f "$qual1_bin/google-chrome"; rmdir "$qual1_bin"' EXIT
if [[ -x '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' ]]; then
  cat > "$qual1_bin/google-chrome" <<'CHROME'
#!/bin/sh
exec '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' "$@"
CHROME
  chmod +x "$qual1_bin/google-chrome"
  export PATH="$qual1_bin:$PATH"
fi
export CARGO_INCREMENTAL=0
export RUSTC_WRAPPER=''
git rev-parse HEAD
node --version
pnpm --version
rustc -Vv
# Fresh checkout prerequisite: Tauri's generate_context needs frontend/dist.
wasm-pack build crates/ivac-wasm --target web --release
pnpm --dir frontend install --frozen-lockfile
pnpm --dir frontend run build
scripts/pre-release.sh
