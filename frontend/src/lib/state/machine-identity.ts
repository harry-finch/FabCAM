/** Presentation of the persisted name, never a machine-validation verdict.
 * Profiles currently have no short-name or validation-status metadata. */
export function machineIdentity(name: string | null | undefined, fallback: string) {
  const fullName = name?.trim() || fallback;
  const qualifierPattern =
    /software[ -]only|unvalidated|not validated|not for (?:physical machining|machining|production)|simulation[ -]only|non valid[ée]e?|nicht validiert/gi;
  const limitations = fullName.match(qualifierPattern) ?? [];
  if (!limitations.length) return { fullName, shortName: fullName, limitation: '' };
  const parts = fullName.split(/\s+[—–]\s+/);
  // Only remove a fixture prefix when it explicitly carries a limitation.
  const prefixIsFixture = /^(?:FS|UX|CAM)\d[\w.]*\s+/i.test(parts[0]);
  const shortName =
    parts.length > 1 && prefixIsFixture && parts[0].match(qualifierPattern)
      ? parts.slice(1).join(' — ')
      : fullName
          .replace(qualifierPattern, '')
          .replace(/\s+/g, ' ')
          .replace(/^[\s—–:]+|[\s—–:]+$/g, '');
  return {
    fullName,
    shortName: shortName || fullName,
    limitation: [...new Set(limitations)].join(' · '),
  };
}
