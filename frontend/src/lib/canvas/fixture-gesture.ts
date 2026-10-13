import { updateFixtureCommand, type CommandTarget } from '../state/commands';
import type { History } from '../state/history';

/** Live model placement, one command on release; no shadow visual position. */
export class FixtureGesture {
  private before: [number, number];
  private dirty: boolean;
  private moved = false;
  constructor(
    readonly id: number,
    readonly pointerId: number,
    private start: { x: number; y: number },
    private target: CommandTarget,
    private history: History,
  ) {
    this.before = [...target.fixtures.find((f) => f.id === id)!.origin];
    this.dirty = target.dirty;
  }
  move(point: { x: number; y: number }) {
    const origin: [number, number] = [
      this.before[0] + point.x - this.start.x,
      this.before[1] + point.y - this.start.y,
    ];
    if (!origin.every(Number.isFinite)) return;
    this.moved = origin[0] !== this.before[0] || origin[1] !== this.before[1];
    this.place(origin);
    this.target.dirty = this.dirty || this.moved;
  }
  finish(cancel = false) {
    const after = this.target.fixtures.find((f) => f.id === this.id)?.origin;
    this.place(this.before);
    this.target.dirty = this.dirty;
    if (cancel || !this.moved || !after) return;
    const command = updateFixtureCommand(this.id, { origin: [...after] });
    // Two successive complete drags must remain two independent Undo steps.
    delete command.coalesce_key;
    this.history.exec(command, this.target);
  }
  private place(origin: [number, number]) {
    this.target.fixtures = this.target.fixtures.map((f) =>
      f.id === this.id ? { ...f, origin: [...origin] } : f,
    );
  }
}
