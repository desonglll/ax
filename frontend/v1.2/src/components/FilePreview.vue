<script setup lang="ts">
import { Download, FileText } from "lucide-vue-next";
import { fileApi } from "../api";
import type { FileRecord } from "../types";

defineProps<{ file: FileRecord }>();
const formatSize = (size: number) => size < 1024 * 1024 ? `${(size / 1024).toFixed(1)} KB` : `${(size / 1024 / 1024).toFixed(1)} MB`;
</script>

<template>
  <figure v-if="file.contentType.startsWith('image/')" class="overflow-hidden rounded-box border border-base-300 bg-base-200">
    <img :src="fileApi.downloadUrl(file.id)" :alt="file.name" class="max-h-[34rem] w-full object-contain" loading="lazy" />
  </figure>
  <video v-else-if="file.contentType.startsWith('video/')" :src="fileApi.streamUrl(file.id)" controls class="max-h-[34rem] w-full rounded-box bg-neutral"></video>
  <a v-else :href="fileApi.downloadUrl(file.id)" class="flex items-center gap-3 rounded-box border border-base-300 bg-base-200 p-3 hover:border-primary">
    <span class="grid size-10 place-items-center rounded-xl bg-base-100"><FileText :size="20" /></span>
    <span class="min-w-0 flex-1"><strong class="block truncate text-sm">{{ file.name }}</strong><small class="ax-muted">{{ formatSize(file.size) }} · {{ file.contentType }}</small></span>
    <Download :size="18" />
  </a>
</template>
