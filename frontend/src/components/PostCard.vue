<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import { ChevronDown, ChevronUp, MessageCircle, MoreHorizontal, Pencil, Trash2 } from "lucide-vue-next";
import { postApi } from "../api";
import { getApiError } from "../api/client";
import { fullDate, timeAgo } from "../lib/format";
import { embedsImage } from "../lib/markdown";
import { useAuthStore } from "../stores/auth";
import { useDialogStore } from "../stores/dialog";
import { usePostStore } from "../stores/posts";
import { useToastStore } from "../stores/toast";
import type { FileRecord, Post } from "../types";
import Avatar from "./Avatar.vue";
import FilePreview from "./FilePreview.vue";
import MarkdownBody from "./MarkdownBody.vue";
import MarkdownEditor from "./MarkdownEditor.vue";
import ReactionBar from "./ReactionBar.vue";

/**
 * In lists the whole card opens the post (except its links, buttons, images
 * and text selections). On the detail page (`detailed`) the comment button
 * asks the page to focus its comment box instead of navigating.
 */
const props = defineProps<{ post: Post; detailed?: boolean }>();
const emit = defineEmits<{ deleted: [id: string]; updated: [post: Post]; comment: [] }>();
const { t } = useI18n();
const router = useRouter();
const auth = useAuthStore();
const toast = useToastStore();
const dialog = useDialogStore();
const posts = usePostStore();

// Changes made on another page (detail view, another list) show up here too.
const view = computed(() => posts.view(props.post));

const likes = computed({ get: () => view.value.likeCount, set: likeCount => posts.patch(props.post.id, { likeCount }) });
const dislikes = computed({ get: () => view.value.dislikeCount, set: dislikeCount => posts.patch(props.post.id, { dislikeCount }) });
const mine = computed({ get: () => view.value.viewerReaction, set: viewerReaction => posts.patch(props.post.id, { viewerReaction }) });

const editing = ref(false);
const busy = ref(false);
const editTitle = ref("");
const editContent = ref("");
const editAttachments = ref<FileRecord[]>([]);
const expanded = ref(false);

const link = computed(() => `/posts/${props.post.id}`);
const canManage = computed(() => auth.user?.id === view.value.userId || auth.user?.isAdmin);
const edited = computed(() => view.value.updatedAt !== view.value.createdAt);
const long = computed(() => !props.detailed && (view.value.content.length > 700 || view.value.content.split("\n").length > 14));
const extraAttachments = computed(() => (view.value.attachments || []).filter(file => !embedsImage(view.value.content, file.id)));

/** Card click → open the post, unless the click was meant for something inside it. */
const open = (event: MouseEvent) => {
  if (props.detailed || editing.value) return;
  // composedPath() is fixed at dispatch time; `target.closest` would miss a button
  // whose icon was already re-rendered (and detached) by its own click handler.
  const interactive = "a, button, input, textarea, img, video, audio, label, summary, .dropdown";
  if (event.composedPath().some(node => node instanceof Element && node !== event.currentTarget && node.matches(interactive))) return;
  if (window.getSelection()?.toString()) return;
  if (event.metaKey || event.ctrlKey) window.open(router.resolve(link.value).href, "_blank");
  else router.push(link.value);
};

const onComment = () => {
  if (props.detailed) emit("comment");
  else router.push(`${link.value}#comments`);
};

const startEdit = () => {
  (document.activeElement as HTMLElement | null)?.blur();
  editTitle.value = view.value.title;
  editContent.value = view.value.content;
  editAttachments.value = [...(view.value.attachments || [])];
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
    const saved = response.body?.data;
    if (saved) {
      // The update response isn't hydrated with viewer state, so only take the edited fields.
      const changes = { title: saved.title, content: saved.content, attachments: saved.attachments, updatedAt: saved.updatedAt };
      emit("updated", { ...view.value, ...changes });
      posts.patch(saved.id, changes);
    }
    editing.value = false;
    toast.show(t("common.saved"), "success");
  } catch (error) {
    toast.show(getApiError(error, t("errors.save")), "error");
  } finally {
    busy.value = false;
  }
};

const remove = async () => {
  (document.activeElement as HTMLElement | null)?.blur();
  if (!(await dialog.confirm({ title: t("post.confirmDelete"), message: t("common.irreversible"), confirmLabel: t("common.delete"), danger: true }))) return;
  try {
    await postApi.delete(props.post.id);
    posts.markDeleted(props.post.id);
    emit("deleted", props.post.id);
    toast.show(t("common.deleted"), "success");
  } catch (error) {
    toast.show(getApiError(error, t("errors.delete")), "error");
  }
};
</script>

<template>
  <article
    v-if="detailed || !posts.deleted[post.id]"
    class="ax-panel overflow-visible transition-[box-shadow,border-color]"
    :class="{ 'cursor-pointer hover:border-base-content/15 hover:shadow-sm': !detailed && !editing }"
    @click="open"
  >
    <div class="card-body gap-4 p-4 md:p-5">
      <header class="flex items-start gap-3">
        <RouterLink :to="`/profile/${view.userId}`"><Avatar :name="view.userName" /></RouterLink>
        <div class="min-w-0 flex-1">
          <RouterLink :to="`/profile/${view.userId}`" class="font-bold hover:text-primary">{{ view.userName }}</RouterLink>
          <div class="text-xs text-base-content/50">
            <RouterLink v-if="!detailed" :to="link" class="hover:underline" :title="fullDate(view.createdAt)">{{ timeAgo(view.createdAt) }}</RouterLink>
            <span v-else :title="fullDate(view.createdAt)">{{ fullDate(view.createdAt) }}</span>
            <span v-if="edited" :title="fullDate(view.updatedAt)"> · {{ t("common.edited") }}</span>
          </div>
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
        <h1 v-if="view.title && detailed" class="text-2xl font-bold tracking-tight">{{ view.title }}</h1>
        <RouterLink v-else-if="view.title" :to="link" class="text-xl font-bold tracking-tight hover:text-primary">{{ view.title }}</RouterLink>
        <div class="relative transition-[max-height] duration-300" :class="{ 'max-h-[26rem] overflow-hidden': long && !expanded }">
          <MarkdownBody :source="view.content" />
          <div v-if="long && !expanded" class="absolute inset-x-0 bottom-0 flex h-24 items-end justify-center bg-gradient-to-t from-base-100 via-base-100/80 to-transparent">
            <button class="btn btn-ghost btn-sm" @click="expanded = true">{{ t("common.showMore") }} <ChevronDown :size="16" /></button>
          </div>
        </div>
        <button v-if="long && expanded" class="btn btn-ghost btn-sm self-center" @click="expanded = false">{{ t("common.showLess") }} <ChevronUp :size="16" /></button>
        <div v-if="extraAttachments.length" class="grid gap-3"><FilePreview v-for="file in extraAttachments" :key="file.id" :file="file" /></div>
      </template>

      <footer class="flex flex-wrap items-center gap-1 border-t border-base-300 pt-3">
        <ReactionBar v-model:likes="likes" v-model:dislikes="dislikes" v-model:mine="mine" :target-id="post.id" target-type="post" />
        <button class="btn btn-ghost btn-sm" :aria-label="t('post.comments')" @click="onComment"><MessageCircle :size="17" /> {{ view.commentCount }}</button>
      </footer>
    </div>
  </article>
</template>
