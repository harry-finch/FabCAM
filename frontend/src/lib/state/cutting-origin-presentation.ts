/** Presentation only: unknown/legacy origins must never inherit a recommended label. */
export function cuttingOriginPresentation(origins: (string | null | undefined)[]) {
  if (origins.length && origins.every((o) => o === 'recommendation'))
    return { shared: 'recommendation', show: (_: string | null | undefined) => false };
  if (origins.length && origins.every((o) => o === 'manual'))
    return { shared: 'manual', show: (_: string | null | undefined) => false };
  if (
    origins.some((o) => o === 'recommendation') &&
    origins.every((o) => o === 'recommendation' || o === 'manual')
  )
    return { shared: 'mixed', show: (origin: string | null | undefined) => origin === 'manual' };
  return {
    shared: null,
    show: (origin: string | null | undefined) => origin === 'manual' || origin === 'recommendation',
  };
}
