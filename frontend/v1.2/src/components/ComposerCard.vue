<script setup lang="ts">
import { ImagePlus, LoaderCircle, Send, X } from "lucide-vue-next";
import { ref } from "vue";
import { fileApi, postApi } from "../api";
import { getApiError } from "../api/client";
import { useToastStore } from "../stores/toast";
import type { Post } from "../types";

const emit = defineEmits<{ created: [post: Post] }>();
const toast = useToastStore();
const title = ref("");
const content = ref("");
const files = ref<File[]>([]);
const busy = ref(false);

const selectFiles = (event: Event) => {
  files.value = Array.from((event.target as HTMLInputElement).files || []).slice(0, 6);
};

const publish = async () => {
  if (!content.value.trim()) return;
  busy.value = true;
  try {
    let attachments: string[] = [];
    if (files.value.length) {
      const form = new FormData();
      files.value.forEach(file => form.append("files", file));
      form.append("description", title.value || "Post attachment");
      const uploaded = await fileApi.upload(form, true);
      attachments = (uploaded.body?.data || []).map(file => file.id);
    }
    const response = await postApi.create({ title: title.value.trim(), content: content.value.trim(), attachments });
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
    <div class="card-body gap-4 p-4 md:p-5">
      <input v-model="title" class="input input-ghost w-full px-0 text-lg font-bold focus:outline-none" maxlength="120" placeholder="Give your idea a title" />
      <textarea v-model="content" class="textarea textarea-ghost min-h-28 w-full resize-y px-0 text-base focus:outline-none" maxlength="10000" placeholder="Share a thought, update, or question…"></textarea>
      <div v-if="files.length" class="flex flex-wrap gap-2">
        <span v-for="file in files" :key="`${file.name}-${file.size}`" class="badge badge-outline gap-1 py-3">{{ file.name }}<button aria-label="Remove file" @click="files = files.filter(item => item !== file)"><X :size="12" /></button></span>
      </div>
      <div class="flex items-center justify-between border-t border-base-300 pt-3">
        <label class="btn btn-ghost btn-sm"><ImagePlus :size="18" /> Attach<input type="file" multiple class="hidden" accept="image/*,video/*,audio/*,.pdf,.txt,.zip" @change="selectFiles" /></label>
        <div class="flex items-center gap-3"><small class="ax-muted">{{ content.length }}/10000</small><button class="btn btn-primary btn-sm" :disabled="busy || !content.trim()" @click="publish"><LoaderCircle v-if="busy" :size="16" class="animate-spin" /><Send v-else :size="16" /> Publish</button></div>
      </div>
    </div>
  </section>
</template>
