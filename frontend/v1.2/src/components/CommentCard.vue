<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Heart, ThumbsDown, Trash2 } from "lucide-vue-next";
import { commentApi, reactionApi } from "../api";
import { useAuthStore } from "../stores/auth";
import type { Comment, Reaction } from "../types";
import FilePreview from "./FilePreview.vue";

const props = defineProps<{ comment: Comment }>();
const emit = defineEmits<{ deleted: [id: string] }>();
const auth = useAuthStore();
const table = ref({ like: 0, dislike: 0 });
const own = ref<Reaction | null>(null);

const load = async () => {
  const [counts, reactions] = await Promise.all([
    reactionApi.table(props.comment.id, "comment"),
    auth.user ? reactionApi.list({ toId: props.comment.id, toType: "comment", userId: auth.user.id }) : Promise.resolve(null),
  ]);
  table.value = counts.body?.data || table.value;
  own.value = reactions?.body?.data?.[0] || null;
};

const react = async (kind: "Like" | "Dislike") => {
  if (!auth.user) return;
  if (own.value?.reactionName === kind) await reactionApi.delete(own.value.id);
  else kind === "Like" ? await reactionApi.like(props.comment.id, "comment") : await reactionApi.dislike(props.comment.id, "comment");
  await load();
};

const remove = async () => {
  if (!confirm("Delete this comment?")) return;
  await commentApi.delete(props.comment.id);
  emit("deleted", props.comment.id);
};

onMounted(load);
</script>

<template>
  <article class="rounded-box border border-base-300 bg-base-100 p-4">
    <header class="mb-3 flex items-center gap-3">
      <RouterLink :to="`/profile/${comment.userId}`" class="avatar placeholder"><div class="w-9 rounded-full bg-secondary text-secondary-content"><span>{{ comment.userName.slice(0, 2).toUpperCase() }}</span></div></RouterLink>
      <div class="flex-1"><RouterLink :to="`/profile/${comment.userId}`" class="text-sm font-bold">{{ comment.userName }}</RouterLink><small class="block text-base-content/45">{{ new Date(comment.createdAt).toLocaleString() }}</small></div>
      <button v-if="auth.user?.id === comment.userId || auth.user?.isAdmin" class="btn btn-ghost btn-circle btn-xs text-error" @click="remove"><Trash2 :size="14" /></button>
    </header>
    <p class="ax-prose text-sm">{{ comment.content }}</p>
    <div v-if="comment.attachments?.length" class="mt-3 grid gap-2"><FilePreview v-for="file in comment.attachments" :key="file.id" :file="file" /></div>
    <footer class="mt-3 flex gap-1 border-t border-base-300 pt-2">
      <button class="btn btn-ghost btn-xs" :class="{ 'text-error': own?.reactionName === 'Like' }" @click="react('Like')"><Heart :size="14" /> {{ table.like }}</button>
      <button class="btn btn-ghost btn-xs" :class="{ 'text-warning': own?.reactionName === 'Dislike' }" @click="react('Dislike')"><ThumbsDown :size="14" /> {{ table.dislike }}</button>
    </footer>
  </article>
</template>
