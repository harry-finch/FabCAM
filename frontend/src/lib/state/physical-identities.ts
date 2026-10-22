/** Assign identity at creation/load boundaries, never during generation or rendering.
 * Keep stored workshop identities and unrelated library fields intact. Old reference-pack
 * bindings are inert metadata; they do not establish physical compatibility.
 */
export function withPhysicalIdentity<T extends { physicalId?: string }>(
  value: T,
  kind: 'machine' | 'tool',
): T & { physicalId?: string } {
  if (!value || typeof value !== 'object' || value.physicalId !== undefined) return value;
  const stored = (value as T & { fsLocalIdentity?: { id?: unknown } }).fsLocalIdentity?.id;
  const physicalId =
    typeof stored === 'string' && stored.trim() ? stored : `lab-${kind}-${crypto.randomUUID()}`;
  return { ...value, physicalId };
}
