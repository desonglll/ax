<script setup lang="ts">
import { computed } from "vue";
import { initials } from "../lib/format";

const props = withDefaults(defineProps<{ name: string; size?: "xs" | "sm" | "md" | "lg" | "xl"; tone?: "auto" | "neutral" | "primary" | "secondary" }>(), { size: "md", tone: "auto" });

const sizes = { xs: "size-7 text-[0.65rem]", sm: "size-9 text-xs", md: "size-11 text-sm", lg: "size-14 text-lg", xl: "size-24 text-3xl" };
const tones = { neutral: "bg-neutral text-neutral-content", primary: "bg-primary text-primary-content", secondary: "bg-secondary text-secondary-content" };

/** Stable per-name hue so the same person always gets the same colour. */
const hue = computed(() => {
  let h = 0;
  for (const ch of props.name) h = (h * 31 + ch.charCodeAt(0)) >>> 0;
  return h % 360;
});
const style = computed(() => (props.tone === "auto" ? { background: `oklch(62% 0.14 ${hue.value})`, color: "white" } : undefined));
</script>

<template>
  <span class="inline-grid shrink-0 select-none place-items-center rounded-full font-bold leading-none" :class="[sizes[size], tone !== 'auto' && tones[tone]]" :style="style" :aria-label="name">
    {{ initials(name) }}
  </span>
</template>
