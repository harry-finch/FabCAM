/** ADR-0029: manually authored inactive recipe; no automatic scaling or operation mutation. */
import { recipeProblems, type RecipeData } from './recipe-values';
import type { components } from '../api/generated';
import type { PresetRow } from './preset-library';
import type { MachineSettings, ToolEntry, MaterialIdentity } from './project-types';
import type { SourcePack } from './feeds-speeds-sources';
import { NORMALIZED_MATERIALS } from './feeds-speeds-sources';
import { effectiveModes, toolCompatibleWithAnyMode } from './tool_family';
import { referenceIdentity } from './cutting-identities';
import { revisionPack, addCatalog, isLabPackId } from './lab-presets';
type S = components['schemas'];
type Identity = S['FsIdentity'];
export type AdaptTarget = {
  machine: MachineSettings;
  tool?: ToolEntry;
  material?: MaterialIdentity;
};
export type AdaptProblem =
  | 'identity'
  | 'process'
  | 'machine'
  | 'material'
  | 'tool'
  | 'geometry'
  | 'rights'
  | 'source'
  | 'values'
  | 'limits';
const same = (a?: Identity | null, b?: Identity | null) =>
  !!a && !!b && a.id === b.id && a.revision === b.revision;
const complete = (v?: Identity | null) =>
  typeof v?.id === 'string' &&
  !!v.id.trim() &&
  typeof v.revision === 'string' &&
  !!v.revision.trim();
export function localCopyAttestation(row: PresetRow): boolean {
  return (
    !!row.record?.lab &&
    isLabPackId(row.pack.manifest.identity.id) &&
    row.pack.manifest.visibility === 'private' &&
    row.pack.manifest.distribution === 'local_only' &&
    !row.pack.manifest.transformation_permitted
  );
}
function selectorMatches(
  s: S['FsIdentitySelector'],
  id: Identity | undefined | null,
  families: S['FsFamilyMembership'][],
) {
  return (
    s.kind === 'any' ||
    (s.kind === 'exact'
      ? same(s.identity, id)
      : families.some((f) => f.id === s.id && f.members.some((m) => same(m, id))))
  );
}
function taxonMatches(taxon: string, id: Identity) {
  return (
    id.revision === '1' &&
    NORMALIZED_MATERIALS.some((m) => same(m.identity, id)) &&
    (id.id === `material/${taxon}` || id.id.startsWith(`material/${taxon}.`))
  );
}
export function adaptedToolFamily(tool: ToolEntry): S['FsCuttingFamily'] | undefined {
  return tool.kind === 'endmill'
    ? 'flat_endmill'
    : tool.kind === 'compression'
      ? 'compression'
      : tool.kind === 'v_bit'
        ? 'v_bit'
        : undefined;
}
function contextIssues(
  row: PresetRow,
  target: AdaptTarget,
  ownershipConfirmed: boolean,
  data: RecipeData = row.profile.data,
): AdaptProblem[] {
  const { machine, tool, material } = target;
  const out: AdaptProblem[] = [];
  if (!row.enabled || row.record?.lab?.deleted || !row.record) out.push('source');
  if (
    !row.pack.manifest.transformation_permitted &&
    !(localCopyAttestation(row) && ownershipConfirmed)
  )
    out.push('rights');
  if (
    !complete(machine.fsLocalIdentity) ||
    !complete(tool?.fsLocalIdentity) ||
    !complete(material) ||
    same(machine.fsLocalIdentity, referenceIdentity(machine.fsIdentity)) ||
    same(tool?.fsLocalIdentity, referenceIdentity(tool?.fsIdentity))
  )
    out.push('identity');
  const modes = effectiveModes(machine);
  if (!modes.includes('mill') || (tool && !toolCompatibleWithAnyMode(tool.kind, modes)))
    out.push('process');
  const sourceMachine = row.record?.lab
    ? machine.fsLocalIdentity
    : referenceIdentity(machine.fsIdentity);
  if (
    !selectorMatches(
      row.profile.scope.machine,
      sourceMachine,
      row.pack.dataset.profile_set.machine_families,
    ) &&
    !row.record?.adaptation?.machines.some((id) => same(id, sourceMachine))
  )
    out.push('machine');
  if (
    !material ||
    !(
      selectorMatches(
        row.profile.scope.material,
        material,
        row.pack.dataset.profile_set.material_families,
      ) ||
      [row.record?.material_taxon, ...(row.record?.adaptation?.material_taxa ?? [])].some(
        (t) => t && taxonMatches(t, material),
      )
    )
  )
    out.push('material');
  if (
    !tool ||
    !adaptedToolFamily(tool) ||
    adaptedToolFamily(tool) !== row.profile.scope.cutting_family
  )
    out.push('tool');
  const diameter = tool ? tool.diameter - (tool.wearOffsetMm ?? 0) : NaN;
  if (
    !tool ||
    !Number.isFinite(diameter) ||
    diameter <= 0 ||
    !Number.isInteger(tool.flutes) ||
    tool.flutes <= 0 ||
    (tool.fluteLengthMm != null &&
      (!Number.isFinite(tool.fluteLengthMm) ||
        tool.fluteLengthMm <= 0 ||
        tool.fluteLengthMm < data.doc_mm))
  )
    out.push('geometry');
  return [...new Set(out)];
}
/** Different source applicability is review information for an inactive recipe, not permission to apply it. */
export function adaptationWarnings(row: PresetRow, target: AdaptTarget): AdaptProblem[] {
  return contextIssues(row, target, true).filter((p) => p === 'machine' || p === 'material');
}
export function adaptationProblems(
  row: PresetRow,
  target: AdaptTarget,
  ownershipConfirmed: boolean,
  data: RecipeData = row.profile.data,
): AdaptProblem[] {
  return [
    ...contextIssues(row, target, ownershipConfirmed, data).filter(
      (p) => p !== 'machine' && p !== 'material',
    ),
    ...recipeProblems(data, row.profile.scope.operation, target.machine),
  ];
}
export type AdaptDifference = {
  field: 'machine' | 'material' | 'tool' | 'diameter' | 'flutes';
  before: string;
  after: string;
};
export function adaptationDifferences(row: PresetRow, target: AdaptTarget): AdaptDifference[] {
  const result: AdaptDifference[] = [];
  const label = (id?: Identity | null) => (id ? `${id.id}@${id.revision}` : '—');
  for (const field of ['machine', 'material', 'tool'] as const) {
    const original =
      field === 'tool'
        ? row.profile.scope.tool
        : row.profile.scope[field].kind === 'exact'
          ? (row.profile.scope[field] as { kind: 'exact'; identity: Identity }).identity
          : undefined;
    const next =
      field === 'machine'
        ? target.machine.fsLocalIdentity
        : field === 'material'
          ? target.material
          : target.tool?.fsLocalIdentity;
    if (!same(original, next))
      result.push({
        field,
        before: original ? label(original) : JSON.stringify(row.profile.scope[field]),
        after: label(next),
      });
  }
  const d = row.profile.scope.diameter_mm;
  const diameter = target.tool ? target.tool.diameter - (target.tool.wearOffsetMm ?? 0) : undefined;
  if (diameter !== d.min || diameter !== d.max)
    result.push({
      field: 'diameter',
      before: `${d.min}–${d.max} mm`,
      after: `${diameter ?? '—'} mm`,
    });
  if (target.tool?.flutes !== row.profile.scope.flute_count)
    result.push({
      field: 'flutes',
      before: String(row.profile.scope.flute_count ?? '—'),
      after: String(target.tool?.flutes ?? '—'),
    });
  return result;
}
export function duplicatePresetAsDraft(
  previous: SourcePack,
  row: PresetRow,
  target: AdaptTarget,
  form: {
    name: string;
    reviewed: boolean;
    ownershipConfirmed: boolean;
    notes: string;
    data?: RecipeData;
  },
  now: string,
  id: string,
): SourcePack {
  const data = form.data ?? row.profile.data;
  const problems = adaptationProblems(row, target, form.ownershipConfirmed, data);
  if (problems.length) throw new Error(`Cannot adapt preset: ${problems.join(', ')}`);
  if (!form.reviewed || !form.name.trim())
    throw new Error('Review copied values and provide a name');
  const p = revisionPack(previous, now);
  if (p.records.some((r) => r.profile.id === id)) throw new Error('Duplicate preset identity');
  const tool = target.tool!;
  const material = target.material!;
  const machine = target.machine;
  const toolLimit = previous.dataset.tools.find((entry) =>
    same(entry.identity, tool.fsLocalIdentity),
  )?.max_rpm;
  if (toolLimit != null && data.rpm > toolLimit)
    throw new Error('Recipe RPM exceeds the recorded tool limit');
  const clone = <T>(v: T): T => JSON.parse(JSON.stringify(v)) as T;
  const profile = clone(row.profile);
  const source = {
    pack: clone(row.pack.manifest.identity),
    manifest: clone(row.pack.manifest),
    targetMachineReference: referenceIdentity(machine.fsIdentity),
    targetToolReference: referenceIdentity(tool.fsIdentity),
    profile: clone(row.profile.identity),
    sourceRecordId: row.record!.source_record_id,
    originalCategory: row.profile.category,
    originalEvidence: row.profile.evidence,
    reference: row.profile.source.reference,
    author: row.profile.source.author,
    date: row.profile.source.date,
    license: row.profile.source.permission_or_license,
    attribution: row.pack.manifest.attribution,
    notes: row.profile.source.notes,
    scope: clone(row.profile.scope),
    data: clone(row.profile.data),
    reductions: clone(row.profile.reductions),
    changes: adaptationDifferences(row, target),
    authoredData: clone(data),
    applicabilityWarnings: adaptationWarnings(row, target),
    authoring: 'manual-inactive-recipe-1',
    ownershipConfirmed: localCopyAttestation(row) && form.ownershipConfirmed,
  };
  profile.identity = { id, revision: p.manifest.identity.revision };
  profile.data = clone(data);
  profile.category = 'theoretical';
  profile.evidence = 'theoretical';
  profile.scope = {
    ...profile.scope,
    machine: { kind: 'exact', identity: clone(machine.fsLocalIdentity!) },
    material: { kind: 'exact', identity: { id: material.id, revision: material.revision } },
    tool: clone(tool.fsLocalIdentity!),
    diameter_mm: {
      min: tool.diameter - (tool.wearOffsetMm ?? 0),
      max: tool.diameter - (tool.wearOffsetMm ?? 0),
    },
    flute_count: tool.flutes,
    doc_mm: { min: data.doc_mm, max: data.doc_mm },
    thickness_mm: null,
    engagement:
      profile.scope.engagement.kind === 'nominal' && data.stepover_ratio != null
        ? { kind: 'nominal', ratio: { min: data.stepover_ratio, max: data.stepover_ratio } }
        : profile.scope.engagement,
  };
  // Reference reductions are not silently recalculated or represented as target-qualified rules.
  profile.reductions = [];
  profile.source = {
    ...profile.source,
    reference: `${source.pack.id}@${source.pack.revision}/${source.profile.id}@${source.profile.revision}`,
    evidence_record: null,
    notes: `Copied source (not target qualification):\n${JSON.stringify(source, null, 2)}`,
  };
  addCatalog(p.dataset.machines, {
    identity: clone(machine.fsLocalIdentity!),
    label: machine.name?.trim() || machine.fsLocalIdentity!.id,
    notes: 'Exact machine context',
  });
  addCatalog(p.dataset.materials, {
    identity: { id: material.id, revision: material.revision },
    label: material.label || material.id,
    notes: 'Exact material identity; grade is not generalized',
  });
  addCatalog(p.dataset.tools, {
    identity: clone(tool.fsLocalIdentity!),
    label: tool.name || tool.fsLocalIdentity!.id,
    cutting_family: profile.scope.cutting_family,
    diameter_mm: profile.scope.diameter_mm.min,
    flute_count: tool.flutes,
    max_rpm: p.dataset.tools.find((t) => same(t.identity, tool.fsLocalIdentity))?.max_rpm ?? null,
    notes: 'Captured physical tool; helix/coating not inferred from name',
  });
  p.dataset.profile_set.profiles.push(profile);
  p.records.push({
    profile: clone(profile.identity),
    source_record_id: id,
    original_classification: form.name.trim(),
    material_taxon: null,
    tool: {
      family: profile.scope.cutting_family,
      helix: null,
      material: null,
      coating: null,
      shape: null,
    },
    lab: {
      name: form.name.trim(),
      state: 'draft',
      created_at: now,
      updated_at: now,
      tested_at: null,
      physically_tested: false,
      no_critical_failure: false,
      operator: '',
      job_reference: '',
      observations: form.notes,
      detailed_notes:
        'Source values, licensing and context are retained in source provenance; manual values and context changes are recorded; no physical evidence is inherited.',
      finishing_context:
        'Manually authored recipe requires a new physical trial; no parameters were automatically calculated or applied.',
      enabled: false,
      deleted: false,
      review: null,
    },
    lab_history: [],
  });
  return p;
}
