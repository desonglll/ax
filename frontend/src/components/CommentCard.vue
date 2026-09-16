<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { Trash2 } from "lucide-vue-next";
import { commentApi } from "../api";
import { getApiError } from "../api/client";
import { fullDate, timeAgo } from "../lib/format";
import { embedsImage } from "../lib/markdown";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { Comment } from "../types";
import Avatar from "./Avatar.vue";
import FilePreview from "./FilePreview.vue";
import MarkdownBody from "./MarkdownBody.vue";
import ReactionBar from "./ReactionBar.vue";

const props = defineProps<{ comment: Comment }>();
const emit = defineEmits<{ deleted: [id: string] }>();
const auth = useAuthStore();
const toast = useToastStore();

const likes = ref(props.comment.likeCount);
const dislikes = ref(props.comment.dislikeCount);
const mine = ref(props.comment.viewerReaction);
watch(() => props.comment, c => { likes.value = c.likeCount; dislikes.value = c.dislikeCount; mine.value = c.viewerReaction; });
const extraAttachments = computed(() => (props.comment.attachments || []).filter(file => !embedsImage(props.comment.content, file.id)));

const remove = async () => {
  if (!confirm("Delete this comment?")) return;
  try {
    await commentApi.delete(props.comment.id);
    emit("deleted", props.comment.id);
  } catch (error) {
    toast.show(getApiError(error, "Could not delete"), "error");
  }
};
</script>

<template>
  <article class="rounded-box border border-base-300 bg-base-100 p-4">
    <header class="mb-2 flex items-center gap-3">
      <RouterLink :to="`/profile/${comment.userId}`"><Avatar :name="comment.userName" size="sm" tone="secondary" /></RouterLink>
      <div class="min-w-0 flex-1">
        <RouterLink :to="`/profile/${comment.userId}`" class="text-sm font-bold hover:text-primary">{{ comment.userName }}</RouterLink>
        <small class="block text-base-content/45" :title="fullDate(comment.createdAt)">{{ timeAgo(comment.createdAt) }}</small>
      </div>
      <button v-if="auth.user?.id === comment.userId || auth.user?.isAdmin" class="btn btn-ghost btn-circle btn-xs text-error" aria-label="Delete comment" @click="remove"><Trash2 :size="14" /></button>
    </header>
    <MarkdownBody :source="comment.content" compact />
    <div v-if="extraAttachments.length" class="mt-3 grid gap-2"><FilePreview v-for="file in extraAttachments" :key="file.id" :file="file" /></div>
    <footer class="mt-2 border-t border-base-300 pt-2">
      <ReactionBar v-model:likes="likes" v-model:dislikes="dislikes" v-model:mine="mine" :target-id="comment.id" target-type="comment" compact />
    </footer>
  </article>
</template>
