/** Owner bindings are disposable; a result is only an offer, never a queued command. */
export interface ActionState {
  token: string;
  reason?: string;
}
export interface ActionOwner {
  state: (argument?: string) => ActionState;
  run: (argument?: string) => void | Promise<unknown>;
}
export class PaletteDispatch {
  private owners = new Map<string, ActionOwner>();
  private listeners = new Set<() => void>();
  private pending = new Set<string>();
  private serial = 0;
  private generations = new Map<string, number>();
  subscribe(listener: () => void) {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }
  register(id: string, owner: ActionOwner) {
    this.owners.set(id, owner);
    this.generations.set(id, ++this.serial);
    this.changed();
    return () => {
      if (this.owners.get(id) === owner) {
        this.owners.delete(id);
        this.changed();
      }
    };
  }
  private changed() {
    this.listeners.forEach((listener) => listener());
  }
  state(id: string, argument?: string): ActionState | undefined {
    const state = this.owners.get(id)?.state(argument);
    return state ? { ...state, token: `${this.generations.get(id)}:${state.token}` } : undefined;
  }
  async execute(
    id: string,
    expected: ActionState,
    argument?: string,
  ): Promise<'done' | 'changed' | 'disabled' | 'pending'> {
    if (this.pending.has(id) || (this.pending.size > 0 && id !== 'cancel')) return 'pending';
    const owner = this.owners.get(id);
    const current = this.state(id, argument);
    if (!owner || !current || current.token !== expected.token) return 'changed';
    if (current.reason) return 'disabled';
    this.pending.add(id);
    try {
      await owner.run(argument);
      return 'done';
    } finally {
      this.pending.delete(id);
    }
  }
}
export const paletteDispatch = new PaletteDispatch();
