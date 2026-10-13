/// Inspector accordion: one section or none. Explicit reveals never toggle.
export type SidebarPane = 'stock' | 'layers' | 'text' | 'operations';
export interface PaneState {
  active: SidebarPane | null;
}
export function togglePane(state: PaneState, target: SidebarPane): PaneState {
  return { active: target === state.active ? null : target };
}
export function revealPane(state: PaneState, target: SidebarPane): PaneState {
  return target === state.active ? state : { active: target };
}
