<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useRoute, useRouter } from "vue-router";
import { Bell, Flame, Home, Languages, LogIn, LogOut, Menu, Moon, Search, Sun, UsersRound } from "lucide-vue-next";
import { locales, setLocale, type Locale } from "../i18n";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import Avatar from "./Avatar.vue";
import BottomNav from "./BottomNav.vue";
import NotificationBell from "./NotificationBell.vue";
import ScrollTop from "./ScrollTop.vue";
import { useNotificationStore } from "../stores/notifications";

const { t, locale } = useI18n();
const auth = useAuthStore();
const toast = useToastStore();
const router = useRouter();
const route = useRoute();
const query = ref(String(route.query.search || ""));
const dark = ref(false);
const drawer = ref(false);
const mobileSearch = ref(false);
const notifications = useNotificationStore();
watch(() => auth.authenticated, signedIn => (signedIn ? notifications.start() : notifications.stop()), { immediate: true });

watch(() => route.query.search, value => { query.value = String(value || ""); });

const nav = computed(() => [
  { to: "/", label: t("nav.home"), icon: Home },
  { to: "/trending", label: t("nav.trending"), icon: Flame },
  { to: "/people", label: t("nav.people"), icon: UsersRound },
  ...(auth.authenticated ? [{ to: "/notifications", label: t("nav.notifications"), icon: Bell }] : []),
]);

const applyTheme = (isDark: boolean) => {
  dark.value = isDark;
  document.documentElement.dataset.theme = isDark ? "axdark" : "axlight";
  try { localStorage.setItem("ax-theme", isDark ? "axdark" : "axlight"); } catch { /* storage unavailable */ }
};

// index.html applies the saved/system theme before first paint; mirror it here.
dark.value = document.documentElement.dataset.theme === "axdark";

const blur = () => (document.activeElement as HTMLElement | null)?.blur();
const searchInput = ref<HTMLInputElement>();

const titles: Record<string, string> = { home: "nav.home", trending: "nav.trending", people: "nav.people", notifications: "nav.notifications", profile: "nav.profile", login: "nav.signIn", register: "nav.register" };
watch([() => route.name, locale], () => {
  const key = titles[String(route.name)];
  document.title = key ? `${t(key)} · ${t("app.name")}` : t("app.name");
}, { immediate: true });

const onKey = (event: KeyboardEvent) => {
  const target = event.target as HTMLElement | null;
  const typing = target && (target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.isContentEditable);
  if (event.key === "/" && !typing && !event.metaKey && !event.ctrlKey) {
    event.preventDefault();
    searchInput.value?.focus();
  }
};
onMounted(() => window.addEventListener("keydown", onKey));
onBeforeUnmount(() => window.removeEventListener("keydown", onKey));
const chooseLocale = (value: Locale) => { setLocale(value); blur(); };

const search = () => {
  const value = query.value.trim();
  drawer.value = false;
  mobileSearch.value = false;
  router.push({ path: "/", query: value ? { search: value } : {} });
};

const logout = async () => {
  await auth.logout();
  toast.show(t("auth.signedOut"), "success");
  router.push("/");
};
</script>

<template>
  <div class="drawer lg:drawer-open">
    <input id="ax-drawer" v-model="drawer" type="checkbox" class="drawer-toggle" />
    <div class="drawer-content min-h-screen">
      <header class="navbar sticky top-0 z-40 border-b border-base-300 bg-base-100/95 px-3 backdrop-blur md:px-6">
        <div class="navbar-start gap-2">
          <label for="ax-drawer" class="btn btn-ghost btn-circle lg:hidden" :aria-label="t('nav.openMenu')"><Menu :size="21" /></label>
          <RouterLink to="/" class="text-lg font-bold lg:hidden">{{ t("app.name") }}</RouterLink>
        </div>
        <div class="navbar-center hidden w-full max-w-xl md:flex">
          <form class="join w-full" role="search" @submit.prevent="search">
            <label class="input join-item flex w-full items-center gap-2">
              <Search :size="17" class="text-base-content/45" />
              <input ref="searchInput" v-model="query" type="search" class="grow" :placeholder="t('nav.search')" />
              <kbd class="kbd kbd-xs hidden lg:inline-flex">/</kbd>
            </label>
            <button class="btn join-item" type="submit">{{ t("nav.searchButton") }}</button>
          </form>
        </div>
        <div class="navbar-end gap-1">
          <button class="btn btn-ghost btn-circle md:hidden" :aria-label="t('nav.openSearch')" @click="mobileSearch = !mobileSearch"><Search :size="19" /></button>
          <div class="dropdown dropdown-end hidden md:block">
            <button tabindex="0" class="btn btn-ghost btn-circle" :aria-label="t('nav.language')"><Languages :size="19" /></button>
            <ul tabindex="0" class="dropdown-content menu z-50 mt-2 w-40 rounded-box border border-base-300 bg-base-100 p-2 shadow-xl">
              <li v-for="item in locales" :key="item.value"><button :class="{ 'menu-active': locale === item.value }" @click="chooseLocale(item.value)">{{ item.label }}</button></li>
            </ul>
          </div>
          <button class="btn btn-ghost btn-circle hidden md:inline-flex" :aria-label="dark ? t('nav.lightTheme') : t('nav.darkTheme')" @click="applyTheme(!dark)">
            <Transition name="spin" mode="out-in"><Sun v-if="dark" :size="19" /><Moon v-else :size="19" /></Transition>
          </button>
          <NotificationBell v-if="auth.authenticated" />
          <RouterLink v-if="auth.user" :to="`/profile/${auth.user.id}`" class="btn btn-ghost btn-circle" :aria-label="t('nav.yourProfile')"><Avatar :name="auth.user.userName" size="sm" tone="primary" /></RouterLink>
          <RouterLink v-else :to="{ name: 'login', query: { redirect: route.fullPath } }" class="btn btn-primary btn-sm"><LogIn :size="16" /> {{ t("nav.signIn") }}</RouterLink>
        </div>
        <Transition name="collapse">
          <form v-if="mobileSearch" class="absolute inset-x-0 top-full border-b border-base-300 bg-base-100 p-3 md:hidden" role="search" @submit.prevent="search">
            <label class="input flex w-full items-center gap-2"><Search :size="16" class="text-base-content/45" /><input v-model="query" type="search" class="grow" :placeholder="t('nav.search')" autofocus /></label>
          </form>
        </Transition>
      </header>
      <main class="pb-24 pt-5 md:py-8"><slot /></main>
      <BottomNav />
      <ScrollTop />
    </div>

    <aside class="drawer-side z-50 border-r border-base-300">
      <label for="ax-drawer" class="drawer-overlay" :aria-label="t('nav.closeMenu')"></label>
      <div class="flex min-h-full w-64 flex-col bg-base-100 p-4">
        <RouterLink to="/" class="mb-6 px-2 text-2xl font-bold" @click="drawer = false">{{ t("app.name") }}</RouterLink>
        <ul class="menu w-full gap-1 p-0">
          <li v-for="item in nav" :key="item.to">
            <RouterLink :to="item.to" active-class="menu-active" @click="drawer = false"><component :is="item.icon" :size="19" />{{ item.label }}</RouterLink>
          </li>
        </ul>
        <div class="mt-auto space-y-3 border-t border-base-300 pt-4">
          <div class="flex items-center gap-2 md:hidden">
            <select class="select select-sm flex-1" :value="locale" :aria-label="t('nav.language')" @change="chooseLocale(($event.target as HTMLSelectElement).value as Locale)">
              <option v-for="item in locales" :key="item.value" :value="item.value">{{ item.label }}</option>
            </select>
            <button class="btn btn-ghost btn-sm btn-square" :aria-label="dark ? t('nav.lightTheme') : t('nav.darkTheme')" @click="applyTheme(!dark)"><Sun v-if="dark" :size="18" /><Moon v-else :size="18" /></button>
          </div>
          <RouterLink v-if="auth.user" :to="`/profile/${auth.user.id}`" class="flex items-center gap-3 rounded-box p-2 hover:bg-base-200" @click="drawer = false">
            <Avatar :name="auth.user.userName" size="sm" />
            <div class="min-w-0 flex-1"><strong class="block truncate text-sm">{{ auth.user.userName }}</strong><small class="ax-muted">{{ auth.user.isAdmin ? t("nav.admin") : t("nav.member") }}</small></div>
          </RouterLink>
          <button v-if="auth.user" class="btn btn-ghost btn-block justify-start" @click="logout"><LogOut :size="18" /> {{ t("nav.signOut") }}</button>
          <div v-else class="grid grid-cols-2 gap-2">
            <RouterLink to="/login" class="btn btn-outline btn-sm" @click="drawer = false">{{ t("nav.signIn") }}</RouterLink>
            <RouterLink to="/register" class="btn btn-primary btn-sm" @click="drawer = false">{{ t("nav.register") }}</RouterLink>
          </div>
        </div>
      </div>
    </aside>
  </div>
</template>
