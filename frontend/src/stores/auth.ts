import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { authApi } from "../api";
import type { User } from "../types";

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

  return { user, loading, authenticated, restore, login, logout };
});
