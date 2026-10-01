<script setup lang="ts">
import { X } from "lucide-vue-next";
import { nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useLightboxStore } from "../stores/lightbox";

/**
 * Full-screen image viewer. A modal <dialog> keeps focus inside, closes on
 * Esc and hands focus back to the image that opened it.
 */
const { t } = useI18n();
const lightbox = useLightboxStore();
const el = ref<HTMLDialogElement>();

watch(() => lightbox.src, async src => {
  if (!src) {
    if (el.value?.open) el.value.close();
    return;
  }
  await nextTick();
  if (!el.value?.open) el.value?.showModal();
});
</script>

<template>
  <dialog
    ref="el"
    class="ax-lightbox"
    :aria-label="lightbox.alt || t('common.imagePreview')"
    @cancel.prevent="lightbox.close()"
    @close="lightbox.close()"
    @click="lightbox.close()"
  >
    <div v-if="lightbox.src" class="flex size-full items-center justify-center p-4">
      <img :src="lightbox.src" :alt="lightbox.alt" class="max-h-full max-w-full rounded-box object-contain shadow-2xl" @click.stop />
      <button type="button" class="btn btn-circle btn-ghost absolute right-4 top-4 text-white" :aria-label="t('common.close')" autofocus @click="lightbox.close()"><X :size="22" /></button>
    </div>
  </dialog>
</template>
