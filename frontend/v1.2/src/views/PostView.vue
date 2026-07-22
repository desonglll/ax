<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";
import { LoaderCircle, MessageCircle, Send } from "lucide-vue-next";
import { commentApi, postApi } from "../api";
import CommentCard from "../components/CommentCard.vue";
import PostCard from "../components/PostCard.vue";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { Comment, Post } from "../types";

const route = useRoute();
const auth = useAuthStore();
const toast = useToastStore();
const post = ref<Post>();
const comments = ref<Comment[]>([]);
const content = ref("");
const loading = ref(true);
const busy = ref(false);

const load = async () => {
  loading.value = true;
  const id = String(route.params.id);
  try {
    const [postResponse, commentsResponse] = await Promise.all([postApi.get(id), commentApi.list({ replyTo: id, limit: 100 })]);
    post.value = postResponse.body?.data;
    comments.value = commentsResponse.body?.data || [];
  } finally { loading.value = false; }
};

const submit = async () => {
  if (!content.value.trim() || !post.value) return;
  busy.value = true;
  try {
    const response = await commentApi.create({ content: content.value.trim(), replyTo: post.value.id });
    if (response.body?.data) comments.value.push(response.body.data);
    content.value = "";
    toast.show("Comment added", "success");
  } finally { busy.value = false; }
};

onMounted(load);
watch(() => route.params.id, load);
</script>

<template>
  <div class="ax-container max-w-4xl space-y-5">
    <div v-if="loading" class="grid place-items-center py-24"><LoaderCircle class="animate-spin" /></div>
    <template v-else-if="post">
      <PostCard :post="post" detailed @updated="value => post = value" />
      <section class="ax-panel"><div class="card-body p-4 md:p-5"><h2 class="card-title"><MessageCircle :size="20" /> Conversation <span class="badge badge-ghost">{{ comments.length }}</span></h2><div v-if="auth.user" class="join w-full"><textarea v-model="content" class="textarea join-item min-h-20 w-full" placeholder="Add to the conversation…"></textarea><button class="btn btn-primary join-item h-auto" :disabled="busy || !content.trim()" @click="submit"><Send :size="18" /></button></div><div v-else class="alert"><span>Sign in to comment.</span><RouterLink to="/login" class="btn btn-sm">Sign in</RouterLink></div></div></section>
      <div class="space-y-3"><CommentCard v-for="comment in comments" :key="comment.id" :comment="comment" @deleted="id => comments = comments.filter(item => item.id !== id)" /><p v-if="!comments.length" class="py-12 text-center ax-muted">No comments yet. Start the conversation.</p></div>
    </template>
  </div>
</template>
