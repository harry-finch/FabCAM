import type { CornerRef, CornerReliefResponse } from '../api/types';
export const cornerReliefUi = $state({
  editing: false,
  corners: [] as CornerRef[],
  response: null as CornerReliefResponse | null,
  diameter: 0,
  original: null as import('../api/types').ImportResponse | null,
});
export function cornerKey(c: CornerRef): string {
  return `${c.object_id}:${c.corner_index}`;
}
export function toggleCorner(c: CornerRef) {
  const k = cornerKey(c);
  cornerReliefUi.corners = cornerReliefUi.corners.some((x) => cornerKey(x) === k)
    ? cornerReliefUi.corners.filter((x) => cornerKey(x) !== k)
    : [...cornerReliefUi.corners, c];
}
export function closeCornerRelief() {
  cornerReliefUi.editing = false;
  cornerReliefUi.original = null;
  cornerReliefUi.response = null;
  cornerReliefUi.corners = [];
}
