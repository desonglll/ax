<script setup lang="ts">
import { CircleAlert, CircleCheck, Info, TriangleAlert, X } from "lucide-vue-next";
import { useI18n } from "vue-i18n";
import { useToastStore } from "../stores/toast";

const { t } = useI18n();
const toast = useToastStore();
const classes = { success: "alert-success", error: "alert-error", info: "alert-info", warning: "alert-warning" };
const icons = { success: CircleCheck, error: CircleAlert, info: Info, warning: TriangleAlert };
</script>

<template>
  <TransitionGroup name="list" tag="div" class="toast toast-end z-[100] mb-16 md:mb-0">
    <div v-for="item in toast.items" :key="item.id" class="alert shadow-lg" :class="classes[item.tone]">
      <component :is="icons[item.tone]" :size="18" />
      <span>{{ item.message }}</span>
      <button class="btn btn-ghost btn-xs btn-circle" :aria-label="t('common.close')" @click="toast.remove(item.id)"><X :size="14" /></button>
    </div>
  </TransitionGroup>
</template>
