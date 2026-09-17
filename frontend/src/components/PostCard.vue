<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { ChevronDown, ChevronUp, MessageCircle, MoreHorizontal, Pencil, Trash2 } from "lucide-vue-next";
import { postApi } from "../api";
import { getApiError } from "../api/client";
import { fullDate, timeAgo } from "../lib/format";
import { embedsImage } from "../lib/markdown";
import { useAuthStore } from "../stores/auth";
import { useDialogStore } from "../stores/dialog";
import { useToastStore } from "../stores/toast";
import type { FileRecord, Post } from "../types";
import Avatar from "./Avatar.vue";
import FilePreview from "./FilePreview.vue";
import MarkdownBody from "./MarkdownBody.vue";
import MarkdownEditor from "./MarkdownEditor.vue";
import ReactionBar from "./ReactionBar.vue";

const props = defineProps<{ post: Post; detailed?: boolean }>();
const emit = defineEmits<{ deleted: [id: string]; updated: [post: Post] }>();
const { t } = useI18n();
const auth = useAuthStore();
const toast = useToastStore();
const dialog = useDialogStore();

const likes = ref(props.post.likeCount);
const dislikes = ref(props.post.dislikeCount);
const mine = ref(props.post.viewerReaction);
watch(() => props.post, p => { likes.value = p.likeCount; dislikes.value = p.dislikeCount; mine.value = p.viewerReaction; });

const editing = ref(false);
const busy = ref(false);
const editTitle = ref("");
const editContent = ref("");
const editAttachments = ref<FileRecord[]>([]);
const expanded = ref(false);

const canManage = computed(() => auth.user?.id === props.post.userId || auth.user?.isAdmin);
const edited = computed(() => props.post.updatedAt !== props.post.createdAt);
const long = computed(() => !props.detailed && (props.post.content.length > 700 || props.post.content.split("\n").length > 14));
const extraAttachments = computed(() => (props.post.attachments || []).filter(file => !embedsImage(props.post.content, file.id)));

const startEdit = () => {
  editTitle.value = props.post.title;
  editContent.value = props.post.content;
  editAttachments.value = [...(props.post.attachments || [])];
  editing.value = true;
};

const save = async () => {
  if (!editContent.value.trim() || busy.value) return;
  busy.value = true;
  try {
    const response = await postApi.update(props.post.id, {
      title: editTitle.value.trim(),
      content: editContent.value.trim(),
      attachments: editAttachments.value.map(file => file.id),
    });
    if (response.body?.data) emit("updated", response.body.data);
    editing.value = false;
    toast.show(t("common.saved"), "success");
  } catch (error) {
    toast.show(getApiError(error, t("errors.save")), "error");
  } finally {
    busy.value = false;
  }
};

const remove = async () => {
  if (!(await dialog.confirm({ title: t("post.confirmDelete"), message: t("common.irreversible"), confirmLabel: t("common.delete"), danger: true }))) return;
  try {
    await postApi.delete(props.post.id);
    emit("deleted", props.post.id);
    toast.show(t("common.deleted"), "success");
  } catch (error) {
    toast.show(getApiError(error, t("errors.delete")), "error");
  }
};
</script>

<template>
  <article class="ax-panel overflow-visible transition-shadow hover:shadow-sm">
    <div class="card-body gap-4 p-4 md:p-5">
      <header class="flex items-start gap-3">
        <RouterLink :to="`/profile/${post.userId}`"><Avatar :name="post.userName" /></RouterLink>
        <div class="min-w-0 flex-1">
          <RouterLink :to="`/profile/${post.userId}`" class="font-bold hover:text-primary">{{ post.userName }}</RouterLink>
          <div class="text-xs text-base-content/50" :title="fullDate(post.createdAt)">{{ timeAgo(post.createdAt) }}<span v-if="edited"> · {{ t("common.edited") }}</span></div>
        </div>
        <div v-if="canManage && !editing" class="dropdown dropdown-end">
          <button tabindex="0" class="btn btn-ghost btn-circle btn-sm" :aria-label="t('post.menu')"><MoreHorizontal :size="18" /></button>
          <ul tabindex="0" class="dropdown-content menu z-20 w-40 rounded-box border border-base-300 bg-base-100 p-2 shadow-xl">
            <li><button @click="startEdit"><Pencil :size="15" /> {{ t("common.edit") }}</button></li>
            <li><button class="text-error" @click="remove"><Trash2 :size="15" /> {{ t("common.delete") }}</button></li>
          </ul>
        </div>
      </header>

      <form v-if="editing" class="space-y-3" @submit.prevent="save">
        <input v-model="editTitle" class="input w-full" maxlength="120" :placeholder="t('composer.titlePlaceholder')" />
        <MarkdownEditor v-model="editContent" v-model:attachments="editAttachments" autofocus @submit="save" />
        <div class="flex justify-end gap-2">
          <button type="button" class="btn btn-ghost btn-sm" @click="editing = false">{{ t("common.cancel") }}</button>
          <button type="submit" class="btn btn-primary btn-sm" :disabled="busy || !editContent.trim()">{{ t("common.save") }}</button>
        </div>
      </form>

      <template v-else>
        <h2 v-if="post.title && detailed" class="text-2xl font-bold tracking-tight">{{ post.title }}</h2>
        <RouterLink v-else-if="post.title" :to="`/posts/${post.id}`" class="text-xl font-bold tracking-tight hover:text-primary">{{ post.title }}</RouterLink>
        <div class="relative transition-[max-height] duration-300" :class="{ 'max-h-[26rem] overflow-hidden': long && !expanded }">
          <MarkdownBody :source="post.content" />
          <div v-if="long && !expanded" class="absolute inset-x-0 bottom-0 flex h-24 items-end justify-center bg-gradient-to-t from-base-100 via-base-100/80 to-transparent">
            <button class="btn btn-ghost btn-sm" @click="expanded = true">{{ t("common.showMore") }} <ChevronDown :size="16" /></button>
          </div>
        </div>
        <button v-if="long && expanded" class="btn btn-ghost btn-sm self-center" @click="expanded = false">{{ t("common.showLess") }} <ChevronUp :size="16" /></button>
        <div v-if="extraAttachments.length" class="grid gap-3"><FilePreview v-for="file in extraAttachments" :key="file.id" :file="file" /></div>
      </template>

      <footer class="flex flex-wrap items-center gap-1 border-t border-base-300 pt-3">
        <ReactionBar v-model:likes="likes" v-model:dislikes="dislikes" v-model:mine="mine" :target-id="post.id" target-type="post" />
        <RouterLink :to="`/posts/${post.id}#comments`" class="btn btn-ghost btn-sm" :aria-label="t('post.comments')"><MessageCircle :size="17" /> {{ post.commentCount }}</RouterLink>
      </footer>
    </div>
  </article>
</template>
