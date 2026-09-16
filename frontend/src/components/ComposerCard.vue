<script setup lang="ts">
import { ImagePlus, LoaderCircle, Send, Sparkles, X } from "lucide-vue-next";
import { ref } from "vue";
import { fileApi, postApi } from "../api";
import { getApiError } from "../api/client";
import { formatSize } from "../lib/format";
import { useToastStore } from "../stores/toast";
import type { Post } from "../types";

const MAX_FILES = 6;
const emit = defineEmits<{ created: [post: Post] }>();
const toast = useToastStore();
const title = ref("");
const content = ref("");
const files = ref<File[]>([]);
const busy = ref(false);

const selectFiles = (event: Event) => {
  const input = event.target as HTMLInputElement;
  const picked = Array.from(input.files || []);
  const merged = [...files.value, ...picked.filter(f => !files.value.some(e => e.name === f.name && e.size === f.size))];
  if (merged.length > MAX_FILES) toast.show(`Up to ${MAX_FILES} attachments per post`, "warning");
  files.value = merged.slice(0, MAX_FILES);
  input.value = "";
};

const publish = async () => {
  if (!content.value.trim() || busy.value) return;
  busy.value = true;
  try {
    let attachments: string[] = [];
    if (files.value.length) {
      const uploaded = await fileApi.upload(files.value, { isPublic: true, description: title.value.trim() || undefined });
      attachments = (uploaded.body?.data || []).map(file => file.id);
    }
    const response = await postApi.create({ title: title.value.trim() || undefined, content: content.value.trim(), attachments });
    if (!response.body?.data) throw new Error(response.message);
    emit("created", response.body.data);
    title.value = "";
    content.value = "";
    files.value = [];
    toast.show("Post published", "success");
  } catch (error) {
    toast.show(getApiError(error, "Unable to publish"), "error");
  } finally {
    busy.value = false;
  }
};
</script>

<template>
  <section class="ax-panel">
    <form class="card-body gap-3 p-4 md:p-5" @submit.prevent="publish">
      <input v-model="title" class="input input-ghost w-full px-0 text-lg font-bold focus:outline-none" maxlength="120" placeholder="Title (optional — leave blank for an AI suggestion)" />
      <textarea v-model="content" class="textarea textarea-ghost min-h-28 w-full resize-y px-0 text-base focus:outline-none" maxlength="10000" placeholder="Share a thought, update, or question…" @keydown.meta.enter="publish" @keydown.ctrl.enter="publish"></textarea>
      <div v-if="files.length" class="flex flex-wrap gap-2">
        <span v-for="file in files" :key="`${file.name}-${file.size}`" class="badge badge-outline gap-1 py-3">
          {{ file.name }} <small class="opacity-60">{{ formatSize(file.size) }}</small>
          <button type="button" aria-label="Remove file" @click="files = files.filter(item => item !== file)"><X :size="12" /></button>
        </span>
      </div>
      <div class="flex items-center justify-between border-t border-base-300 pt-3">
        <label class="btn btn-ghost btn-sm" :class="{ 'btn-disabled': files.length >= MAX_FILES }">
          <ImagePlus :size="18" /> Attach
          <input type="file" multiple class="hidden" accept="image/*,video/*,audio/*,.pdf,.txt,.md,.zip" @change="selectFiles" />
        </label>
        <div class="flex items-center gap-3">
          <small v-if="!title.trim() && content.trim()" class="ax-muted hidden items-center gap-1 sm:flex"><Sparkles :size="13" /> AI title</small>
          <small class="ax-muted" :class="{ 'text-warning': content.length > 9000 }">{{ content.length }}/10000</small>
          <button type="submit" class="btn btn-primary btn-sm" :disabled="busy || !content.trim()">
            <LoaderCircle v-if="busy" :size="16" class="animate-spin" /><Send v-else :size="16" /> Publish
          </button>
        </div>
      </div>
    </form>
  </section>
</template>
