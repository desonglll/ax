<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { CalendarDays, LoaderCircle, Pencil, UserMinus, UserPlus, UserRound } from "lucide-vue-next";
import { followApi, postApi, userApi } from "../api";
import { getApiError } from "../api/client";
import Avatar from "../components/Avatar.vue";
import EmptyState from "../components/EmptyState.vue";
import PaginationBar from "../components/PaginationBar.vue";
import PostCard from "../components/PostCard.vue";
import UserRow from "../components/UserRow.vue";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { FollowStats, Post, User } from "../types";

const LIMIT = 10;
const route = useRoute();
const router = useRouter();
const auth = useAuthStore();
const toast = useToastStore();

const profile = ref<User>();
const posts = ref<Post[]>([]);
const postCount = ref<number>();
const offset = ref(0);
const follows = ref<FollowStats>();
const relationship = ref<{ title: string; users: User[] }>({ title: "", users: [] });
const loading = ref(true);
const busy = ref(false);
const editing = ref(false);
const form = ref({ userName: "", email: "", fullName: "", phone: "", password: "" });

const targetId = computed(() => Number(route.params.id || auth.user?.id || 0));
const own = computed(() => auth.user?.id === targetId.value);

const loadPosts = async () => {
  const response = await postApi.list({ user_id: targetId.value, limit: LIMIT, offset: offset.value });
  posts.value = response.body?.data || [];
  postCount.value = response.body?.pagination?.count;
};

const load = async () => {
  if (!targetId.value) return router.push({ name: "login", query: { redirect: route.fullPath } });
  loading.value = true;
  offset.value = 0;
  try {
    const [userResponse, statsResponse] = await Promise.all([userApi.get(targetId.value), followApi.stats(targetId.value)]);
    profile.value = userResponse.body?.data;
    follows.value = statsResponse.body?.data;
    await loadPosts();
    if (profile.value) form.value = { userName: profile.value.userName, email: profile.value.email, fullName: profile.value.fullName || "", phone: profile.value.phone || "", password: "" };
  } catch (error) {
    profile.value = undefined;
    toast.show(getApiError(error, "Profile not found"), "error");
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
    toast.show(getApiError(error, "Could not update follow"), "error");
  } finally {
    busy.value = false;
  }
};

const showRelationships = async (kind: "followers" | "following") => {
  const response = kind === "followers" ? await followApi.followers(targetId.value, { limit: 100 }) : await followApi.following(targetId.value, { limit: 100 });
  relationship.value = { title: kind === "followers" ? "Followers" : "Following", users: response.body?.data || [] };
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
    toast.show("Profile updated", "success");
  } catch (error) {
    toast.show(getApiError(error, "Could not update profile"), "error");
  } finally {
    busy.value = false;
  }
};

onMounted(load);
watch(() => route.params.id, load);
</script>

<template>
  <div class="ax-container max-w-5xl space-y-6">
    <div v-if="loading" class="grid place-items-center py-24"><LoaderCircle class="animate-spin" /></div>
    <EmptyState v-else-if="!profile" :icon="UserRound" title="User not found">
      <RouterLink to="/people" class="btn btn-primary btn-sm mt-2">Browse people</RouterLink>
    </EmptyState>
    <template v-else>
      <section class="ax-panel">
        <div class="card-body p-5 md:p-6">
          <div class="flex flex-wrap items-start justify-between gap-4">
            <Avatar :name="profile.userName" size="xl" />
            <div class="flex gap-2">
              <button v-if="own" class="btn btn-outline btn-sm" @click="editing = !editing"><Pencil :size="16" /> {{ editing ? "Close" : "Edit profile" }}</button>
              <button v-else class="btn btn-sm" :class="follows?.isFollowing ? 'btn-outline' : 'btn-primary'" :disabled="busy" @click="toggleFollow">
                <UserMinus v-if="follows?.isFollowing" :size="16" /><UserPlus v-else :size="16" />{{ follows?.isFollowing ? "Unfollow" : "Follow" }}
              </button>
            </div>
          </div>
          <div class="mt-4">
            <div class="flex flex-wrap items-center gap-2">
              <h1 class="text-2xl font-bold">{{ profile.fullName || profile.userName }}</h1>
              <span v-if="profile.isAdmin" class="badge badge-primary">Admin</span>
              <span v-if="!profile.isActive" class="badge badge-warning">Deactivated</span>
            </div>
            <p class="text-base-content/55">@{{ profile.userName }}</p>
            <p v-if="own" class="mt-2 text-sm ax-muted">{{ profile.email }}<template v-if="profile.phone"> · {{ profile.phone }}</template></p>
            <p class="ax-muted mt-2 flex items-center gap-2 text-sm"><CalendarDays :size="15" /> Joined {{ profile.createdAt ? new Date(profile.createdAt).toLocaleDateString() : "" }}</p>
          </div>
          <div class="mt-5 flex gap-5 text-sm">
            <button class="hover:text-primary" @click="showRelationships('followers')"><strong>{{ follows?.followersCount || 0 }}</strong> <span class="ax-muted">followers</span></button>
            <button class="hover:text-primary" @click="showRelationships('following')"><strong>{{ follows?.followingCount || 0 }}</strong> <span class="ax-muted">following</span></button>
            <span><strong>{{ postCount ?? posts.length }}</strong> <span class="ax-muted">posts</span></span>
          </div>
        </div>
      </section>

      <section v-if="editing" class="ax-panel">
        <form class="card-body" @submit.prevent="save">
          <h2 class="card-title">Edit profile</h2>
          <div class="grid gap-4 md:grid-cols-2">
            <label class="fieldset"><span class="fieldset-legend">Username</span><input v-model="form.userName" class="input w-full" minlength="3" maxlength="32" required /></label>
            <label class="fieldset"><span class="fieldset-legend">Email</span><input v-model="form.email" type="email" class="input w-full" required /></label>
            <label class="fieldset"><span class="fieldset-legend">Full name</span><input v-model="form.fullName" class="input w-full" /></label>
            <label class="fieldset"><span class="fieldset-legend">Phone</span><input v-model="form.phone" class="input w-full" /></label>
            <label class="fieldset md:col-span-2"><span class="fieldset-legend">New password</span><input v-model="form.password" type="password" class="input w-full" minlength="8" maxlength="128" autocomplete="new-password" placeholder="Leave blank to keep current password" /></label>
          </div>
          <div class="card-actions justify-end"><button type="button" class="btn btn-ghost" @click="editing = false">Cancel</button><button type="submit" class="btn btn-primary" :disabled="busy">Save changes</button></div>
        </form>
      </section>

      <div>
        <h2 class="mb-3 text-lg font-bold">Posts</h2>
        <div class="space-y-4">
          <PostCard v-for="post in posts" :key="post.id" :post="post" @deleted="id => { posts = posts.filter(item => item.id !== id); if (postCount) postCount -= 1; }" @updated="value => posts = posts.map(item => item.id === value.id ? value : item)" />
        </div>
        <p v-if="!posts.length" class="ax-muted py-12 text-center">No posts yet.</p>
        <div class="mt-4"><PaginationBar :offset="offset" :limit="LIMIT" :count="postCount" @change="value => { offset = value; loadPosts(); }" /></div>
      </div>
    </template>

    <dialog id="relationships-modal" class="modal">
      <div class="modal-box">
        <h3 class="text-lg font-bold">{{ relationship.title }}</h3>
        <div class="mt-4 space-y-1">
          <UserRow v-for="person in relationship.users" :key="person.id" :user="person" onclick="this.closest('dialog').close()" />
          <p v-if="!relationship.users.length" class="ax-muted py-10 text-center">Nobody yet.</p>
        </div>
        <div class="modal-action"><form method="dialog"><button class="btn">Close</button></form></div>
      </div>
      <form method="dialog" class="modal-backdrop"><button>Close</button></form>
    </dialog>
  </div>
</template>
