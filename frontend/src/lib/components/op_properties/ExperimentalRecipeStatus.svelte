<script lang="ts">
  import type { OpEntry } from '../../state/op_types';
  import { experimentalModified } from '../../state/experimental-recipe';
  import ExperimentalBadge from '../ExperimentalBadge.svelte';
  import TrialProvenance from '../TrialProvenance.svelte';
  let { op }: { op: OpEntry } = $props();
</script>

{#if op.appliedCutting?.experimental_recipe}
  <ExperimentalBadge modified={experimentalModified(op)} />
  <TrialProvenance source={op.appliedCutting.experimental_recipe.source} />
{/if}
