<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Flame, LoaderCircle } from "lucide-vue-next";
import { postApi } from "../api";
import PostCard from "../components/PostCard.vue";
import type { Post } from "../types";

const posts = ref<Post[]>([]);
const loading = ref(true);
onMounted(async () => {
  try { posts.value = (await postApi.trending()).body?.data || []; }
  finally { loading.value = false; }
});
</script>

<template>
  <div class="ax-container max-w-4xl space-y-6">
    <header><p class="ax-section-title">What's resonating</p><h1 class="ax-page-title flex items-center gap-2"><Flame class="text-error" /> Trending now</h1><p class="mt-2 ax-muted">Ranked by current engagement across the Ax community.</p></header>
    <div v-if="loading" class="grid place-items-center py-24"><LoaderCircle class="animate-spin" /></div>
    <div v-else class="space-y-4"><PostCard v-for="post in posts" :key="post.id" :post="post" @deleted="id => posts = posts.filter(item => item.id !== id)" /></div>
  </div>
</template>
