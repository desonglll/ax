<script setup lang="ts">
import { Bold, Code, Columns2, Eye, Heading2, ImagePlus, Italic, Link2, List, LoaderCircle, Paperclip, PenLine, Quote, X } from "lucide-vue-next";
import { computed, nextTick, ref } from "vue";
import { useI18n } from "vue-i18n";
import { fileApi } from "../api";
import { getApiError } from "../api/client";
import { formatSize } from "../lib/format";
import { useDialogStore } from "../stores/dialog";
import { useToastStore } from "../stores/toast";
import type { FileRecord } from "../types";
import MarkdownBody from "./MarkdownBody.vue";

/**
 * Markdown textarea with toolbar, live preview and uploads. On wide screens
 * "Preview" opens a side-by-side pane; on phones it replaces the textarea.
 * Uploaded images are inserted at the cursor as `![name](url)`, other files
 * as `[name](url)`, and every upload is added to `attachments`.
 */
const props = withDefaults(defineProps<{ placeholder?: string; compact?: boolean; maxLength?: number; autofocus?: boolean }>(), {
  placeholder: undefined,
  compact: false,
  maxLength: 10000,
  autofocus: false,
});
const content = defineModel<string>({ required: true });
const attachments = defineModel<FileRecord[]>("attachments", { default: () => [] });
const emit = defineEmits<{ submit: [] }>();

const { t } = useI18n();
const toast = useToastStore();
const dialog = useDialogStore();
const textarea = ref<HTMLTextAreaElement>();
const fileInput = ref<HTMLInputElement>();
const preview = ref(false);
const uploading = ref(false);
const dragging = ref(false);
const pickImagesOnly = ref(false);
const remaining = computed(() => props.maxLength - content.value.length);
const isImage = (file: FileRecord) => file.contentType.startsWith("image/");

const focusAt = async (pos: number) => {
  await nextTick();
  textarea.value?.focus();
  textarea.value?.setSelectionRange(pos, pos);
};

const wrap = async (before: string, after = before, placeholder = t("editor.text")) => {
  const el = textarea.value;
  if (!el) return;
  const { selectionStart: start, selectionEnd: end, value } = el;
  const selected = value.slice(start, end) || placeholder;
  content.value = value.slice(0, start) + before + selected + after + value.slice(end);
  await focusAt(start + before.length + selected.length);
};

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
  const el = textarea.value;
  const selection = el ? { start: el.selectionStart, end: el.selectionEnd } : null;
  const url = await dialog.prompt({ title: t("editor.insertLink"), confirmLabel: t("editor.insertLink"), input: { placeholder: "https://", initial: "https://" } });
  if (!url || url === "https://") return;
  // The dialog steals focus; put the caret back where it was before wrapping.
  if (el && selection) el.setSelectionRange(selection.start, selection.end);
  await wrap("[", `](${url})`, t("editor.linkText"));
};

const upload = async (files: File[]) => {
  if (!files.length || uploading.value) return;
  uploading.value = true;
  try {
    const response = await fileApi.upload(files, { isPublic: true });
    const saved = response.body?.data || [];
    attachments.value = [...attachments.value, ...saved];
    await insertAtCursor(saved.map(file => (isImage(file) ? `![${file.name}](${fileApi.downloadUrl(file.id)})` : `[${file.name}](${fileApi.downloadUrl(file.id)})`)).join("\n"));
  } catch (error) {
    toast.show(getApiError(error, t("errors.upload")), "error");
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
  { icon: Bold, label: t("editor.bold"), run: () => wrap("**") },
  { icon: Italic, label: t("editor.italic"), run: () => wrap("_") },
  ...(props.compact ? [] : [{ icon: Heading2, label: t("editor.heading"), run: () => prefixLines("## ") }]),
  { icon: Quote, label: t("editor.quote"), run: () => prefixLines("> ") },
  { icon: List, label: t("editor.list"), run: () => prefixLines("- ") },
  { icon: Code, label: t("editor.code"), run: () => wrap("`") },
  { icon: Link2, label: t("editor.link"), run: link },
]);
</script>

<template>
  <div class="min-w-0 rounded-box border border-base-300 bg-base-100 transition-shadow" :class="{ 'ring-2 ring-primary': dragging }" @dragover.prevent="dragging = true" @dragleave="dragging = false" @drop.prevent="onDrop">
    <div class="flex flex-wrap items-center gap-0.5 border-b border-base-300 px-2 py-1">
      <button v-for="tool in tools" :key="tool.label" type="button" class="btn btn-ghost btn-square btn-xs" :title="tool.label" :aria-label="tool.label" @click="tool.run()">
        <component :is="tool.icon" :size="15" />
      </button>
      <span class="mx-1 h-4 w-px bg-base-300"></span>
      <button type="button" class="btn btn-ghost btn-xs" :disabled="uploading" @click="pick(true)"><ImagePlus :size="15" /> {{ t("editor.image") }}</button>
      <button type="button" class="btn btn-ghost btn-xs" :disabled="uploading" @click="pick(false)"><Paperclip :size="15" /> {{ t("editor.file") }}</button>
      <LoaderCircle v-if="uploading" :size="15" class="ml-1 animate-spin text-primary" />
      <button type="button" class="btn btn-ghost btn-xs ml-auto" :class="{ 'btn-active': preview }" @click="preview = !preview">
        <Columns2 :size="15" class="hidden md:block" /><component :is="preview ? PenLine : Eye" :size="15" class="md:hidden" /> {{ t("editor.preview") }}
      </button>
    </div>

    <div class="grid min-w-0 grid-cols-1" :class="{ 'md:grid-cols-2 md:divide-x md:divide-base-300': preview }">
      <textarea
        ref="textarea"
        v-model="content"
        class="textarea field-sizing-fixed min-w-0 w-full resize-y rounded-none border-0 bg-transparent px-4 py-3 font-mono text-[0.95rem] leading-relaxed focus:outline-none"
        :class="[compact ? 'min-h-20' : 'min-h-40', { 'hidden md:block': preview }]"
        :placeholder="placeholder ?? t('editor.placeholder')"
        :maxlength="maxLength"
        :autofocus="autofocus"
        @keydown="onKeydown"
        @paste="onPaste"
      ></textarea>
      <MarkdownBody v-if="preview" :source="content || `*${t('editor.nothingToPreview')}*`" class="min-w-0 overflow-y-auto px-4 py-3" :class="compact ? 'min-h-20 max-h-80' : 'min-h-40 max-h-[32rem]'" />
    </div>

    <TransitionGroup v-if="attachments.length" name="list" tag="div" class="flex flex-wrap gap-2 border-t border-base-300 px-3 py-2">
      <span v-for="file in attachments" :key="file.id" class="flex items-center gap-2 rounded-box border border-base-300 bg-base-200 py-1 pl-1 pr-2 text-xs">
        <img v-if="isImage(file)" :src="fileApi.downloadUrl(file.id)" :alt="file.name" class="size-8 rounded object-cover" />
        <Paperclip v-else :size="14" class="ml-1" />
        <span class="max-w-40 truncate">{{ file.name }}</span>
        <small class="opacity-60">{{ formatSize(file.size) }}</small>
        <button type="button" class="btn btn-ghost btn-circle btn-xs" :aria-label="t('editor.removeAttachment')" @click="removeAttachment(file)"><X :size="12" /></button>
      </span>
    </TransitionGroup>

    <div class="flex items-center justify-between gap-3 px-3 py-1 text-xs text-base-content/45">
      <span class="min-w-0 truncate">{{ t("editor.hint") }}</span>
      <span class="shrink-0 tabular-nums" :class="{ 'text-warning': remaining < 500 }">{{ remaining }}</span>
    </div>
    <input ref="fileInput" type="file" multiple class="hidden" :accept="pickImagesOnly ? 'image/*' : undefined" @change="onPick" />
  </div>
</template>
