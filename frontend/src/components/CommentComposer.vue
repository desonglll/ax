<script setup lang="ts">
import { LoaderCircle, Send } from "lucide-vue-next";
import { nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { commentApi } from "../api";
import { getApiError } from "../api/client";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { Comment, FileRecord } from "../types";
import Avatar from "./Avatar.vue";
import MarkdownEditor from "./MarkdownEditor.vue";

/**
 * One-line "Write a comment…" box that expands into the editor. Guests get
 * the sign-in dialog first and land in the editor afterwards. The unsent
 * draft is kept per post for the browser session.
 */
const props = defineProps<{ postId: string }>();
const emit = defineEmits<{ created: [comment: Comment] }>();

const { t } = useI18n();
const auth = useAuthStore();
const toast = useToastStore();
const root = ref<HTMLElement>();
const expanded = ref(false);
const content = ref("");
const attachments = ref<FileRecord[]>([]);
const busy = ref(false);

const DRAFT_KEY = `ax-comment-draft:${props.postId}`;
try {
  const saved = JSON.parse(sessionStorage.getItem(DRAFT_KEY) || "null");
  if (saved && typeof saved.content === "string") {
    content.value = saved.content;
    attachments.value = Array.isArray(saved.attachments) ? saved.attachments : [];
    expanded.value = Boolean(content.value || attachments.value.length);
  }
} catch { /* storage unavailable or corrupt */ }
watch([content, attachments], () => {
  try {
    if (content.value || attachments.value.length) sessionStorage.setItem(DRAFT_KEY, JSON.stringify({ content: content.value, attachments: attachments.value }));
    else sessionStorage.removeItem(DRAFT_KEY);
  } catch { /* storage unavailable */ }
}, { deep: true });

/** Opens the editor (after sign-in if needed) with the caret at the end. */
const activate = async () => {
  if (!(await auth.ensure("comment"))) return false;
  expanded.value = true;
  await nextTick();
  const textarea = root.value?.querySelector("textarea");
  if (textarea) {
    textarea.focus({ preventScroll: true });
    textarea.setSelectionRange(textarea.value.length, textarea.value.length);
  }
  root.value?.scrollIntoView({ behavior: "smooth", block: "center" });
  return true;
};

/** Starts (or continues) a reply that mentions the given user. */
const reply = async (userName: string) => {
  const mention = `@${userName} `;
  if (!content.value.startsWith(mention)) content.value = mention + content.value.replace(/^@\S+\s/, "");
  await activate();
};

defineExpose({ activate, reply });

const submit = async () => {
  if (!content.value.trim() || busy.value) return;
  busy.value = true;
  try {
    const response = await commentApi.create({ content: content.value.trim(), replyTo: props.postId, attachments: attachments.value.map(file => file.id) });
    if (!response.body?.data) throw new Error(response.message);
    emit("created", response.body.data);
    content.value = "";
    attachments.value = [];
    expanded.value = false;
    toast.show(t("comments.posted"), "success");
  } catch (error) {
    toast.show(getApiError(error, t("errors.comment")), "error");
  } finally {
    busy.value = false;
  }
};
</script>

<template>
  <div ref="root" class="scroll-mt-24">
    <button v-if="!expanded" type="button" class="flex w-full items-center gap-3 rounded-box border border-base-300 bg-base-100 p-3 text-left transition hover:border-base-content/20" @click="activate">
      <Avatar v-if="auth.user" :name="auth.user.userName" size="sm" />
      <span class="flex-1 px-1 text-base-content/50">{{ t("comments.placeholder") }}</span>
      <span class="btn btn-primary btn-sm pointer-events-none"><Send :size="15" /> {{ t("comments.submit") }}</span>
    </button>
    <form v-else class="space-y-2" @submit.prevent="submit">
      <MarkdownEditor v-model="content" v-model:attachments="attachments" compact :max-length="5000" :placeholder="t('comments.placeholder')" @submit="submit" />
      <div class="flex justify-end gap-2">
        <button type="button" class="btn btn-ghost btn-sm" @click="expanded = false">{{ t("common.cancel") }}</button>
        <button type="submit" class="btn btn-primary btn-sm" :disabled="busy || !content.trim()">
          <LoaderCircle v-if="busy" :size="16" class="animate-spin" /><Send v-else :size="16" /> {{ t("comments.submit") }}
        </button>
      </div>
    </form>
  </div>
</template>
