import type { components } from '../api/generated';
type Identity = components['schemas']['FsIdentity'];
type Binding = { fsIdentity?: Identity | null; fsLocalIdentity?: Identity | null };

/** Run at creation/load boundaries, never while building a request or rendering. */
export function withLocalIdentity<T extends Binding>(
  value: T,
  kind: 'machine' | 'tool',
  inherited?: Identity | null,
  inheritedLocal?: Identity | null,
): T & Binding {
  if (!value || typeof value !== 'object') return value;
  const reference = value.fsIdentity !== undefined ? value.fsIdentity : inherited;
  if (value.fsLocalIdentity && (value.fsIdentity !== undefined || inherited === undefined))
    return value;
  // Materialize legacy context links at load time. A new machine/tool can then
  // never inherit an unrelated project's old reference just because its field is absent.
  return {
    ...value,
    ...(value.fsIdentity === undefined && inherited !== undefined ? { fsIdentity: inherited } : {}),
    fsLocalIdentity: value.fsLocalIdentity ??
      inheritedLocal ??
      reference ?? {
        id: `lab-${kind}-${crypto.randomUUID()}`,
        revision: '1',
      },
  };
}

export function referenceIdentity(value?: Identity | null): Identity | null {
  // The former Enable presets button stored a local id in the reference field.
  return value?.id.startsWith('lab-machine-') || value?.id.startsWith('lab-tool-')
    ? null
    : (value ?? null);
}
