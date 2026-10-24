import { expect, it } from 'vitest';
import { DrawingFrame } from './drawing-frame';
import type { ImportEntry } from '../state/project-types';
import { identityFileTransform } from '../state/project-types';
it('keeps the camera still through transforms, resets for new sources, documents and explicit Fit', () => {
  const frame = new DrawingFrame();
  const entry = { id: 1, source: {}, fileTransform: identityFileTransform() } as ImportEntry;
  const box = { min_x: 0, min_y: 0, max_x: 60, max_y: 30 };
  const moved = { min_x: 20, min_y: 20, max_x: 80, max_y: 50 };
  const stock = { min_x: -10, min_y: -10, max_x: 150, max_y: 100 };
  expect(frame.fit(1, [entry], box, stock)).toEqual(stock);
  expect(
    frame.fit(1, [{ ...entry, fileTransform: { ...entry.fileTransform, scale: 2 } }], moved),
  ).toEqual(stock);
  expect(frame.fit(1, JSON.parse(JSON.stringify([entry])), moved)).toEqual(stock);
  frame.reset();
  expect(frame.fit(1, [entry], moved)).toEqual(moved);
  expect(frame.fit(2, [entry], box)).toEqual(box);
  expect(frame.fit(2, [{ ...entry, source: { ...entry.source, bbox: moved } }], moved)).toEqual(
    moved,
  );
});
