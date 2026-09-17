import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { authApi } from "../api";
import type { User } from "../types";

/** Why the sign-in dialog was opened; picks the explanatory line under its title. */
export type AuthReason = "generic" | "like" | "comment" | "follow" | "following" | "post";

export const useAuthStore = defineStore("auth", () => {
  const user = ref<User | null>(null);
  const loading = ref(true);
  const authenticated = computed(() => Boolean(user.value));

  /** Re-reads the session cookie's owner; a 401 simply means signed out. */
  const restore = async () => {
    loading.value = true;
    try {
      user.value = (await authApi.me()).body?.data || null;
    } catch {
      user.value = null;
    } finally {
      loading.value = false;
    }
  };

  const login = async (userName: string, password: string) => {
    const response = await authApi.login(userName, password);
    if (!response.body?.data) throw new Error(response.message);
    user.value = response.body.data;
  };

  const logout = async () => {
    try {
      await authApi.logout();
    } finally {
      user.value = null;
    }
  };

  // Sign-in dialog (rendered by AuthDialog) so guests can act without leaving the page.
  const prompt = ref<AuthReason | null>(null);
  let resolvePrompt: ((signedIn: boolean) => void) | null = null;

  /** Resolves true once there is a signed-in user, asking the guest to sign in first. */
  const ensure = (reason: AuthReason = "generic") => {
    if (user.value) return Promise.resolve(true);
    resolvePrompt?.(false);
    prompt.value = reason;
    return new Promise<boolean>(resolve => {
      resolvePrompt = resolve;
    });
  };

  const settlePrompt = (signedIn: boolean) => {
    resolvePrompt?.(signedIn && Boolean(user.value));
    resolvePrompt = null;
    prompt.value = null;
  };

  return { user, loading, authenticated, restore, login, logout, prompt, ensure, settlePrompt };
});
