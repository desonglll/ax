/** One run of text, marked when it matched the search. */
export interface Segment {
  text: string;
  match: boolean;
}

const escapeRegExp = (value: string) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

/**
 * Words worth highlighting for a search query, mirroring what the backend
 * matches: the whole query, each `"quoted phrase"` and each bare word.
 * `-excluded` words and the `or` operator are dropped. Longest first so a
 * phrase wins over the words inside it.
 */
export const searchTerms = (query: string): string[] => {
  const terms = new Set<string>();
  const whole = query.trim().replace(/\s+/g, " ");
  if (whole && !/["-]/.test(whole)) terms.add(whole);
  for (const [, phrase, word] of whole.matchAll(/"([^"]+)"?|(\S+)/g)) {
    const term = (phrase ?? word ?? "").trim();
    if (!term || term.toLowerCase() === "or" || (word !== undefined && word.startsWith("-"))) continue;
    terms.add(term);
  }
  return [...terms].sort((a, b) => b.length - a.length);
};

/** Splits `text` into matched / unmatched runs (case-insensitive). Never produces HTML. */
export const highlight = (text: string, terms: string[]): Segment[] => {
  if (!text || !terms.length) return text ? [{ text, match: false }] : [];
  const pattern = new RegExp(`(${terms.map(escapeRegExp).join("|")})`, "giu");
  return text
    .split(pattern)
    .filter(part => part !== "")
    .map(part => ({ text: part, match: terms.some(term => term.toLowerCase() === part.toLowerCase()) }));
};

/**
 * A window of `text` around the first match, so a hit deep inside a long
 * post is visible in the result. Falls back to the start of the text.
 */
export const snippet = (text: string, terms: string[], max = 180): string => {
  if (text.length <= max) return text;
  const lower = text.toLowerCase();
  const first = terms.map(term => lower.indexOf(term.toLowerCase())).filter(index => index >= 0).sort((a, b) => a - b)[0];
  if (first === undefined || first < max / 3) return `${text.slice(0, max).trimEnd()}…`;
  const start = Math.max(0, first - Math.floor(max / 3));
  const end = Math.min(text.length, start + max);
  return `${start > 0 ? "…" : ""}${text.slice(start, end).trim()}${end < text.length ? "…" : ""}`;
};
