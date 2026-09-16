<script setup lang="ts">
import { Bell, CheckCheck, LoaderCircle } from "lucide-vue-next";
import { onBeforeUnmount, onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { notificationApi } from "../api";
import { timeAgo } from "../lib/format";
import { describe } from "../lib/notifications";
import type { Notification } from "../types";

const router = useRouter();
const unread = ref(0);
const items = ref<Notification[]>([]);
const loading = ref(false);
let timer = 0;

const refresh = async () => {
  const response = await notificationApi.unread();
  unread.value = response.body?.data || 0;
};

const load = async () => {
  loading.value = true;
  try {
    const response = await notificationApi.list({ limit: 6 });
    items.value = response.body?.data || [];
  } finally {
    loading.value = false;
  }
};

/** daisyUI dropdowns stay open while focused; blur closes them. */
const close = () => (document.activeElement as HTMLElement | null)?.blur();

const open = async (item: Notification) => {
  if (!item.isRead) {
    await notificationApi.read(item.id).catch(() => undefined);
    item.isRead = true;
    unread.value = Math.max(0, unread.value - 1);
  }
  close();
  router.push(item.postId ? `/posts/${item.postId}` : `/profile/${item.actorId}`);
};

const readAll = async () => {
  await notificationApi.readAll();
  unread.value = 0;
  items.value = items.value.map(item => ({ ...item, isRead: true }));
};

onMounted(() => {
  refresh().catch(() => undefined);
  timer = window.setInterval(() => refresh().catch(() => undefined), 60_000);
});
onBeforeUnmount(() => window.clearInterval(timer));
</script>

<template>
  <div class="dropdown dropdown-end">
    <button tabindex="0" class="btn btn-ghost btn-circle" aria-label="Notifications" @click="load">
      <div class="indicator"><Bell :size="19" /><span v-if="unread" class="badge badge-error badge-xs indicator-item">{{ unread > 99 ? "99+" : unread }}</span></div>
    </button>
    <div tabindex="0" class="dropdown-content card card-border z-50 mt-3 w-80 bg-base-100 shadow-2xl">
      <div class="card-body gap-2 p-3">
        <div class="flex items-center justify-between px-1"><h3 class="font-bold">Notifications</h3><button class="btn btn-ghost btn-xs" :disabled="!unread" @click="readAll"><CheckCheck :size="15" /> Read all</button></div>
        <div v-if="loading" class="grid place-items-center py-8"><LoaderCircle class="animate-spin" /></div>
        <template v-else>
          <button v-for="item in items" :key="item.id" class="rounded-box p-3 text-left transition hover:bg-base-200" :class="{ 'bg-primary/8': !item.isRead }" @click="open(item)">
            <span class="text-sm"><strong>{{ item.actorName || `User ${item.actorId}` }}</strong> {{ describe(item) }}</span>
            <small class="mt-1 block text-base-content/50">{{ timeAgo(item.createdAt) }}</small>
          </button>
          <p v-if="!items.length" class="py-8 text-center text-sm text-base-content/55">You're all caught up.</p>
        </template>
        <RouterLink to="/notifications" class="btn btn-ghost btn-sm" @click="close">View all</RouterLink>
      </div>
    </div>
  </div>
</template>
