<script lang="ts">
  import { untrack } from 'svelte';
  import { project } from '../state/project.svelte';
  import { workspace } from '../state/workspace.svelte';
  import { coerceGenerationPost } from '../state/generation-context';
  import { qualificationKey, verificationInput } from '../state/program-qualification';
  import { QualificationRun } from '../sim/qualification-run';
  const run = new QualificationRun(
    () =>
      new Worker(new URL('../sim/qualification.worker.ts', import.meta.url), { type: 'module' }),
  );
  const post = $derived(
    coerceGenerationPost(project.data.machine.gcodeDialect ?? workspace.get().last_post_processor),
  );
  const key = $derived(qualificationKey(project, post));
  $effect(() => {
    const requested = key;
    untrack(() => {
      run.cancel();
      if (!requested) return;
      try {
        // Svelte proxies cannot be structured-cloned. Snapshot once per run.
        const input = JSON.parse(JSON.stringify(verificationInput(project, requested)));
        run.start(input, (state) => {
          if (qualificationKey(project, post) === requested) project.gen.qualification = state;
        });
      } catch (error) {
        project.gen.qualification = {
          key: requested,
          phase: 'unavailable',
          message: String(error),
        };
      }
    });
    return () => run.cancel();
  });
</script>
