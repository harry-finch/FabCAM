import type { ProjectState } from './project.svelte';

function capture<T, K extends keyof T>(source: T, keys: K[]): Pick<T, K> {
  return Object.fromEntries(keys.map((key) => [key, source[key]])) as Pick<T, K>;
}

/** Loading is serialized by file_ops. Keep references to the previous job until
 * replacement/conversion succeeds, including history and session identity. */
export async function commitDocumentLoad(
  p: ProjectState,
  apply: () => Promise<void>,
): Promise<void> {
  const data = capture(p.data, [
    'imports',
    'drawing',
    'serializedExtensions',
    'cornerRelief',
    'operations',
    'material',
    'fixtures',
    'textLayers',
    'reliefSources',
    'groupOpsByTool',
    'stock',
    'workOffset',
    'tools',
    'machine',
    'machineProfileId',
    'visibleLayers',
    'dirty',
  ]);
  const generated = capture(p.gen, [
    'generated',
    'generatedBack',
    'generatedVersion',
    'inputContextKey',
    'remainingVisible',
    'remainingOpId',
    'toolpath3dVisible',
    'toolpathCumLen',
    'toolpathTotalLen',
    'simDiagnostics',
    'qualification',
    'lastGenerateCachedCount',
    'lastGenerateOpCount',
  ]);
  const selection = capture(p.sel, [
    'selectedEntities',
    'selectedObjects',
    'selectedOpId',
    'selectedFixtureId',
    'selectedTextLayerId',
    'hoverSegment',
    'pickMode',
    'toolsDialogFocusId',
    'selectionAnchorObjectId',
  ]);
  const session = capture(p, [
    'documentActive',
    'documentName',
    'documentBaseline',
    'savedToProject',
    'activeProjectPath',
    'sourceFileStaleNotice',
    'playhead',
  ]);
  const history = p.history.checkpoint();
  try {
    await apply();
  } catch (error) {
    // Any request produced for the failed replacement is now obsolete as well.
    p.documentEpoch += 1;
    Object.assign(p.data, data);
    Object.assign(p.gen, generated);
    p.gen.endGenerate();
    p.gen.pipelineState = 'idle';
    Object.assign(p.sel, selection);
    Object.assign(p, session);
    p.history.restoreCheckpoint(history);
    const [{ resetPreviewCache }, { refreshSourceWatch }] = await Promise.all([
      import('./text_preview.svelte'),
      import('./import-ops'),
    ]);
    resetPreviewCache();
    void refreshSourceWatch(p);
    throw error;
  }
}
