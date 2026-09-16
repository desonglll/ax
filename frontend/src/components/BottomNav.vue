<script setup lang="ts">
import { Bell, Flame, Home, UserRound, UsersRound } from "lucide-vue-next";
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { useRoute } from "vue-router";
import { useAuthStore } from "../stores/auth";
import { useNotificationStore } from "../stores/notifications";

/** Phone-only bottom tab bar. */
const { t } = useI18n();
const auth = useAuthStore();
const route = useRoute();
const notifications = useNotificationStore();

const items = computed(() => [
  { to: "/", label: t("nav.home"), icon: Home, active: route.name === "home" },
  { to: "/trending", label: t("nav.trending"), icon: Flame, active: route.name === "trending" },
  { to: "/people", label: t("nav.people"), icon: UsersRound, active: route.name === "people" },
  ...(auth.user
    ? [
        { to: "/notifications", label: t("nav.notifications"), icon: Bell, active: route.name === "notifications", badge: notifications.unread },
        { to: `/profile/${auth.user.id}`, label: t("nav.profile"), icon: UserRound, active: route.name === "profile" },
      ]
    : [{ to: "/login", label: t("nav.signIn"), icon: UserRound, active: route.name === "login" }]),
]);
</script>

<template>
  <nav class="dock dock-sm border-t border-base-300 bg-base-100/95 pb-[env(safe-area-inset-bottom)] backdrop-blur md:hidden" aria-label="Primary">
    <RouterLink v-for="item in items" :key="item.to" :to="item.to" :class="{ 'dock-active': item.active }">
      <span class="indicator">
        <component :is="item.icon" :size="20" />
        <span v-if="'badge' in item && item.badge" class="badge badge-error badge-xs indicator-item">{{ item.badge > 99 ? "99+" : item.badge }}</span>
      </span>
      <span class="dock-label">{{ item.label }}</span>
    </RouterLink>
  </nav>
</template>
