<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { Flame } from "lucide-vue-next";
import { postApi } from "../api";
import { getApiError } from "../api/client";
import EmptyState from "../components/EmptyState.vue";
import PostCard from "../components/PostCard.vue";
import PostSkeleton from "../components/PostSkeleton.vue";
import { useToastStore } from "../stores/toast";
import type { Post } from "../types";

defineOptions({ name: "TrendingView" });

const { t } = useI18n();
const toast = useToastStore();
const posts = ref<Post[]>([]);
const loading = ref(true);

onMounted(async () => {
  try {
    posts.value = (await postApi.trending({ limit: 20 })).body?.data || [];
  } catch (error) {
    toast.show(getApiError(error, t("errors.loadTrending")), "error");
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <div class="ax-container max-w-4xl space-y-5">
    <header>
      <h1 class="ax-page-title flex items-center gap-2"><Flame :size="22" /> {{ t("trending.title") }}</h1>
      <p class="ax-muted mt-1 text-sm">{{ t("trending.subtitle") }}</p>
    </header>
    <div v-if="loading" class="space-y-4"><PostSkeleton v-for="n in 3" :key="n" /></div>
    <EmptyState v-else-if="!posts.length" :icon="Flame" :title="t('trending.empty')" />
    <TransitionGroup v-else name="list" tag="ol" class="relative space-y-4">
      <li v-for="(post, index) in posts" :key="post.id" class="relative">
        <span class="absolute -left-3 top-4 hidden size-7 -translate-x-full place-items-center rounded-full bg-base-300 text-sm font-bold text-base-content/60 md:grid">{{ index + 1 }}</span>
        <PostCard :post="post" @deleted="id => posts = posts.filter(item => item.id !== id)" @updated="value => posts = posts.map(item => item.id === value.id ? value : item)" />
      </li>
    </TransitionGroup>
  </div>
</template>
