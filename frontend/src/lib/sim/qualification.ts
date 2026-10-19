import type { SimDiagnostics, ToolpathSegment } from '../api/types';
import type { Fixture } from '../state/project-types';

// Runtime-only contract; never serialized into a project or reused across sessions.
export const QUALIFICATION_REVISION = 'full-sim-2';
export type QualificationPhase = 'unavailable' | 'pending' | 'current' | 'stale' | 'failed';
export interface Qualification {
  key: string;
  phase: QualificationPhase;
  front?: SimDiagnostics;
  back?: SimDiagnostics;
  message?: string;
  elapsedMs?: number;
  simulatorVersion?: unknown;
}
export interface VerificationInput {
  key: string;
  grid: {
    minX: number;
    minY: number;
    maxX: number;
    maxY: number;
    cellSize: number;
    originalCellSize: number;
    topZ: number;
    bottomZ: number;
  };
  fixtures: Fixture[];
  tools: Record<number, Record<string, unknown>>;
  backTools?: Record<number, Record<string, unknown>>;
  front: ToolpathSegment[];
  back?: ToolpathSegment[];
}
export interface VerificationResult {
  key: string;
  front: SimDiagnostics;
  back?: SimDiagnostics;
  elapsedMs: number;
  simulatorVersion: unknown;
}
export type VerificationReply = { result: VerificationResult } | { error: string };
