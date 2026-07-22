<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Bell, FileArchive, Flame, Gauge, Home, LogIn, LogOut, Menu, Moon, Search, Sun, UserRound, UsersRound } from "lucide-vue-next";
import NotificationBell from "./NotificationBell.vue";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";

const auth = useAuthStore();
const toast = useToastStore();
const router = useRouter();
const route = useRoute();
const query = ref("");
const dark = ref(false);
const drawer = ref(false);

const nav = computed(() => [
  { to: "/", label: "Home", icon: Home },
  { to: "/trending", label: "Trending", icon: Flame },
  { to: "/people", label: "People", icon: UsersRound },
  { to: "/files", label: "Files", icon: FileArchive },
  ...(auth.authenticated ? [{ to: "/notifications", label: "Notifications", icon: Bell }] : []),
  ...(auth.user?.isAdmin ? [{ to: "/system", label: "System", icon: Gauge }] : []),
]);

onMounted(() => {
  dark.value = localStorage.getItem("ax-theme") === "axdark";
  document.documentElement.dataset.theme = dark.value ? "axdark" : "axlight";
});

const toggleTheme = () => {
  dark.value = !dark.value;
  const theme = dark.value ? "axdark" : "axlight";
  document.documentElement.dataset.theme = theme;
  localStorage.setItem("ax-theme", theme);
};

const search = () => {
  const value = query.value.trim();
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
      <header class="navbar sticky top-0 z-40 border-b border-base-300 bg-base-100/90 px-3 backdrop-blur-xl md:px-6">
        <div class="navbar-start gap-2">
          <label for="ax-drawer" class="btn btn-ghost btn-circle lg:hidden" aria-label="Open navigation"><Menu :size="21" /></label>
          <RouterLink to="/" class="flex items-center gap-2 font-black tracking-tight lg:hidden">
            <span class="grid size-9 place-items-center rounded-xl bg-primary text-lg text-primary-content">A</span> Ax
          </RouterLink>
        </div>
        <div class="navbar-center hidden w-full max-w-xl md:flex">
          <form class="join w-full" @submit.prevent="search">
            <label class="input join-item flex w-full items-center gap-2">
              <Search :size="17" class="text-base-content/45" />
              <input v-model="query" type="search" class="grow" placeholder="Search posts, ideas, conversations" />
            </label>
            <button class="btn btn-primary join-item" type="submit">Search</button>
          </form>
        </div>
        <div class="navbar-end gap-1">
          <button class="btn btn-ghost btn-circle" aria-label="Toggle theme" @click="toggleTheme">
            <Sun v-if="dark" :size="19" /><Moon v-else :size="19" />
          </button>
          <NotificationBell v-if="auth.authenticated" />
          <RouterLink v-if="auth.user" :to="`/profile/${auth.user.id}`" class="btn btn-ghost btn-circle avatar placeholder" aria-label="Profile">
            <div class="w-9 rounded-full bg-primary text-primary-content"><span>{{ auth.user.userName.slice(0, 2).toUpperCase() }}</span></div>
          </RouterLink>
          <RouterLink v-else to="/login" class="btn btn-primary btn-sm"><LogIn :size="16" /> Sign in</RouterLink>
        </div>
      </header>
      <main :key="route.fullPath" class="py-6 md:py-8"><slot /></main>
    </div>
    <aside class="drawer-side z-50 border-r border-base-300">
      <label for="ax-drawer" class="drawer-overlay" aria-label="Close navigation"></label>
      <div class="flex min-h-full w-72 flex-col bg-base-100 p-4">
        <RouterLink to="/" class="mb-8 flex items-center gap-3 px-2" @click="drawer = false">
          <span class="grid size-11 place-items-center rounded-2xl bg-primary text-xl font-black text-primary-content shadow-lg shadow-primary/20">A</span>
          <span><strong class="block text-xl leading-none">Ax Social</strong><small class="ax-muted">Create with clarity</small></span>
        </RouterLink>
        <ul class="menu w-full gap-1 p-0">
          <li v-for="item in nav" :key="item.to">
            <RouterLink :to="item.to" active-class="menu-active" @click="drawer = false"><component :is="item.icon" :size="19" />{{ item.label }}</RouterLink>
          </li>
        </ul>
        <div class="mt-auto border-t border-base-300 pt-4">
          <div v-if="auth.user" class="mb-3 flex items-center gap-3 rounded-box bg-base-200 p-3">
            <div class="avatar placeholder"><div class="w-10 rounded-full bg-neutral text-neutral-content"><span>{{ auth.user.userName.slice(0, 2).toUpperCase() }}</span></div></div>
            <div class="min-w-0 flex-1"><strong class="block truncate text-sm">{{ auth.user.userName }}</strong><span class="badge badge-primary badge-xs">{{ auth.user.isAdmin ? "Admin" : "Member" }}</span></div>
          </div>
          <button v-if="auth.user" class="btn btn-ghost btn-block justify-start" @click="logout"><LogOut :size="18" /> Sign out</button>
          <div v-else class="grid grid-cols-2 gap-2">
            <RouterLink to="/login" class="btn btn-outline btn-sm"><UserRound :size="16" /> Login</RouterLink>
            <RouterLink to="/register" class="btn btn-primary btn-sm">Join Ax</RouterLink>
          </div>
        </div>
      </div>
    </aside>
  </div>
</template>
