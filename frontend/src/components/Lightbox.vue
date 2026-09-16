<script setup lang="ts">
import { X } from "lucide-vue-next";
import { onBeforeUnmount, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useLightboxStore } from "../stores/lightbox";

const { t } = useI18n();
const lightbox = useLightboxStore();
const onKey = (event: KeyboardEvent) => {
  if (event.key === "Escape") lightbox.close();
};
onMounted(() => window.addEventListener("keydown", onKey));
onBeforeUnmount(() => window.removeEventListener("keydown", onKey));
</script>

<template>
  <Transition name="page">
    <div v-if="lightbox.src" class="fixed inset-0 z-[90] flex items-center justify-center bg-black/85 p-4 backdrop-blur-sm" role="dialog" aria-modal="true" @click="lightbox.close()">
      <img :src="lightbox.src" :alt="lightbox.alt" class="max-h-full max-w-full rounded-box object-contain shadow-2xl" @click.stop />
      <button class="btn btn-circle btn-ghost absolute right-4 top-4 text-white" :aria-label="t('common.close')" @click="lightbox.close()"><X :size="22" /></button>
    </div>
  </Transition>
</template>
