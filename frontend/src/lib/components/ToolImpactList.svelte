<script lang="ts">
  import ToolCopyComparison from './ToolCopyComparison.svelte';
  import { t } from '../i18n';
  import type { toolLibraryImpacts } from '../state/tool-library-review';
  let { impacts }: { impacts: ReturnType<typeof toolLibraryImpacts> } = $props();
</script>

{#if impacts.length}<ul class="impact-list">
    {#each impacts as impact (impact.id)}<li>
        <strong>#{impact.id}</strong>
        <ToolCopyComparison
          before={{
            label: t('uxb.before'),
            name: impact.before,
            kind: impact.kindBefore,
            diameter: impact.diameterBefore,
          }}
          after={{
            label: t('uxb.after'),
            name: impact.after,
            kind: impact.kindAfter,
            diameter: impact.diameterAfter,
          }}
        />
        <p>
          {t('ux6b.operations')}: {impact.operations.join(', ') || t('ux6b.no_operations')}
        </p>
      </li>{/each}
  </ul>{/if}

<style>
  .impact-list {
    max-height: 220px;
    overflow: auto;
  }
</style>
