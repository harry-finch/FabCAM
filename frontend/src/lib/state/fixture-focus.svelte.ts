/** View intent only; never modifies a fixture or work zero. */
class FixtureFocus {
  id = $state<number | null>(null);
  seq = $state(0);
  focus(id: number) {
    this.id = id;
    this.seq++;
  }
}
export const fixtureFocus = new FixtureFocus();
