import { readFileSync, writeFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { buildProject } from '../api/build-project';
import { toCpsPostRequest } from '../components/cps-post-form';
import type { ProjectFile } from '../state/project-types';
import { augmentWithStockOutline } from '../state/stock-outline';
import { computeFootprint } from '../sim/driver';

it('the packaged offline project builds a square CPS request without setup work', () => {
  const file: ProjectFile = JSON.parse(
    readFileSync(
      new URL(
        '../../../../tests/fixtures/shopbot-m4/offline-square.ivac-project.json',
        import.meta.url,
      ),
      'utf8',
    ),
  );
  expect(file.kind).toBe('ivac-project');
  expect(file.machine!.name).toContain('OFFLINE DEMO ONLY');
  expect(file.imports[0].lastImportPath).toBeNull();
  const project = buildProject({
    transformedImport: file.imports[0].source,
    geometryView: augmentWithStockOutline(
      file.imports[0].source,
      computeFootprint(file.imports[0].source, file.stock!, file.machine!.workArea),
    ),
    machine: file.machine!,
    tools: file.tools!,
    operations: file.operations!,
    stock: file.stock,
    workOffset: file.workOffset,
  });
  expect(project).not.toBeNull();
  // The application includes the selectable stock outline. Explicit layer
  // selection must keep the demo from also machining the stock's perimeter.
  expect(project!.segments).toHaveLength(8);
  expect(project!.operations[0].source).toEqual({ kind: 'layers', layers: ['svg'] });
  expect(project!.tools[0]).toMatchObject({
    diameter: 6,
    feed_rate: 1200,
    plunge_rate: 200,
    speed: 18000,
  });
  expect(project!.operations[0].params).toMatchObject({ depth: -5, step: -2, fast_move_z: 5 });
  const request = {
    project,
    post_processor: 'cps',
    cps_post: toCpsPostRequest(file.machine!.cpsPost),
  };
  expect(request.cps_post).toEqual({ source: { kind: 'bundled', id: 'shopbot' }, properties: {} });
  // Explicit qualification command can export the real frontend request for
  // the native CLI; normal tests do not write fixtures or require Rust tools.
  if (process.env.IVAC_M4_REQUEST_OUT) {
    writeFileSync(process.env.IVAC_M4_REQUEST_OUT, JSON.stringify(request));
  }
});
