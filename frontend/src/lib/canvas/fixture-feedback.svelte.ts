import { computeViewportTransform } from './viewport';
import type { BBox } from '../api/types';
import { fixtureAt } from './fixture-hit';
import { tick } from 'svelte';
import { fixtureFocus } from '../state/fixture-focus.svelte';
import { fixtureCollisions } from '../state/fixture-collisions';
import { generationContextKey } from '../state/generation-context';
import { workspace } from '../state/workspace.svelte';
import { FixtureInteraction, frameFixture } from './fixture-interaction';
import type { ProjectState } from '../state/project.svelte';
import type { ViewController } from './view-controller.svelte';

export function createFixtureTools(
  project: ProjectState,
  view: () => ViewController,
  canvas: () => HTMLCanvasElement,
  bounds: () => BBox | null,
  toData: (x: number, y: number) => { x: number; y: number } | null,
  activate: () => void,
) {
  const ids = $derived(
    new Set(
      fixtureCollisions(
        project.data.fixtures,
        project.gen.generated?.warnings ?? [],
        project.gen.simDiagnostics?.warnings ?? [],
        project.gen.inputContextKey !==
          generationContextKey(
            project,
            project.data.machine.gcodeDialect ?? workspace.get().last_post_processor,
          ),
      ).map((c) => c.fixtureId),
    ),
  );
  let lastSeq = 0;
  $effect(() => {
    if (fixtureFocus.seq === lastSeq) return;
    lastSeq = fixtureFocus.seq;
    const id = fixtureFocus.id;
    void tick().then(() =>
      requestAnimationFrame(() => {
        // The 2D pane may have been hidden (zero size) when Show was clicked.
        const f = project.data.fixtures.find((f) => f.id === id);
        const element = canvas(),
          box = bounds();
        // The backing canvas can still be zero-sized until ResizeObserver
        // paints the newly revealed pane. Use its live container dimensions.
        const width = element?.parentElement?.clientWidth ?? 0;
        const height = element?.parentElement?.clientHeight ?? 0;
        if (!f || !box || width <= 0 || height <= 0) return;
        const transform = computeViewportTransform(
          box,
          { w: width, h: height },
          { zoom: 1, panX: 0, panY: 0 },
        );
        Object.assign(
          view(),
          frameFixture(
            f,
            { scale: transform.baseScale, offX: transform.baseOffX, offY: transform.baseOffY },
            width,
            height,
          ),
        );
      }),
    );
  });
  const interaction = new FixtureInteraction(project, canvas, toData, activate);
  return {
    interaction,
    hit(x: number, y: number) {
      return fixtureAt(project.data.fixtures, x, y, project.sel.selectedFixtureId);
    },
    get ids() {
      return ids;
    },
  };
}
