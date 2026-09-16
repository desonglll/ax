<script setup lang="ts">
import { Bell, CheckCheck, LoaderCircle } from "lucide-vue-next";
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import { notificationApi } from "../api";
import { timeAgo } from "../lib/format";
import { useNotificationStore } from "../stores/notifications";
import type { Notification } from "../types";

const { t } = useI18n();
const router = useRouter();
const store = useNotificationStore();
const items = ref<Notification[]>([]);
const loading = ref(false);

const load = async () => {
  loading.value = true;
  try {
    items.value = (await notificationApi.list({ limit: 6 })).body?.data || [];
  } finally {
    loading.value = false;
  }
};

/** daisyUI dropdowns stay open while focused; blur closes them. */
const close = () => (document.activeElement as HTMLElement | null)?.blur();

const open = async (item: Notification) => {
  if (!item.isRead) {
    item.isRead = true;
    await store.markRead(item.id);
  }
  close();
  router.push(item.postId ? `/posts/${item.postId}` : `/profile/${item.actorId}`);
};

const readAll = async () => {
  await store.markAllRead();
  items.value = items.value.map(item => ({ ...item, isRead: true }));
};
</script>

<template>
  <div class="dropdown dropdown-end">
    <button tabindex="0" class="btn btn-ghost btn-circle" :aria-label="t('nav.notifications')" @click="load">
      <div class="indicator"><Bell :size="19" /><Transition name="pop"><span v-if="store.unread" class="badge badge-error badge-xs indicator-item">{{ store.unread > 99 ? "99+" : store.unread }}</span></Transition></div>
    </button>
    <div tabindex="0" class="dropdown-content card card-border z-50 mt-3 w-80 max-w-[calc(100vw-1.5rem)] bg-base-100 shadow-2xl">
      <div class="card-body gap-2 p-3">
        <div class="flex items-center justify-between px-1"><h3 class="font-bold">{{ t("notifications.title") }}</h3><button class="btn btn-ghost btn-xs" :disabled="!store.unread" @click="readAll"><CheckCheck :size="15" /> {{ t("notifications.readAll") }}</button></div>
        <div v-if="loading" class="grid place-items-center py-8"><LoaderCircle class="animate-spin" /></div>
        <template v-else>
          <button v-for="item in items" :key="item.id" class="rounded-box p-3 text-left transition hover:bg-base-200" :class="{ 'bg-primary/8': !item.isRead }" @click="open(item)">
            <span class="text-sm"><strong>{{ item.actorName || t("notifications.user", { id: item.actorId }) }}</strong> {{ t(`notifications.${item.kind}`) }}</span>
            <small class="mt-1 block text-base-content/50">{{ timeAgo(item.createdAt) }}</small>
          </button>
          <p v-if="!items.length" class="py-8 text-center text-sm text-base-content/55">{{ t("notifications.empty") }}</p>
        </template>
        <RouterLink to="/notifications" class="btn btn-ghost btn-sm" @click="close">{{ t("notifications.viewAll") }}</RouterLink>
      </div>
    </div>
  </div>
</template>
