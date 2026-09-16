<script setup lang="ts">
import type { Component } from "vue";

defineProps<{ icon: Component; title: string; description?: string }>();
</script>

<template>
  <div class="ax-panel">
    <div class="card-body items-center gap-2 py-16 text-center">
      <component :is="icon" :size="36" class="text-base-content/25" />
      <h2 class="card-title">{{ title }}</h2>
      <p v-if="description" class="ax-muted max-w-sm">{{ description }}</p>
      <slot />
    </div>
  </div>
</template>
