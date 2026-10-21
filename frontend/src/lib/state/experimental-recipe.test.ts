import { expect, it } from 'vitest';
import { validateExperimentalRecipe, experimentalModified } from './experimental-recipe';
import { validateProjectFile } from './feeds-speeds-persistence';
import { validateProjectFile as legacy } from './fixtures/preset4-legacy-v7';
import { editOperationRate, preserveManualCutting } from './feeds-speeds';
import { applyCuttingCommand, type CommandTarget } from './commands';
import type { ProjectFile, ToolEntry } from './project-types';
import type { OpEntry } from './op_types';
import { trialOp } from './experimental-recipe.fixture';
function file(op = trialOp()): ProjectFile {
  return {
    kind: 'ivac-project-experimental',
    version: 8,
    imports: [],
    visibleLayers: [],
    selectedEntities: [],
    operations: [op],
  };
}
it('round trips guarded v8, refuses disguised metadata and proves old v7 reader rejection', () => {
  const f = JSON.parse(JSON.stringify(file()));
  expect(() => validateProjectFile(f)).not.toThrow();
  expect(() => legacy(f)).toThrow('Unsupported project format/version');
  expect(() => validateProjectFile({ ...f, kind: 'ivac-project-cutting', version: 3 })).toThrow(
    'version 8',
  );
  expect(f.operations[0].appliedCutting.experimental_recipe).toEqual(
    trialOp().appliedCutting!.experimental_recipe,
  );
});
it('legacy v1 through v7 reader behavior is unchanged without experimental data', () => {
  const kinds = [
    'ivac-project',
    'ivac-project-fs',
    'ivac-project-cutting',
    'ivac-project-knowledge',
    'ivac-project-cad',
    'ivac-project-surfacing',
    'ivac-project-geometry',
  ];
  kinds.forEach((kind, index) => {
    const f = { ...file(), kind, version: index + 1, operations: [] } as unknown as ProjectFile;
    const outcome = (fn: typeof validateProjectFile) => {
      try {
        fn(f);
        return 'ok';
      } catch (e) {
        return String(e);
      }
    };
    expect(outcome(validateProjectFile)).toEqual(outcome(legacy));
  });
});
it('rejects private extension bodies, paths, fractional or malformed baselines', () => {
  const recipe = trialOp().appliedCutting!.experimental_recipe!;
  expect(() =>
    validateExperimentalRecipe({ ...recipe, notes: 'secret' } as typeof recipe),
  ).toThrow();
  expect(() =>
    validateExperimentalRecipe({
      ...recipe,
      source: { ...recipe.source, record_id: '/Users/private' },
    }),
  ).toThrow();
  expect(() =>
    validateExperimentalRecipe({
      ...recipe,
      applied_values: { ...recipe.applied_values, rpm: 18000.1 },
    }),
  ).toThrow();
});
it('manual values remain authoritative and retain ancestry with a modified indicator', () => {
  const op = trialOp();
  expect(experimentalModified(op)).toBe(false);
  const applied = editOperationRate(op, [{ id: 1 } as ToolEntry], 1, 'rough', 'rpm', 17000);
  expect(applied.experimental_recipe).toEqual(op.appliedCutting!.experimental_recipe);
  expect(experimentalModified({ ...op, appliedCutting: applied })).toBe(true);
  const patch = preserveManualCutting(op, { step: -1 });
  expect(experimentalModified({ ...op, ...patch } as OpEntry)).toBe(true);
  expect(patch.appliedCutting?.experimental_recipe).toEqual(op.appliedCutting!.experimental_recipe);
});
it('atomic trial command restores absent metadata on Undo and restores it on Redo', () => {
  const trial = trialOp();
  const plain = { ...trial, appliedCutting: undefined };
  const data = { operations: [plain], material: undefined, fsContext: undefined };
  const target = data as unknown as CommandTarget;
  const command = applyCuttingCommand(
    1,
    { step: trial.step, appliedCutting: trial.appliedCutting },
    undefined,
    undefined,
  );
  command.apply(target);
  expect(target.operations[0].appliedCutting).toEqual(trial.appliedCutting);
  command.revert(target);
  expect(target.operations[0].appliedCutting).toBeUndefined();
  command.apply(target);
  expect(target.operations[0].appliedCutting).toEqual(trial.appliedCutting);
});
