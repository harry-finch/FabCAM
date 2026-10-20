import { isFatalWasmError, WasmFatalError } from './wasm-fatal';
import type { DrawingRequest, DrawingResponse, DrawingGenerationRequest } from './types';
import type { CornerReliefRequest, CornerReliefResponse } from './types';
// WASM implementation of WiacClient. Loads the ivac-wasm pkg lazily so it
// only ships when the user opts in via `?api=wasm`. Useful for offline
// demos and CI smoke tests; the same JSON contract the HTTP / Tauri
// transports speak.

import { CancelledError, type PipelineEvent, type ProgressEvent, type WiacClient } from './client';
import type {
  PostListEntry,
  PostMeta,
  GenerateRequest,
  GenerateResponse,
  TwoSidedGenerateResponse,
  FsCamRequest,
  FsCamResponse,
  HelixRadiusRequest,
  HelixRadiusResponse,
  ImportResponse,
  RenderTextRequest,
  RenderTextResponse,
  RenderTextLayerResponse,
  SurfaceField,
  WireTextLayer,
  VersionResponse,
} from './types';

type SourcePack = import('./generated').components['schemas']['FsSourcePack'];
type LabCapture = import('./generated').components['schemas']['FsLabCapture'];
export type WasmModule = {
  evaluateDrawing?: (request: DrawingRequest) => DrawingResponse;
  generateDrawing?: (request: DrawingGenerationRequest) => GenerateResponse;
  captureLabPreset?: (
    project: import('./generated').components['schemas']['Project'],
    opId: number,
  ) => LabCapture;
  validateSourcePack?: (json: string) => SourcePack;
  default?: () => Promise<unknown>;
  healthz: () => { ok: boolean };
  listPosts?: () => unknown;
  inspectPost?: (script: string, filename?: string) => unknown;
  version: () => VersionResponse;
  importBytes: (filename: string, bytes: Uint8Array, scale?: number) => ImportResponse;
  generate: (request: GenerateRequest) => GenerateResponse;
  generateStreaming?: (
    request: GenerateRequest,
    onEvent: (event: PipelineEvent) => void,
  ) => GenerateResponse | null;
  /// Two-sided (flip-stock) generate → `{ front, back? }`. Optional because
  /// older wasm builds predate the export.
  generateTwoSided?: (request: GenerateRequest) => TwoSidedGenerateResponse;
  renderText: (request: RenderTextRequest) => RenderTextResponse;
  renderTextLayer: (layer: WireTextLayer) => RenderTextLayerResponse;
  feedsSpeeds?: (request: FsCamRequest) => FsCamResponse;
  cornerRelief: (request: CornerReliefRequest) => CornerReliefResponse;
  computeHelixRadius: (request: HelixRadiusRequest) => HelixRadiusResponse;
  /// Rasterize an STL byte stream to a relief height grid — the serialized
  /// SurfaceField `{origin,cell,cols,rows,z}`, or null when the mesh has no
  /// XY footprint. `maxDim` bounds the longer XY side's cell count. Loaded
  /// on demand by the STL relief path (see state/relief_stl.ts). Returns
  /// the serialized `SurfaceField`, or null when the mesh has no XY
  /// footprint. Optional because older wasm builds may predate the export.
  fromStl?: (bytes: Uint8Array, maxDim: number) => SurfaceField | null;
};

let modPromise: Promise<WasmModule> | null = null;

/// NUMSAFE: set when the main-thread instance traps. wasm-bindgen cannot
/// re-instantiate it in this realm, so every later call refuses with an
/// explicit error instead of running on an undefined instance.
let mainThreadFatal: WasmFatalError | null = null;

/// Request/response exports routed through the trap guard. The simulator's
/// classes are not wrapped (it owns its own instance lifecycle).
const GUARDED_EXPORTS = new Set([
  'healthz',
  'version',
  'importBytes',
  'generate',
  'generateTwoSided',
  'generateStreaming',
  'renderText',
  'renderTextLayer',
  'captureLabPreset',
  'validateSourcePack',
  'feedsSpeeds',
  'evaluateDrawing',
  'generateDrawing',
  'cornerRelief',
  'computeHelixRadius',
  'fromStl',
  'listPosts',
  'inspectPost',
]);

function guardModule(wasm: WasmModule): WasmModule {
  const guarded: Record<string, unknown> = { ...(wasm as unknown as Record<string, unknown>) };
  for (const name of GUARDED_EXPORTS) {
    const f = guarded[name];
    if (typeof f !== 'function') continue;
    guarded[name] = (...args: unknown[]) => {
      if (mainThreadFatal) throw mainThreadFatal;
      try {
        return (f as (...a: unknown[]) => unknown)(...args);
      } catch (err) {
        if (isFatalWasmError(err)) {
          mainThreadFatal = new WasmFatalError(
            'the in-browser CAM engine stopped after a fatal error; reload the page to start it again',
          );
        }
        throw err;
      }
    };
  }
  return guarded as unknown as WasmModule;
}

async function loadModule(): Promise<WasmModule> {
  if (!modPromise) {
    modPromise = (async () => {
      // The pkg is produced by `wasm-pack build crates/ivac-wasm --target web`
      // and lives under crates/ivac-wasm/pkg/. Vite resolves it relative to
      // the frontend root once the symlink (or pnpm linked dep) is in place.
      const wasm = (await import('ivac-wasm')) as WasmModule;
      if (typeof wasm.default === 'function') {
        await wasm.default();
      }
      return guardModule(wasm);
    })();
  }
  return modPromise;
}

export class WasmWiacClient implements WiacClient {
  async health(): Promise<boolean> {
    const m = await loadModule();
    return m.healthz().ok === true;
  }

  async listPosts(): Promise<PostListEntry[]> {
    const m = await loadModule();
    if (typeof m.listPosts !== 'function') {
      throw new Error('this wasm build lacks CPS support');
    }
    return m.listPosts() as PostListEntry[];
  }

  async inspectPost(script: string, filename?: string): Promise<PostMeta> {
    const m = await loadModule();
    if (typeof m.inspectPost !== 'function') {
      throw new Error('this wasm build lacks CPS support');
    }
    return m.inspectPost(script, filename) as PostMeta;
  }

  async version(): Promise<VersionResponse> {
    const m = await loadModule();
    return m.version();
  }

  async importFile(file: File, _format?: string, scale?: number): Promise<ImportResponse> {
    const m = await loadModule();
    const bytes = new Uint8Array(await file.arrayBuffer());
    return m.importBytes(file.name, bytes, scale);
  }

  async generate(request: GenerateRequest): Promise<GenerateResponse> {
    const m = await loadModule();
    return m.generate(request);
  }

  async generateTwoSided(request: GenerateRequest): Promise<TwoSidedGenerateResponse> {
    const m = await loadModule();
    if (!m.generateTwoSided) {
      throw new Error('this wasm build predates two-sided generation');
    }
    return m.generateTwoSided(request);
  }

  async generateStream(
    request: GenerateRequest,
    onProgress: (e: ProgressEvent) => void,
  ): Promise<GenerateResponse> {
    onProgress({ phase: 'import', fraction: 0.05, message: 'in-browser core' });
    const r = await this.generate(request);
    onProgress({ phase: 'done', fraction: 1.0, message: 'complete' });
    return r;
  }

  /**
   * WASM v1 is single-threaded — the Rust call holds the JS event
   * loop, so the cancel signal cannot fire mid-run. We still emit the
   * per-op event stream so the progress UI updates between ops, and
   * yield with `await Promise.resolve()` between events would require
   * the Rust→JS bridge to suspend (it can't here). Cancel support
   * arrives with web-worker threading in v2.
   */
  async generateStreaming(
    request: GenerateRequest,
    onEvent: (event: PipelineEvent) => void,
    cancelToken?: AbortSignal,
  ): Promise<GenerateResponse> {
    if (cancelToken?.aborted) throw new CancelledError();
    const m = await loadModule();
    if (!m.generateStreaming) {
      const r = m.generate(request);
      onEvent({ kind: 'done', op_count: r.stats?.offset_count ?? 0, total_time_s: 0 });
      return r;
    }
    const buffered: PipelineEvent[] = [];
    const r = m.generateStreaming(request, (ev) => {
      buffered.push(ev);
    });
    for (const ev of buffered) onEvent(ev);
    if (r === null) {
      onEvent({ kind: 'cancelled' });
      throw new CancelledError();
    }
    return r;
  }

  async renderText(request: RenderTextRequest): Promise<RenderTextResponse> {
    const m = await loadModule();
    return m.renderText(request);
  }

  async renderTextLayer(layer: WireTextLayer): Promise<RenderTextLayerResponse> {
    const m = await loadModule();
    return m.renderTextLayer(layer);
  }

  async captureLabPreset(
    project: import('./generated').components['schemas']['Project'],
    opId: number,
  ): Promise<LabCapture> {
    const m = await loadModule();
    if (!m.captureLabPreset) throw new Error('Lab capture unavailable in this WASM build');
    return m.captureLabPreset(project, opId);
  }
  async validateSourcePack(json: string): Promise<SourcePack> {
    const m = await loadModule();
    if (!m.validateSourcePack) throw new Error('Source packs unavailable in this WASM build');
    return m.validateSourcePack(json);
  }

  async feedsSpeeds(request: FsCamRequest): Promise<FsCamResponse> {
    const m = await loadModule();
    if (!m.feedsSpeeds) throw new Error('Feeds & speeds unavailable in this WASM build');
    return m.feedsSpeeds(request);
  }

  async evaluateDrawing(request: DrawingRequest): Promise<DrawingResponse> {
    const m = await loadModule();
    if (!m.evaluateDrawing) throw new Error('WASM does not support GEOM1');
    return m.evaluateDrawing(request);
  }
  async generateDrawing(request: DrawingGenerationRequest): Promise<GenerateResponse> {
    const m = await loadModule();
    if (!m.generateDrawing) throw new Error('WASM does not support GEOM1');
    return m.generateDrawing(request);
  }
  async cornerRelief(request: CornerReliefRequest): Promise<CornerReliefResponse> {
    const m = await loadModule();
    if (!m.cornerRelief) throw new Error('Corner relief unavailable in this WASM build');
    return m.cornerRelief(request);
  }
  async computeHelixRadius(request: HelixRadiusRequest): Promise<HelixRadiusResponse> {
    const m = await loadModule();
    return m.computeHelixRadius(request);
  }

  async rasterizeStl(bytes: Uint8Array, maxDim: number): Promise<SurfaceField | null> {
    const m = await loadModule();
    if (!m.fromStl) throw new Error('STL rasterization is unavailable in this build');
    return (m.fromStl(bytes, maxDim) as SurfaceField | null) ?? null;
  }
}
