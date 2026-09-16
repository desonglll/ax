<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { MessageCircle, MoreHorizontal, Pencil, Trash2 } from "lucide-vue-next";
import { postApi } from "../api";
import { getApiError } from "../api/client";
import { fullDate, timeAgo } from "../lib/format";
import { embedsImage } from "../lib/markdown";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { FileRecord, Post } from "../types";
import Avatar from "./Avatar.vue";
import FilePreview from "./FilePreview.vue";
import MarkdownBody from "./MarkdownBody.vue";
import MarkdownEditor from "./MarkdownEditor.vue";
import ReactionBar from "./ReactionBar.vue";

const props = defineProps<{ post: Post; detailed?: boolean }>();
const emit = defineEmits<{ deleted: [id: string]; updated: [post: Post] }>();
const auth = useAuthStore();
const toast = useToastStore();

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
/** Attachments not already shown inline as images. */
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
    toast.show("Saved", "success");
  } catch (error) {
    toast.show(getApiError(error, "Could not save"), "error");
  } finally {
    busy.value = false;
  }
};

const remove = async () => {
  if (!confirm("Delete this post?")) return;
  try {
    await postApi.delete(props.post.id);
    emit("deleted", props.post.id);
    toast.show("Deleted", "success");
  } catch (error) {
    toast.show(getApiError(error, "Could not delete"), "error");
  }
};
</script>

<template>
  <article class="ax-panel overflow-visible">
    <div class="card-body gap-4 p-4 md:p-5">
      <header class="flex items-start gap-3">
        <RouterLink :to="`/profile/${post.userId}`"><Avatar :name="post.userName" /></RouterLink>
        <div class="min-w-0 flex-1">
          <RouterLink :to="`/profile/${post.userId}`" class="font-bold hover:text-primary">{{ post.userName }}</RouterLink>
          <div class="text-xs text-base-content/50" :title="fullDate(post.createdAt)">{{ timeAgo(post.createdAt) }}<span v-if="edited"> · edited</span></div>
        </div>
        <div v-if="canManage && !editing" class="dropdown dropdown-end">
          <button tabindex="0" class="btn btn-ghost btn-circle btn-sm" aria-label="Post menu"><MoreHorizontal :size="18" /></button>
          <ul tabindex="0" class="dropdown-content menu z-20 w-40 rounded-box border border-base-300 bg-base-100 p-2 shadow-xl">
            <li><button @click="startEdit"><Pencil :size="15" /> Edit</button></li>
            <li><button class="text-error" @click="remove"><Trash2 :size="15" /> Delete</button></li>
          </ul>
        </div>
      </header>

      <form v-if="editing" class="space-y-3" @submit.prevent="save">
        <input v-model="editTitle" class="input w-full" maxlength="120" placeholder="Title (optional)" />
        <MarkdownEditor v-model="editContent" v-model:attachments="editAttachments" autofocus @submit="save" />
        <div class="flex justify-end gap-2">
          <button type="button" class="btn btn-ghost btn-sm" @click="editing = false">Cancel</button>
          <button type="submit" class="btn btn-primary btn-sm" :disabled="busy || !editContent.trim()">Save</button>
        </div>
      </form>

      <template v-else>
        <RouterLink v-if="post.title" :to="`/posts/${post.id}`" class="text-xl font-bold tracking-tight hover:text-primary">{{ post.title }}</RouterLink>
        <div class="relative" :class="{ 'max-h-[26rem] overflow-hidden': long && !expanded }">
          <MarkdownBody :source="post.content" />
          <div v-if="long && !expanded" class="absolute inset-x-0 bottom-0 flex h-20 items-end justify-center bg-gradient-to-t from-base-100 to-transparent">
            <RouterLink :to="`/posts/${post.id}`" class="btn btn-ghost btn-sm">Read more</RouterLink>
          </div>
        </div>
        <div v-if="extraAttachments.length" class="grid gap-3"><FilePreview v-for="file in extraAttachments" :key="file.id" :file="file" /></div>
      </template>

      <footer class="flex flex-wrap items-center gap-1 border-t border-base-300 pt-3">
        <ReactionBar v-model:likes="likes" v-model:dislikes="dislikes" v-model:mine="mine" :target-id="post.id" target-type="post" />
        <RouterLink :to="`/posts/${post.id}`" class="btn btn-ghost btn-sm"><MessageCircle :size="17" /> {{ post.commentCount }}</RouterLink>
      </footer>
    </div>
  </article>
</template>
