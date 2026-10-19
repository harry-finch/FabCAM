import type { GenerateResponse, ToolpathSegment } from '../api/types';
import type { ToolEntry } from '../state/project-types';

export type SimulationTool = ToolEntry & { simulationDefinition: Record<string, unknown> };
/** Immutable generated definitions, never look up a live project tool by operation. */
export function programTools(program: GenerateResponse, requireResolved = false) {
  const provenance = program.tool_provenance;
  if (!provenance || provenance.revision !== 1 || provenance.authority === 'unavailable')
    throw new Error('Tool provenance unavailable — regenerate with a compatible runtime');
  if (requireResolved && provenance.authority !== 'resolved')
    throw new Error(provenance.diagnostics.join('; ') || 'Emitted tool intent is not qualified');
  const tools = new Map<number, SimulationTool>();
  const ids = new Set<number>();
  for (const binding of provenance.bindings) {
    if (
      !Number.isInteger(binding.reference) ||
      binding.reference <= 0 ||
      tools.has(binding.reference) ||
      ids.has(binding.project_tool_id)
    )
      throw new Error('Duplicate or invalid generated tool binding');
    if (binding.tool.id !== binding.project_tool_id)
      throw new Error('Generated tool identity disagrees with its definition');
    ids.add(binding.project_tool_id);
    const wire = binding.tool;
    // Display/driver shape only. WASM receives the exact complete wire definition.
    const display: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(wire))
      display[key.replace(/_([a-z])/g, (_, letter: string) => letter.toUpperCase())] = value;
    display.kind = wire.kind === 'kegel' ? 'cone' : wire.kind;
    display.formProfileMm = wire.form_profile_mm?.map((v) => ({ zMm: v.z_mm, rMm: v.r_mm }));
    tools.set(binding.reference, {
      ...display,
      id: binding.reference,
      simulationDefinition: wire,
    } as unknown as SimulationTool);
  }
  const resolve = (segment: ToolpathSegment): SimulationTool => {
    const tool = segment.tool_ref == null ? undefined : tools.get(segment.tool_ref);
    if (!tool) throw new Error('Movement has unknown tool provenance');
    return tool;
  };
  for (const segment of program.toolpath) resolve(segment);
  const cutting = program.toolpath.filter((s) => s.kind !== 'rapid' && s.kind !== 'retract');
  const used = cutting.length ? cutting : program.toolpath;
  let diameter = Infinity;
  for (const segment of used) {
    const tool = resolve(segment);
    diameter = Math.min(diameter, Math.max(0.01, tool.diameter - (tool.wearOffsetMm ?? 0)));
  }
  if (!Number.isFinite(diameter)) throw new Error('No complete toolpath to simulate');
  return { tools, resolve, diameter };
}
