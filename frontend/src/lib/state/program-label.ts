/** Label the actual generated format, never the currently selected post. */
export function programLabel(extension: string | null | undefined, fallback: string): string {
  const ext = extension?.replace(/^\./, '').toLowerCase();
  if (ext === 'sbp') return 'OpenSBP';
  if (ext === 'hpgl' || ext === 'plt') return 'HPGL';
  return ext && /^[a-z0-9]+$/.test(ext) ? `${fallback} (.${ext})` : fallback;
}
