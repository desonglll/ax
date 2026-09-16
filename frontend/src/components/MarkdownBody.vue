<script setup lang="ts">
import { computed } from "vue";
import { renderMarkdown } from "../lib/markdown";
import { useLightboxStore } from "../stores/lightbox";

const props = defineProps<{ source: string; compact?: boolean }>();
const html = computed(() => renderMarkdown(props.source));
const lightbox = useLightboxStore();

/** Clicking an embedded image opens it full-screen. */
const onClick = (event: MouseEvent) => {
  const target = event.target as HTMLElement;
  if (target instanceof HTMLImageElement) {
    event.preventDefault();
    lightbox.open(target.currentSrc || target.src, target.alt);
  }
};
</script>

<template>
  <div class="md" :class="{ 'md-compact': compact }" @click="onClick" v-html="html"></div>
</template>
