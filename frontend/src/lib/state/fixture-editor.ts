// UI polygon coordinates are fixture-local, mm. Refuse degenerate/self-crossing
// outlines before updating the existing Fixture model.
export function parseFixturePolygon(raw: string): [number, number][] | null {
  const points = raw
    .trim()
    .split(/\n/)
    .map((line) =>
      line
        .trim()
        .split(/[,\s]+/)
        .map(Number),
    );
  if (
    points.length < 3 ||
    points.length > 256 ||
    points.some((p) => p.length !== 2 || p.some((n) => !Number.isFinite(n)))
  )
    return null;
  const p = points as [number, number][];
  const cross = (a: number[], b: number[], c: number[]) =>
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
  const area = p.reduce((s, a, i) => {
    const b = p[(i + 1) % p.length];
    return s + a[0] * b[1] - a[1] * b[0];
  }, 0);
  if (Math.abs(area) < 1e-9) return null;
  for (let i = 0; i < p.length; i++) {
    const a = p[i],
      b = p[(i + 1) % p.length];
    if (Math.hypot(a[0] - b[0], a[1] - b[1]) < 1e-9) return null;
    for (let j = i + 1; j < p.length; j++) {
      if (j === i + 1 || (i === 0 && j === p.length - 1)) continue;
      const c = p[j],
        d = p[(j + 1) % p.length];
      if (
        Math.max(a[0], b[0]) < Math.min(c[0], d[0]) ||
        Math.max(c[0], d[0]) < Math.min(a[0], b[0]) ||
        Math.max(a[1], b[1]) < Math.min(c[1], d[1]) ||
        Math.max(c[1], d[1]) < Math.min(a[1], b[1])
      )
        continue;
      if (cross(a, b, c) * cross(a, b, d) <= 0 && cross(c, d, a) * cross(c, d, b) <= 0) return null;
    }
  }
  return p;
}
