<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { onBeforeRouteUpdate, useRoute, useRouter } from "vue-router";
import { ArrowLeft, MessageCircle, Send } from "lucide-vue-next";
import { commentApi, postApi } from "../api";
import { getApiError } from "../api/client";
import CommentCard from "../components/CommentCard.vue";
import EmptyState from "../components/EmptyState.vue";
import LoadMore from "../components/LoadMore.vue";
import MarkdownEditor from "../components/MarkdownEditor.vue";
import PostCard from "../components/PostCard.vue";
import PostSkeleton from "../components/PostSkeleton.vue";
import { useInfiniteList } from "../composables/useInfiniteList";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { Comment, FileRecord, Post } from "../types";

const { t } = useI18n();
const route = useRoute();
const router = useRouter();
const auth = useAuthStore();
const toast = useToastStore();

const postId = ref(String(route.params.id));
const post = ref<Post>();
const content = ref("");
const attachments = ref<FileRecord[]>([]);
const loading = ref(true);
const busy = ref(false);
const missing = ref(false);

const comments = useInfiniteList<Comment>(async (offset, limit) => {
  const response = await commentApi.list({ replyTo: postId.value, limit, offset });
  return { items: response.body?.data || [], count: response.body?.pagination?.count };
}, 30);

const load = async () => {
  loading.value = true;
  missing.value = false;
  try {
    post.value = (await postApi.get(postId.value)).body?.data;
    await comments.reset();
  } catch (error) {
    missing.value = true;
    toast.show(getApiError(error, t("errors.loadPost")), "error");
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
      comments.items.value = [...comments.items.value, response.body.data];
      post.value = { ...post.value, commentCount: post.value.commentCount + 1 };
    }
    content.value = "";
    attachments.value = [];
  } catch (error) {
    toast.show(getApiError(error, t("errors.comment")), "error");
  } finally {
    busy.value = false;
  }
};

const removed = (id: string) => {
  comments.remove(id);
  if (post.value) post.value = { ...post.value, commentCount: Math.max(0, post.value.commentCount - 1) };
};

onBeforeRouteUpdate(to => {
  postId.value = String(to.params.id);
  load();
});
onMounted(load);
</script>

<template>
  <div class="ax-container max-w-4xl space-y-5">
    <button class="btn btn-ghost btn-sm -ml-2" @click="router.back()"><ArrowLeft :size="16" /> {{ t("common.back") }}</button>
    <PostSkeleton v-if="loading" />
    <EmptyState v-else-if="missing || !post" :icon="MessageCircle" :title="t('post.notFound')">
      <RouterLink to="/" class="btn btn-primary btn-sm mt-2">{{ t("post.home") }}</RouterLink>
    </EmptyState>
    <template v-else>
      <PostCard :post="post" detailed @updated="value => post = value" @deleted="router.push('/')" />

      <section id="comments" class="space-y-3">
        <h2 class="flex items-center gap-2 font-bold"><MessageCircle :size="18" /> {{ t("comments.title") }} <span class="badge badge-ghost">{{ comments.total.value ?? post.commentCount }}</span></h2>
        <form v-if="auth.user" class="space-y-2" @submit.prevent="submit">
          <MarkdownEditor v-model="content" v-model:attachments="attachments" compact :max-length="5000" :placeholder="t('comments.placeholder')" @submit="submit" />
          <div class="flex justify-end"><button type="submit" class="btn btn-primary btn-sm" :disabled="busy || !content.trim()"><Send :size="16" /> {{ t("comments.submit") }}</button></div>
        </form>
        <div v-else class="alert"><span>{{ t("comments.signIn") }}</span><RouterLink :to="{ name: 'login', query: { redirect: route.fullPath } }" class="btn btn-sm">{{ t("nav.signIn") }}</RouterLink></div>

        <TransitionGroup name="list" tag="div" class="relative space-y-3">
          <CommentCard v-for="comment in comments.items.value" :key="comment.id" :comment="comment" @deleted="removed" />
        </TransitionGroup>
        <p v-if="!comments.loading.value && !comments.items.value.length" class="ax-muted py-8 text-center">{{ t("comments.empty") }}</p>
        <LoadMore :loading="comments.loading.value || comments.loadingMore.value" :done="comments.done.value" :count="comments.items.value.length" :error="comments.error.value" @more="comments.loadMore()" />
      </section>
    </template>
  </div>
</template>
