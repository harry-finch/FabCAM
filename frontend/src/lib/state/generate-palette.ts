import { onMount } from 'svelte';
import { project } from './project.svelte';
import { paletteDispatch } from './palette-dispatch';
import { t } from '../i18n';
import { exportSafetyProblem, type ExportStatus } from './generate-palette-policy';
/** Both entry points use the same complete safety policy. */
export function exportProblem(status: ExportStatus, side: 'front' | 'back') {
  const problem = exportSafetyProblem(status, side, project.data.settings);
  return problem ? t(problem.key, { count: problem.count }) : undefined;
}
export function bindGeneratePalette(
  status: () => ExportStatus,
  run: () => Promise<void>,
  cancel: () => void,
  download: (side: 'front' | 'back') => Promise<void>,
  warnings: () => void,
) {
  onMount(() => {
    const identities = new WeakMap<object, number>();
    let next = 0;
    const identity = (value: object | null) => {
      if (!value) return 0;
      if (!identities.has(value)) identities.set(value, ++next);
      return identities.get(value)!;
    };
    const state = (action: string) => {
      const info = status();
      const generating = project.gen.pipelineState !== 'idle';
      let reason: string | undefined;
      if (action === 'cancel') reason = generating ? undefined : t('palette.not_running');
      else if (!project.documentActive) reason = t('palette.no_document');
      else if (project.loading || generating) reason = t('palette.busy');
      else if (action === 'generate')
        reason =
          project.geometryView && project.data.operations.length
            ? undefined
            : t('palette.geometry_required');
      else if (action === 'warnings')
        reason = project.gen.generated ? undefined : t('palette.no_output');
      else if (!project.gen.generated || (action === 'export-back' && !project.gen.generatedBack))
        reason = t('palette.no_output');
      else if (info.stale) reason = t('palette.stale');
      else reason = exportProblem(info, action === 'export-back' ? 'back' : 'front');
      return {
        token: `${project.documentEpoch}:${project.historyVersion}:${generating}:${identity(project.gen.generated)}:${identity(project.gen.generatedBack)}:${JSON.stringify(info)}`,
        reason,
      };
    };
    const owners: [string, () => void | Promise<void>][] = [
      ['generate', run],
      ['cancel', cancel],
      ['warnings', warnings],
      ['export-front', () => download('front')],
      ['export-back', () => download('back')],
    ];
    const dispose = owners.map(([id, run]) =>
      paletteDispatch.register(id, { state: () => state(id), run }),
    );
    return () => dispose.forEach((fn) => fn());
  });
}
