import { ref } from "vue";

/** Page-specific document title (e.g. a post's title); AppShell falls back to the route name. */
export const pageTitle = ref<string | null>(null);
