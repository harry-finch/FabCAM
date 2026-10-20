import type { DrawingRequest, DrawingGenerationRequest } from './types';
import type { CornerReliefRequest } from './types';
import { notificationError } from './error-notification';
import { isFatalWasmError } from './wasm-fatal';
// Web Worker host for the wasm CAM client. Runs the blocking
// pipeline OFF the main thread so the UI stays responsive, and streams
// per-op events back via postMessage as they are produced — postMessage
// enqueues fine even while this worker is synchronously busy inside the
// Rust call, so the main thread receives progress live.
//
// There is no in-worker cancel: the wasm call holds this worker's event
// loop, so a "cancel" message couldn't be read mid-run anyway. Real
// cancellation is the main thread calling `worker.terminate()` (see
// wasm-worker-client.ts), which kills the blocked run outright.
//
// The sim `Simulator` is NOT hosted here — it stays on the main thread
// (its per-frame zero-copy heightfield would otherwise need transferring
// every frame). Only the request→response pipeline calls cross over.

import type { WasmModule } from './wasm';
import type { WorkerRequest, WorkerResponse } from './wasm-worker-protocol';
import type {
  GenerateRequest,
  FsCamRequest,
  HelixRadiusRequest,
  RenderTextRequest,
  WireTextLayer,
} from './types';

// Minimal view of the dedicated-worker global. Declared locally rather
// than via `/// <reference lib="webworker" />` so this file doesn't pull
// the WebWorker lib into the DOM-typed program (which double-declares
// `self` / `postMessage` and fails the build).
interface WorkerScope {
  onmessage: ((ev: MessageEvent<WorkerRequest>) => void) | null;
  postMessage(message: unknown): void;
}
const ctx = self as unknown as WorkerScope;

let modPromise: Promise<WasmModule> | null = null;
function loadModule(): Promise<WasmModule> {
  if (!modPromise) {
    modPromise = (async () => {
      const wasm = (await import('ivac-wasm')) as unknown as WasmModule;
      if (typeof wasm.default === 'function') {
        await wasm.default();
      }
      return wasm;
    })();
  }
  return modPromise;
}

function post(msg: WorkerResponse) {
  ctx.postMessage(msg);
}

function errText(err: unknown): string {
  const normalized = notificationError(err);
  return typeof normalized === 'string' ? normalized : JSON.stringify(normalized);
}

// Set once the instance traps (NUMSAFE). From then on this worker answers
// every queued request with a fatal error and never calls into wasm again;
// the client terminates it and starts a fresh one.
let trapped: string | null = null;

ctx.onmessage = async (e: MessageEvent<WorkerRequest>) => {
  const { id, method, args } = e.data;
  if (trapped) {
    post({ id, type: 'error', error: `wasm engine stopped: ${trapped}`, fatal: true });
    return;
  }
  try {
    const m = await loadModule();
    switch (method) {
      case 'health':
        post({ id, type: 'result', value: m.healthz().ok === true });
        break;
      case 'version':
        post({ id, type: 'result', value: m.version() });
        break;
      case 'importBytes':
        post({
          id,
          type: 'result',
          value: m.importBytes(
            args[0] as string,
            args[1] as Uint8Array,
            args[2] as number | undefined,
          ),
        });
        break;
      case 'generate':
        post({ id, type: 'result', value: m.generate(args[0] as GenerateRequest) });
        break;
      case 'generateTwoSided': {
        if (!m.generateTwoSided) throw new Error('this wasm build predates two-sided generation');
        post({ id, type: 'result', value: m.generateTwoSided(args[0] as GenerateRequest) });
        break;
      }
      case 'generateStreaming': {
        const req = args[0] as GenerateRequest;
        if (m.generateStreaming) {
          // The Rust call fires `on_event` synchronously per op; each
          // hop posts an event that reaches the (unblocked) main thread
          // live. A null return is the wasm's internal-cancel sentinel.
          const r = m.generateStreaming(req, (ev) => post({ id, type: 'event', event: ev }));
          if (r === null) post({ id, type: 'cancelled' });
          else post({ id, type: 'result', value: r });
        } else {
          const r = m.generate(req);
          post({
            id,
            type: 'event',
            event: { kind: 'done', op_count: r.stats?.offset_count ?? 0, total_time_s: 0 },
          });
          post({ id, type: 'result', value: r });
        }
        break;
      }
      case 'renderText':
        post({ id, type: 'result', value: m.renderText(args[0] as RenderTextRequest) });
        break;
      case 'renderTextLayer':
        post({ id, type: 'result', value: m.renderTextLayer(args[0] as WireTextLayer) });
        break;
      case 'feedsSpeeds':
        if (!m.feedsSpeeds) throw new Error('Feeds & speeds unavailable in this WASM build');
        post({ id, type: 'result', value: m.feedsSpeeds(args[0] as FsCamRequest) });
        break;
      case 'evaluateDrawing':
        if (!m.evaluateDrawing) throw new Error('WASM does not support GEOM1');
        post({ id, type: 'result', value: m.evaluateDrawing(args[0] as DrawingRequest) });
        break;
      case 'generateDrawing':
        if (!m.generateDrawing) throw new Error('WASM does not support GEOM1');
        post({ id, type: 'result', value: m.generateDrawing(args[0] as DrawingGenerationRequest) });
        break;
      case 'cornerRelief':
        post({ id, type: 'result', value: m.cornerRelief(args[0] as CornerReliefRequest) });
        break;
      case 'computeHelixRadius':
        post({ id, type: 'result', value: m.computeHelixRadius(args[0] as HelixRadiusRequest) });
        break;
      case 'rasterizeStl':
        // Rasterize on the worker's existing wasm instance — no second
        // main-thread instance (the whole point of ivac-fm06).
        if (!m.fromStl) {
          post({ id, type: 'error', error: 'STL rasterization is unavailable in this build' });
        } else {
          post({
            id,
            type: 'result',
            value: m.fromStl(args[0] as Uint8Array, args[1] as number) ?? null,
          });
        }
        break;
      default:
        post({ id, type: 'error', error: `unknown method: ${String(method)}` });
    }
  } catch (err) {
    if (isFatalWasmError(err)) {
      trapped = errText(err);
      post({ id, type: 'error', error: trapped, fatal: true });
    } else {
      post({ id, type: 'error', error: errText(err) });
    }
  }
};
