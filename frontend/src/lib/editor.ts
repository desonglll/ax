import { defineAsyncComponent } from "vue";

/**
 * The Markdown editor is only needed once someone starts writing, so it is
 * split out of the feed bundle. Await `loadMarkdownEditor()` before showing
 * it when the caller needs the textarea in the DOM on the next tick.
 */
export const MarkdownEditor = defineAsyncComponent(() => import("../components/MarkdownEditor.vue"));

// `__asyncLoader` resolves the wrapper itself, so it renders synchronously afterwards.
const loader = (MarkdownEditor as unknown as { __asyncLoader: () => Promise<unknown> }).__asyncLoader;
export const loadMarkdownEditor = () => loader().catch(() => undefined);
