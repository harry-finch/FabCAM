// NUMSAFE: fatal WebAssembly failures. A Rust panic in wasm32 aborts as a
// trap (`RuntimeError: unreachable`) — `catch_unwind` cannot catch it — and
// a wasm stack overflow surfaces as a `RangeError`. After either the
// instance's state is undefined (shadow stack, borrowed cells, half-drained
// diagnostics), so it must never serve another request.

/// True for errors that leave the wasm instance unusable.
export function isFatalWasmError(err: unknown): boolean {
  if (err instanceof RangeError) return true;
  return typeof WebAssembly !== 'undefined' && err instanceof WebAssembly.RuntimeError;
}

/// A request that failed because the wasm engine stopped, not because of
/// its input. Never a partial result.
export class WasmFatalError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'WasmFatalError';
  }
}

/// Fresh engines tried after consecutive fatal failures before giving up
/// (each one is started by a new request, never by an automatic retry).
export const MAX_CONSECUTIVE_FATAL = 3;
