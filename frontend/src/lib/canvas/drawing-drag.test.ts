import { expect, it } from 'vitest';
import { DrawingDrag, importAtSegment } from './drawing-drag';
import { identityFileTransform, type ImportEntry } from '../state/project-types';
const entry = {
  id: 7,
  source: { segments: [{}, {}] },
  fileTransform: { ...identityFileTransform(), translate: { x: 10, y: 20 } },
} as ImportEntry;
it('maps combined segments to their file and excludes the appended stock outline', () => {
  const second = { ...entry, id: 9 };
  expect(importAtSegment([entry, second], 1)?.id).toBe(7);
  expect(importAtSegment([entry, second], 2)?.id).toBe(9);
  expect(importAtSegment([entry, second], 4)).toBeNull();
  expect(importAtSegment([entry], -1)).toBeNull();
});
it('keeps taps inert, converts screen movement to mm and never mutates the source', () => {
  const before = JSON.stringify(entry);
  const drag = new DrawingDrag(3, entry, 100, 200, 2);
  drag.move(102, 201);
  expect(drag.active).toBe(false);
  drag.move(120, 180);
  expect(drag.translation).toEqual({ x: 20, y: 30 });
  drag.move(100, 200);
  expect(drag.translation).toEqual({ x: 10, y: 20 });
  expect(JSON.stringify(entry)).toBe(before);
});
it('refuses an invalid view scale', () => {
  const drag = new DrawingDrag(1, entry, 0, 0, 0);
  drag.move(30, 30);
  expect(drag.active).toBe(false);
});
