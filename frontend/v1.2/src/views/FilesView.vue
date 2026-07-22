<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { Download, FileArchive, LoaderCircle, UploadCloud } from "lucide-vue-next";
import { fileApi } from "../api";
import { getApiError } from "../api/client";
import FilePreview from "../components/FilePreview.vue";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { FileRecord } from "../types";

const auth = useAuthStore();
const toast = useToastStore();
const files = ref<FileRecord[]>([]);
const selected = ref<File[]>([]);
const description = ref("");
const isPublic = ref(true);
const busy = ref(false);
const totalSize = computed(() => files.value.reduce((sum, file) => sum + file.size, 0));
const formatSize = (size: number) => `${(size / 1024 / 1024).toFixed(2)} MB`;

const load = async () => {
  const response = auth.user?.isAdmin ? await fileApi.all() : auth.user ? await fileApi.user(auth.user.id) : await fileApi.public();
  files.value = (response.body?.data || []).filter(file => !file.isDeleted);
};

const upload = async () => {
  if (!selected.value.length) return;
  busy.value = true;
  try {
    const form = new FormData();
    selected.value.forEach(file => form.append("files", file));
    form.append("description", description.value);
    const response = await fileApi.upload(form, isPublic.value);
    files.value.unshift(...(response.body?.data || []));
    selected.value = [];
    description.value = "";
    toast.show("Files uploaded", "success");
  } catch (error) { toast.show(getApiError(error, "Upload failed"), "error"); }
  finally { busy.value = false; }
};

onMounted(load);
</script>

<template>
  <div class="ax-container space-y-6">
    <header><p class="ax-section-title">Media library</p><h1 class="ax-page-title">Files & attachments</h1><p class="mt-2 ax-muted">{{ files.length }} files · {{ formatSize(totalSize) }} stored</p></header>
    <section v-if="auth.user" class="ax-panel"><div class="card-body"><h2 class="card-title"><UploadCloud :size="20" /> Upload files</h2><div class="grid gap-4 md:grid-cols-[1fr_1fr_auto]"><input type="file" multiple class="file-input w-full" @change="selected = Array.from(($event.target as HTMLInputElement).files || [])" /><input v-model="description" class="input w-full" placeholder="Description" /><button class="btn btn-primary" :disabled="busy || !selected.length" @click="upload"><LoaderCircle v-if="busy" class="animate-spin" :size="17" /><UploadCloud v-else :size="17" /> Upload</button></div><label class="label cursor-pointer justify-start gap-3"><input v-model="isPublic" type="checkbox" class="toggle toggle-primary" /><span>Publicly accessible</span></label></div></section>
    <div v-if="files.length" class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3"><article v-for="file in files" :key="file.id" class="ax-panel overflow-hidden"><FilePreview v-if="file.contentType.startsWith('image/') || file.contentType.startsWith('video/')" :file="file" /><div class="card-body p-4"><div class="flex items-start gap-3"><span class="grid size-10 place-items-center rounded-xl bg-base-200"><FileArchive :size="19" /></span><div class="min-w-0 flex-1"><h2 class="truncate font-bold">{{ file.name }}</h2><p class="text-xs ax-muted">{{ formatSize(file.size) }} · {{ file.isPub ? "Public" : "Private" }}</p></div></div><p v-if="file.description" class="text-sm ax-muted">{{ file.description }}</p><a :href="fileApi.downloadUrl(file.id)" class="btn btn-outline btn-sm"><Download :size="15" /> Download</a></div></article></div>
    <div v-else class="ax-panel"><div class="card-body items-center py-16"><FileArchive :size="36" class="text-base-content/25" /><h2 class="card-title">No files found</h2></div></div>
  </div>
</template>
