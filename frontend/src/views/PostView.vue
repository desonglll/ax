<script setup lang="ts">
import { AxiosError } from "axios";
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { onBeforeRouteUpdate, useRoute, useRouter } from "vue-router";
import { ArrowLeft, CircleAlert, MessageCircle, RefreshCw } from "lucide-vue-next";
import { commentApi, postApi } from "../api";
import CommentCard from "../components/CommentCard.vue";
import CommentComposer from "../components/CommentComposer.vue";
import EmptyState from "../components/EmptyState.vue";
import LoadMore from "../components/LoadMore.vue";
import PostCard from "../components/PostCard.vue";
import PostSkeleton from "../components/PostSkeleton.vue";
import { useInfiniteList } from "../composables/useInfiniteList";
import { excerpt } from "../lib/markdown";
import { pageTitle } from "../lib/title";
import { useAuthStore } from "../stores/auth";
import { usePostStore } from "../stores/posts";
import type { Comment, Post } from "../types";

const { t } = useI18n();
const route = useRoute();
const router = useRouter();
const auth = useAuthStore();
const posts = usePostStore();

const postId = ref(String(route.params.id));
const post = ref<Post>();
const state = ref<"loading" | "ready" | "missing" | "error">("loading");
const composer = ref<InstanceType<typeof CommentComposer>>();
/** Comments the viewer posted during this visit, shown first so they're visible right away. */
const mine = ref<Comment[]>([]);
const highlighted = ref<string>();

const current = computed(() => post.value && posts.view(post.value));

const comments = useInfiniteList<Comment>(async (offset, limit) => {
  const response = await commentApi.list({ replyTo: postId.value, limit, offset });
  return { items: response.body?.data || [], count: response.body?.pagination?.count };
}, 30);
const shown = computed(() => {
  const ids = new Set(mine.value.map(comment => comment.id));
  return [...mine.value, ...comments.items.value.filter(comment => !ids.has(comment.id))];
});

/** Scrolls to the comments (opening the box for signed-in users) when the URL asks for it. */
const honourHash = async () => {
  if (route.hash !== "#comments") return;
  await new Promise(resolve => window.setTimeout(resolve, 220)); // after the page transition
  if (auth.user) composer.value?.activate();
  else document.getElementById("comments")?.scrollIntoView({ behavior: "smooth", block: "start" });
};

const load = async () => {
  const id = postId.value;
  // A post opened from a list renders immediately; the request below refreshes it.
  post.value = posts.peek(id);
  state.value = post.value ? "ready" : "loading";
  mine.value = [];
  comments.items.value = [];
  comments.reset();
  if (post.value) honourHash();
  try {
    const data = (await postApi.get(id)).body?.data;
    if (id !== postId.value) return;
    if (!data) throw new Error("empty");
    const first = state.value !== "ready";
    post.value = data;
    state.value = "ready";
    if (first) honourHash();
  } catch (error) {
    if (id !== postId.value) return;
    const status = error instanceof AxiosError ? error.response?.status : undefined;
    if (status === 404 || status === 400) state.value = "missing";
    else if (!post.value) state.value = "error";
  }
};

const created = (comment: Comment) => {
  mine.value = [comment, ...mine.value];
  if (current.value) posts.patch(postId.value, { commentCount: current.value.commentCount + 1 });
  highlighted.value = comment.id;
  window.setTimeout(() => {
    if (highlighted.value === comment.id) highlighted.value = undefined;
  }, 2400);
  nextTick(() => document.getElementById(`comment-${comment.id}`)?.scrollIntoView({ behavior: "smooth", block: "nearest" }));
};

const removed = (id: string) => {
  mine.value = mine.value.filter(comment => comment.id !== id);
  comments.remove(id);
  if (current.value) posts.patch(postId.value, { commentCount: Math.max(0, current.value.commentCount - 1) });
};

/** Back to wherever the viewer came from; opened directly, go home instead of leaving the app. */
const back = () => (window.history.state?.back ? router.back() : router.push("/"));

const deleted = () => (window.history.state?.back ? router.back() : router.replace("/"));

watch(current, value => {
  pageTitle.value = value ? value.title || excerpt(value.content, 40) || null : null;
});
onBeforeUnmount(() => {
  pageTitle.value = null;
});

onBeforeRouteUpdate(to => {
  if (String(to.params.id) === postId.value) return;
  postId.value = String(to.params.id);
  load();
});
onMounted(load);
</script>

<template>
  <div class="ax-container max-w-4xl space-y-5">
    <button class="btn btn-ghost btn-sm -ml-2" @click="back"><ArrowLeft :size="16" /> {{ t("common.back") }}</button>
    <PostSkeleton v-if="state === 'loading'" />
    <EmptyState v-else-if="state === 'missing'" :icon="MessageCircle" :title="t('post.notFound')" :description="t('post.notFoundHint')">
      <RouterLink to="/" class="btn btn-primary btn-sm mt-2">{{ t("post.home") }}</RouterLink>
    </EmptyState>
    <EmptyState v-else-if="state === 'error' || !post" :icon="CircleAlert" :title="t('post.loadFailed')">
      <button class="btn btn-primary btn-sm mt-2" @click="load"><RefreshCw :size="15" /> {{ t("common.retry") }}</button>
    </EmptyState>
    <template v-else>
      <PostCard :post="post" detailed @updated="value => post = value" @deleted="deleted" @comment="composer?.activate()" />

      <section id="comments" class="scroll-mt-20 space-y-3">
        <h2 class="flex items-center gap-2 font-bold"><MessageCircle :size="18" /> {{ t("comments.title") }} <span class="badge badge-ghost">{{ current?.commentCount ?? 0 }}</span></h2>
        <CommentComposer :key="postId" ref="composer" :post-id="postId" @created="created" />

        <div v-if="comments.loading.value && !shown.length" class="space-y-3">
          <div v-for="n in 2" :key="n" class="rounded-box border border-base-300 bg-base-100 p-4">
            <div class="flex items-center gap-3"><div class="skeleton size-9 rounded-full"></div><div class="flex-1 space-y-2"><div class="skeleton h-3 w-28"></div><div class="skeleton h-3 w-16"></div></div></div>
            <div class="skeleton mt-3 h-3 w-2/3"></div>
          </div>
        </div>
        <TransitionGroup v-else name="list" tag="div" class="relative space-y-3">
          <CommentCard v-for="comment in shown" :key="comment.id" :comment="comment" :highlight="highlighted === comment.id" @deleted="removed" @reply="c => composer?.reply(c.userName)" />
        </TransitionGroup>
        <p v-if="!comments.loading.value && !comments.error.value && !shown.length" class="ax-muted py-8 text-center">{{ t("comments.empty") }}</p>
        <LoadMore v-if="!comments.loading.value || shown.length" :loading="comments.loadingMore.value" :done="comments.done.value" :count="shown.length" :error="comments.error.value" @more="comments.loadMore()" />
      </section>
    </template>
  </div>
</template>
