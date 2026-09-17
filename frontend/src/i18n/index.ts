import { createI18n } from "vue-i18n";
import { en } from "./en";
import { zhCN } from "./zh-CN";

export type Locale = "en" | "zh-CN";
export const locales: { value: Locale; label: string }[] = [
  { value: "en", label: "English" },
  { value: "zh-CN", label: "简体中文" },
];

const STORAGE_KEY = "ax-locale";

const detect = (): Locale => {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved === "en" || saved === "zh-CN") return saved;
  } catch {
    /* storage unavailable */
  }
  return navigator.language.toLowerCase().startsWith("zh") ? "zh-CN" : "en";
};

export const i18n = createI18n({
  legacy: false,
  locale: detect(),
  fallbackLocale: "en",
  messages: { en, "zh-CN": zhCN },
});

export const currentLocale = (): Locale => i18n.global.locale.value as Locale;

export const setLocale = (locale: Locale) => {
  i18n.global.locale.value = locale;
  document.documentElement.lang = locale;
  try {
    localStorage.setItem(STORAGE_KEY, locale);
  } catch {
    /* storage unavailable */
  }
};

document.documentElement.lang = currentLocale();
