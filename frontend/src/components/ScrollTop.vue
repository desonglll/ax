<script setup lang="ts">
import { ArrowUp } from "lucide-vue-next";
import { onBeforeUnmount, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const visible = ref(false);
let frame = 0;

const toTop = () => window.scrollTo({ top: 0, behavior: "smooth" });

const onScroll = () => {
  cancelAnimationFrame(frame);
  frame = requestAnimationFrame(() => { visible.value = window.scrollY > 1200; });
};
onMounted(() => window.addEventListener("scroll", onScroll, { passive: true }));
onBeforeUnmount(() => window.removeEventListener("scroll", onScroll));
</script>

<template>
  <Transition name="pop">
    <button v-if="visible" class="btn btn-circle btn-neutral fixed bottom-36 right-4 z-30 shadow-lg md:bottom-8 md:right-8" :aria-label="t('common.backToTop')" @click="toTop">
      <ArrowUp :size="20" />
    </button>
  </Transition>
</template>
