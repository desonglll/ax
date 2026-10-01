<script setup lang="ts">
import { computed, onActivated, onBeforeUnmount, onDeactivated, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { onBeforeRouteUpdate, useRoute, useRouter, type RouteLocationNormalized } from "vue-router";
import { FileText, Search, ShieldCheck, UsersRound } from "lucide-vue-next";
import { searchApi } from "../api";
import Avatar from "../components/Avatar.vue";
import EmptyState from "../components/EmptyState.vue";
import Highlighted from "../components/Highlighted.vue";
import LoadMore from "../components/LoadMore.vue";
import PostSkeleton from "../components/PostSkeleton.vue";
import SearchPostResult from "../components/SearchPostResult.vue";
import { useInfiniteList } from "../composables/useInfiniteList";
import { searchTerms } from "../lib/highlight";
import { pageTitle } from "../lib/title";
import type { Post, User } from "../types";

defineOptions({ name: "SearchView" });

type Tab = "posts" | "people";
const DEBOUNCE_MS = 300;
const MAX_LENGTH = 100;

const { t } = useI18n();
const route = useRoute();
const router = useRouter();

const fromRoute = (to: RouteLocationNormalized) => ({
  q: String(to.query.q || "").trim().slice(0, MAX_LENGTH),
  tab: (to.query.type === "people" ? "people" : "posts") as Tab,
});

// `query` is what the results are for; `draft` is what is in the box.
const query = ref(fromRoute(route).q);
const tab = ref<Tab>(fromRoute(route).tab);
const draft = ref(query.value);
const input = ref<HTMLInputElement>();
const terms = computed(() => searchTerms(query.value));

const postResults = useInfiniteList<Post>(async (offset, limit) => {
  const response = await searchApi.posts(query.value, { limit, offset });
  return { items: response.body?.data || [], count: response.body?.pagination?.count };
}, 10);
const peopleResults = useInfiniteList<User>(async (offset, limit) => {
  const response = await searchApi.users(query.value, { limit, offset });
  return { items: response.body?.data || [], count: response.body?.pagination?.count };
}, 20);
const active = computed(() => (tab.value === "posts" ? postResults : peopleResults));

const run = () => {
  if (!query.value) return;
  postResults.reset();
  peopleResults.reset();
};

/** Pushes the box's contents into the URL; the route update then runs the search. */
let timer: number | undefined;
const commit = (replace = true) => {
  window.clearTimeout(timer);
  const q = draft.value.trim();
  if (q === query.value) return;
  const target = { query: { ...route.query, q: q || undefined } };
  if (replace) router.replace(target);
  else router.push(target);
};
watch(draft, () => {
  window.clearTimeout(timer);
  timer = window.setTimeout(commit, DEBOUNCE_MS);
});

const setTab = (value: Tab) => router.replace({ query: { ...route.query, type: value === "posts" ? undefined : value } });

const apply = (to: RouteLocationNormalized) => {
  const next = fromRoute(to);
  tab.value = next.tab;
  if (next.q !== draft.value.trim()) draft.value = next.q;
  if (next.q !== query.value) {
    query.value = next.q;
    run();
  }
};
onBeforeRouteUpdate(to => { apply(to); });

watch(query, value => { pageTitle.value = value ? t("search.titleFor", { query: value }) : null; }, { immediate: true });
onMounted(() => {
  run();
  if (!query.value) input.value?.focus();
});
// Kept alive: a fresh visit from the navbar may carry a new query.
onActivated(() => {
  apply(route);
  pageTitle.value = query.value ? t("search.titleFor", { query: query.value }) : null;
});
onDeactivated(() => { pageTitle.value = null; window.clearTimeout(timer); });
onBeforeUnmount(() => { pageTitle.value = null; window.clearTimeout(timer); });

const countLabel = (count: number | undefined) => (count === undefined ? "" : count > 999 ? "999+" : String(count));
</script>

<template>
  <div class="ax-container max-w-4xl space-y-4">
    <header class="space-y-3">
      <h1 class="ax-page-title flex items-center gap-2"><Search :size="22" /> {{ t("search.title") }}</h1>
      <form role="search" @submit.prevent="commit(false)">
        <label class="input input-lg flex w-full items-center gap-2">
          <Search :size="18" class="text-base-content/45" />
          <input ref="input" v-model="draft" type="search" class="grow" :maxlength="MAX_LENGTH" :placeholder="t('search.placeholder')" :aria-label="t('search.placeholder')" enterkeyhint="search" />
        </label>
      </form>
    </header>

    <EmptyState v-if="!query" :icon="Search" :title="t('search.promptTitle')" :description="t('search.prompt')" />

    <template v-else>
      <div role="tablist" class="tabs tabs-box tabs-sm w-fit" :aria-label="t('search.title')">
        <button role="tab" class="tab gap-1.5" :class="{ 'tab-active': tab === 'posts' }" :aria-selected="tab === 'posts'" @click="setTab('posts')">
          <FileText :size="14" /> {{ t("search.posts") }}
          <span v-if="!postResults.loading.value && postResults.total.value !== undefined" class="badge badge-ghost badge-xs">{{ countLabel(postResults.total.value) }}</span>
        </button>
        <button role="tab" class="tab gap-1.5" :class="{ 'tab-active': tab === 'people' }" :aria-selected="tab === 'people'" @click="setTab('people')">
          <UsersRound :size="14" /> {{ t("search.people") }}
          <span v-if="!peopleResults.loading.value && peopleResults.total.value !== undefined" class="badge badge-ghost badge-xs">{{ countLabel(peopleResults.total.value) }}</span>
        </button>
      </div>

      <div aria-live="polite">
        <div v-if="active.loading.value" class="space-y-4"><PostSkeleton v-for="n in 3" :key="n" /></div>
        <EmptyState v-else-if="active.error.value && !active.items.value.length" :icon="Search" :title="t('search.failed')" :description="active.error.value">
          <button class="btn btn-primary btn-sm mt-2" @click="active.reset()">{{ t("common.retry") }}</button>
        </EmptyState>
        <EmptyState v-else-if="!active.items.value.length" :icon="tab === 'posts' ? FileText : UsersRound" :title="tab === 'posts' ? t('search.noPosts', { query }) : t('search.noPeople', { query })" :description="t('search.noResultsHint')" />

        <div v-else-if="tab === 'posts'" class="space-y-3">
          <SearchPostResult v-for="post in postResults.items.value" :key="post.id" :post="post" :terms="terms" />
        </div>
        <div v-else class="grid gap-3 sm:grid-cols-2">
          <RouterLink v-for="person in peopleResults.items.value" :key="person.id" :to="`/profile/${person.id}`" class="ax-panel transition-shadow hover:shadow-sm">
            <span class="card-body flex-row items-center gap-3 p-4">
              <Avatar :name="person.userName" />
              <span class="min-w-0 flex-1">
                <strong class="block truncate"><Highlighted :text="person.fullName || person.userName" :terms="terms" /></strong>
                <small class="ax-muted block truncate">@<Highlighted :text="person.userName" :terms="terms" /></small>
              </span>
              <span v-if="person.isAdmin" class="badge badge-primary badge-sm"><ShieldCheck :size="12" /> {{ t("nav.admin") }}</span>
            </span>
          </RouterLink>
        </div>
      </div>

      <LoadMore v-if="!active.loading.value && active.items.value.length" :loading="active.loadingMore.value" :done="active.done.value" :count="active.items.value.length" :error="active.error.value" @more="active.loadMore()" />
    </template>
  </div>
</template>
