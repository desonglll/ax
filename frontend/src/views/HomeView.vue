<script setup lang="ts">
import { computed, onActivated, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { onBeforeRouteUpdate, useRoute, useRouter, type RouteLocationNormalized } from "vue-router";
import { Flame, PenLine, RefreshCw, Rss, UsersRound } from "lucide-vue-next";
import { postApi, userApi } from "../api";
import ComposerCard from "../components/ComposerCard.vue";
import EmptyState from "../components/EmptyState.vue";
import LoadMore from "../components/LoadMore.vue";
import PostCard from "../components/PostCard.vue";
import PostSkeleton from "../components/PostSkeleton.vue";
import SectionCard from "../components/SectionCard.vue";
import UserRow from "../components/UserRow.vue";
import { useInfiniteList } from "../composables/useInfiniteList";
import { excerpt } from "../lib/markdown";
import { useAuthStore } from "../stores/auth";
import type { Post, User } from "../types";

defineOptions({ name: "HomeView" });

type Tab = "all" | "following";
const { t } = useI18n();
const route = useRoute();
const router = useRouter();
const auth = useAuthStore();

// Read from the route so the tab and search survive reloads and back/forward.
const search = ref(String(route.query.search || ""));
const tab = ref<Tab>(route.query.tab === "following" && auth.user ? "following" : "all");
const people = ref<User[]>([]);
const trending = ref<Post[]>([]);
const composer = ref<InstanceType<typeof ComposerCard>>();

const feed = useInfiniteList<Post>(async (offset, limit) => {
  const response = tab.value === "following"
    ? await postApi.feed({ limit, offset })
    : await postApi.list({ limit, offset, search: search.value || undefined });
  return { items: response.body?.data || [], count: response.body?.pagination?.count };
}, 12);

const title = computed(() => (search.value ? t("home.searchTitle", { query: search.value }) : tab.value === "following" ? t("home.following") : t("home.latest")));

const setTab = (value: Tab) => router.push({ query: { ...route.query, tab: value === "all" ? undefined : value } });

const applyRoute = (to: RouteLocationNormalized) => {
  search.value = String(to.query.search || "");
  tab.value = to.query.tab === "following" && auth.user ? "following" : "all";
  feed.reset();
};
onBeforeRouteUpdate(to => { applyRoute(to); });
// Kept alive: coming back via the nav (fresh query) must re-sync; via Back (same query) keeps position.
onActivated(() => {
  const wantTab: Tab = route.query.tab === "following" && auth.user ? "following" : "all";
  if (String(route.query.search || "") !== search.value || wantTab !== tab.value) applyRoute(route);
});

const loadSidebar = async () => {
  const [users, hot] = await Promise.allSettled([userApi.list({ limit: 6 }), postApi.trending({ limit: 5 })]);
  if (users.status === "fulfilled") people.value = (users.value.body?.data || []).filter(user => user.id !== auth.user?.id).slice(0, 4);
  if (hot.status === "fulfilled") trending.value = hot.value.body?.data || [];
};

const focusComposer = () => {
  window.scrollTo({ top: 0, behavior: "smooth" });
  window.setTimeout(() => composer.value?.expand(), 300);
};

onMounted(() => { feed.reset(); loadSidebar(); });
</script>

<template>
  <div class="ax-container grid grid-cols-1 gap-6 lg:grid-cols-[minmax(0,1fr)_18rem]">
    <div class="min-w-0 space-y-4">
      <ComposerCard v-if="auth.user" ref="composer" @created="post => feed.prepend(post)" />
      <div v-else class="alert"><span>{{ t("home.signInPrompt") }}</span><RouterLink to="/login" class="btn btn-sm">{{ t("nav.signIn") }}</RouterLink></div>

      <div class="sticky top-16 z-30 flex flex-wrap items-center justify-between gap-2 rounded-box border border-base-300 bg-base-100/95 px-3 py-2 backdrop-blur md:top-[4.25rem]">
        <h1 class="text-base font-bold">{{ title }}</h1>
        <div class="flex items-center gap-1">
          <RouterLink v-if="search" to="/" class="btn btn-ghost btn-xs">{{ t("common.clear") }}</RouterLink>
          <div role="tablist" class="tabs tabs-box tabs-sm">
            <button role="tab" class="tab gap-1" :class="{ 'tab-active': tab === 'all' }" @click="setTab('all')"><Rss :size="14" /> {{ t("home.all") }}</button>
            <button role="tab" class="tab gap-1" :class="{ 'tab-active': tab === 'following' }" :disabled="!auth.user" :title="auth.user ? '' : t('home.followingHint')" @click="setTab('following')"><UsersRound :size="14" /> {{ t("home.following") }}</button>
          </div>
          <button class="btn btn-ghost btn-xs btn-square" :disabled="feed.loading.value" :aria-label="t('common.refresh')" @click="feed.reset()"><RefreshCw :size="14" :class="{ 'animate-spin': feed.loading.value }" /></button>
        </div>
      </div>

      <div v-if="feed.loading.value" class="space-y-4"><PostSkeleton v-for="n in 3" :key="n" /></div>
      <EmptyState v-else-if="!feed.items.value.length" :icon="tab === 'following' ? UsersRound : Rss" :title="tab === 'following' ? t('home.noFollowing') : search ? t('home.noResults') : t('home.noPosts')">
        <RouterLink v-if="tab === 'following'" to="/people" class="btn btn-primary btn-sm mt-2">{{ t("home.findPeople") }}</RouterLink>
      </EmptyState>
      <TransitionGroup v-else name="list" tag="div" class="relative space-y-4">
        <PostCard v-for="post in feed.items.value" :key="post.id" :post="post" @deleted="feed.remove" @updated="feed.replace" />
      </TransitionGroup>
      <LoadMore v-if="!feed.loading.value" :loading="feed.loadingMore.value" :done="feed.done.value" :count="feed.items.value.length" :error="feed.error.value" @more="feed.loadMore()" />
    </div>

    <aside class="min-w-0 space-y-4">
      <SectionCard v-if="trending.length" :title="t('home.trending')" :icon="Flame" to="/trending" :action="t('home.seeAll')">
        <RouterLink v-for="(post, index) in trending" :key="post.id" :to="`/posts/${post.id}`" class="flex gap-3 rounded-box p-2 transition hover:bg-base-200">
          <span class="w-4 shrink-0 pt-0.5 text-sm font-bold text-base-content/40">{{ index + 1 }}</span>
          <span class="min-w-0">
            <span class="line-clamp-2 text-sm">{{ post.title || excerpt(post.content, 80) }}</span>
            <small class="ax-muted block truncate">{{ post.userName }} · {{ t("home.likesComments", { likes: post.likeCount, comments: post.commentCount }) }}</small>
          </span>
        </RouterLink>
      </SectionCard>
      <SectionCard v-if="people.length" :title="t('home.whoToFollow')" :icon="UsersRound" to="/people" :action="t('home.seeAll')">
        <UserRow v-for="person in people" :key="person.id" :user="person" />
      </SectionCard>
    </aside>

    <button v-if="auth.user" class="btn btn-primary btn-circle btn-lg fixed bottom-20 right-4 z-30 shadow-lg md:hidden" :aria-label="t('nav.newPost')" @click="focusComposer"><PenLine :size="22" /></button>
  </div>
</template>
