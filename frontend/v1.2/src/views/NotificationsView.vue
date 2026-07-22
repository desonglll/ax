<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Bell, CheckCheck, Heart, MessageCircle, UserPlus } from "lucide-vue-next";
import { useRouter } from "vue-router";
import { notificationApi } from "../api";
import PaginationBar from "../components/PaginationBar.vue";
import type { Notification } from "../types";

const router = useRouter();
const items = ref<Notification[]>([]);
const offset = ref(0);
const count = ref<number>();
const limit = 20;
const loading = ref(false);
const icons = { follow: UserPlus, comment: MessageCircle, reaction: Heart };

const load = async () => {
  loading.value = true;
  try {
    const response = await notificationApi.list({ limit, offset: offset.value });
    items.value = response.body?.data || [];
    count.value = response.body?.pagination?.count;
  } finally { loading.value = false; }
};

const open = async (item: Notification) => {
  if (!item.isRead) await notificationApi.read(item.id);
  item.isRead = true;
  router.push(item.postId ? `/posts/${item.postId}` : `/profile/${item.actorId}`);
};

const readAll = async () => {
  await notificationApi.readAll();
  items.value = items.value.map(item => ({ ...item, isRead: true }));
};

onMounted(load);
</script>

<template>
  <div class="ax-container max-w-4xl space-y-6">
    <header class="flex items-end justify-between"><div><p class="ax-section-title">Activity center</p><h1 class="ax-page-title">Notifications</h1></div><button class="btn btn-outline btn-sm" @click="readAll"><CheckCheck :size="16" /> Mark all read</button></header>
    <section class="ax-panel"><div class="divide-y divide-base-300"><button v-for="item in items" :key="item.id" class="flex w-full items-start gap-4 p-4 text-left transition hover:bg-base-200 md:p-5" :class="{ 'bg-primary/8': !item.isRead }" @click="open(item)"><span class="grid size-11 place-items-center rounded-2xl" :class="item.isRead ? 'bg-base-200' : 'bg-primary text-primary-content'"><component :is="icons[item.kind]" :size="19" /></span><span class="min-w-0 flex-1"><span class="block"><strong>{{ item.actorName || `User ${item.actorId}` }}</strong> {{ item.kind === "follow" ? "started following you" : item.kind === "comment" ? "commented on your post" : "reacted to your post" }}</span><small class="mt-1 block ax-muted">{{ new Date(item.createdAt).toLocaleString() }}</small></span><span v-if="!item.isRead" class="status status-primary mt-2"></span></button><div v-if="!items.length" class="grid place-items-center gap-3 py-20 text-center"><Bell :size="38" class="text-base-content/25" /><h2 class="font-bold">No notifications yet</h2><p class="ax-muted">Interactions with your content will appear here.</p></div></div></section>
    <PaginationBar :offset="offset" :limit="limit" :count="count" :loading="loading" @change="value => { offset = value; load(); }" />
  </div>
</template>
