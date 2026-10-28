export async function validateDrawingSnapshot(
  file: import('./project-types').ProjectFile,
  client: import('../api/client').WiacClient,
) {
  if (!file.drawing) return file;
  const input = {
    kind: 'evaluate' as const,
    version: 1,
    tolerance_mm: 0.1,
    simplification_mm: null,
    drawing: file.drawing.response.drawing,
  };
  const evaluated =
    file.drawing.version === 2
      ? await (() => {
          if (!client.evaluateDrawingV2)
            throw new Error('Backend cannot validate scoped geometry v2; project was not opened.');
          return client.evaluateDrawingV2({ kind: 'analyze', version: 2, input });
        })()
      : await (() => {
          if (!client.evaluateDrawing)
            throw new Error('Backend cannot validate edited geometry; project was not opened.');
          return client.evaluateDrawing(input);
        })();
  if (evaluated.version !== file.drawing.version)
    throw new Error('Unsupported edited drawing backend');
  if (evaluated.drawing.imports.length !== file.imports.length)
    throw new Error('Edited import count mismatch');
  const geometryKey = (segments: import('../api/types').Segment[]) =>
    JSON.stringify(
      segments.map((s) => [
        s.type,
        s.start.x,
        s.start.y,
        s.end.x,
        s.end.y,
        s.bulge ?? 0,
        s.center?.x ?? null,
        s.center?.y ?? null,
        s.layer ?? '0',
        s.color ?? 7,
      ]),
    );
  for (const source of evaluated.drawing.imports) {
    const original = file.imports.find((i) => i.id === source.id);
    if (!original || geometryKey(original.source.segments) !== geometryKey(source.original))
      throw new Error('Edited drawing original source mismatch');
    const t = original.fileTransform;
    if (
      source.transform.scale !== t.scale ||
      source.transform.rotation_deg !== t.rotateDeg ||
      source.transform.translate.x !== t.translate.x ||
      source.transform.translate.y !== t.translate.y ||
      source.transform.mirror_x !== t.mirrorX ||
      source.transform.mirror_y !== t.mirrorY ||
      source.pivot.x !== (original.source.bbox.min_x + original.source.bbox.max_x) / 2 ||
      source.pivot.y !== (original.source.bbox.min_y + original.source.bbox.max_y) / 2
    )
      throw new Error('Edited drawing transform/pivot mismatch');
  }
  return { ...file, drawing: { ...file.drawing, response: evaluated } };
}
