<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { Heart, MessageCircle, MoreHorizontal, Pencil, ThumbsDown, Trash2 } from "lucide-vue-next";
import { postApi, reactionApi } from "../api";
import { getApiError } from "../api/client";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { Post, Reaction } from "../types";
import FilePreview from "./FilePreview.vue";

const props = defineProps<{ post: Post; detailed?: boolean }>();
const emit = defineEmits<{ deleted: [id: string]; updated: [post: Post] }>();
const auth = useAuthStore();
const toast = useToastStore();
const router = useRouter();
const likes = ref(props.post.likeCount || 0);
const dislikes = ref(props.post.dislikeCount || 0);
const ownReaction = ref<Reaction | null>(null);
const busy = ref(false);
const editing = ref(false);
const editTitle = ref(props.post.title);
const editContent = ref(props.post.content);
const canManage = computed(() => auth.user?.id === props.post.userId || auth.user?.isAdmin);

onMounted(async () => {
  if (!auth.user) return;
  const response = await reactionApi.list({ toId: props.post.id, toType: "post", userId: auth.user.id });
  ownReaction.value = response.body?.data?.[0] || null;
});

const react = async (kind: "Like" | "Dislike") => {
  if (!auth.user) return router.push("/login");
  if (busy.value) return;
  busy.value = true;
  try {
    if (ownReaction.value?.reactionName === kind) {
      await reactionApi.delete(ownReaction.value.id);
      kind === "Like" ? likes.value-- : dislikes.value--;
      ownReaction.value = null;
    } else {
      const previous = ownReaction.value?.reactionName;
      const response = kind === "Like" ? await reactionApi.like(props.post.id, "post") : await reactionApi.dislike(props.post.id, "post");
      if (previous === "Like") likes.value--;
      if (previous === "Dislike") dislikes.value--;
      kind === "Like" ? likes.value++ : dislikes.value++;
      ownReaction.value = response.body?.data || null;
    }
  } catch (error) {
    toast.show(getApiError(error, "Reaction failed"), "error");
  } finally {
    busy.value = false;
  }
};

const remove = async () => {
  if (!confirm("Delete this post? This cannot be undone.")) return;
  await postApi.delete(props.post.id);
  emit("deleted", props.post.id);
  toast.show("Post deleted", "success");
};

const save = async () => {
  busy.value = true;
  try {
    const response = await postApi.update(props.post.id, { title: editTitle.value, content: editContent.value });
    if (response.body?.data) emit("updated", response.body.data);
    editing.value = false;
    toast.show("Post updated", "success");
  } finally {
    busy.value = false;
  }
};
</script>

<template>
  <article class="ax-panel overflow-visible">
    <div class="card-body gap-4 p-4 md:p-5">
      <header class="flex items-start gap-3">
        <RouterLink :to="`/profile/${post.userId}`" class="avatar placeholder"><div class="w-11 rounded-full bg-neutral text-neutral-content"><span>{{ post.userName.slice(0, 2).toUpperCase() }}</span></div></RouterLink>
        <div class="min-w-0 flex-1"><RouterLink :to="`/profile/${post.userId}`" class="font-bold hover:text-primary">{{ post.userName }}</RouterLink><div class="text-xs text-base-content/50">{{ new Date(post.createdAt).toLocaleString() }}</div></div>
        <div v-if="canManage" class="dropdown dropdown-end">
          <button tabindex="0" class="btn btn-ghost btn-circle btn-sm" aria-label="Post menu"><MoreHorizontal :size="18" /></button>
          <ul tabindex="0" class="dropdown-content menu z-20 w-40 rounded-box border border-base-300 bg-base-100 p-2 shadow-xl">
            <li><button @click="editing = true"><Pencil :size="15" /> Edit</button></li>
            <li><button class="text-error" @click="remove"><Trash2 :size="15" /> Delete</button></li>
          </ul>
        </div>
      </header>
      <div v-if="editing" class="space-y-3">
        <input v-model="editTitle" class="input w-full" />
        <textarea v-model="editContent" class="textarea min-h-36 w-full"></textarea>
        <div class="flex justify-end gap-2"><button class="btn btn-ghost btn-sm" @click="editing = false">Cancel</button><button class="btn btn-primary btn-sm" :disabled="busy" @click="save">Save</button></div>
      </div>
      <template v-else>
        <RouterLink :to="`/posts/${post.id}`" class="group"><h2 v-if="post.title" class="mb-2 text-xl font-black tracking-tight group-hover:text-primary">{{ post.title }}</h2><p class="ax-prose text-[0.98rem]">{{ post.content }}</p></RouterLink>
        <div v-if="post.attachments?.length" class="grid gap-3"><FilePreview v-for="file in post.attachments" :key="file.id" :file="file" /></div>
      </template>
      <footer class="flex items-center gap-1 border-t border-base-300 pt-3">
        <button class="btn btn-ghost btn-sm" :class="{ 'text-error': ownReaction?.reactionName === 'Like' }" :disabled="busy" @click="react('Like')"><Heart :size="17" :fill="ownReaction?.reactionName === 'Like' ? 'currentColor' : 'none'" /> {{ likes }}</button>
        <button class="btn btn-ghost btn-sm" :class="{ 'text-warning': ownReaction?.reactionName === 'Dislike' }" :disabled="busy" @click="react('Dislike')"><ThumbsDown :size="17" /> {{ dislikes }}</button>
        <RouterLink :to="`/posts/${post.id}`" class="btn btn-ghost btn-sm"><MessageCircle :size="17" /> Discuss</RouterLink>
        <span class="ml-auto badge badge-ghost">{{ Math.round((post.engagementRate || 0) * 100) }}% positive</span>
      </footer>
    </div>
  </article>
</template>
