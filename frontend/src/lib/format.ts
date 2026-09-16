import { currentLocale, i18n } from "../i18n";

const UNITS: [Intl.RelativeTimeFormatUnit, number][] = [
  ["year", 60 * 60 * 24 * 365],
  ["month", 60 * 60 * 24 * 30],
  ["week", 60 * 60 * 24 * 7],
  ["day", 60 * 60 * 24],
  ["hour", 60 * 60],
  ["minute", 60],
];

const formatters = new Map<string, Intl.RelativeTimeFormat>();
const relative = () => {
  const locale = currentLocale();
  let f = formatters.get(locale);
  if (!f) {
    f = new Intl.RelativeTimeFormat(locale, { numeric: "auto" });
    formatters.set(locale, f);
  }
  return f;
};

/** "3 hours ago" / "3 小时前"; follows the active locale. */
export const timeAgo = (iso: string) => {
  const then = new Date(iso).getTime();
  if (Number.isNaN(then)) return "";
  const seconds = Math.round((then - Date.now()) / 1000);
  if (Math.abs(seconds) < 45) return i18n.global.t("common.justNow");
  for (const [unit, size] of UNITS) {
    if (Math.abs(seconds) >= size) return relative().format(Math.round(seconds / size), unit);
  }
  return relative().format(seconds, "second");
};

export const fullDate = (iso?: string) => (iso ? new Date(iso).toLocaleString(currentLocale()) : "");
export const shortDate = (iso?: string) => (iso ? new Date(iso).toLocaleDateString(currentLocale()) : "");

export const formatSize = (bytes: number) => {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  return `${(bytes / 1024 / 1024 / 1024).toFixed(2)} GB`;
};

/** Two-character avatar initials. */
export const initials = (name: string) => name.trim().slice(0, 2).toUpperCase() || "?";
