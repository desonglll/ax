<script setup lang="ts">
import { computed } from "vue";
import { highlight } from "../lib/highlight";

/** Text with the search terms wrapped in `<mark>`; rendered as text nodes, never as HTML. */
const props = defineProps<{ text: string; terms: string[] }>();
const segments = computed(() => highlight(props.text, props.terms));
</script>

<template>
  <template v-for="(segment, index) in segments" :key="index"><mark v-if="segment.match" class="rounded-sm bg-warning/40 px-0.5 text-inherit">{{ segment.text }}</mark><template v-else>{{ segment.text }}</template></template>
</template>
