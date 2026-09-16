<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Compass, Flame, LoaderCircle, RefreshCw, UsersRound } from "lucide-vue-next";
import { postApi, userApi } from "../api";
import { getApiError } from "../api/client";
import ComposerCard from "../components/ComposerCard.vue";
import EmptyState from "../components/EmptyState.vue";
import PaginationBar from "../components/PaginationBar.vue";
import PostCard from "../components/PostCard.vue";
import UserRow from "../components/UserRow.vue";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { Post, User } from "../types";

type Tab = "all" | "following";
const LIMIT = 12;

const route = useRoute();
const router = useRouter();
const auth = useAuthStore();
const toast = useToastStore();

const posts = ref<Post[]>([]);
const people = ref<User[]>([]);
const trending = ref<Post[]>([]);
const loading = ref(true);
const count = ref<number>();

const search = computed(() => String(route.query.search || ""));
const tab = computed<Tab>(() => (route.query.tab === "following" && auth.user ? "following" : "all"));
const offset = computed(() => Math.max(0, Number(route.query.offset) || 0));

const load = async () => {
  loading.value = true;
  try {
    const response = tab.value === "following"
      ? await postApi.feed({ limit: LIMIT, offset: offset.value })
      : await postApi.list({ limit: LIMIT, offset: offset.value, search: search.value || undefined });
    posts.value = response.body?.data || [];
    count.value = response.body?.pagination?.count;
  } catch (error) {
    toast.show(getApiError(error, "Could not load posts"), "error");
  } finally {
    loading.value = false;
  }
};

const navigate = (patch: Record<string, string | number | undefined>) =>
  router.push({ query: { ...route.query, ...Object.fromEntries(Object.entries(patch).map(([k, v]) => [k, v || undefined])) } });

const setTab = (value: Tab) => navigate({ tab: value === "all" ? undefined : value, offset: undefined });
const setOffset = (value: number) => navigate({ offset: value || undefined });

const loadSidebar = async () => {
  const [users, hot] = await Promise.allSettled([userApi.list({ limit: 6 }), postApi.trending({ limit: 4 })]);
  if (users.status === "fulfilled") people.value = (users.value.body?.data || []).filter(user => user.id !== auth.user?.id).slice(0, 4);
  if (hot.status === "fulfilled") trending.value = hot.value.body?.data || [];
};

onMounted(() => { load(); loadSidebar(); });
watch(() => [route.query.search, route.query.tab, route.query.offset], load);
</script>

<template>
  <div class="ax-container grid gap-6 lg:grid-cols-[minmax(0,1fr)_19rem]">
    <div class="space-y-5">
      <div class="flex flex-wrap items-end justify-between gap-3">
        <div>
          <p class="ax-section-title">{{ tab === "following" ? "Your network" : "Community feed" }}</p>
          <h1 class="ax-page-title">{{ search ? `Results for “${search}”` : tab === "following" ? "From people you follow" : "Ideas in motion" }}</h1>
        </div>
        <div class="flex items-center gap-2">
          <RouterLink v-if="search" to="/" class="btn btn-ghost btn-sm">Clear search</RouterLink>
          <button class="btn btn-ghost btn-sm" :disabled="loading" @click="load"><RefreshCw :size="16" :class="{ 'animate-spin': loading }" /> Refresh</button>
        </div>
      </div>

      <ComposerCard v-if="auth.user" @created="post => posts.unshift(post)" />
      <div v-else class="alert alert-info"><Compass :size="20" /><span>Sign in to publish, react, follow people, and join conversations.</span><RouterLink to="/login" class="btn btn-sm">Sign in</RouterLink></div>

      <div role="tablist" class="tabs tabs-box w-fit">
        <button role="tab" class="tab gap-2" :class="{ 'tab-active': tab === 'all' }" @click="setTab('all')"><Compass :size="15" /> Discover</button>
        <button role="tab" class="tab gap-2" :class="{ 'tab-active': tab === 'following' }" :disabled="!auth.user" :title="auth.user ? '' : 'Sign in to see your network'" @click="setTab('following')"><UsersRound :size="15" /> Following</button>
      </div>

      <div v-if="loading" class="grid place-items-center py-24"><LoaderCircle class="animate-spin text-primary" :size="32" /></div>
      <EmptyState v-else-if="!posts.length" :icon="tab === 'following' ? UsersRound : Compass" :title="tab === 'following' ? 'Your feed is quiet' : 'Nothing here yet'" :description="tab === 'following' ? 'Follow a few people and their posts will show up here.' : search ? 'Try another search term.' : 'Be the first to start the conversation.'">
        <RouterLink v-if="tab === 'following'" to="/people" class="btn btn-primary btn-sm mt-2">Find people</RouterLink>
      </EmptyState>
      <div v-else class="space-y-4">
        <PostCard v-for="post in posts" :key="post.id" :post="post" @deleted="id => posts = posts.filter(item => item.id !== id)" @updated="value => posts = posts.map(item => item.id === value.id ? value : item)" />
      </div>
      <PaginationBar :offset="offset" :limit="LIMIT" :count="count" :loading="loading" @change="setOffset" />
    </div>

    <aside class="space-y-5">
      <section v-if="trending.length" class="ax-panel"><div class="card-body gap-1 p-4">
        <h2 class="ax-section-title mb-2 flex items-center gap-1"><Flame :size="14" class="text-error" /> Trending</h2>
        <RouterLink v-for="post in trending" :key="post.id" :to="`/posts/${post.id}`" class="rounded-box p-2 hover:bg-base-200">
          <strong class="line-clamp-1 text-sm">{{ post.title || post.content }}</strong>
          <small class="ax-muted">@{{ post.userName }} · {{ post.likeCount }} likes · {{ post.commentCount }} comments</small>
        </RouterLink>
        <RouterLink to="/trending" class="btn btn-ghost btn-sm mt-1">See all trending</RouterLink>
      </div></section>
      <section v-if="people.length" class="ax-panel"><div class="card-body gap-1 p-4">
        <h2 class="ax-section-title mb-2">People to discover</h2>
        <UserRow v-for="person in people" :key="person.id" :user="person" />
        <RouterLink to="/people" class="btn btn-ghost btn-sm mt-1">Browse people</RouterLink>
      </div></section>
    </aside>
  </div>
</template>
