<script lang="ts">
  import { project } from '../state/project.svelte';
  import { setMaterialCommand } from '../state/commands';
  import MaterialPicker from './MaterialPicker.svelte';
</script>

<MaterialPicker
  value={project.data.material}
  onchange={(value) => project.history.exec(setMaterialCommand(value), project.target())}
/>
