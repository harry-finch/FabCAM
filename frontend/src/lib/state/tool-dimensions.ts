import type { ToolEntry } from './project-types';

/** Presentation only. Zero is valid for the free shank; no value is written to CAM. */
export function dimensionState(value: number | null | undefined, allowZero = false) {
  return value == null
    ? 'unknown'
    : Number.isFinite(value) && (allowZero ? value >= 0 : value > 0)
      ? 'known'
      : 'invalid';
}
export function toolDimensions(tool: ToolEntry) {
  const known = (v: number | undefined, zero = false) => dimensionState(v, zero) === 'known';
  const extent = (v: number | undefined, fallback: number, zero = false) =>
    known(v, zero) ? Math.min(v!, 1e6) : fallback;
  const h = tool.holder;
  const holderValues = !h
    ? []
    : h.kind === 'cylinder'
      ? [h.diameter_mm, h.length_mm]
      : h.kind === 'cone'
        ? [h.bottom_diameter_mm, h.top_diameter_mm, h.length_mm]
        : [h.cylinder_diameter_mm, h.cylinder_length_mm, h.cone_top_diameter_mm, h.cone_length_mm];
  const holderValid = !!h && holderValues.every((v) => known(v));
  const holderDiameter = !h
    ? undefined
    : !holderValid
      ? NaN
      : h.kind === 'cylinder'
        ? h.diameter_mm
        : h.kind === 'cone'
          ? Math.max(h.bottom_diameter_mm, h.top_diameter_mm)
          : Math.max(h.cylinder_diameter_mm, h.cone_top_diameter_mm);
  const holderLength = !h
    ? undefined
    : !holderValid
      ? NaN
      : h.kind === 'stepped'
        ? h.cylinder_length_mm + h.cone_length_mm
        : h.length_mm;
  // Rust: stickout_length_mm is FREE SHANK ABOVE THE FLUTES, not tip-to-collet.
  const flute = extent(tool.fluteLengthMm, 20);
  const freeShank = extent(tool.stickoutLengthMm, 15, true);
  const holderBase = flute + freeShank;
  const overall = extent(tool.lengthMm, holderBase + 10);
  const inconsistent =
    known(tool.lengthMm) &&
    known(tool.fluteLengthMm) &&
    known(tool.stickoutLengthMm, true) &&
    holderBase > tool.lengthMm!;
  const scale = 160 / Math.max(overall, holderBase + extent(holderLength, 25), 1);
  const width = (v: number | undefined) => (known(v) ? Math.min(90, Math.max(2, v! * scale)) : 24);
  const bottom = 210 - holderBase * scale;
  const hh = extent(holderLength, 25) * scale;
  const bottomWidth =
    h?.kind === 'cone'
      ? width(h.bottom_diameter_mm)
      : h?.kind === 'stepped'
        ? width(h.cylinder_diameter_mm)
        : width(holderDiameter);
  const topWidth = width(
    h?.kind === 'cone'
      ? h.top_diameter_mm
      : h?.kind === 'stepped'
        ? h.cone_top_diameter_mm
        : holderDiameter,
  );
  const step = h?.kind === 'stepped' ? extent(h.cylinder_length_mm, 10) * scale : 0;
  const holderPath = `M${140 - bottomWidth / 2} ${bottom} v${-step} L${140 - topWidth / 2} ${bottom - hh} H${140 + topWidth / 2} L${140 + bottomWidth / 2} ${bottom - step} V${bottom} Z`;
  return {
    holderDiameter,
    holderLength,
    holderValid,
    holderPath,
    inconsistent,
    overall: overall * scale,
    holderBase: holderBase * scale,
    flute: flute * scale,
    diameter: width(tool.diameter),
    shank: width(tool.shankDiameterMm),
  };
}
