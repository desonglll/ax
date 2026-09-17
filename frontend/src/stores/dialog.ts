import { defineStore } from "pinia";
import { ref } from "vue";

export interface DialogRequest {
  title: string;
  message?: string;
  confirmLabel?: string;
  danger?: boolean;
  /** When set, the dialog shows a text input and resolves with its value. */
  input?: { placeholder?: string; initial?: string };
}

/**
 * Promise-based replacement for window.confirm / window.prompt, rendered by
 * ConfirmDialog so it matches the theme and works on phones.
 */
export const useDialogStore = defineStore("dialog", () => {
  const current = ref<DialogRequest | null>(null);
  let resolver: ((value: string | boolean | null) => void) | null = null;

  const open = (request: DialogRequest) =>
    new Promise<string | boolean | null>(resolve => {
      resolver?.(null);
      current.value = request;
      resolver = resolve;
    });

  const settle = (value: string | boolean | null) => {
    resolver?.(value);
    resolver = null;
    current.value = null;
  };

  const confirm = async (request: Omit<DialogRequest, "input">) => (await open(request)) === true;
  const prompt = async (request: DialogRequest) => {
    const value = await open({ ...request, input: request.input ?? {} });
    return typeof value === "string" ? value : null;
  };

  return { current, confirm, prompt, settle };
});
