import { project } from './project.svelte';
import { workspace } from './workspace.svelte';
import { paletteDispatch, type ActionState } from './palette-dispatch';
import { openAny, addDrawing, saveProject } from '../services/file_ops';
import { openRecentProject } from '../services/workspace-session.svelte';
import { effectiveModes } from './tool_family';
import { t } from '../i18n';
import type { SidebarPane } from './sidebar-pane';
export interface PaletteShell {
  newDocument: () => Promise<unknown>;
  closeDocument: () => Promise<unknown>;
  navigate: (tab: 'project' | 'machine' | 'tools' | 'settings' | 'help' | 'about') => void;
  pane: (pane: '2d' | '3d') => void;
  sidebar: (pane: SidebarPane) => void;
}
export function paletteBusy() {
  return project.loading || project.gen.pipelineState !== 'idle';
}
export function paletteContext(): ActionState {
  return {
    token: `${project.documentEpoch}:${project.historyVersion}:${project.documentActive}:${project.sel.selectedOpId}:${[...project.sel.selectedObjects].join(',')}`,
    reason: paletteBusy() ? t('palette.busy') : undefined,
  };
}
export function bindPaletteShell(shell: PaletteShell) {
  const disposers: (() => void)[] = [];
  function bind(
    id: string,
    run: () => void | Promise<unknown>,
    requirement?: () => string | undefined,
  ) {
    disposers.push(
      paletteDispatch.register(id, {
        state: () => {
          const state = paletteContext();
          return { ...state, reason: state.reason ?? requirement?.() };
        },
        run,
      }),
    );
  }
  const document = () => (project.documentActive ? undefined : t('palette.no_document'));
  const selected = () =>
    document() ??
    (project.data.operations.some((op) => op.id === project.sel.selectedOpId)
      ? undefined
      : t('palette.select_operation'));
  bind('new', shell.newDocument);
  bind('open', openAny);
  bind('import', addDrawing, document);
  bind('save', saveProject, document);
  bind('close', shell.closeDocument, document);
  bind(
    'undo',
    () => {
      project.undo();
    },
    () => document() ?? (project.canUndo() ? undefined : t('palette.no_history')),
  );
  bind(
    'redo',
    () => {
      project.redo();
    },
    () => document() ?? (project.canRedo() ? undefined : t('palette.no_history')),
  );
  const reveal = (pane: SidebarPane) => {
    shell.navigate('project');
    shell.sidebar(pane);
  };
  for (const pane of ['stock', 'layers', 'text', 'operations'] as const)
    bind(`sidebar-${pane}`, () => reveal(pane), document);
  bind('cad', () => reveal('layers'), document);
  for (const tab of ['machine', 'tools', 'settings', 'help', 'about'] as const)
    bind(`nav-${tab}`, () => shell.navigate(tab));
  for (const pane of ['2d', '3d'] as const)
    bind(
      `view-${pane}`,
      () => {
        shell.navigate('project');
        shell.pane(pane);
      },
      document,
    );
  for (const kind of ['profile', 'pocket', 'drill'] as const)
    bind(
      `add-${kind}`,
      () => {
        project.addOperation(kind);
        reveal('operations');
      },
      () => {
        if (document()) return document();
        const modes = effectiveModes(project.data.machine);
        if (
          !(kind === 'profile'
            ? modes.some((mode) => ['mill', 'laser', 'plasma'].includes(mode))
            : modes.includes('mill'))
        )
          return t('palette.incompatible');
        const objects = project.geometryView?.object_meta ?? [];
        const ids = project.sel.selectedObjects;
        const candidates = ids.size ? objects.filter((object) => ids.has(object.id)) : objects;
        return candidates.length &&
          (kind !== 'pocket' || candidates.some((object) => object.closed))
          ? undefined
          : t('palette.geometry_required');
      },
    );
  bind(
    'duplicate',
    () => {
      project.duplicateOperation(project.sel.selectedOpId!);
      reveal('operations');
    },
    selected,
  );
  bind('delete', () => project.removeOperation(project.sel.selectedOpId!), selected);
  disposers.push(
    paletteDispatch.register('recent', {
      state: (path) => {
        const state = paletteContext();
        return {
          ...state,
          reason:
            state.reason ??
            (workspace.get().recent_projects.some((recent) => recent.path === path)
              ? undefined
              : t('palette.changed')),
        };
      },
      run: (path) => openRecentProject(path!),
    }),
  );
  return () => disposers.forEach((dispose) => dispose());
}
