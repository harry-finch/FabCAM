// Runtime-only ownership and wall-clock timing; no scheduler or persisted state.
export type GenerationOutcome =
  | 'running'
  | 'success'
  | 'failed'
  | 'cancelled'
  | 'obsolete'
  | 'blocked';
export interface GenerationRun {
  id: number;
  epoch: number;
  context: string;
  startedAt: number;
  endedAt?: number;
  outcome: GenerationOutcome;
}
export class GenerationRuns {
  private sequence = 0;
  private current: GenerationRun | null = null;
  constructor(private now: () => number = () => performance.now()) {}
  start(epoch: number, context: string): GenerationRun | null {
    if (this.current?.outcome === 'running') return null;
    this.current = {
      id: ++this.sequence,
      epoch,
      context,
      startedAt: this.now(),
      outcome: 'running',
    };
    return this.current;
  }
  owns(run: GenerationRun, epoch: number): boolean {
    return this.current?.id === run.id && run.epoch === epoch && this.current.outcome === 'running';
  }
  accepts(run: GenerationRun, epoch: number, context: string): boolean {
    return this.owns(run, epoch) && run.context === context;
  }
  finish(
    run: GenerationRun,
    epoch: number,
    outcome: Exclude<GenerationOutcome, 'running'>,
  ): GenerationRun | null {
    if (!this.owns(run, epoch)) return null;
    this.current = { ...run, endedAt: Math.max(run.startedAt, this.now()), outcome };
    return this.current;
  }
  invalidate(): void {
    this.current = null;
  }
}
export function elapsedSeconds(run: GenerationRun, now: number): string {
  return (Math.max(0, (run.endedAt ?? now) - run.startedAt) / 1000).toFixed(1);
}
