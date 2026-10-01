<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import { MessageCircle, Paperclip, ThumbsUp } from "lucide-vue-next";
import { fullDate, timeAgo } from "../lib/format";
import { snippet } from "../lib/highlight";
import { excerpt } from "../lib/markdown";
import { usePostStore } from "../stores/posts";
import type { Post } from "../types";
import Avatar from "./Avatar.vue";
import Highlighted from "./Highlighted.vue";

/** A compact search hit: highlighted title and the part of the body that matched. */
const props = defineProps<{ post: Post; terms: string[] }>();
const { t } = useI18n();
const router = useRouter();
const posts = usePostStore();

// Registers the post so opening it renders instantly, and picks up likes made elsewhere.
const view = computed(() => posts.view(props.post));
const link = computed(() => `/posts/${props.post.id}`);
const body = computed(() => snippet(excerpt(view.value.content, 4000), props.terms));

const open = (event: MouseEvent) => {
  if (event.composedPath().some(node => node instanceof Element && node !== event.currentTarget && node.matches("a, button"))) return;
  if (window.getSelection()?.toString()) return;
  if (event.metaKey || event.ctrlKey) window.open(router.resolve(link.value).href, "_blank");
  else router.push(link.value);
};
</script>

<template>
  <article class="ax-panel cursor-pointer transition-shadow hover:shadow-sm" @click="open">
    <div class="card-body gap-2 p-4">
      <header class="flex items-center gap-2 text-sm">
        <RouterLink :to="`/profile/${view.userId}`"><Avatar :name="view.userName" size="xs" /></RouterLink>
        <RouterLink :to="`/profile/${view.userId}`" class="font-semibold hover:underline">{{ view.userName }}</RouterLink>
        <span class="ax-muted">·</span>
        <time class="ax-muted" :datetime="view.createdAt" :title="fullDate(view.createdAt)">{{ timeAgo(view.createdAt) }}</time>
      </header>
      <h2 class="text-base font-bold leading-snug">
        <RouterLink :to="link" class="hover:text-primary"><Highlighted :text="view.title || t('search.untitled')" :terms="terms" /></RouterLink>
      </h2>
      <p class="ax-muted break-words text-sm leading-relaxed"><Highlighted :text="body" :terms="terms" /></p>
      <footer class="ax-muted flex items-center gap-4 text-xs">
        <span class="flex items-center gap-1" :aria-label="t('search.likes', { count: view.likeCount })"><ThumbsUp :size="13" /> {{ view.likeCount }}</span>
        <span class="flex items-center gap-1" :aria-label="t('search.comments', { count: view.commentCount })"><MessageCircle :size="13" /> {{ view.commentCount }}</span>
        <span v-if="view.attachments?.length" class="flex items-center gap-1" :aria-label="t('search.attachments', { count: view.attachments.length })"><Paperclip :size="13" /> {{ view.attachments.length }}</span>
      </footer>
    </div>
  </article>
</template>
