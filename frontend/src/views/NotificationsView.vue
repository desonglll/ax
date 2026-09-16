<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { Bell, CheckCheck, Heart, LoaderCircle, MessageCircle, UserPlus } from "lucide-vue-next";
import { useRouter } from "vue-router";
import { notificationApi } from "../api";
import { getApiError } from "../api/client";
import EmptyState from "../components/EmptyState.vue";
import PaginationBar from "../components/PaginationBar.vue";
import { timeAgo } from "../lib/format";
import { describe } from "../lib/notifications";
import { useToastStore } from "../stores/toast";
import type { Notification } from "../types";

const LIMIT = 20;
const router = useRouter();
const toast = useToastStore();
const items = ref<Notification[]>([]);
const offset = ref(0);
const count = ref<number>();
const loading = ref(true);
const icons = { follow: UserPlus, comment: MessageCircle, reaction: Heart };
const unread = computed(() => items.value.filter(item => !item.isRead).length);

const load = async () => {
  loading.value = true;
  try {
    const response = await notificationApi.list({ limit: LIMIT, offset: offset.value });
    items.value = response.body?.data || [];
    count.value = response.body?.pagination?.count;
  } catch (error) {
    toast.show(getApiError(error, "Could not load notifications"), "error");
  } finally {
    loading.value = false;
  }
};

const open = async (item: Notification) => {
  if (!item.isRead) {
    await notificationApi.read(item.id).catch(() => undefined);
    item.isRead = true;
  }
  router.push(item.postId ? `/posts/${item.postId}` : `/profile/${item.actorId}`);
};

const readAll = async () => {
  await notificationApi.readAll();
  items.value = items.value.map(item => ({ ...item, isRead: true }));
  toast.show("All notifications marked read", "success");
};

onMounted(load);
</script>

<template>
  <div class="ax-container max-w-4xl space-y-6">
    <header class="flex flex-wrap items-end justify-between gap-3">
      <div><p class="ax-section-title">Activity center</p><h1 class="ax-page-title">Notifications</h1></div>
      <button class="btn btn-outline btn-sm" :disabled="!unread" @click="readAll"><CheckCheck :size="16" /> Mark all read</button>
    </header>

    <div v-if="loading" class="grid place-items-center py-24"><LoaderCircle class="animate-spin" /></div>
    <EmptyState v-else-if="!items.length" :icon="Bell" title="No notifications yet" description="Follows, comments and reactions on your posts will appear here." />
    <section v-else class="ax-panel">
      <div class="divide-y divide-base-300">
        <button v-for="item in items" :key="item.id" class="flex w-full items-start gap-4 p-4 text-left transition hover:bg-base-200 md:p-5" :class="{ 'bg-primary/8': !item.isRead }" @click="open(item)">
          <span class="grid size-11 shrink-0 place-items-center rounded-2xl" :class="item.isRead ? 'bg-base-200' : 'bg-primary text-primary-content'"><component :is="icons[item.kind]" :size="19" /></span>
          <span class="min-w-0 flex-1">
            <span class="block"><strong>{{ item.actorName || `User ${item.actorId}` }}</strong> {{ describe(item) }}</span>
            <small class="ax-muted mt-1 block">{{ timeAgo(item.createdAt) }}</small>
          </span>
          <span v-if="!item.isRead" class="status status-primary mt-2"></span>
        </button>
      </div>
    </section>
    <PaginationBar :offset="offset" :limit="LIMIT" :count="count" :loading="loading" @change="value => { offset = value; load(); }" />
  </div>
</template>
