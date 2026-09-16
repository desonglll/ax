<script setup lang="ts">
import { Heart, ThumbsDown } from "lucide-vue-next";
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import { reactionApi } from "../api";
import { getApiError } from "../api/client";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";
import type { ReactionKind, ReactionTargetType } from "../types";

/**
 * Like / Dislike buttons with optimistic updates. The parent owns the counts
 * and the viewer's reaction via v-model; a failed request rolls them back.
 */
const props = withDefaults(defineProps<{ targetId: string; targetType: ReactionTargetType; compact?: boolean }>(), { compact: false });
const likes = defineModel<number>("likes", { required: true });
const dislikes = defineModel<number>("dislikes", { required: true });
const mine = defineModel<ReactionKind | null>("mine", { required: true });

const { t } = useI18n();
const auth = useAuthStore();
const toast = useToastStore();
const router = useRouter();
const busy = ref(false);

const bump = (kind: ReactionKind, delta: number) => {
  if (kind === "Like") likes.value = Math.max(0, likes.value + delta);
  else dislikes.value = Math.max(0, dislikes.value + delta);
};

const react = async (kind: ReactionKind) => {
  if (!auth.user) return router.push({ name: "login", query: { redirect: router.currentRoute.value.fullPath } });
  if (busy.value) return;
  busy.value = true;
  const snapshot = { likes: likes.value, dislikes: dislikes.value, mine: mine.value };
  try {
    if (mine.value === kind) {
      mine.value = null;
      bump(kind, -1);
      await reactionApi.remove(props.targetId, props.targetType);
    } else {
      if (mine.value) bump(mine.value, -1);
      mine.value = kind;
      bump(kind, 1);
      await reactionApi.set(props.targetId, props.targetType, kind);
    }
  } catch (error) {
    likes.value = snapshot.likes;
    dislikes.value = snapshot.dislikes;
    mine.value = snapshot.mine;
    toast.show(getApiError(error, t("errors.reaction")), "error");
  } finally {
    busy.value = false;
  }
};
</script>

<template>
  <div class="flex items-center gap-1">
    <button class="btn btn-ghost" :class="[compact ? 'btn-xs' : 'btn-sm', { 'text-error': mine === 'Like' }]" :disabled="busy" :aria-label="t('post.like')" :aria-pressed="mine === 'Like'" @click="react('Like')">
      <Heart :key="`like-${mine === 'Like'}`" :size="compact ? 14 : 17" :fill="mine === 'Like' ? 'currentColor' : 'none'" class="pop" /> {{ likes }}
    </button>
    <button class="btn btn-ghost" :class="[compact ? 'btn-xs' : 'btn-sm', { 'text-warning': mine === 'Dislike' }]" :disabled="busy" :aria-label="t('post.dislike')" :aria-pressed="mine === 'Dislike'" @click="react('Dislike')">
      <ThumbsDown :key="`dislike-${mine === 'Dislike'}`" :size="compact ? 14 : 17" :fill="mine === 'Dislike' ? 'currentColor' : 'none'" class="pop" /> {{ dislikes }}
    </button>
  </div>
</template>
