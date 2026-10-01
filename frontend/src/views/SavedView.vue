<script setup lang="ts">
import { onActivated, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { Bookmark } from "lucide-vue-next";
import { bookmarkApi } from "../api";
import EmptyState from "../components/EmptyState.vue";
import LoadMore from "../components/LoadMore.vue";
import PostCard from "../components/PostCard.vue";
import PostSkeleton from "../components/PostSkeleton.vue";
import { useInfiniteList } from "../composables/useInfiniteList";
import type { Post } from "../types";

defineOptions({ name: "SavedView" });

const { t } = useI18n();

const posts = useInfiniteList<Post>(async (offset, limit) => {
  const response = await bookmarkApi.list({ limit, offset });
  return { items: response.body?.data || [], count: response.body?.pagination?.count };
}, 10);

// Unsaving here keeps the card (so it can be re-saved); coming back reloads the list.
onMounted(() => posts.reset());
onActivated(() => {
  if (posts.items.value.length) posts.reset();
});
</script>

<template>
  <div class="ax-container max-w-4xl space-y-5">
    <header>
      <h1 class="ax-page-title flex items-center gap-2"><Bookmark :size="22" /> {{ t("bookmarks.title") }}</h1>
      <p class="ax-muted mt-1 text-sm">{{ t("bookmarks.subtitle") }}</p>
    </header>
    <div v-if="posts.loading.value" class="space-y-4"><PostSkeleton v-for="n in 3" :key="n" /></div>
    <EmptyState v-else-if="!posts.items.value.length && !posts.error.value" :icon="Bookmark" :title="t('bookmarks.empty')" :description="t('bookmarks.emptyHint')">
      <RouterLink to="/" class="btn btn-primary btn-sm mt-2">{{ t("bookmarks.browse") }}</RouterLink>
    </EmptyState>
    <TransitionGroup v-else name="list" tag="div" class="relative space-y-4">
      <PostCard v-for="post in posts.items.value" :key="post.id" :post="post" @deleted="posts.remove" @updated="posts.replace" />
    </TransitionGroup>
    <LoadMore v-if="!posts.loading.value" :loading="posts.loadingMore.value" :done="posts.done.value" :count="posts.items.value.length" :error="posts.error.value" @more="posts.loadMore()" />
  </div>
</template>
