/** Search consumes explicit metadata only. No persistence, FS evidence or CAM payloads. */
export interface SearchRecord {
  id: string;
  category: 'command' | 'tool' | 'template';
  label: string;
  aliases?: string;
  diameter?: number;
  flutes?: number;
  angle?: number;
  relevance?: number;
}
export interface IndexedRecord<T extends SearchRecord = SearchRecord> {
  value: T;
  text: string;
  words: string[];
  labelText: string;
  labelWords: string[];
}
export function normalizeSearch(text: string) {
  return text
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .toLowerCase()
    .replace(/[_-]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}
export function buildSearchIndex<T extends SearchRecord>(
  records: readonly T[],
): IndexedRecord<T>[] {
  return records.map((value) => {
    const text = normalizeSearch(`${value.label} ${value.aliases ?? ''}`);
    const labelText = normalizeSearch(value.label);
    return { value, text, words: text.split(' '), labelText, labelWords: labelText.split(' ') };
  });
}
export function parseSearch(query: string) {
  let text = normalizeSearch(query).replace(/(\d),(\d)/g, '$1.$2');
  let diameter: number | undefined;
  let flutes: number | undefined;
  let angle: number | undefined;
  text = text.replace(
    /\b(\d+(?:\.\d+)?)(?:\/(\d+))?\s*(mm|inches|inch|in|")(?=\s|$)/g,
    (_, number: string, denominator: string, unit: string) => {
      const v = Number(number) / (denominator ? Number(denominator) : 1);
      diameter = unit === 'mm' ? v : v * 25.4;
      return ' ';
    },
  );
  text = text.replace(
    /\b(single|one|two|three|four|\d+)\s*(?:flutes?|dents?|schneiden)\b/g,
    (_, count: string) => {
      flutes =
        ({ single: 1, one: 1, two: 2, three: 3, four: 4 } as Record<string, number>)[count] ??
        Number(count);
      return ' ';
    },
  );
  // An explicit decimal without a unit denotes internal millimetres. Integers stay textual.
  text = text.replace(/\b\d+\.\d+\b/g, (number) => {
    diameter ??= Number(number);
    return ' ';
  });
  if (/\b(v bit|cone|engraver|fraise en v)\b/.test(text)) {
    text = text.replace(/\b(\d+)(?:°|\s*(?:deg|degrees))?(?=\s|$)/g, (_, number: string) => {
      angle = Number(number);
      return ' ';
    });
  }
  return { terms: text.trim().split(/\s+/).filter(Boolean), diameter, flutes, angle };
}
function oneEdit(a: string, b: string): boolean {
  if (Math.abs(a.length - b.length) > 1) return false;
  let i = 0;
  while (i < Math.min(a.length, b.length) && a[i] === b[i]) i++;
  if (i === Math.min(a.length, b.length)) return true;
  if (a.length === b.length)
    return (
      a.slice(i + 1) === b.slice(i + 1) ||
      (a[i] === b[i + 1] && a[i + 1] === b[i] && a.slice(i + 2) === b.slice(i + 2))
    );
  return a.length > b.length ? a.slice(i + 1) === b.slice(i) : a.slice(i) === b.slice(i + 1);
}
export function searchIndex<T extends SearchRecord>(
  index: readonly IndexedRecord<T>[],
  query: string,
): T[] {
  const parsed = parseSearch(query);
  const matches: { value: T; score: number }[] = [];
  for (const { value, text, words, labelText, labelWords } of index) {
    if (
      parsed.diameter !== undefined &&
      (value.diameter === undefined || Math.abs(value.diameter - parsed.diameter) > 1e-6)
    )
      continue;
    if (parsed.flutes !== undefined && value.flutes !== parsed.flutes) continue;
    if (parsed.angle !== undefined && value.angle !== parsed.angle) continue;
    let score = 0;
    for (const term of parsed.terms) {
      const quality = labelWords.includes(term)
        ? 0
        : labelWords.some((word) => word.startsWith(term))
          ? 1
          : labelText.includes(term)
            ? 2
            : words.includes(term)
              ? 3
              : words.some((word) => word.startsWith(term))
                ? 4
                : text.includes(term)
                  ? 5
                  : /^[a-z]{4,}$/.test(term) && words.some((word) => oneEdit(term, word))
                    ? 6
                    : -1;
      if (quality < 0) {
        score = -1;
        break;
      }
      score += quality;
    }
    if (score >= 0) matches.push({ value, score });
  }
  return matches
    .sort(
      (a, b) =>
        a.score - b.score ||
        (b.value.relevance ?? 0) - (a.value.relevance ?? 0) ||
        (a.value.id < b.value.id ? -1 : a.value.id > b.value.id ? 1 : 0),
    )
    .map((m) => m.value);
}
