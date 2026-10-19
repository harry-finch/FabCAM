import type { VerificationInput, VerificationReply } from './qualification';
import { verifyProgram, type SimulatorConstructor } from './verify-program';
interface Scope {
  onmessage: ((event: MessageEvent<VerificationInput>) => void) | null;
  postMessage(value: VerificationReply): void;
}
const scope = self as unknown as Scope;
scope.onmessage = async ({ data }) => {
  const started = performance.now();
  try {
    const wasm = await import('ivac-wasm');
    await wasm.default();
    const version = wasm.version() as { capabilities?: string[] };
    if (!version.capabilities?.includes('movement-tool-provenance-1'))
      throw new Error('Simulator tool provenance contract unavailable — reload a compatible build');
    const Simulator = (wasm as unknown as { Simulator: SimulatorConstructor }).Simulator;
    const front = verifyProgram(Simulator, data, 'front');
    const back = data.back ? verifyProgram(Simulator, data, 'back') : undefined;
    scope.postMessage({
      result: {
        key: data.key,
        front,
        back,
        elapsedMs: performance.now() - started,
        simulatorVersion: version,
      },
    });
  } catch (error) {
    scope.postMessage({ error: String(error) });
  }
};
