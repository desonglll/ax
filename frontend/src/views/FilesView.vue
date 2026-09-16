<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { Download, FileArchive, LoaderCircle, UploadCloud, X } from "lucide-vue-next";
import { fileApi } from "../api";
import { getApiError } from "../api/client";
import EmptyState from "../components/EmptyState.vue";
import FilePreview from "../components/FilePreview.vue";
import { formatSize, timeAgo } from "../lib/format";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { FileRecord } from "../types";

type Scope = "public" | "mine" | "all";
const auth = useAuthStore();
const toast = useToastStore();
const files = ref<FileRecord[]>([]);
const selected = ref<File[]>([]);
const description = ref("");
const isPublic = ref(true);
const busy = ref(false);
const loading = ref(true);
const scope = ref<Scope>(auth.user ? "mine" : "public");

const scopes = computed<{ value: Scope; label: string }[]>(() => [
  { value: "public", label: "Public" },
  ...(auth.user ? [{ value: "mine" as Scope, label: "My files" }] : []),
  ...(auth.user?.isAdmin ? [{ value: "all" as Scope, label: "Everything" }] : []),
]);
const totalSize = computed(() => files.value.reduce((sum, file) => sum + file.size, 0));

const load = async () => {
  loading.value = true;
  try {
    files.value = (await fileApi.list(scope.value)).body?.data || [];
  } catch (error) {
    toast.show(getApiError(error, "Could not load files"), "error");
  } finally {
    loading.value = false;
  }
};

const upload = async () => {
  if (!selected.value.length || busy.value) return;
  busy.value = true;
  try {
    const response = await fileApi.upload(selected.value, { isPublic: isPublic.value, description: description.value.trim() || undefined });
    files.value.unshift(...(response.body?.data || []));
    selected.value = [];
    description.value = "";
    toast.show("Files uploaded", "success");
  } catch (error) {
    toast.show(getApiError(error, "Upload failed"), "error");
  } finally {
    busy.value = false;
  }
};

onMounted(load);
watch(scope, load);
watch(() => auth.user, user => { if (!user) scope.value = "public"; });
</script>

<template>
  <div class="ax-container space-y-6">
    <header class="flex flex-wrap items-end justify-between gap-4">
      <div><p class="ax-section-title">Media library</p><h1 class="ax-page-title">Files & attachments</h1><p class="ax-muted mt-2">{{ files.length }} files · {{ formatSize(totalSize) }}</p></div>
      <div v-if="scopes.length > 1" role="tablist" class="tabs tabs-box">
        <button v-for="item in scopes" :key="item.value" role="tab" class="tab" :class="{ 'tab-active': scope === item.value }" @click="scope = item.value">{{ item.label }}</button>
      </div>
    </header>

    <section v-if="auth.user" class="ax-panel">
      <form class="card-body gap-4" @submit.prevent="upload">
        <h2 class="card-title"><UploadCloud :size="20" /> Upload files</h2>
        <div class="grid gap-3 md:grid-cols-[1fr_1fr_auto]">
          <input type="file" multiple class="file-input w-full" @change="selected = Array.from(($event.target as HTMLInputElement).files || [])" />
          <input v-model="description" class="input w-full" maxlength="500" placeholder="Description (optional)" />
          <button type="submit" class="btn btn-primary" :disabled="busy || !selected.length"><LoaderCircle v-if="busy" class="animate-spin" :size="17" /><UploadCloud v-else :size="17" /> Upload</button>
        </div>
        <div class="flex flex-wrap items-center justify-between gap-3">
          <label class="label cursor-pointer gap-3"><input v-model="isPublic" type="checkbox" class="toggle toggle-primary" /><span>{{ isPublic ? "Anyone can view these files" : "Only you (and admins) can view these files" }}</span></label>
          <div v-if="selected.length" class="flex flex-wrap gap-2">
            <span v-for="file in selected" :key="`${file.name}-${file.size}`" class="badge badge-outline gap-1">{{ file.name }}<button type="button" aria-label="Remove" @click="selected = selected.filter(f => f !== file)"><X :size="12" /></button></span>
          </div>
        </div>
      </form>
    </section>

    <div v-if="loading" class="grid place-items-center py-24"><LoaderCircle class="animate-spin" /></div>
    <EmptyState v-else-if="!files.length" :icon="FileArchive" title="No files here" :description="auth.user ? 'Upload something above or attach files to a post.' : 'Sign in to upload your own files.'" />
    <div v-else class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
      <article v-for="file in files" :key="file.id" class="ax-panel overflow-hidden">
        <FilePreview v-if="file.contentType.startsWith('image/') || file.contentType.startsWith('video/')" :file="file" />
        <div class="card-body gap-3 p-4">
          <div class="flex items-start gap-3">
            <span class="grid size-10 shrink-0 place-items-center rounded-xl bg-base-200"><FileArchive :size="19" /></span>
            <div class="min-w-0 flex-1">
              <h2 class="truncate font-bold" :title="file.name">{{ file.name }}</h2>
              <p class="ax-muted text-xs">{{ formatSize(file.size) }} · {{ file.isPub ? "Public" : "Private" }}<template v-if="file.createdAt"> · {{ timeAgo(file.createdAt) }}</template></p>
            </div>
          </div>
          <p v-if="file.description" class="ax-muted line-clamp-2 text-sm">{{ file.description }}</p>
          <div class="flex items-center gap-2">
            <a :href="fileApi.downloadUrl(file.id)" class="btn btn-outline btn-sm"><Download :size="15" /> Download</a>
            <RouterLink v-if="file.postId" :to="`/posts/${file.postId}`" class="btn btn-ghost btn-sm">View post</RouterLink>
          </div>
        </div>
      </article>
    </div>
  </div>
</template>
