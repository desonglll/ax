<script setup lang="ts">
import { computed, onActivated, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { onBeforeRouteUpdate, useRoute, useRouter } from "vue-router";
import { CalendarDays, Pencil, UserMinus, UserPlus, UserRound } from "lucide-vue-next";
import { followApi, postApi, userApi } from "../api";
import { getApiError } from "../api/client";
import Avatar from "../components/Avatar.vue";
import EmptyState from "../components/EmptyState.vue";
import LoadMore from "../components/LoadMore.vue";
import PostCard from "../components/PostCard.vue";
import PostSkeleton from "../components/PostSkeleton.vue";
import UserRow from "../components/UserRow.vue";
import { useInfiniteList } from "../composables/useInfiniteList";
import { shortDate } from "../lib/format";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { FollowStats, Post, User } from "../types";

defineOptions({ name: "ProfileView" });

const { t } = useI18n();
const route = useRoute();
const router = useRouter();
const auth = useAuthStore();
const toast = useToastStore();

const targetId = ref(Number(route.params.id || auth.user?.id || 0));
const profile = ref<User>();
const follows = ref<FollowStats>();
const relationship = ref<{ title: string; users: User[] }>({ title: "", users: [] });
const loading = ref(true);
const busy = ref(false);
const editing = ref(false);
const form = ref({ userName: "", email: "", fullName: "", phone: "", password: "" });
const own = computed(() => auth.user?.id === targetId.value);

const posts = useInfiniteList<Post>(async (offset, limit) => {
  const response = await postApi.list({ user_id: targetId.value, limit, offset });
  return { items: response.body?.data || [], count: response.body?.pagination?.count };
}, 10);

const load = async () => {
  if (!targetId.value) return router.push({ name: "login", query: { redirect: route.fullPath } });
  loading.value = true;
  editing.value = false;
  try {
    const [userResponse, statsResponse] = await Promise.all([userApi.get(targetId.value), followApi.stats(targetId.value)]);
    profile.value = userResponse.body?.data;
    follows.value = statsResponse.body?.data;
    if (profile.value) form.value = { userName: profile.value.userName, email: profile.value.email, fullName: profile.value.fullName || "", phone: profile.value.phone || "", password: "" };
    posts.reset();
  } catch (error) {
    profile.value = undefined;
    toast.show(getApiError(error, t("errors.loadProfile")), "error");
  } finally {
    loading.value = false;
  }
};

const toggleFollow = async () => {
  if (!auth.user) return router.push({ name: "login", query: { redirect: route.fullPath } });
  busy.value = true;
  try {
    const response = follows.value?.isFollowing ? await followApi.unfollow(targetId.value) : await followApi.follow(targetId.value);
    follows.value = response.body?.data;
  } catch (error) {
    toast.show(getApiError(error, t("errors.follow")), "error");
  } finally {
    busy.value = false;
  }
};

const showRelationships = async (kind: "followers" | "following") => {
  const response = kind === "followers" ? await followApi.followers(targetId.value, { limit: 100 }) : await followApi.following(targetId.value, { limit: 100 });
  relationship.value = { title: kind === "followers" ? t("profile.followersTitle") : t("profile.followingTitle"), users: response.body?.data || [] };
  (document.getElementById("relationships-modal") as HTMLDialogElement).showModal();
};

const save = async () => {
  if (!profile.value) return;
  busy.value = true;
  try {
    const response = await userApi.update(profile.value.id, { ...form.value, password: form.value.password || undefined });
    profile.value = response.body?.data;
    editing.value = false;
    form.value.password = "";
    await auth.restore();
    toast.show(t("profile.updated"), "success");
  } catch (error) {
    toast.show(getApiError(error, t("errors.save")), "error");
  } finally {
    busy.value = false;
  }
};

onBeforeRouteUpdate(to => {
  const next = Number(to.params.id || auth.user?.id || 0);
  if (next !== targetId.value) {
    targetId.value = next;
    load();
  }
});
onActivated(() => {
  const next = Number(route.params.id || auth.user?.id || 0);
  if (next !== targetId.value) {
    targetId.value = next;
    load();
  }
});
onMounted(load);
</script>

<template>
  <div class="ax-container max-w-5xl space-y-5">
    <div v-if="loading" class="space-y-5">
      <div class="ax-panel"><div class="card-body gap-4 p-5"><div class="flex items-center gap-4"><div class="skeleton size-24 rounded-full"></div><div class="space-y-2"><div class="skeleton h-5 w-40"></div><div class="skeleton h-3 w-24"></div></div></div></div></div>
      <PostSkeleton />
    </div>
    <EmptyState v-else-if="!profile" :icon="UserRound" :title="t('profile.notFound')">
      <RouterLink to="/people" class="btn btn-primary btn-sm mt-2">{{ t("profile.browsePeople") }}</RouterLink>
    </EmptyState>
    <template v-else>
      <section class="ax-panel">
        <div class="card-body p-5 md:p-6">
          <div class="flex flex-wrap items-start justify-between gap-4">
            <Avatar :name="profile.userName" size="xl" />
            <div class="flex gap-2">
              <button v-if="own" class="btn btn-outline btn-sm" @click="editing = !editing"><Pencil :size="16" /> {{ editing ? t("common.close") : t("profile.edit") }}</button>
              <button v-else class="btn btn-sm transition-all" :class="follows?.isFollowing ? 'btn-outline' : 'btn-primary'" :disabled="busy" @click="toggleFollow">
                <UserMinus v-if="follows?.isFollowing" :size="16" /><UserPlus v-else :size="16" />{{ follows?.isFollowing ? t("profile.unfollow") : t("profile.follow") }}
              </button>
            </div>
          </div>
          <div class="mt-4">
            <div class="flex flex-wrap items-center gap-2">
              <h1 class="text-2xl font-bold">{{ profile.fullName || profile.userName }}</h1>
              <span v-if="profile.isAdmin" class="badge badge-primary">{{ t("nav.admin") }}</span>
              <span v-if="!profile.isActive" class="badge badge-warning">{{ t("profile.deactivated") }}</span>
            </div>
            <p class="text-base-content/55">@{{ profile.userName }}</p>
            <p v-if="own" class="ax-muted mt-2 text-sm">{{ profile.email }}<template v-if="profile.phone"> · {{ profile.phone }}</template></p>
            <p v-if="profile.createdAt" class="ax-muted mt-2 flex items-center gap-2 text-sm"><CalendarDays :size="15" /> {{ t("profile.joined", { date: shortDate(profile.createdAt) }) }}</p>
          </div>
          <div class="mt-5 flex gap-5 text-sm">
            <button class="hover:text-primary" @click="showRelationships('followers')"><strong>{{ follows?.followersCount || 0 }}</strong> <span class="ax-muted">{{ t("profile.followers") }}</span></button>
            <button class="hover:text-primary" @click="showRelationships('following')"><strong>{{ follows?.followingCount || 0 }}</strong> <span class="ax-muted">{{ t("profile.following") }}</span></button>
            <span><strong>{{ posts.total.value ?? posts.items.value.length }}</strong> <span class="ax-muted">{{ t("profile.posts") }}</span></span>
          </div>
        </div>
      </section>

      <Transition name="collapse">
        <section v-if="editing" class="ax-panel">
          <form class="card-body" @submit.prevent="save">
            <h2 class="card-title">{{ t("profile.edit") }}</h2>
            <div class="grid gap-4 md:grid-cols-2">
              <label class="fieldset"><span class="fieldset-legend">{{ t("profile.username") }}</span><input v-model="form.userName" class="input w-full" minlength="3" maxlength="32" required /></label>
              <label class="fieldset"><span class="fieldset-legend">{{ t("profile.email") }}</span><input v-model="form.email" type="email" class="input w-full" required /></label>
              <label class="fieldset"><span class="fieldset-legend">{{ t("profile.fullName") }}</span><input v-model="form.fullName" class="input w-full" /></label>
              <label class="fieldset"><span class="fieldset-legend">{{ t("profile.phone") }}</span><input v-model="form.phone" class="input w-full" /></label>
              <label class="fieldset md:col-span-2"><span class="fieldset-legend">{{ t("profile.newPassword") }}</span><input v-model="form.password" type="password" class="input w-full" minlength="8" maxlength="128" autocomplete="new-password" :placeholder="t('profile.keepPassword')" /></label>
            </div>
            <div class="card-actions justify-end"><button type="button" class="btn btn-ghost" @click="editing = false">{{ t("common.cancel") }}</button><button type="submit" class="btn btn-primary" :disabled="busy">{{ t("profile.saveChanges") }}</button></div>
          </form>
        </section>
      </Transition>

      <div>
        <h2 class="mb-3 text-lg font-bold">{{ t("profile.postsTitle") }}</h2>
        <div v-if="posts.loading.value" class="space-y-4"><PostSkeleton v-for="n in 2" :key="n" /></div>
        <TransitionGroup v-else name="list" tag="div" class="relative space-y-4">
          <PostCard v-for="post in posts.items.value" :key="post.id" :post="post" @deleted="posts.remove" @updated="posts.replace" />
        </TransitionGroup>
        <p v-if="!posts.loading.value && !posts.items.value.length" class="ax-muted py-12 text-center">{{ t("profile.noPosts") }}</p>
        <LoadMore v-if="!posts.loading.value" :loading="posts.loadingMore.value" :done="posts.done.value" :count="posts.items.value.length" :error="posts.error.value" @more="posts.loadMore()" />
      </div>
    </template>

    <dialog id="relationships-modal" class="modal">
      <div class="modal-box">
        <h3 class="text-lg font-bold">{{ relationship.title }}</h3>
        <div class="mt-4 space-y-1">
          <UserRow v-for="person in relationship.users" :key="person.id" :user="person" onclick="this.closest('dialog').close()" />
          <p v-if="!relationship.users.length" class="ax-muted py-10 text-center">{{ t("profile.nobody") }}</p>
        </div>
        <div class="modal-action"><form method="dialog"><button class="btn">{{ t("common.close") }}</button></form></div>
      </div>
      <form method="dialog" class="modal-backdrop"><button>{{ t("common.close") }}</button></form>
    </dialog>
  </div>
</template>
