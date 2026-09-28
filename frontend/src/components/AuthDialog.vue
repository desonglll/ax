<script setup lang="ts">
import { LockKeyhole, X } from "lucide-vue-next";
import { nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useAuthStore } from "../stores/auth";
import AuthForm from "./AuthForm.vue";

/** In-place sign-in for guests who try to like, comment or follow. */
const { t } = useI18n();
const auth = useAuthStore();
const el = ref<HTMLDialogElement>();
const mode = ref<"login" | "register">("login");

watch(() => auth.prompt, async reason => {
  if (!reason) {
    if (el.value?.open) el.value.close();
    return;
  }
  mode.value = "login";
  await nextTick();
  if (!el.value?.open) el.value?.showModal();
});
</script>

<template>
  <dialog ref="el" class="modal modal-bottom sm:modal-middle" @cancel.prevent="auth.settlePrompt(false)">
    <div v-if="auth.prompt" class="modal-box max-w-sm pb-[max(1.5rem,env(safe-area-inset-bottom))]">
      <button type="button" class="btn btn-ghost btn-circle btn-sm absolute right-3 top-3" :aria-label="t('common.close')" @click="auth.settlePrompt(false)"><X :size="18" /></button>
      <div class="mb-4 flex items-start gap-3 pr-8">
        <span class="grid size-10 shrink-0 place-items-center rounded-full bg-primary/12 text-primary"><LockKeyhole :size="19" /></span>
        <div>
          <h3 class="text-lg font-bold leading-tight">{{ mode === "register" ? t("auth.promptRegisterTitle") : t("auth.promptTitle") }}</h3>
          <p class="ax-muted mt-1 text-sm">{{ t(`auth.reasons.${auth.prompt}`) }}</p>
        </div>
      </div>
      <AuthForm :key="mode" :mode="mode" @done="auth.settlePrompt(true)" @switch="mode = mode === 'login' ? 'register' : 'login'" />
    </div>
    <div class="modal-backdrop" @click="auth.settlePrompt(false)"></div>
  </dialog>
</template>
