import type { components } from '../api/generated';
type Project = components['schemas']['Project'];
/** Report the actual missing bindings, including legacy context already resolved by buildProject. */
export function missingPresetIdentities(
  project: Project,
  toolId: number,
): ('machine' | 'material' | 'tool')[] {
  const missing: ('machine' | 'material' | 'tool')[] = [];
  if (!(project.fs_context?.machine_local_identity ?? project.fs_context?.machine_identity))
    missing.push('machine');
  if (!project.material) missing.push('material');
  if (
    !project.fs_context?.tools.some((t) => t.tool_id === toolId && (t.local_identity ?? t.identity))
  )
    missing.push('tool');
  return missing;
}
