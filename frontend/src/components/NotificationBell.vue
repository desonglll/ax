<script setup lang="ts">
import { Bell, CheckCheck, Heart, LoaderCircle, MessageCircle, UserPlus } from "lucide-vue-next";
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import { notificationApi } from "../api";
import { getApiError } from "../api/client";
import { timeAgo } from "../lib/format";
import { useNotificationStore } from "../stores/notifications";
import type { Notification } from "../types";

const { t } = useI18n();
const router = useRouter();
const store = useNotificationStore();
const root = ref<HTMLElement>();
const items = ref<Notification[]>([]);
const loading = ref(false);
const error = ref("");
const icons = { follow: UserPlus, comment: MessageCircle, reaction: Heart };

const load = async () => {
  loading.value = true;
  error.value = "";
  try {
    items.value = (await notificationApi.list({ limit: 6 })).body?.data || [];
  } catch (err) {
    error.value = getApiError(err, t("errors.loadNotifications"));
  } finally {
    loading.value = false;
  }
  store.refresh();
};

/**
 * daisyUI opens the dropdown on focus and then disables pointer events on
 * the trigger, so a click handler on the button never fires. Load whenever
 * focus first enters the dropdown instead (mouse, touch or keyboard).
 */
const onFocusIn = (event: FocusEvent) => {
  if (!root.value?.contains(event.relatedTarget as Node | null)) load();
};

/** daisyUI dropdowns stay open while focused; blur closes them. */
const close = () => (document.activeElement as HTMLElement | null)?.blur();

const open = async (item: Notification) => {
  close();
  router.push(item.postId ? `/posts/${item.postId}` : `/profile/${item.actorId}`);
  if (!item.isRead) {
    item.isRead = true;
    await store.markRead(item.id);
  }
};

const readAll = async () => {
  await store.markAllRead();
  items.value = items.value.map(item => ({ ...item, isRead: true }));
};
</script>

<template>
  <div ref="root" class="dropdown dropdown-end" @focusin="onFocusIn">
    <button tabindex="0" class="btn btn-ghost btn-circle" :aria-label="store.unread ? `${t('nav.notifications')} (${store.unread})` : t('nav.notifications')">
      <div class="indicator"><Bell :size="19" /><Transition name="pop"><span v-if="store.unread" class="badge badge-error badge-xs indicator-item">{{ store.unread > 99 ? "99+" : store.unread }}</span></Transition></div>
    </button>
    <div tabindex="0" class="dropdown-content card card-border z-50 mt-3 w-80 max-w-[calc(100vw-1.5rem)] bg-base-100 shadow-2xl">
      <div class="card-body gap-1 p-2">
        <div class="flex items-center justify-between px-2 py-1">
          <h3 class="font-bold">{{ t("notifications.title") }}</h3>
          <button class="btn btn-ghost btn-xs" :disabled="!store.unread" @click="readAll"><CheckCheck :size="15" /> {{ t("notifications.readAll") }}</button>
        </div>
        <div v-if="loading && !items.length" class="grid place-items-center py-8"><LoaderCircle class="animate-spin text-base-content/40" /></div>
        <div v-else-if="error && !items.length" class="flex flex-col items-center gap-2 py-6 text-sm text-base-content/60">
          <span>{{ error }}</span>
          <button class="btn btn-ghost btn-xs" @click="load">{{ t("common.retry") }}</button>
        </div>
        <template v-else>
          <button v-for="item in items" :key="item.id" class="flex items-start gap-3 rounded-box p-2.5 text-left transition hover:bg-base-200" :class="{ 'bg-primary/8': !item.isRead }" @click="open(item)">
            <span class="grid size-8 shrink-0 place-items-center rounded-full" :class="item.isRead ? 'bg-base-200 text-base-content/60' : 'bg-primary text-primary-content'"><component :is="icons[item.kind]" :size="15" /></span>
            <span class="min-w-0 flex-1">
              <span class="block text-sm"><strong>{{ item.actorName || t("notifications.user", { id: item.actorId }) }}</strong> {{ t(`notifications.${item.kind}`) }}</span>
              <small class="mt-0.5 block text-base-content/50">{{ timeAgo(item.createdAt) }}</small>
            </span>
            <span v-if="!item.isRead" class="status status-primary mt-2"></span>
          </button>
          <p v-if="!items.length" class="py-8 text-center text-sm text-base-content/55">{{ t("notifications.empty") }}</p>
        </template>
        <RouterLink to="/notifications" class="btn btn-ghost btn-sm mt-1" @click="close">{{ t("notifications.viewAll") }}</RouterLink>
      </div>
    </div>
  </div>
</template>
