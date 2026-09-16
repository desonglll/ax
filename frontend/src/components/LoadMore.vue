<script setup lang="ts">
import { LoaderCircle } from "lucide-vue-next";
import { onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

/**
 * Sentinel placed after a list. Emits `more` whenever it scrolls into view
 * and the list is neither loading nor finished.
 */
const props = defineProps<{ loading: boolean; done: boolean; count: number; error?: string }>();
const emit = defineEmits<{ more: [] }>();
const { t } = useI18n();
const el = ref<HTMLElement>();
let observer: IntersectionObserver | undefined;

const check = () => {
  if (!props.loading && !props.done && !props.error) emit("more");
};

onMounted(() => {
  observer = new IntersectionObserver(entries => {
    if (entries.some(entry => entry.isIntersecting)) check();
  }, { rootMargin: "600px 0px" });
  if (el.value) observer.observe(el.value);
});
onBeforeUnmount(() => observer?.disconnect());

// A short first page might not fill the viewport; re-check once it settles.
watch(() => [props.loading, props.count], () => {
  if (!props.loading && el.value) {
    const rect = el.value.getBoundingClientRect();
    if (rect.top < window.innerHeight + 600) check();
  }
});
</script>

<template>
  <div ref="el" class="flex min-h-12 items-center justify-center py-4 text-sm text-base-content/50">
    <span v-if="loading" class="flex items-center gap-2"><LoaderCircle :size="16" class="animate-spin" /> {{ t("common.loadingMore") }}</span>
    <button v-else-if="error" class="btn btn-ghost btn-sm" @click="emit('more')">{{ error }} · {{ t("common.retry") }}</button>
    <span v-else-if="done && count > 0">{{ t("common.end") }}</span>
  </div>
</template>
