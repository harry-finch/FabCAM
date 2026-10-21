#!/usr/bin/env bash
# macOS double-click launcher: keep this terminal open while testing the web app.
set -euo pipefail
cd "$(dirname "$0")/.."
if [[ -d /opt/homebrew/opt/node@24/bin ]]; then
  export PATH="/opt/homebrew/opt/node@24/bin:$PATH"
fi
# The dev launcher also works when the optional sccache daemon is unavailable.
export RUSTC_WRAPPER="${RUSTC_WRAPPER-}"
for port in 8766 5173; do
  if lsof -nP -iTCP:"$port" -sTCP:LISTEN > /dev/null 2>&1; then
    printf 'Port %s is already in use. Stop the previous FabCAM server before restarting.\n' "$port" >&2
    exit 1
  fi
done
log_dir="${TMPDIR:-/tmp}/fabcam-web-$$"
mkdir -p "$log_dir"
server_pid=""
cleanup() {
  if [[ -n "$server_pid" ]]; then kill "$server_pid" 2>/dev/null || true; fi
}
trap cleanup EXIT
trap 'exit 130' INT TERM
printf 'Building the web engine. Log: %s/server.log\n' "$log_dir"
cargo run -p ivac-server --locked >"$log_dir/server.log" 2>&1 &
server_pid=$!
ready=false
for ((attempt = 0; attempt < 240; attempt++)); do
  if ! kill -0 "$server_pid" 2>/dev/null; then
    cat "$log_dir/server.log"
    exit 1
  fi
  if curl --silent --fail http://127.0.0.1:8766/version > /dev/null; then
    ready=true
    break
  fi
  sleep 1
done
if [[ "$ready" != true ]]; then
  cat "$log_dir/server.log"
  printf 'The engine did not become ready on port 8766.\n' >&2
  exit 1
fi
cd frontend
node scripts/ensure-wasm-fresh.mjs --dev
printf '\nOpen http://127.0.0.1:5173/ — keep this terminal open. Ctrl+C stops both servers.\n\n'
node_modules/.bin/vite --host 127.0.0.1 --port 5173 --strictPort
