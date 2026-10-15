// Opt-in developer console diagnostics (?perf=1). No UI or project data fields.
// Preview marks bind to the result's generation version, rejecting stale work.
export class GenerationProbe {
  private readonly started: number;
  private version: number | undefined;
  private seen = new Set<string>();
  constructor(
    readonly id: number,
    private readonly clock: () => number,
    private readonly emit: (event: Record<string, unknown>) => void,
  ) {
    this.started = clock();
  }
  mark(stage: string, detail: Record<string, unknown> = {}) {
    if (this.seen.has(stage)) return;
    this.seen.add(stage);
    this.emit({ id: this.id, stage, elapsed_ms: this.clock() - this.started, ...detail });
  }
  bind(version: number) {
    this.version = version;
    this.mark('state_applied', { version });
  }
  preview(stage: string, version: number) {
    if (this.version !== version) return;
    if (stage === 'first_3d_frame' && !this.seen.has('wire_buffers_ready')) return;
    if (stage === 'first_3d_frame_after_simulation' && !this.seen.has('simulation_lod_ready'))
      return;
    this.mark(stage, { version });
  }
}
let sequence = 0;
let active: GenerationProbe | undefined;
export function beginGenerationProbe(post: string): GenerationProbe | undefined {
  if (
    typeof window === 'undefined' ||
    new URLSearchParams(window.location.search).get('perf') !== '1'
  )
    return;
  const query = new URLSearchParams(window.location.search);
  active = new GenerationProbe(
    ++sequence,
    () => performance.now(),
    (event) => console.info('PERF1', JSON.stringify(event)),
  );
  active.mark('start', { post, requested_api: query.get('api') ?? 'default' });
  return active;
}
export function probePreview(stage: string, version: number) {
  active?.preview(stage, version);
}
