// Format a duration in seconds as HH:MM:SS (always two digits per
// unit). Negative / NaN inputs render as 00:00:00.
export function formatHms(s: number | undefined): string {
  if (!s || !isFinite(s) || s < 0) return '00:00:00';
  const total = Math.round(s);
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const sec = total % 60;
  return [h, m, sec].map((n) => String(n).padStart(2, '0')).join(':');
}

// Short form for breakdown items: M:SS for sub-hour, just seconds for
// tiny values. Returns "—" for zero / undefined.
export function formatShort(s: number | undefined): string {
  if (s === undefined || !isFinite(s)) return '—';
  if (s <= 0.05) return '0s';
  if (s < 60) return `${s.toFixed(1)}s`;
  if (s < 3600) {
    const m = Math.floor(s / 60);
    const sec = Math.round(s % 60);
    return `${m}:${String(sec).padStart(2, '0')}`;
  }
  return formatHms(s);
}

/** Explicit units, rounded before decomposition (never displays 1 min 60 s). */
export function formatDuration(seconds: number | undefined): string {
  if (seconds == null || !Number.isFinite(seconds) || seconds < 0) return '—';
  const total = Math.round(seconds);
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  return [h ? `${h} h` : '', h || m ? `${m} min` : '', `${s} s`].filter(Boolean).join(' ');
}
