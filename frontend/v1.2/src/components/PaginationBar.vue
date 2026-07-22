<script setup lang="ts">
const props = defineProps<{ offset: number; limit: number; count?: number; loading?: boolean }>();
const emit = defineEmits<{ change: [offset: number] }>();
</script>

<template>
  <div class="flex items-center justify-between gap-3">
    <button class="btn btn-outline btn-sm" :disabled="offset === 0 || loading" @click="emit('change', Math.max(0, offset - limit))">Previous</button>
    <span class="text-sm text-base-content/55">{{ Math.floor(offset / limit) + 1 }}<template v-if="count !== undefined"> / {{ Math.max(1, Math.ceil(count / limit)) }}</template></span>
    <button class="btn btn-outline btn-sm" :disabled="loading || (count !== undefined ? offset + limit >= count : false)" @click="emit('change', offset + limit)">Next</button>
  </div>
</template>
