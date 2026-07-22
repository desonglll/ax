<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { CalendarDays, LoaderCircle, Pencil, UserMinus, UserPlus, UsersRound } from "lucide-vue-next";
import { followApi, postApi, userApi } from "../api";
import PostCard from "../components/PostCard.vue";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { FollowStats, Post, User } from "../types";

const route = useRoute();
const router = useRouter();
const auth = useAuthStore();
const toast = useToastStore();
const profile = ref<User>();
const posts = ref<Post[]>([]);
const follows = ref<FollowStats>();
const relationshipUsers = ref<User[]>([]);
const relationshipTitle = ref("");
const loading = ref(true);
const busy = ref(false);
const editing = ref(false);
const form = ref({ userName: "", email: "", fullName: "", phone: "", password: "" });
const targetId = computed(() => Number(route.params.id || auth.user?.id || 0));
const own = computed(() => auth.user?.id === targetId.value);

const load = async () => {
  if (!targetId.value) return router.push("/login");
  loading.value = true;
  try {
    const [userResponse, postsResponse, statsResponse] = await Promise.all([
      own.value ? userApi.profile() : userApi.get(targetId.value),
      postApi.list({ user_id: targetId.value, limit: 100, order_by: "created_at", sort: "desc" }),
      followApi.stats(targetId.value),
    ]);
    profile.value = userResponse.body?.data;
    posts.value = postsResponse.body?.data || [];
    follows.value = statsResponse.body?.data;
    if (profile.value) form.value = { userName: profile.value.userName, email: profile.value.email, fullName: profile.value.fullName || "", phone: profile.value.phone || "", password: "" };
  } finally { loading.value = false; }
};

const toggleFollow = async () => {
  if (!auth.user) return router.push("/login");
  busy.value = true;
  try { follows.value = (follows.value?.isFollowing ? await followApi.unfollow(targetId.value) : await followApi.follow(targetId.value)).body?.data; }
  finally { busy.value = false; }
};

const showRelationships = async (kind: "followers" | "following") => {
  relationshipTitle.value = kind === "followers" ? "Followers" : "Following";
  relationshipUsers.value = (kind === "followers" ? await followApi.followers(targetId.value, { limit: 100 }) : await followApi.following(targetId.value, { limit: 100 })).body?.data || [];
  (document.getElementById("relationships-modal") as HTMLDialogElement).showModal();
};

const save = async () => {
  if (!profile.value) return;
  busy.value = true;
  try {
    const payload = { ...form.value, password: form.value.password || undefined };
    const response = await userApi.update(profile.value.id, payload);
    profile.value = response.body?.data;
    editing.value = false;
    await auth.restore();
    toast.show("Profile updated", "success");
  } finally { busy.value = false; }
};

onMounted(load);
watch(() => route.params.id, load);
</script>

<template>
  <div class="ax-container max-w-5xl space-y-6">
    <div v-if="loading" class="grid place-items-center py-24"><LoaderCircle class="animate-spin" /></div>
    <template v-else-if="profile">
      <section class="overflow-hidden rounded-box border border-base-300 bg-base-100 shadow-sm">
        <div class="h-32 bg-gradient-to-br from-primary via-secondary to-accent md:h-44"></div>
        <div class="px-5 pb-5 md:px-8">
          <div class="-mt-12 flex flex-wrap items-end justify-between gap-4">
            <div class="avatar placeholder"><div class="w-24 rounded-3xl border-4 border-base-100 bg-neutral text-3xl text-neutral-content"><span>{{ profile.userName.slice(0, 2).toUpperCase() }}</span></div></div>
            <div class="flex gap-2">
              <button v-if="own" class="btn btn-outline btn-sm" @click="editing = !editing"><Pencil :size="16" /> Edit profile</button>
              <button v-else class="btn btn-primary btn-sm" :disabled="busy" @click="toggleFollow"><UserMinus v-if="follows?.isFollowing" :size="16" /><UserPlus v-else :size="16" />{{ follows?.isFollowing ? "Unfollow" : "Follow" }}</button>
            </div>
          </div>
          <div class="mt-4"><div class="flex items-center gap-2"><h1 class="text-3xl font-black">{{ profile.fullName || profile.userName }}</h1><span v-if="profile.isAdmin" class="badge badge-primary">Admin</span></div><p class="text-base-content/55">@{{ profile.userName }}</p><p v-if="profile.phone && own" class="mt-2 text-sm">{{ profile.phone }}</p><p class="mt-3 flex items-center gap-2 text-sm ax-muted"><CalendarDays :size="15" /> Joined {{ profile.createdAt ? new Date(profile.createdAt).toLocaleDateString() : "recently" }}</p></div>
          <div class="mt-5 flex gap-5 text-sm"><button class="hover:text-primary" @click="showRelationships('followers')"><strong>{{ follows?.followersCount || 0 }}</strong> <span class="ax-muted">followers</span></button><button class="hover:text-primary" @click="showRelationships('following')"><strong>{{ follows?.followingCount || 0 }}</strong> <span class="ax-muted">following</span></button><span><strong>{{ posts.length }}</strong> <span class="ax-muted">posts</span></span></div>
        </div>
      </section>
      <section v-if="editing" class="ax-panel"><div class="card-body"><h2 class="card-title">Edit profile</h2><div class="grid gap-4 md:grid-cols-2"><label class="fieldset"><span class="fieldset-legend">Username</span><input v-model="form.userName" class="input w-full" /></label><label class="fieldset"><span class="fieldset-legend">Email</span><input v-model="form.email" type="email" class="input w-full" /></label><label class="fieldset"><span class="fieldset-legend">Full name</span><input v-model="form.fullName" class="input w-full" /></label><label class="fieldset"><span class="fieldset-legend">Phone</span><input v-model="form.phone" class="input w-full" /></label><label class="fieldset md:col-span-2"><span class="fieldset-legend">New password</span><input v-model="form.password" type="password" class="input w-full" placeholder="Leave blank to keep current password" /></label></div><div class="card-actions justify-end"><button class="btn btn-ghost" @click="editing = false">Cancel</button><button class="btn btn-primary" :disabled="busy" @click="save">Save changes</button></div></div></section>
      <div><h2 class="mb-4 text-xl font-black">Posts</h2><div class="space-y-4"><PostCard v-for="post in posts" :key="post.id" :post="post" @deleted="id => posts = posts.filter(item => item.id !== id)" /></div><p v-if="!posts.length" class="py-16 text-center ax-muted">No posts yet.</p></div>
    </template>
    <dialog id="relationships-modal" class="modal"><div class="modal-box"><h3 class="text-lg font-bold">{{ relationshipTitle }}</h3><div class="mt-4 space-y-2"><RouterLink v-for="person in relationshipUsers" :key="person.id" :to="`/profile/${person.id}`" class="flex items-center gap-3 rounded-box p-3 hover:bg-base-200" onclick="this.closest('dialog').close()"><div class="avatar placeholder"><div class="w-10 rounded-full bg-neutral text-neutral-content"><span>{{ person.userName.slice(0, 2).toUpperCase() }}</span></div></div><div><strong>{{ person.userName }}</strong><small class="block ax-muted">{{ person.fullName || "Ax member" }}</small></div></RouterLink><p v-if="!relationshipUsers.length" class="py-10 text-center ax-muted">No one here yet.</p></div><div class="modal-action"><form method="dialog"><button class="btn">Close</button></form></div></div><form method="dialog" class="modal-backdrop"><button>Close</button></form></dialog>
  </div>
</template>
