<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Bell, Flame, Home, LogIn, LogOut, Menu, Moon, Search, Sun, UsersRound } from "lucide-vue-next";
import NotificationBell from "./NotificationBell.vue";
import Avatar from "./Avatar.vue";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";

const auth = useAuthStore();
const toast = useToastStore();
const router = useRouter();
const route = useRoute();
const query = ref(String(route.query.search || ""));
const dark = ref(false);
const drawer = ref(false);

watch(() => route.query.search, value => { query.value = String(value || ""); });

const nav = computed(() => [
  { to: "/", label: "Home", icon: Home },
  { to: "/trending", label: "Trending", icon: Flame },
  { to: "/people", label: "People", icon: UsersRound },
  ...(auth.authenticated ? [{ to: "/notifications", label: "Notifications", icon: Bell }] : []),
]);

const applyTheme = (isDark: boolean) => {
  dark.value = isDark;
  document.documentElement.dataset.theme = isDark ? "axdark" : "axlight";
  try { localStorage.setItem("ax-theme", isDark ? "axdark" : "axlight"); } catch { /* storage unavailable */ }
};

onMounted(() => {
  let saved: string | null = null;
  try { saved = localStorage.getItem("ax-theme"); } catch { /* storage unavailable */ }
  applyTheme(saved ? saved === "axdark" : window.matchMedia("(prefers-color-scheme: dark)").matches);
});

const search = () => {
  const value = query.value.trim();
  drawer.value = false;
  router.push({ path: "/", query: value ? { search: value } : {} });
};

const logout = async () => {
  await auth.logout();
  toast.show("Signed out", "success");
  router.push("/");
};
</script>

<template>
  <div class="drawer lg:drawer-open">
    <input id="ax-drawer" v-model="drawer" type="checkbox" class="drawer-toggle" />
    <div class="drawer-content min-h-screen">
      <header class="navbar sticky top-0 z-40 border-b border-base-300 bg-base-100 px-3 md:px-6">
        <div class="navbar-start gap-2">
          <label for="ax-drawer" class="btn btn-ghost btn-circle lg:hidden" aria-label="Open navigation"><Menu :size="21" /></label>
          <RouterLink to="/" class="text-lg font-bold lg:hidden">Ax</RouterLink>
        </div>
        <div class="navbar-center hidden w-full max-w-xl md:flex">
          <form class="join w-full" role="search" @submit.prevent="search">
            <label class="input join-item flex w-full items-center gap-2">
              <Search :size="17" class="text-base-content/45" />
              <input v-model="query" type="search" class="grow" placeholder="Search posts" />
            </label>
            <button class="btn join-item" type="submit">Search</button>
          </form>
        </div>
        <div class="navbar-end gap-1">
          <button class="btn btn-ghost btn-circle" :aria-label="dark ? 'Light theme' : 'Dark theme'" @click="applyTheme(!dark)">
            <Sun v-if="dark" :size="19" /><Moon v-else :size="19" />
          </button>
          <NotificationBell v-if="auth.authenticated" />
          <RouterLink v-if="auth.user" :to="`/profile/${auth.user.id}`" class="btn btn-ghost btn-circle" aria-label="Your profile"><Avatar :name="auth.user.userName" size="sm" tone="primary" /></RouterLink>
          <RouterLink v-else :to="{ name: 'login', query: { redirect: route.fullPath } }" class="btn btn-primary btn-sm"><LogIn :size="16" /> Sign in</RouterLink>
        </div>
      </header>
      <main class="py-6 md:py-8"><slot /></main>
    </div>

    <aside class="drawer-side z-50 border-r border-base-300">
      <label for="ax-drawer" class="drawer-overlay" aria-label="Close navigation"></label>
      <div class="flex min-h-full w-64 flex-col bg-base-100 p-4">
        <RouterLink to="/" class="mb-6 px-2 text-2xl font-bold" @click="drawer = false">Ax</RouterLink>
        <form class="mb-4 md:hidden" role="search" @submit.prevent="search">
          <label class="input flex w-full items-center gap-2"><Search :size="16" class="text-base-content/45" /><input v-model="query" type="search" class="grow" placeholder="Search posts" /></label>
        </form>
        <ul class="menu w-full gap-1 p-0">
          <li v-for="item in nav" :key="item.to">
            <RouterLink :to="item.to" active-class="menu-active" @click="drawer = false"><component :is="item.icon" :size="19" />{{ item.label }}</RouterLink>
          </li>
        </ul>
        <div class="mt-auto border-t border-base-300 pt-4">
          <RouterLink v-if="auth.user" :to="`/profile/${auth.user.id}`" class="mb-3 flex items-center gap-3 rounded-box p-2 hover:bg-base-200" @click="drawer = false">
            <Avatar :name="auth.user.userName" size="sm" />
            <div class="min-w-0 flex-1"><strong class="block truncate text-sm">{{ auth.user.userName }}</strong><small class="ax-muted">{{ auth.user.isAdmin ? "Admin" : "Member" }}</small></div>
          </RouterLink>
          <button v-if="auth.user" class="btn btn-ghost btn-block justify-start" @click="logout"><LogOut :size="18" /> Sign out</button>
          <div v-else class="grid grid-cols-2 gap-2">
            <RouterLink to="/login" class="btn btn-outline btn-sm" @click="drawer = false">Sign in</RouterLink>
            <RouterLink to="/register" class="btn btn-primary btn-sm" @click="drawer = false">Register</RouterLink>
          </div>
        </div>
      </div>
    </aside>
  </div>
</template>
