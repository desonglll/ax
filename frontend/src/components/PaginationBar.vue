<script setup lang="ts">
import { computed } from "vue";
import { ChevronLeft, ChevronRight } from "lucide-vue-next";

const props = defineProps<{ offset: number; limit: number; count?: number; loading?: boolean }>();
const emit = defineEmits<{ change: [offset: number] }>();

const page = computed(() => Math.floor(props.offset / props.limit) + 1);
const pages = computed(() => (props.count === undefined ? undefined : Math.max(1, Math.ceil(props.count / props.limit))));
const isLast = computed(() => (props.count === undefined ? false : props.offset + props.limit >= props.count));
</script>

<template>
  <nav v-if="pages === undefined || pages > 1" class="flex items-center justify-between gap-3" aria-label="Pagination">
    <button class="btn btn-outline btn-sm" :disabled="offset === 0 || loading" @click="emit('change', Math.max(0, offset - limit))"><ChevronLeft :size="16" /> Previous</button>
    <span class="text-sm text-base-content/55">Page {{ page }}<template v-if="pages"> of {{ pages }}</template></span>
    <button class="btn btn-outline btn-sm" :disabled="loading || isLast" @click="emit('change', offset + limit)">Next <ChevronRight :size="16" /></button>
  </nav>
</template>
