// Local pack authoring; cutting mathematics and physical qualification stay elsewhere.
import type { components } from '../api/generated';
import type { SourcePack } from './feeds-speeds-sources';
type S = components['schemas'];
export type LabCapture = S['FsLabCapture'];
export type LabMetadata = S['FsLabPreset'];
export type LabReview = S['FsLabReview'];
export const LAB_PACK_ID = 'sorbonne-fablab';
export const LAB_POLICY = 'sorbonne-local-validation-1';
export function newestLabPack(packs: SourcePack[]): SourcePack | undefined {
  return packs
    .filter((p) => p.manifest.identity.id === LAB_PACK_ID)
    .sort((a, b) => b.manifest.identity.revision.localeCompare(a.manifest.identity.revision))[0];
}
export function currentLocalPacks(packs: SourcePack[]): SourcePack[] {
  const head = newestLabPack(packs);
  return packs.filter((p) => p.manifest.identity.id !== LAB_PACK_ID || p === head);
}
// Source packs are validated JSON data. JSON cloning also accepts Svelte proxy-backed props.
function cloneJson<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}
function stableJson(value: unknown): string {
  return JSON.stringify(value, (_key, item: unknown) =>
    item !== null && typeof item === 'object' && !Array.isArray(item)
      ? Object.fromEntries(
          Object.entries(item)
            .filter(([, value]) => value !== null && value !== undefined)
            .sort(([a], [b]) => a.localeCompare(b)),
        )
      : item,
  );
}
function exact(n: number): S['FsRange'] {
  return { min: n, max: n };
}
export function captureScope(c: LabCapture): S['FsProfileScope'] {
  const r = c.request;
  if (!r.machine.identity || !r.material.identity || !r.tool.identity)
    throw new Error('Stable machine, material and tool identities are required');
  if (
    !r.operation.doc_mm ||
    ![c.rpm, c.feed_mm_min, c.plunge_mm_min, r.tool.diameter_mm, r.tool.flute_count].every(
      (v) => Number.isFinite(v) && v > 0,
    )
  )
    throw new Error('Cutting values, DOC, diameter and flutes must be positive');
  return {
    machine: { kind: 'exact', identity: r.machine.identity },
    material: { kind: 'exact', identity: r.material.identity },
    tool: r.tool.identity,
    cutting_family: r.tool.cutting_family,
    diameter_mm: exact(r.tool.diameter_mm),
    flute_count: r.tool.flute_count,
    operation: r.operation.kind,
    doc_mm: exact(r.operation.doc_mm),
    entry: r.operation.entry,
    thickness_mm: r.material.thickness_mm ? exact(r.material.thickness_mm) : null,
    engagement:
      r.operation.engagement.kind === 'known_full_width'
        ? { kind: 'full_width' }
        : r.operation.engagement.kind === 'nominal'
          ? { kind: 'nominal', ratio: exact(r.operation.engagement.ratio) }
          : { kind: 'unknown_safe' },
  };
}
export function closePresets(pack: SourcePack | undefined, c: LabCapture) {
  const scope = captureScope(c);
  return (pack?.records ?? [])
    .filter((r) => r.lab && !r.lab.deleted)
    .filter((r) => {
      const p = pack!.dataset.profile_set.profiles.find(
        (p) => p.identity.id === r.profile.id && p.identity.revision === r.profile.revision,
      )!;
      const a = { ...p.scope, doc_mm: null },
        b = { ...scope, doc_mm: null };
      return (
        stableJson(a) === stableJson(b) &&
        Math.abs((p.scope.doc_mm?.max ?? 0) - (scope.doc_mm?.max ?? 0)) <=
          Math.max(0.01, (scope.doc_mm?.max ?? 0) * 0.05)
      );
    });
}
export type CaptureForm = {
  name: string;
  state: 'draft' | 'tested';
  tested_at: string;
  operator: string;
  job_reference: string;
  observations: string;
  detailed_notes: string;
  physically_tested: boolean;
  no_critical_failure: boolean;
};
function emptyPack(version: string): SourcePack {
  return {
    manifest: {
      identity: { id: LAB_PACK_ID, revision: version },
      schema_version: 2,
      name: 'Sorbonne Fablab',
      publisher: 'Sorbonne Fablab local operators',
      origin: 'Operator-authored local machining records',
      license: 'Private local records; no public redistribution license granted',
      license_url: null,
      source_url: null,
      attribution: 'Retain operator and trial provenance; no public license selected',
      visibility: 'private',
      distribution: 'local_only',
      redistributable: false,
      transformation_permitted: false,
      default_evidence: 'theoretical',
      notes: 'Local knowledge only. Physical-use and validation claims are human attestations.',
    },
    dataset: {
      format_version: 1,
      description: 'Sorbonne Fablab local machining presets',
      license: 'Private local records; no public redistribution license granted',
      machines: [],
      materials: [],
      tools: [],
      profile_set: {
        schema_version: 1,
        identity: { id: LAB_PACK_ID, revision: version },
        machine_families: [],
        material_families: [],
        profiles: [],
      },
    },
    records: [],
  };
}
function revisionPack(previous: SourcePack | undefined, now: string) {
  const version = Date.parse(now).toString().padStart(16, '0');
  if (!Number.isFinite(Date.parse(now))) throw new Error('Invalid revision date');
  if (
    previous &&
    (previous.manifest.identity.id !== LAB_PACK_ID ||
      previous.manifest.visibility !== 'private' ||
      previous.manifest.distribution !== 'local_only')
  )
    throw new Error('Only the private Sorbonne pack may be authored');
  if (previous && version <= previous.manifest.identity.revision)
    throw new Error('Revision must follow the installed revision; retry');
  const p = previous ? cloneJson(previous) : emptyPack(version);
  p.manifest.schema_version = 2;
  p.manifest.identity.revision = version;
  p.dataset.profile_set.identity.revision = version;
  return p;
}
function retainPrevious(p: SourcePack, record: SourcePack['records'][number]) {
  const old = p.dataset.profile_set.profiles.find(
    (v) => v.identity.id === record.profile.id && v.identity.revision === record.profile.revision,
  );
  if (!old || !record.lab) throw new Error('Incomplete local preset');
  record.lab_history = [
    ...(record.lab_history ?? []),
    { profile: cloneJson(old), metadata: cloneJson(record.lab) },
  ];
  return old;
}
function addCatalog<T extends { identity: S['FsIdentity'] }>(rows: T[], entry: T) {
  const old = rows.find((r) => r.identity.id === entry.identity.id);
  if (
    old &&
    stableJson({ ...old, label: undefined, notes: undefined }) !==
      stableJson({ ...entry, label: undefined, notes: undefined })
  )
    throw new Error(
      'Catalog identity already describes different context; use a distinct physical identity',
    );
  if (!old) rows.push(entry);
  else rows[rows.indexOf(old)] = entry;
}
export function saveCapturedPreset(
  previous: SourcePack | undefined,
  c: LabCapture,
  form: CaptureForm,
  now: string,
  id: string,
  updateId?: string,
): SourcePack {
  const scope = captureScope(c);
  if (!form.name.trim()) throw new Error('Preset name is required');
  if (
    form.state === 'tested' &&
    (!form.physically_tested ||
      !form.no_critical_failure ||
      !/^\d{4}-\d{2}-\d{2}$/.test(form.tested_at) ||
      !Number.isFinite(Date.parse(form.tested_at)) ||
      new Date(form.tested_at).toISOString().slice(0, 10) !== form.tested_at ||
      form.tested_at > now.slice(0, 10))
  )
    throw new Error('Confirm a dated successful physical cut before marking Tested');
  if (
    form.state === 'tested' &&
    (scope.engagement.kind === 'unknown_safe' || scope.entry === 'unknown')
  )
    throw new Error(
      'Engagement and entry must be known for a Tested preset; save Draft or recapture after confirming context',
    );
  const p = revisionPack(previous, now),
    version = p.manifest.identity.revision;
  const record = updateId ? p.records.find((r) => r.profile.id === updateId) : undefined;
  if (updateId && !record) throw new Error('Preset changed; reopen the dialog');
  if (record) {
    if (!closePresets(previous, c).some((r) => r.profile.id === updateId))
      throw new Error('Update requires the same machining context');
    retainPrevious(p, record);
    p.dataset.profile_set.profiles = p.dataset.profile_set.profiles.filter(
      (v) => v.identity.id !== updateId,
    );
  } else if (p.records.some((r) => r.profile.id === id))
    throw new Error('Preset identity already exists');
  const identity = { id: record?.profile.id ?? id, revision: version };
  const lab: LabMetadata = {
    name: form.name.trim(),
    state: form.state,
    created_at: record?.lab?.created_at ?? now,
    updated_at: now,
    tested_at: form.state === 'tested' ? form.tested_at : null,
    physically_tested: form.state === 'tested' && form.physically_tested,
    no_critical_failure: form.state === 'tested' && form.no_critical_failure,
    operator: form.operator,
    job_reference: form.job_reference,
    observations: form.observations,
    detailed_notes: form.detailed_notes,
    finishing_context: c.finishing_context,
    enabled: form.state === 'tested',
    deleted: false,
    review: null,
  };
  const profile: S['FsProfile'] = {
    identity,
    category: form.state === 'tested' ? 'exact_preset' : 'theoretical',
    evidence: form.state === 'tested' ? 'tested' : 'theoretical',
    aggressiveness: c.request.aggressiveness,
    scope,
    data: {
      kind: 'exact_values',
      rpm: c.rpm,
      feed_mm_min: c.feed_mm_min,
      plunge_mm_min: c.plunge_mm_min,
      doc_mm: c.request.operation.doc_mm!,
      stepover_ratio:
        c.request.operation.engagement.kind === 'nominal'
          ? c.request.operation.engagement.ratio
          : null,
    },
    reductions: [],
    source: {
      reference: `${LAB_PACK_ID}/${identity.id}@${identity.revision}`,
      author: form.operator.trim() || 'Sorbonne Fablab operator (not named)',
      date: form.tested_at || now.slice(0, 10),
      permission_or_license: p.manifest.license,
      notes: form.observations,
      evidence_record:
        form.state === 'tested'
          ? {
              trial_reference: `Physical cut ${form.tested_at}; ${form.job_reference || c.operation_name}`,
              validation_criteria_reference: null,
            }
          : null,
    },
  };
  addCatalog(p.dataset.machines, {
    identity: c.request.machine.identity!,
    label: c.machine_name.trim() || c.request.machine.identity!.id,
    notes: 'Exact machine context',
  });
  addCatalog(p.dataset.materials, {
    identity: c.request.material.identity!,
    label: c.request.material.identity!.id,
    notes: 'Exact material identity; grade is not generalized',
  });
  addCatalog(p.dataset.tools, {
    identity: c.request.tool.identity!,
    label: c.tool_name.trim() || c.request.tool.identity!.id,
    cutting_family: c.request.tool.cutting_family,
    diameter_mm: c.request.tool.diameter_mm,
    flute_count: c.request.tool.flute_count,
    max_rpm: c.request.tool.max_rpm ?? null,
    notes: 'Captured physical tool; helix/coating not inferred from name',
  });
  p.dataset.profile_set.profiles.push(profile);
  if (record) {
    record.profile = identity;
    record.lab = lab;
  } else
    p.records.push({
      profile: identity,
      source_record_id: identity.id,
      original_classification: `${c.machine_name} · ${c.tool_name} · ${c.request.operation.kind}`,
      material_taxon: null,
      tool: {
        family: c.request.tool.cutting_family,
        helix: null,
        material: null,
        coating: null,
        shape: null,
      },
      lab,
      lab_history: [],
    });
  return p;
}
export function revisePreset(
  previous: SourcePack,
  id: string,
  now: string,
  change: { notes?: string; enabled?: boolean; deleted?: boolean; review?: LabReview },
): SourcePack {
  const p = revisionPack(previous, now),
    r = p.records.find((r) => r.profile.id === id);
  if (!r?.lab) throw new Error('Preset is missing');
  const profile = retainPrevious(p, r),
    lab = r.lab;
  if (change.review) {
    if (lab.state !== 'tested' || lab.deleted)
      throw new Error('Only an active Tested record can be promoted');
    const q = change.review;
    if (
      q.policy !== LAB_POLICY ||
      !q.notes.trim() ||
      !q.acceptable_finish_and_behavior ||
      !q.no_overheating_chatter_or_overload ||
      !q.suitable_for_lab_use
    )
      throw new Error('Confirm all validation criteria and add review notes');
    lab.state = 'validated';
    lab.review = q;
    profile.evidence = 'validated';
    profile.source.evidence_record!.validation_criteria_reference = LAB_POLICY;
  }
  if (change.notes !== undefined) {
    lab.observations = change.notes;
    profile.source.notes = change.notes;
  }
  if (change.enabled !== undefined) {
    if (lab.state === 'draft' && change.enabled)
      throw new Error('Draft presets cannot provide recommendations');
    lab.enabled = change.enabled;
  }
  if (change.deleted !== undefined) {
    lab.deleted = change.deleted;
    lab.enabled = !change.deleted && lab.state !== 'draft';
  }
  lab.updated_at = now;
  profile.identity.revision = p.manifest.identity.revision;
  r.profile = cloneJson(profile.identity);
  return p;
}

export function labHasRevision(pack: SourcePack, revision: string): boolean {
  return (
    pack.manifest.identity.id === LAB_PACK_ID &&
    (pack.manifest.identity.revision === revision ||
      pack.records.some(
        (r) =>
          r.profile.revision === revision ||
          r.lab_history?.some((h) => h.profile.identity.revision === revision),
      ))
  );
}
