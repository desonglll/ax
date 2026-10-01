<script setup lang="ts">
import { Bookmark } from "lucide-vue-next";
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { bookmarkApi } from "../api";
import { getApiError } from "../api/client";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";

/**
 * Save / unsave a post, optimistically. The parent owns the state via
 * v-model; a failed request rolls it back. Guests sign in first, then the
 * save goes through.
 */
const props = defineProps<{ postId: string }>();
const saved = defineModel<boolean>({ required: true });

const { t } = useI18n();
const auth = useAuthStore();
const toast = useToastStore();
const busy = ref(false);

const toggle = async () => {
  if (busy.value) return;
  const wasGuest = !auth.user;
  if (wasGuest && !(await auth.ensure("bookmark"))) return;
  busy.value = true;
  // A guest who just signed in wants the post saved, whatever the stale flag says.
  const next = wasGuest ? true : !saved.value;
  const before = saved.value;
  saved.value = next;
  try {
    if (next) await bookmarkApi.add(props.postId);
    else await bookmarkApi.remove(props.postId);
    toast.show(next ? t("bookmarks.saved") : t("bookmarks.removed"), "success");
  } catch (error) {
    saved.value = before;
    toast.show(getApiError(error, t("errors.bookmark")), "error");
  } finally {
    busy.value = false;
  }
};
</script>

<template>
  <button class="btn btn-ghost btn-sm" :class="{ 'text-primary': saved }" :aria-label="saved ? t('bookmarks.unsave') : t('bookmarks.save')" :aria-pressed="saved" :title="saved ? t('bookmarks.unsave') : t('bookmarks.save')" @click="toggle">
    <Bookmark :key="`bookmark-${saved}`" :size="17" :fill="saved ? 'currentColor' : 'none'" class="pop" />
  </button>
</template>
