<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { MessageCircle, MoreHorizontal, Pencil, Trash2 } from "lucide-vue-next";
import { postApi } from "../api";
import { getApiError } from "../api/client";
import { fullDate, timeAgo } from "../lib/format";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { Post } from "../types";
import Avatar from "./Avatar.vue";
import FilePreview from "./FilePreview.vue";
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
const editTitle = ref(props.post.title);
const editContent = ref(props.post.content);
const canManage = computed(() => auth.user?.id === props.post.userId || auth.user?.isAdmin);
const edited = computed(() => props.post.updatedAt !== props.post.createdAt);

const startEdit = () => { editTitle.value = props.post.title; editContent.value = props.post.content; editing.value = true; };

const save = async () => {
  if (!editContent.value.trim()) return;
  busy.value = true;
  try {
    const response = await postApi.update(props.post.id, { title: editTitle.value.trim(), content: editContent.value.trim() });
    if (response.body?.data) emit("updated", response.body.data);
    editing.value = false;
    toast.show("Post updated", "success");
  } catch (error) {
    toast.show(getApiError(error, "Could not update post"), "error");
  } finally {
    busy.value = false;
  }
};

const remove = async () => {
  if (!confirm("Delete this post? This cannot be undone.")) return;
  try {
    await postApi.delete(props.post.id);
    emit("deleted", props.post.id);
    toast.show("Post deleted", "success");
  } catch (error) {
    toast.show(getApiError(error, "Could not delete post"), "error");
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
        <textarea v-model="editContent" class="textarea min-h-36 w-full" maxlength="10000" required></textarea>
        <div class="flex justify-end gap-2">
          <button type="button" class="btn btn-ghost btn-sm" @click="editing = false">Cancel</button>
          <button type="submit" class="btn btn-primary btn-sm" :disabled="busy || !editContent.trim()">Save</button>
        </div>
      </form>

      <template v-else>
        <component :is="detailed ? 'div' : 'RouterLink'" :to="detailed ? undefined : `/posts/${post.id}`" class="group block">
          <h2 v-if="post.title" class="mb-2 text-xl font-black tracking-tight" :class="{ 'group-hover:text-primary': !detailed }">{{ post.title }}</h2>
          <p class="ax-prose text-[0.98rem]" :class="{ 'line-clamp-[12]': !detailed }">{{ post.content }}</p>
        </component>
        <div v-if="post.attachments?.length" class="grid gap-3"><FilePreview v-for="file in post.attachments" :key="file.id" :file="file" /></div>
      </template>

      <footer class="flex flex-wrap items-center gap-1 border-t border-base-300 pt-3">
        <ReactionBar v-model:likes="likes" v-model:dislikes="dislikes" v-model:mine="mine" :target-id="post.id" target-type="post" />
        <RouterLink :to="`/posts/${post.id}`" class="btn btn-ghost btn-sm"><MessageCircle :size="17" /> {{ post.commentCount }}</RouterLink>
        <span v-if="likes + dislikes > 0" class="ml-auto badge badge-ghost">{{ Math.round((likes / (likes + dislikes)) * 100) }}% positive</span>
      </footer>
    </div>
  </article>
</template>
