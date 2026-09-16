<script setup lang="ts">
import { LoaderCircle, Send } from "lucide-vue-next";
import { nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { postApi } from "../api";
import { getApiError } from "../api/client";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { FileRecord, Post } from "../types";
import Avatar from "./Avatar.vue";
import MarkdownEditor from "./MarkdownEditor.vue";

/**
 * Collapsed by default (one line), expands into the full Markdown editor on
 * click so the feed stays visible, especially on phones.
 */
const emit = defineEmits<{ created: [post: Post] }>();
const { t } = useI18n();
const auth = useAuthStore();
const toast = useToastStore();
const root = ref<HTMLElement>();
const expanded = ref(false);
const title = ref("");
const content = ref("");
const attachments = ref<FileRecord[]>([]);
const busy = ref(false);

// Drafts survive reloads and navigation (per browser).
const DRAFT_KEY = "ax-draft";
try {
  const saved = JSON.parse(localStorage.getItem(DRAFT_KEY) || "null");
  if (saved && typeof saved.content === "string") {
    title.value = saved.title || "";
    content.value = saved.content;
    attachments.value = Array.isArray(saved.attachments) ? saved.attachments : [];
    expanded.value = Boolean(saved.content || saved.title);
  }
} catch { /* storage unavailable or corrupt */ }
let saveTimer = 0;
watch([title, content, attachments], () => {
  window.clearTimeout(saveTimer);
  saveTimer = window.setTimeout(() => {
    try {
      if (title.value || content.value || attachments.value.length) localStorage.setItem(DRAFT_KEY, JSON.stringify({ title: title.value, content: content.value, attachments: attachments.value }));
      else localStorage.removeItem(DRAFT_KEY);
    } catch { /* storage unavailable */ }
  }, 400);
}, { deep: true });

const discard = () => {
  title.value = "";
  content.value = "";
  attachments.value = [];
  expanded.value = false;
};

const expand = async () => {
  expanded.value = true;
  await nextTick();
  root.value?.querySelector("textarea")?.focus();
};
defineExpose({ expand });

const publish = async () => {
  if (!content.value.trim() || busy.value) return;
  busy.value = true;
  try {
    const response = await postApi.create({
      title: title.value.trim() || undefined,
      content: content.value.trim(),
      attachments: attachments.value.map(file => file.id),
    });
    if (!response.body?.data) throw new Error(response.message);
    emit("created", response.body.data);
    title.value = "";
    content.value = "";
    attachments.value = [];
    expanded.value = false;
    toast.show(t("composer.posted"), "success");
  } catch (error) {
    toast.show(getApiError(error, t("errors.publish")), "error");
  } finally {
    busy.value = false;
  }
};
</script>

<template>
  <section ref="root" class="ax-panel">
    <button v-if="!expanded" type="button" class="flex w-full items-center gap-3 p-3 text-left transition hover:bg-base-200/60" @click="expand">
      <Avatar v-if="auth.user" :name="auth.user.userName" size="sm" />
      <span class="input input-ghost flex-1 cursor-text items-center text-base-content/50">{{ t("composer.placeholder") }}</span>
      <span class="btn btn-primary btn-sm pointer-events-none hidden sm:inline-flex"><Send :size="16" /> {{ t("composer.post") }}</span>
    </button>
    <form v-else class="card-body min-w-0 gap-3 p-4" @submit.prevent="publish">
      <div class="flex items-center gap-3">
        <Avatar v-if="auth.user" :name="auth.user.userName" size="sm" />
        <input v-model="title" class="input input-ghost min-w-0 w-full px-1 text-lg font-bold focus:outline-none" maxlength="120" :placeholder="t('composer.titlePlaceholder')" />
      </div>
      <MarkdownEditor v-model="content" v-model:attachments="attachments" :placeholder="t('composer.placeholder')" @submit="publish" />
      <div class="flex justify-end gap-2">
        <button v-if="content || title || attachments.length" type="button" class="btn btn-ghost btn-sm mr-auto text-error" @click="discard">{{ t("composer.discard") }}</button>
        <button type="button" class="btn btn-ghost btn-sm" @click="expanded = false">{{ t("common.cancel") }}</button>
        <button type="submit" class="btn btn-primary btn-sm" :disabled="busy || !content.trim()">
          <LoaderCircle v-if="busy" :size="16" class="animate-spin" /><Send v-else :size="16" /> {{ t("composer.post") }}
        </button>
      </div>
    </form>
  </section>
</template>
