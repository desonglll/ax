<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { Search, ShieldCheck, Trash2, UserRound } from "lucide-vue-next";
import { userApi } from "../api";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { User } from "../types";

const auth = useAuthStore();
const toast = useToastStore();
const users = ref<User[]>([]);
const query = ref("");
const filtered = computed(() => users.value.filter(user => `${user.userName} ${user.fullName || ""} ${user.email}`.toLowerCase().includes(query.value.toLowerCase())));
const load = async () => { users.value = (await userApi.list({ limit: 100 })).body?.data || []; };
const toggle = async (user: User, field: "isAdmin" | "isActive") => {
  const response = await userApi.update(user.id, { [field]: !user[field] });
  if (response.body?.data) users.value = users.value.map(item => item.id === user.id ? response.body!.data : item);
  toast.show("User updated", "success");
};
const remove = async (user: User) => {
  if (!confirm(`Delete @${user.userName}?`)) return;
  await userApi.delete(user.id);
  users.value = users.value.filter(item => item.id !== user.id);
};
onMounted(load);
</script>

<template>
  <div class="ax-container space-y-6">
    <header class="flex flex-wrap items-end justify-between gap-4"><div><p class="ax-section-title">Community directory</p><h1 class="ax-page-title">People on Ax</h1></div><label class="input flex items-center gap-2"><Search :size="16" /><input v-model="query" placeholder="Find people" /></label></header>
    <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3"><article v-for="person in filtered" :key="person.id" class="ax-panel"><div class="card-body"><div class="flex items-start gap-3"><RouterLink :to="`/profile/${person.id}`" class="avatar placeholder"><div class="w-12 rounded-2xl bg-neutral text-neutral-content"><span>{{ person.userName.slice(0, 2).toUpperCase() }}</span></div></RouterLink><div class="min-w-0 flex-1"><RouterLink :to="`/profile/${person.id}`" class="font-bold hover:text-primary">{{ person.fullName || person.userName }}</RouterLink><p class="truncate text-sm ax-muted">@{{ person.userName }}</p></div><span v-if="person.isAdmin" class="badge badge-primary badge-sm"><ShieldCheck :size="12" /> Admin</span></div><p class="truncate text-sm ax-muted">{{ person.email }}</p><div class="card-actions mt-2"><RouterLink :to="`/profile/${person.id}`" class="btn btn-outline btn-sm flex-1"><UserRound :size="15" /> View profile</RouterLink><div v-if="auth.user?.isAdmin && auth.user.id !== person.id" class="dropdown dropdown-end"><button tabindex="0" class="btn btn-ghost btn-sm">Manage</button><ul tabindex="0" class="dropdown-content menu z-20 w-48 rounded-box border border-base-300 bg-base-100 p-2 shadow-xl"><li><button @click="toggle(person, 'isAdmin')">{{ person.isAdmin ? "Remove admin" : "Make admin" }}</button></li><li><button @click="toggle(person, 'isActive')">{{ person.isActive ? "Deactivate" : "Activate" }}</button></li><li><button class="text-error" @click="remove(person)"><Trash2 :size="14" /> Delete</button></li></ul></div></div></div></article></div>
  </div>
</template>
