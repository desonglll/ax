<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Compass, LoaderCircle, RefreshCw, UsersRound } from "lucide-vue-next";
import { followApi, postApi, userApi } from "../api";
import ComposerCard from "../components/ComposerCard.vue";
import PaginationBar from "../components/PaginationBar.vue";
import PostCard from "../components/PostCard.vue";
import { useAuthStore } from "../stores/auth";
import type { Post, User } from "../types";

const route = useRoute();
const router = useRouter();
const auth = useAuthStore();
const posts = ref<Post[]>([]);
const people = ref<User[]>([]);
const loading = ref(true);
const tab = ref<"all" | "following">("all");
const offset = ref(0);
const limit = 12;
const count = ref<number>();
const search = computed(() => String(route.query.search || ""));

const load = async () => {
  loading.value = true;
  try {
    const response = tab.value === "following" && auth.user
      ? await followApi.feed({ limit, offset: offset.value })
      : await postApi.list({ limit, offset: offset.value, order_by: "created_at", sort: "desc", search: search.value || undefined });
    posts.value = response.body?.data || [];
    count.value = response.body?.pagination?.count;
  } finally {
    loading.value = false;
  }
};

const setTab = (value: "all" | "following") => {
  tab.value = value;
  offset.value = 0;
  load();
};

onMounted(async () => {
  await load();
  const response = await userApi.list({ limit: 5 });
  people.value = (response.body?.data || []).filter(user => user.id !== auth.user?.id).slice(0, 4);
});
watch(() => route.query.search, () => { offset.value = 0; load(); });
</script>

<template>
  <div class="ax-container grid gap-6 lg:grid-cols-[minmax(0,1fr)_19rem]">
    <div class="space-y-5">
      <div class="flex flex-wrap items-end justify-between gap-3">
        <div><p class="ax-section-title">Community feed</p><h1 class="ax-page-title">{{ search ? `Results for “${search}”` : "Ideas in motion" }}</h1></div>
        <button class="btn btn-ghost btn-sm" @click="load"><RefreshCw :size="16" /> Refresh</button>
      </div>
      <ComposerCard v-if="auth.user" @created="post => posts.unshift(post)" />
      <div v-else class="alert alert-info"><Compass :size="20" /><span>Sign in to publish, react, follow people, and join conversations.</span><RouterLink to="/login" class="btn btn-sm">Sign in</RouterLink></div>
      <div role="tablist" class="tabs tabs-box w-fit">
        <button role="tab" class="tab gap-2" :class="{ 'tab-active': tab === 'all' }" @click="setTab('all')"><Compass :size="15" /> Discover</button>
        <button role="tab" class="tab gap-2" :class="{ 'tab-active': tab === 'following' }" :disabled="!auth.user" @click="setTab('following')"><UsersRound :size="15" /> Following</button>
      </div>
      <div v-if="loading" class="grid place-items-center py-24"><LoaderCircle class="animate-spin text-primary" :size="32" /></div>
      <div v-else-if="!posts.length" class="ax-panel"><div class="card-body items-center py-16 text-center"><Compass :size="36" class="text-base-content/25" /><h2 class="card-title">Nothing here yet</h2><p class="ax-muted">Try another search or start the conversation.</p></div></div>
      <div v-else class="space-y-4"><PostCard v-for="post in posts" :key="post.id" :post="post" @deleted="id => posts = posts.filter(item => item.id !== id)" @updated="value => posts = posts.map(item => item.id === value.id ? value : item)" /></div>
      <PaginationBar :offset="offset" :limit="limit" :count="count" :loading="loading" @change="value => { offset = value; load(); router.replace({ query: { ...route.query, offset: value || undefined } }) }" />
    </div>
    <aside class="space-y-5">
      <section class="ax-panel"><div class="card-body p-4"><h2 class="ax-section-title">People to discover</h2><RouterLink v-for="person in people" :key="person.id" :to="`/profile/${person.id}`" class="flex items-center gap-3 rounded-box p-2 hover:bg-base-200"><div class="avatar placeholder"><div class="w-9 rounded-full bg-neutral text-neutral-content"><span>{{ person.userName.slice(0, 2).toUpperCase() }}</span></div></div><span class="min-w-0"><strong class="block truncate text-sm">{{ person.userName }}</strong><small class="ax-muted">{{ person.fullName || "Ax member" }}</small></span></RouterLink><RouterLink to="/people" class="btn btn-ghost btn-sm mt-2">Browse people</RouterLink></div></section>
      <section class="rounded-box bg-neutral p-5 text-neutral-content"><p class="text-xs font-bold uppercase tracking-widest opacity-60">Ax v1.2</p><h2 class="mt-2 text-xl font-black">A calmer social workspace.</h2><p class="mt-2 text-sm opacity-70">Thoughtful publishing, focused conversations, and transparent community signals.</p></section>
    </aside>
  </div>
</template>
