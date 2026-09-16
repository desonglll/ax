<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { ArrowLeft, LoaderCircle, MessageCircle, Send } from "lucide-vue-next";
import { commentApi, postApi } from "../api";
import { getApiError } from "../api/client";
import CommentCard from "../components/CommentCard.vue";
import EmptyState from "../components/EmptyState.vue";
import MarkdownEditor from "../components/MarkdownEditor.vue";
import PaginationBar from "../components/PaginationBar.vue";
import PostCard from "../components/PostCard.vue";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { Comment, FileRecord, Post } from "../types";

const LIMIT = 50;
const route = useRoute();
const router = useRouter();
const auth = useAuthStore();
const toast = useToastStore();

const post = ref<Post>();
const comments = ref<Comment[]>([]);
const count = ref<number>();
const offset = ref(0);
const content = ref("");
const attachments = ref<FileRecord[]>([]);
const loading = ref(true);
const busy = ref(false);
const missing = ref(false);

const loadComments = async () => {
  if (!post.value) return;
  const response = await commentApi.list({ replyTo: post.value.id, limit: LIMIT, offset: offset.value });
  comments.value = response.body?.data || [];
  count.value = response.body?.pagination?.count;
};

const load = async () => {
  loading.value = true;
  missing.value = false;
  offset.value = 0;
  try {
    post.value = (await postApi.get(String(route.params.id))).body?.data;
    await loadComments();
  } catch (error) {
    missing.value = true;
    toast.show(getApiError(error, "Post not found"), "error");
  } finally {
    loading.value = false;
  }
};

const submit = async () => {
  if (!content.value.trim() || !post.value || busy.value) return;
  busy.value = true;
  try {
    const response = await commentApi.create({ content: content.value.trim(), replyTo: post.value.id, attachments: attachments.value.map(f => f.id) });
    if (response.body?.data) {
      comments.value.push(response.body.data);
      post.value = { ...post.value, commentCount: post.value.commentCount + 1 };
      count.value = (count.value ?? 0) + 1;
    }
    content.value = "";
    attachments.value = [];
  } catch (error) {
    toast.show(getApiError(error, "Could not add comment"), "error");
  } finally {
    busy.value = false;
  }
};

const removed = (id: string) => {
  comments.value = comments.value.filter(item => item.id !== id);
  if (post.value) post.value = { ...post.value, commentCount: Math.max(0, post.value.commentCount - 1) };
  if (count.value) count.value -= 1;
};

onMounted(load);
watch(() => route.params.id, load);
</script>

<template>
  <div class="ax-container max-w-4xl space-y-5">
    <button class="btn btn-ghost btn-sm -ml-2" @click="router.back()"><ArrowLeft :size="16" /> Back</button>
    <div v-if="loading" class="grid place-items-center py-24"><LoaderCircle class="animate-spin" /></div>
    <EmptyState v-else-if="missing || !post" :icon="MessageCircle" title="Post not found">
      <RouterLink to="/" class="btn btn-primary btn-sm mt-2">Home</RouterLink>
    </EmptyState>
    <template v-else>
      <PostCard :post="post" detailed @updated="value => post = value" @deleted="router.push('/')" />

      <section id="comments" class="space-y-3">
        <h2 class="flex items-center gap-2 font-bold"><MessageCircle :size="18" /> Comments <span class="badge badge-ghost">{{ count ?? comments.length }}</span></h2>
        <form v-if="auth.user" class="space-y-2" @submit.prevent="submit">
          <MarkdownEditor v-model="content" v-model:attachments="attachments" compact :max-length="5000" placeholder="Write a comment…" @submit="submit" />
          <div class="flex justify-end"><button type="submit" class="btn btn-primary btn-sm" :disabled="busy || !content.trim()"><Send :size="16" /> Comment</button></div>
        </form>
        <div v-else class="alert"><span>Sign in to comment.</span><RouterLink :to="{ name: 'login', query: { redirect: route.fullPath } }" class="btn btn-sm">Sign in</RouterLink></div>

        <CommentCard v-for="comment in comments" :key="comment.id" :comment="comment" @deleted="removed" />
        <p v-if="!comments.length" class="ax-muted py-8 text-center">No comments yet.</p>
        <PaginationBar :offset="offset" :limit="LIMIT" :count="count" @change="value => { offset = value; loadComments(); }" />
      </section>
    </template>
  </div>
</template>
