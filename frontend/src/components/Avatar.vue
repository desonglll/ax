<script setup lang="ts">
import { initials } from "../lib/format";

withDefaults(defineProps<{ name: string; size?: "xs" | "sm" | "md" | "lg" | "xl"; tone?: "neutral" | "primary" | "secondary" }>(), { size: "md", tone: "neutral" });

const sizes = { xs: "w-7 text-xs", sm: "w-9 text-sm", md: "w-11 text-base", lg: "w-14 text-xl", xl: "w-24 text-3xl" };
const tones = { neutral: "bg-neutral text-neutral-content", primary: "bg-primary text-primary-content", secondary: "bg-secondary text-secondary-content" };
</script>

<template>
  <div class="avatar placeholder shrink-0">
    <div class="rounded-full font-bold" :class="[sizes[size], tones[tone]]"><span>{{ initials(name) }}</span></div>
  </div>
</template>
