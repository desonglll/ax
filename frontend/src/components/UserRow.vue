<script setup lang="ts">
import Avatar from "./Avatar.vue";
import type { User } from "../types";

defineProps<{ user: User; subtitle?: string }>();
</script>

<template>
  <RouterLink :to="`/profile/${user.id}`" class="flex items-center gap-3 rounded-box p-2 transition hover:bg-base-200">
    <Avatar :name="user.userName" size="sm" />
    <span class="min-w-0 flex-1">
      <strong class="block truncate text-sm">{{ user.fullName || user.userName }}</strong>
      <small class="ax-muted block truncate">{{ subtitle ?? `@${user.userName}` }}</small>
    </span>
    <slot />
  </RouterLink>
</template>
