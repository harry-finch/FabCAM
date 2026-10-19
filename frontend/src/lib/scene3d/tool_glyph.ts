/// Playhead tool glyph — a real-scale endmill (cylinder), V-bit (cone), or
/// drag-knife sitting above the work with its cutting tip planted at the
/// current toolpath point. Shape comes from machine.mode + the cutting
/// op's tool; tip color tracks the active move kind (cut/plunge/...).
///
/// The mesh is cached by a shape key so the 60×/sec playhead ticks only
/// move + retint it instead of churning a fresh BufferGeometry per frame.
///
/// Extracted from Scene3D.svelte. Owns its THREE.Group.

import * as THREE from 'three';
import { programTools } from '../sim/tool-provenance';
import type { GenerateResponse } from '../api/types';
import type { OpEntry, ToolEntry } from '../state/project.svelte';
import { playheadToSegment } from '../state/project.svelte';
import { buildToolMesh, disposeMesh } from './tool_mesh';
import type { Builder, BuilderContext, CssColor } from './builder';

export interface ToolGlyphInput {
  generated: GenerateResponse | null;
  playhead: number;
  cumLen: Float64Array | null;
  totalLen: number;
  operations: OpEntry[];
  selectedOpId: number | null;
  tools: ToolEntry[];
  machineMode: string;
}

export class ToolGlyphBuilder implements Builder {
  readonly group = new THREE.Group();
  private mesh: THREE.Mesh | undefined;
  private meshKey = '';
  private program: GenerateResponse | null = null;
  private resolved: ReturnType<typeof programTools> | null = null;
  /// Per-toolpath-kind tip colors. Recomputed by `refreshTipColors` so
  /// theme switches don't leave the tip a stale color.
  private tipColorByKind: Record<string, number> = {
    rapid: 0x35a2ff,
    cut: 0xff5555,
    plunge: 0xffd23a,
    retract: 0x5fd06e,
    arc: 0xff8a3a,
  };

  constructor(
    private ctx: BuilderContext,
    private cssColor: CssColor,
  ) {
    ctx.scene.add(this.group);
  }

  refreshTipColors() {
    this.tipColorByKind = {
      rapid: this.cssColor('--toolpath-rapid', 0x35a2ff).getHex(),
      cut: this.cssColor('--toolpath-cut', 0xff5555).getHex(),
      plunge: this.cssColor('--toolpath-plunge', 0xffd23a).getHex(),
      retract: this.cssColor('--toolpath-retract', 0x5fd06e).getHex(),
      arc: this.cssColor('--toolpath-arc', 0xff8a3a).getHex(),
    };
  }

  build(input: ToolGlyphInput) {
    const gen = input.generated;
    if (!gen || gen.toolpath.length === 0) {
      // No toolpath → drop the cached mesh so a future regenerate starts
      // clean instead of orbiting the previous program's tip.
      if (this.mesh) {
        this.group.remove(this.mesh);
        disposeMesh(this.mesh);
        this.mesh = undefined;
        this.meshKey = '';
      }
      return;
    }
    const total = gen.toolpath.length;
    const mapped = playheadToSegment(input.playhead, input.cumLen, input.totalLen);
    // Fall back to the count-based mapping only if the cum-length table
    // hasn't been built yet (race between setGenerated and the first tick).
    const headIdx =
      mapped.segIdx >= 0
        ? Math.max(0, Math.min(total - 1, mapped.segIdx))
        : Math.max(0, Math.min(total - 1, Math.round(input.playhead * total) - 1));
    const seg = gen.toolpath[headIdx];
    if (!seg) return;
    const t =
      mapped.segIdx >= 0
        ? Math.max(0, Math.min(1, mapped.segT))
        : Math.max(0, Math.min(1, input.playhead * total - headIdx));
    const px = seg.from.x + (seg.to.x - seg.from.x) * t;
    const py = seg.from.y + (seg.to.y - seg.from.y) * t;
    const pz = seg.from.z + (seg.to.z - seg.from.z) * t;

    const colorHex = this.tipColorByKind[seg.kind] ?? this.tipColorByKind.cut;

    // Generated movement identity owns both the glyph and material simulation.
    let tool: ToolEntry;
    try {
      if (this.program !== gen) {
        this.resolved = programTools(gen);
        this.program = gen;
      }
      tool = this.resolved!.resolve(seg);
      this.group.visible = true;
    } catch {
      this.group.visible = false;
      return;
    }
    const diameter = Math.max(0.01, tool.diameter - (tool.wearOffsetMm ?? 0));
    const mode = input.machineMode;
    const dragoff = tool?.dragoff;
    const tipDiameter = tool?.tipDiameter;
    const tipAngleDeg = tool?.tipAngleDeg;
    const kind = tool?.kind ?? 'endmill';
    const fluteLen = tool?.fluteLengthMm;
    const shankDia = tool?.shankDiameterMm;
    const holder = tool?.holder;
    const lengthMm = tool?.lengthMm;

    // Cache key — anything that changes the geometry shape. Color is NOT
    // part of the key; we only mutate material.color on the cached mesh.
    // Holder fields are JSON-stringified so the key updates whenever any
    // part of the holder spec changes.
    const key = `${kind}|${mode}|${diameter}|${tipDiameter ?? ''}|${tipAngleDeg ?? ''}|${dragoff ?? ''}|${fluteLen ?? ''}|${shankDia ?? ''}|${holder ? JSON.stringify(holder) : ''}|${lengthMm ?? ''}`;
    if (key !== this.meshKey || !this.mesh) {
      if (this.mesh) {
        this.group.remove(this.mesh);
        disposeMesh(this.mesh);
      }
      this.mesh = buildToolMesh(
        kind,
        mode,
        diameter,
        tipDiameter,
        dragoff,
        colorHex,
        fluteLen,
        shankDia,
        holder,
        tipAngleDeg,
        lengthMm,
      );
      this.group.add(this.mesh);
      this.meshKey = key;
    } else {
      // Cached mesh — just retint the material to match the active move.
      const m = this.mesh.material as THREE.MeshBasicMaterial;
      if (m.color.getHex() !== colorHex) m.color.setHex(colorHex);
    }
    this.mesh.position.set(px, py, pz);
  }

  dispose() {
    if (this.mesh) {
      this.group.remove(this.mesh);
      disposeMesh(this.mesh);
      this.mesh = undefined;
      this.meshKey = '';
    }
    this.ctx.scene.remove(this.group);
  }
}
