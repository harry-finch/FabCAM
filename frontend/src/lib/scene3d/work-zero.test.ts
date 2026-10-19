import { it, expect, vi } from 'vitest';
import * as THREE from 'three';
import { WorkZeroBuilder } from './work-zero';
it('places the marker at authored XY, updates it without duplicates, disposes, and omits unsupported Z', () => {
  const scene = new THREE.Scene();
  const marker = new WorkZeroBuilder(
    { scene, requestRender: vi.fn() },
    () => new THREE.Color('blue'),
  );
  marker.build({ x_mm: 42, y_mm: -9, z_mm: 0, wcs: 'G54' });
  expect(marker.group.position.toArray()).toEqual([42, -9, 0]);
  expect(marker.group.children).toHaveLength(1);
  marker.build({ x_mm: -11, y_mm: 12, z_mm: 0, wcs: 'G55' });
  expect(marker.group.position.toArray()).toEqual([-11, 12, 0]);
  expect(marker.group.children).toHaveLength(1);
  marker.build({ x_mm: 0, y_mm: 0, z_mm: 5, wcs: 'G54' });
  expect(marker.group.children).toHaveLength(0);
  marker.dispose();
  expect(scene.children).toHaveLength(0);
});
