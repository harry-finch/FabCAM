import * as THREE from 'three';
import { disposeGroup } from './dispose';
import type { BuilderContext, CssColor } from './builder';
import type { WorkOffset } from '../state/project-types';

/** Same authored XY as the 2D work-zero marker. No change to emitted paths. */
export class WorkZeroBuilder {
  readonly group = new THREE.Group();
  constructor(
    private ctx: BuilderContext,
    private color: CssColor,
  ) {
    ctx.scene.add(this.group);
  }
  build(origin: WorkOffset) {
    disposeGroup(this.group);
    this.ctx.requestRender();
    // Nonzero work-offset Z is explicitly unsupported; do not imply support.
    if (origin.z_mm !== 0) return;
    this.group.position.set(origin.x_mm, origin.y_mm, 0);
    this.group.name = `Work zero · ${origin.wcs}`;
    const points = [-3, 0, 0, 3, 0, 0, 0, -3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 5];
    const geometry = new THREE.BufferGeometry();
    geometry.setAttribute('position', new THREE.Float32BufferAttribute(points, 3));
    const marker = new THREE.LineSegments(
      geometry,
      new THREE.LineBasicMaterial({ color: this.color('--accent', 0x4b96ff), depthTest: false }),
    );
    marker.renderOrder = 10;
    this.group.add(marker);
    this.ctx.requestRender();
  }
  dispose() {
    disposeGroup(this.group);
    this.ctx.scene.remove(this.group);
  }
}
