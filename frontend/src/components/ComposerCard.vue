<script setup lang="ts">
import { LoaderCircle, Send } from "lucide-vue-next";
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { postApi } from "../api";
import { getApiError } from "../api/client";
import { useToastStore } from "../stores/toast";
import type { FileRecord, Post } from "../types";
import MarkdownEditor from "./MarkdownEditor.vue";

const emit = defineEmits<{ created: [post: Post] }>();
const { t } = useI18n();
const toast = useToastStore();
const title = ref("");
const content = ref("");
const attachments = ref<FileRecord[]>([]);
const busy = ref(false);

const publish = async () => {
  if (!content.value.trim() || busy.value) return;
  busy.value = true;
  try {
    const response = await postApi.create({
      title: title.value.trim() || undefined,
      content: content.value.trim(),
      attachments: attachments.value.map(file => file.id),
    });
    if (!response.body?.data) throw new Error(response.message);
    emit("created", response.body.data);
    title.value = "";
    content.value = "";
    attachments.value = [];
    toast.show(t("composer.posted"), "success");
  } catch (error) {
    toast.show(getApiError(error, t("errors.publish")), "error");
  } finally {
    busy.value = false;
  }
};
</script>

<template>
  <form class="space-y-3 p-1" @submit.prevent="publish">
    <input v-model="title" class="input input-ghost w-full px-1 text-lg font-bold focus:outline-none" maxlength="120" :placeholder="t('composer.titlePlaceholder')" />
    <MarkdownEditor v-model="content" v-model:attachments="attachments" :placeholder="t('composer.placeholder')" @submit="publish" />
    <div class="flex justify-end">
      <button type="submit" class="btn btn-primary btn-sm" :disabled="busy || !content.trim()">
        <LoaderCircle v-if="busy" :size="16" class="animate-spin" /><Send v-else :size="16" /> {{ t("composer.post") }}
      </button>
    </div>
  </form>
</template>
