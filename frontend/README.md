# FabCAM frontend

Svelte 5, TypeScript and Vite 8 UI, shared by Tauri desktop, HTTP-server and WASM transports. Internal package `ivac-frontend`, `ivac-wasm` linkage, API contracts and persisted storage keys are compatibility identifiers and remain unchanged.

From the repository root, build a clean WASM dependency first:

```sh
wasm-pack build crates/ivac-wasm --target web --release
pnpm --dir frontend install --frozen-lockfile
pnpm --dir frontend dev
pnpm --dir frontend check
pnpm --dir frontend lint
pnpm --dir frontend test
pnpm --dir frontend build
```

Node must satisfy Vite's `^20.19.0 || >=22.12.0` requirement. The dev/build freshness guard maintains the WASM package thereafter. Current standalone WASM has no CPS support; native server/desktop is needed for ShopBot. See [BUILDING](../docs/BUILDING.md).

The title and About use the same `__IVAC_PKG_VERSION__` build define. The private frontend is unversioned, so `vite.config.ts` resolves the unchanged Tauri release version; Cargo is checked for consistency. Git/date defines remain technical identifiers. About prose and preserved upstream acknowledgements are authored in [docs/ABOUT.md](../docs/ABOUT.md).

Approved branding masters and generated runtime copies: [brand specification](../assets/branding/fabcam/README.md). Public product identity and compatibility exceptions: [ADR-0012](../docs/shopbot-modernization/adr/ADR-0012-fabcam-product-identity.md). No runtime font dependency is needed for the production logos.
