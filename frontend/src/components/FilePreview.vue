<script setup lang="ts">
import { Download, FileText } from "lucide-vue-next";
import { useI18n } from "vue-i18n";
import { fileApi } from "../api";
import { formatSize } from "../lib/format";
import { useLightboxStore } from "../stores/lightbox";
import type { FileRecord } from "../types";

defineProps<{ file: FileRecord }>();
const { t } = useI18n();
const lightbox = useLightboxStore();
</script>

<template>
  <figure v-if="file.contentType.startsWith('image/')" class="overflow-hidden rounded-box border border-base-300 bg-base-200">
    <button type="button" class="block w-full cursor-zoom-in" :aria-label="`${t('common.imagePreview')}: ${file.name}`" @click="lightbox.open(fileApi.downloadUrl(file.id), file.name)">
      <img :src="fileApi.downloadUrl(file.id)" :alt="file.name" class="max-h-[34rem] w-full object-contain" loading="lazy" decoding="async" />
    </button>
  </figure>
  <video v-else-if="file.contentType.startsWith('video/')" :src="fileApi.streamUrl(file.id)" controls preload="metadata" class="max-h-[34rem] w-full rounded-box bg-neutral"></video>
  <audio v-else-if="file.contentType.startsWith('audio/')" :src="fileApi.streamUrl(file.id)" controls preload="metadata" class="w-full"></audio>
  <a v-else :href="fileApi.downloadUrl(file.id)" class="flex items-center gap-3 rounded-box border border-base-300 bg-base-200 p-3 transition hover:border-primary">
    <span class="grid size-10 shrink-0 place-items-center rounded-xl bg-base-100"><FileText :size="20" aria-hidden="true" /></span>
    <span class="min-w-0 flex-1"><strong class="block truncate text-sm">{{ file.name }}</strong><small class="ax-muted">{{ formatSize(file.size) }} · {{ file.contentType }}</small></span>
    <Download :size="18" />
  </a>
</template>
