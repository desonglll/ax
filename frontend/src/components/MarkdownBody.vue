<script setup lang="ts">
import { computed } from "vue";
import { renderMarkdown } from "../lib/markdown";

const props = defineProps<{ source: string; compact?: boolean }>();
const html = computed(() => renderMarkdown(props.source));
</script>

<template>
  <div class="md" :class="{ 'md-compact': compact }" v-html="html"></div>
</template>
