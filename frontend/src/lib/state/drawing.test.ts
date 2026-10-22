import { describe, expect, it } from 'vitest';
import {
  drawingCommand,
  drawingInputKey,
  initialBinding,
  selectedBinding,
  type DrawingState,
} from './drawing';
import { History } from './history';
import { validateProjectV9 as validateProjectFile } from './project-v9';
import { explicitProjectFixture } from './explicit-project.fixture';

const op = {
  id: 1,
  kind: 'profile',
  sourceLayers: [],
  sourceObjects: [2],
  name: 'cut',
  enabled: true,
  toolId: 1,
} as unknown as import('./op_types').OpEntry;
const drawing = {
  version: 1,
  originalBindings: [],
  inputKey: '[]',
  response: {
    version: 1,
    drawing: { version: 1 },
    evaluated: {
      contours: [
        { import: 1, entity: 9 },
        { import: 2, entity: 1 },
      ],
    },
  },
} as unknown as DrawingState;
describe('edited drawing lifecycle adapters', () => {
  it('never guesses a legacy numeric binding', () => {
    expect(initialBinding(op).kind).toBe('unresolved');
    expect(selectedBinding(op, drawing)).toEqual({
      kind: 'contours',
      ids: [{ import: 2, entity: 1 }],
    });
    expect(selectedBinding({ ...op, sourceObjects: [3] }, drawing).kind).toBe('unresolved');
  });
  it('restores geometry and references in one Undo/Redo', () => {
    const nextOps = [{ ...op, geometryBinding: initialBinding(op) }];
    const state = {
      drawing: undefined as DrawingState | undefined,
      operations: [op],
      dirty: false,
    };
    const h = new History();
    h.exec(drawingCommand(undefined, drawing, [op], nextOps), state);
    expect(state.operations[0].geometryBinding?.kind).toBe('unresolved');
    expect(state.drawing).toEqual(drawing);
    h.undo(state);
    expect(state.drawing).toBeUndefined();
    expect(state.operations).toEqual([op]);
    h.redo(state);
    expect(state.drawing).toEqual(drawing);
    expect(state.operations).toEqual(nextOps);
  });
  it('requires a guarded format and matching source revision', () => {
    const file = explicitProjectFixture({
      kind: 'ivac-project-geometry',
      version: 7,
      imports: [],
      operations: [],
      drawing,
    } as unknown as import('./project-types').ProjectFile);
    expect(() => validateProjectFile(file)).not.toThrow();
    expect(() => validateProjectFile({ ...file, version: 1, kind: 'ivac-project' })).toThrow();
    expect(() =>
      validateProjectFile({ ...file, drawing: { ...drawing, inputKey: 'changed' } }),
    ).toThrow();
    expect(drawingInputKey([])).toBe('[]');
  });
});

it('old backend is never called through an unedited fallback', async () => {
  const { generateEditedDrawing } = await import('./drawing');
  let legacyCalls = 0;
  const oldBackend = {
    generate: async () => {
      legacyCalls++;
      return {};
    },
  } as unknown as import('../api/client').WiacClient;
  const p = {
    data: { drawing, operations: [op] },
  } as unknown as import('./project.svelte').ProjectState;
  await expect(
    generateEditedDrawing(p, oldBackend, {} as never, new AbortController().signal),
  ).rejects.toThrow('No legacy fallback');
  expect(legacyCalls).toBe(0);
});

it('cannot reset old numeric selections onto changed imports or newly created operations', async () => {
  const { resetDrawingOperations, nextDrawingOperationId } = await import('./drawing');
  const original = {
    ...drawing,
    originalBindings: [{ id: 1, sourceObjects: [2], sourceLayers: [] }],
  };
  const bound = {
    ...op,
    geometryBinding: { kind: 'contours' as const, ids: [{ import: 2, entity: 1 }] },
  };
  const reset = resetDrawingOperations(original, [], [bound]);
  expect(reset[0].sourceObjects).toEqual([2]);
  expect(reset[0].geometryBinding).toBeUndefined();
  expect(() => resetDrawingOperations({ ...original, inputKey: 'changed' }, [], [bound])).toThrow(
    'Undo source/transform',
  );
  expect(() => resetDrawingOperations(original, [], [{ ...bound, id: 3 }])).toThrow(
    'created after cleanup',
  );
  expect(nextDrawingOperationId([], original)).toBe(2);
});
it('never writes stable or unresolved references into an old readable envelope', () => {
  for (const binding of [
    { kind: 'contours', ids: [{ import: 1, entity: 1 }] },
    { kind: 'unresolved', previous: [], reason: 'review' },
  ]) {
    expect(() =>
      validateProjectFile({
        kind: 'ivac-project',
        version: 1,
        imports: [],
        operations: [{ ...op, geometryBinding: binding }],
      } as unknown as import('./project-types').ProjectFile),
    ).toThrow('requires project format v9');
  }
});

it('rejects malformed reset provenance and unknown saved binding variants', () => {
  const base = explicitProjectFixture({
    kind: 'ivac-project-geometry',
    version: 7,
    imports: [],
    operations: [],
    drawing,
  } as unknown as import('./project-types').ProjectFile);
  expect(() =>
    validateProjectFile({
      ...base,
      drawing: {
        ...drawing,
        originalBindings: [
          { id: 1, sourceLayers: null },
          { id: 1, sourceLayers: null },
        ],
      },
    }),
  ).toThrow('original operation bindings');
  expect(() =>
    validateProjectFile({
      ...base,
      operations: [{ ...op, geometryBinding: { kind: 'future' } }],
    } as unknown as import('./project-types').ProjectFile),
  ).toThrow('edited operation binding');
});
