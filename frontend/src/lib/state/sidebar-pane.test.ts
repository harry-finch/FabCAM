import { describe, expect, it } from 'vitest';
import { togglePane, revealPane, type SidebarPane } from './sidebar-pane';
const sections: SidebarPane[] = ['stock', 'layers', 'text', 'operations'];
describe('inspector accordion', () => {
  for (const section of sections)
    it(`closes ${section} without opening another section`, () => {
      const closed = togglePane({ active: section }, section);
      expect(closed.active).toBeNull();
      expect(togglePane(closed, section).active).toBe(section);
    });
  it('switches sections, then permits all sections to stay closed', () => {
    let state = togglePane({ active: 'layers' }, 'stock');
    state = togglePane(state, 'layers');
    expect(togglePane(state, 'layers').active).toBeNull();
    state = togglePane(state, 'operations');
    expect(togglePane(state, 'operations').active).toBeNull();
  });
  it('explicit reveals open a closed section and keep repeated reveals open', () => {
    const state = revealPane({ active: null }, 'operations');
    expect(state.active).toBe('operations');
    expect(revealPane(state, 'operations')).toBe(state);
    expect(revealPane(state, 'stock').active).toBe('stock');
  });
});
