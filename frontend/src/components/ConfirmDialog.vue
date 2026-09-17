<script setup lang="ts">
import { nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useDialogStore } from "../stores/dialog";

const { t } = useI18n();
const dialog = useDialogStore();
const el = ref<HTMLDialogElement>();
const input = ref<HTMLInputElement>();
const value = ref("");

watch(() => dialog.current, async request => {
  if (!request) {
    if (el.value?.open) el.value.close();
    return;
  }
  value.value = request.input?.initial ?? "";
  await nextTick();
  if (!el.value?.open) el.value?.showModal();
  if (request.input) {
    input.value?.focus();
    input.value?.select();
  }
});

const submit = () => dialog.settle(dialog.current?.input ? value.value.trim() || null : true);
const cancel = () => dialog.settle(dialog.current?.input ? null : false);
</script>

<template>
  <dialog ref="el" class="modal modal-bottom sm:modal-middle" @cancel.prevent="cancel">
    <form v-if="dialog.current" class="modal-box" @submit.prevent="submit">
      <h3 class="text-lg font-bold">{{ dialog.current.title }}</h3>
      <p v-if="dialog.current.message" class="ax-muted mt-2">{{ dialog.current.message }}</p>
      <input v-if="dialog.current.input" ref="input" v-model="value" class="input mt-4 w-full" :placeholder="dialog.current.input.placeholder" />
      <div class="modal-action">
        <button type="button" class="btn btn-ghost" @click="cancel">{{ t("common.cancel") }}</button>
        <button type="submit" class="btn" :class="dialog.current.danger ? 'btn-error' : 'btn-primary'">{{ dialog.current.confirmLabel || t("common.confirm") }}</button>
      </div>
    </form>
    <div class="modal-backdrop" @click="cancel"></div>
  </dialog>
</template>
