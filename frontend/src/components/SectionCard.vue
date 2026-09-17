<script setup lang="ts">
import type { Component } from "vue";

/** A panel with a clearly separated header row: icon, title, optional action. */
defineProps<{ title: string; icon?: Component; to?: string; action?: string }>();
</script>

<template>
  <section class="ax-panel overflow-hidden">
    <header class="flex items-center gap-2 border-b border-base-300 bg-base-200/60 px-4 py-2.5">
      <component :is="icon" v-if="icon" :size="16" class="text-primary" />
      <h2 class="text-sm font-semibold">{{ title }}</h2>
      <slot name="action">
        <RouterLink v-if="to && action" :to="to" class="link link-hover ml-auto text-xs text-base-content/60">{{ action }}</RouterLink>
      </slot>
    </header>
    <div class="card-body gap-1 p-3">
      <slot />
    </div>
  </section>
</template>
