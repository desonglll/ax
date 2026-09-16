<script setup lang="ts">
import { computed, onActivated, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { Bell, CheckCheck, Heart, MessageCircle, UserPlus } from "lucide-vue-next";
import { useRouter } from "vue-router";
import { notificationApi } from "../api";
import EmptyState from "../components/EmptyState.vue";
import LoadMore from "../components/LoadMore.vue";
import { useInfiniteList } from "../composables/useInfiniteList";
import { timeAgo } from "../lib/format";
import type { Notification } from "../types";

defineOptions({ name: "NotificationsView" });

const { t } = useI18n();
const router = useRouter();
const icons = { follow: UserPlus, comment: MessageCircle, reaction: Heart };

const list = useInfiniteList<Notification>(async (offset, limit) => {
  const response = await notificationApi.list({ limit, offset });
  return { items: response.body?.data || [], count: response.body?.pagination?.count };
}, 20);
const unread = computed(() => list.items.value.filter(item => !item.isRead).length);

const open = async (item: Notification) => {
  if (!item.isRead) {
    await notificationApi.read(item.id).catch(() => undefined);
    list.replace({ ...item, isRead: true });
  }
  router.push(item.postId ? `/posts/${item.postId}` : `/profile/${item.actorId}`);
};

const readAll = async () => {
  await notificationApi.readAll();
  list.items.value = list.items.value.map(item => ({ ...item, isRead: true }));
};

onMounted(() => list.reset());
// Coming back from a post: pick up anything new since we left.
onActivated(() => { if (list.items.value.length) list.reset(); });
</script>

<template>
  <div class="ax-container max-w-4xl space-y-5">
    <header class="flex flex-wrap items-center justify-between gap-3">
      <h1 class="ax-page-title">{{ t("notifications.title") }}</h1>
      <button class="btn btn-outline btn-sm" :disabled="!unread" @click="readAll"><CheckCheck :size="16" /> {{ t("notifications.markAllRead") }}</button>
    </header>

    <div v-if="list.loading.value" class="ax-panel divide-y divide-base-300">
      <div v-for="n in 5" :key="n" class="flex items-center gap-4 p-4"><div class="skeleton size-11 rounded-2xl"></div><div class="flex-1 space-y-2"><div class="skeleton h-3 w-2/3"></div><div class="skeleton h-3 w-24"></div></div></div>
    </div>
    <EmptyState v-else-if="!list.items.value.length" :icon="Bell" :title="t('notifications.empty')" />
    <section v-else class="ax-panel">
      <TransitionGroup name="list" tag="div" class="divide-y divide-base-300">
        <button v-for="item in list.items.value" :key="item.id" class="flex w-full items-start gap-4 p-4 text-left transition hover:bg-base-200 md:p-5" :class="{ 'bg-primary/8': !item.isRead }" @click="open(item)">
          <span class="grid size-11 shrink-0 place-items-center rounded-2xl transition-colors" :class="item.isRead ? 'bg-base-200' : 'bg-primary text-primary-content'"><component :is="icons[item.kind]" :size="19" /></span>
          <span class="min-w-0 flex-1">
            <span class="block"><strong>{{ item.actorName || t("notifications.user", { id: item.actorId }) }}</strong> {{ t(`notifications.${item.kind}`) }}</span>
            <small class="ax-muted mt-1 block">{{ timeAgo(item.createdAt) }}</small>
          </span>
          <span v-if="!item.isRead" class="status status-primary mt-2"></span>
        </button>
      </TransitionGroup>
    </section>
    <LoadMore v-if="!list.loading.value" :loading="list.loadingMore.value" :done="list.done.value" :count="list.items.value.length" :error="list.error.value" @more="list.loadMore()" />
  </div>
</template>
