import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { authApi } from "../api";
import type { User } from "../types";

export const useAuthStore = defineStore("auth", () => {
  const user = ref<User | null>(null);
  const loading = ref(true);
  const authenticated = computed(() => Boolean(user.value));

  const restore = async () => {
    loading.value = true;
    try {
      const response = await authApi.check();
      user.value = response.code === 200 ? response.body?.data || null : null;
    } catch {
      user.value = null;
    } finally {
      loading.value = false;
    }
  };

  const login = async (userName: string, password: string) => {
    const response = await authApi.login(userName, password);
    if (response.code !== 200 || !response.body?.data) throw new Error(response.message);
    user.value = response.body.data;
  };

  const logout = async () => {
    await authApi.logout();
    user.value = null;
  };

  return { user, loading, authenticated, restore, login, logout };
});
