import { defineStore } from "pinia";
import { ref } from "vue";

export type ToastTone = "success" | "error" | "info" | "warning";
export interface ToastItem { id: number; message: string; tone: ToastTone }

export const useToastStore = defineStore("toast", () => {
  const items = ref<ToastItem[]>([]);
  let id = 0;
  const show = (message: string, tone: ToastTone = "info") => {
    const toast = { id: ++id, message, tone };
    items.value.push(toast);
    window.setTimeout(() => remove(toast.id), 3600);
  };
  const remove = (toastId: number) => {
    items.value = items.value.filter(item => item.id !== toastId);
  };
  return { items, show, remove };
});
