import { defineStore } from "pinia";
import { ref } from "vue";

/** Full-screen image viewer, opened from any rendered image. */
export const useLightboxStore = defineStore("lightbox", () => {
  const src = ref<string | null>(null);
  const alt = ref("");
  const open = (source: string, label = "") => {
    src.value = source;
    alt.value = label;
  };
  const close = () => {
    src.value = null;
  };
  return { src, alt, open, close };
});
