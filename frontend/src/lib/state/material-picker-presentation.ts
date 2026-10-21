/** Compact labels only. Reuses approved taxonomy; never participates in matching. */
import type { MsgKey } from '../i18n';
import {
  filterMaterials,
  materialPresentation,
  type MaterialOption,
} from './material-presentation';
export function materialPickerLabel(
  identity: { id: string; revision: string },
  fallback: string,
  t: (key: MsgKey) => string,
): string {
  const { keys, broad } = materialPresentation(identity);
  if (broad) return `${t('mat.source_category')} — ${t('mat.broad')}`;
  const last = keys.at(-1);
  if (!last) return fallback && fallback !== identity.id ? fallback : t('mat.custom_unknown');
  if (last === 'mat.6061' || last === 'mat.7075') return `${t('mat.aluminium')} ${t(last)}`;
  if (
    ['mat.wood', 'mat.plastic', 'mat.metal', 'mat.plywood', 'mat.pmma', 'mat.aluminium'].includes(
      last,
    )
  )
    return `${t(last)} — ${t('mat.generic')}`;
  return keys.length > 2 ? `${t(keys.at(-2)!)} — ${t(last)}` : t(last);
}
export function materialPage(
  options: MaterialOption[],
  query: string,
  family: string,
  source: string,
  requestedPage: number,
) {
  const shown = filterMaterials(options, query, family, source);
  const pages = Math.max(1, Math.ceil(shown.length / 30));
  const page = Math.max(0, Math.min(requestedPage, pages - 1));
  return { count: shown.length, pages, page, visible: shown.slice(page * 30, (page + 1) * 30) };
}
