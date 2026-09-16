<script setup lang="ts">
import { Bold, Code, Eye, Heading2, ImagePlus, Italic, Link2, List, LoaderCircle, Paperclip, PenLine, Quote, X } from "lucide-vue-next";
import { computed, nextTick, ref } from "vue";
import { fileApi } from "../api";
import { getApiError } from "../api/client";
import { formatSize } from "../lib/format";
import { useToastStore } from "../stores/toast";
import type { FileRecord } from "../types";
import MarkdownBody from "./MarkdownBody.vue";

/**
 * Markdown textarea with a small toolbar, write/preview toggle and file
 * uploads. Uploaded images are inserted at the cursor as `![name](url)`,
 * other files as `[name](url)`; every upload is also added to `attachments`
 * so the server links it to the post or comment.
 */
const props = withDefaults(defineProps<{ placeholder?: string; compact?: boolean; maxLength?: number; autofocus?: boolean }>(), {
  placeholder: "Write in Markdown…",
  compact: false,
  maxLength: 10000,
  autofocus: false,
});
const content = defineModel<string>({ required: true });
const attachments = defineModel<FileRecord[]>("attachments", { default: () => [] });
const emit = defineEmits<{ submit: [] }>();

const toast = useToastStore();
const textarea = ref<HTMLTextAreaElement>();
const fileInput = ref<HTMLInputElement>();
const preview = ref(false);
const uploading = ref(false);
const dragging = ref(false);
const pickImagesOnly = ref(false);
const remaining = computed(() => props.maxLength - content.value.length);

const focusAt = async (pos: number) => {
  await nextTick();
  textarea.value?.focus();
  textarea.value?.setSelectionRange(pos, pos);
};

/** Wraps the selection (or inserts a placeholder) with `before`/`after`. */
const wrap = async (before: string, after = before, placeholder = "text") => {
  const el = textarea.value;
  if (!el) return;
  const { selectionStart: start, selectionEnd: end, value } = el;
  const selected = value.slice(start, end) || placeholder;
  content.value = value.slice(0, start) + before + selected + after + value.slice(end);
  await focusAt(start + before.length + selected.length);
};

/** Prefixes each selected line (headings, quotes, lists). */
const prefixLines = async (prefix: string) => {
  const el = textarea.value;
  if (!el) return;
  const { selectionStart: start, selectionEnd: end, value } = el;
  const lineStart = value.lastIndexOf("\n", start - 1) + 1;
  const block = value.slice(lineStart, end);
  const prefixed = block.split("\n").map(line => (line.startsWith(prefix) ? line.slice(prefix.length) : prefix + line)).join("\n");
  content.value = value.slice(0, lineStart) + prefixed + value.slice(end);
  await focusAt(lineStart + prefixed.length);
};

const insertAtCursor = async (text: string) => {
  const el = textarea.value;
  const pos = el ? el.selectionEnd : content.value.length;
  const value = content.value;
  const needsNewline = pos > 0 && value[pos - 1] !== "\n";
  const snippet = `${needsNewline ? "\n" : ""}${text}\n`;
  content.value = value.slice(0, pos) + snippet + value.slice(pos);
  await focusAt(pos + snippet.length);
};

const link = async () => {
  const url = prompt("Link URL", "https://");
  if (!url) return;
  await wrap("[", `](${url})`, "link text");
};

const upload = async (files: File[]) => {
  if (!files.length || uploading.value) return;
  uploading.value = true;
  try {
    const response = await fileApi.upload(files, { isPublic: true });
    const saved = response.body?.data || [];
    attachments.value = [...attachments.value, ...saved];
    const snippets = saved.map(file => {
      const url = fileApi.downloadUrl(file.id);
      return file.contentType.startsWith("image/") ? `![${file.name}](${url})` : `[${file.name}](${url})`;
    });
    await insertAtCursor(snippets.join("\n"));
  } catch (error) {
    toast.show(getApiError(error, "Upload failed"), "error");
  } finally {
    uploading.value = false;
  }
};

const pick = (imagesOnly: boolean) => {
  pickImagesOnly.value = imagesOnly;
  nextTick(() => fileInput.value?.click());
};

const onPick = (event: Event) => {
  const input = event.target as HTMLInputElement;
  upload(Array.from(input.files || []));
  input.value = "";
};

const onPaste = (event: ClipboardEvent) => {
  const files = Array.from(event.clipboardData?.files || []);
  if (files.length) {
    event.preventDefault();
    upload(files);
  }
};

const onDrop = (event: DragEvent) => {
  dragging.value = false;
  const files = Array.from(event.dataTransfer?.files || []);
  if (files.length) upload(files);
};

const removeAttachment = (file: FileRecord) => {
  attachments.value = attachments.value.filter(item => item.id !== file.id);
  // Drop the reference from the text as well.
  const url = fileApi.downloadUrl(file.id).replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  content.value = content.value.replace(new RegExp(`!?\\[[^\\]]*\\]\\(${url}\\)\\n?`, "g"), "");
};

const onKeydown = (event: KeyboardEvent) => {
  const mod = event.metaKey || event.ctrlKey;
  if (!mod) return;
  if (event.key === "Enter") { event.preventDefault(); emit("submit"); }
  else if (event.key === "b") { event.preventDefault(); wrap("**"); }
  else if (event.key === "i") { event.preventDefault(); wrap("_"); }
  else if (event.key === "k") { event.preventDefault(); link(); }
};

const tools = computed(() => [
  { icon: Bold, label: "Bold (⌘B)", run: () => wrap("**") },
  { icon: Italic, label: "Italic (⌘I)", run: () => wrap("_") },
  ...(props.compact ? [] : [{ icon: Heading2, label: "Heading", run: () => prefixLines("## ") }]),
  { icon: Quote, label: "Quote", run: () => prefixLines("> ") },
  { icon: List, label: "List", run: () => prefixLines("- ") },
  { icon: Code, label: "Code", run: () => wrap("`") },
  { icon: Link2, label: "Link (⌘K)", run: link },
]);
</script>

<template>
  <div class="rounded-box border border-base-300 bg-base-100" :class="{ 'ring-2 ring-primary': dragging }" @dragover.prevent="dragging = true" @dragleave="dragging = false" @drop.prevent="onDrop">
    <div class="flex flex-wrap items-center gap-0.5 border-b border-base-300 px-2 py-1">
      <button v-for="tool in tools" :key="tool.label" type="button" class="btn btn-ghost btn-square btn-xs" :title="tool.label" :aria-label="tool.label" :disabled="preview" @click="tool.run()">
        <component :is="tool.icon" :size="15" />
      </button>
      <span class="mx-1 h-4 w-px bg-base-300"></span>
      <button type="button" class="btn btn-ghost btn-xs" title="Insert image" :disabled="preview || uploading" @click="pick(true)"><ImagePlus :size="15" /> Image</button>
      <button type="button" class="btn btn-ghost btn-xs" title="Attach file" :disabled="preview || uploading" @click="pick(false)"><Paperclip :size="15" /> File</button>
      <LoaderCircle v-if="uploading" :size="15" class="ml-1 animate-spin text-primary" />
      <button type="button" class="btn btn-ghost btn-xs ml-auto" @click="preview = !preview">
        <PenLine v-if="preview" :size="15" /><Eye v-else :size="15" /> {{ preview ? "Write" : "Preview" }}
      </button>
    </div>

    <MarkdownBody v-if="preview" :source="content || '*Nothing to preview*'" class="px-4 py-3" :class="compact ? 'min-h-20' : 'min-h-40'" />
    <textarea
      v-else
      ref="textarea"
      v-model="content"
      class="textarea w-full resize-y rounded-none border-0 bg-transparent px-4 py-3 font-mono text-[0.95rem] leading-relaxed focus:outline-none"
      :class="compact ? 'min-h-20' : 'min-h-40'"
      :placeholder="placeholder"
      :maxlength="maxLength"
      :autofocus="autofocus"
      @keydown="onKeydown"
      @paste="onPaste"
    ></textarea>

    <div v-if="attachments.length" class="flex flex-wrap gap-2 border-t border-base-300 px-3 py-2">
      <span v-for="file in attachments" :key="file.id" class="badge badge-outline gap-1 py-3">
        {{ file.name }} <small class="opacity-60">{{ formatSize(file.size) }}</small>
        <button type="button" aria-label="Remove attachment" @click="removeAttachment(file)"><X :size="12" /></button>
      </span>
    </div>

    <div class="flex items-center justify-between px-3 py-1 text-xs text-base-content/45">
      <span>Markdown · paste or drop files to upload · ⌘↵ to submit</span>
      <span :class="{ 'text-warning': remaining < 500 }">{{ remaining }}</span>
    </div>
    <input ref="fileInput" type="file" multiple class="hidden" :accept="pickImagesOnly ? 'image/*' : undefined" @change="onPick" />
  </div>
</template>
