import { describe, it, expect } from 'vitest';
import { vcarveStrategy, vcarveStrategyPatch, vcarveDepthPresentation } from './vcarve-strategy-ui';
import { History } from './history';
import { updateOperationCommand } from './commands';
import { buildOpEntry } from './op_defaults';
import { captureTemplate } from './operation-templates';
import en from '../i18n/messages/en.json';
import de from '../i18n/messages/de.json';
import type { VCarveOp } from './op_types';
const fixture = () =>
  buildOpEntry('vcarve', {
    nextId: 1,
    tools: [],
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  }) as VCarveOp;
describe('V-Carve presentation mapping', () => {
  it.each([undefined, false])('keeps legacy/default %s as Outline groove', (value) => {
    expect(vcarveStrategy(value).value).toBe('perimeter');
    expect(en[vcarveStrategy(value).label]).toBe('Outline groove');
    expect(vcarveStrategy(value).depthLabel).toBe('opprops.cut.final_depth');
  });
  it('maps the existing true flag to variable-depth engraving and maximum depth', () => {
    const view = vcarveStrategy(true);
    expect(en[view.label]).toBe('Variable-depth engraving');
    expect(en[view.depthLabel]).toBe('Maximum carving depth');
    expect(en[view.scope]).toContain('Does not guarantee complete pocket clearing');
  });
  it('provides result-oriented localized descriptions and truthful cap/pass help', () => {
    for (const catalog of [en, de])
      for (const mode of [false, true]) {
        const view = vcarveStrategy(mode);
        expect(catalog[view.label]).toBeTruthy();
        expect(catalog[view.description]).toBeTruthy();
        expect(catalog[view.depthHelp]).toContain('0');
      }
    expect(en['ops.vcarve.step.help']).toContain('previously cleared groove');
  });
  it('does not change generic operation labels or tooltips', () =>
    expect(vcarveDepthPresentation({ kind: 'pocket' })).toEqual({
      label: 'opprops.cut.final_depth',
      help: undefined,
      stepHelp: undefined,
    }));
  it('changes only the existing mode field and preserves values through history/save/reopen', () => {
    const op = {
      ...fixture(),
      depth: -1,
      step: -0.2,
      fullMedialAxis: false,
      carveMaxWidthMm: 2,
      sourceInsetMm: 0.1,
    };
    const state = { operations: [op], dirty: false };
    const history = new History();
    expect(vcarveStrategyPatch('medial')).toEqual({ fullMedialAxis: true });
    history.exec(updateOperationCommand(op.id, vcarveStrategyPatch('medial')), state);
    expect(state.operations[0]).toEqual({ ...op, fullMedialAxis: true });
    history.undo(state);
    expect(state.operations[0]).toEqual(op);
    history.redo(state);
    const saved = JSON.parse(JSON.stringify(state.operations[0]));
    expect(saved).toEqual({ ...op, fullMedialAxis: true });
    expect(vcarveStrategy(saved.fullMedialAxis).value).toBe('medial');
    history.exec(updateOperationCommand(op.id, vcarveStrategyPatch('perimeter')), state);
    expect(state.operations[0].depth).toBe(-1);
    expect(state.operations[0].step).toBe(-0.2);
    expect(vcarveStrategy(state.operations[0].fullMedialAxis).value).toBe('perimeter');
  });
  it('keeps the default and the existing template support boundary', () => {
    const op = fixture();
    expect(op.fullMedialAxis ?? false).toBe(false);
    expect(() => captureTemplate(op, [], 'V-Carve', 'test')).toThrow('templates.unsupported');
  });
});

it('handles a cleared operation selection without a model edit', () =>
  expect(vcarveDepthPresentation(null)).toEqual(vcarveDepthPresentation({ kind: 'profile' })));
