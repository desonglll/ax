<script setup lang="ts">
import { LoaderCircle } from "lucide-vue-next";
import { nextTick, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { userApi } from "../api";
import { getApiError } from "../api/client";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";

/**
 * Sign-in / register form shared by the login and register pages and the
 * in-place sign-in dialog. The parent decides what "switch mode" means
 * (a route change on the pages, a local toggle in the dialog).
 */
const props = defineProps<{ mode: "login" | "register" }>();
const emit = defineEmits<{ done: []; switch: [] }>();

const { t } = useI18n();
const auth = useAuthStore();
const toast = useToastStore();
const root = ref<HTMLFormElement>();
const userName = ref("");
const email = ref("");
const password = ref("");
const busy = ref(false);
const error = ref("");

const focusFirst = async () => {
  await nextTick();
  root.value?.querySelector<HTMLInputElement>("input")?.focus();
};
onMounted(focusFirst);
watch(() => props.mode, () => {
  error.value = "";
  focusFirst();
});

const submit = async () => {
  if (busy.value) return;
  busy.value = true;
  error.value = "";
  const name = userName.value.trim();
  try {
    if (props.mode === "register") {
      await userApi.register({ userName: name, email: email.value.trim(), password: password.value });
      await auth.login(name, password.value);
      toast.show(t("auth.accountCreated"), "success");
    } else {
      await auth.login(name, password.value);
      toast.show(t("auth.signedInAs", { name: auth.user?.userName }), "success");
    }
    password.value = "";
    emit("done");
  } catch (err) {
    error.value = getApiError(err, props.mode === "register" ? t("errors.register") : t("errors.signIn"));
  } finally {
    busy.value = false;
  }
};
</script>

<template>
  <form ref="root" class="flex flex-col gap-3" @submit.prevent="submit">
    <label class="fieldset py-0">
      <span class="fieldset-legend">{{ t("auth.username") }}</span>
      <input v-model="userName" class="input w-full" autocomplete="username" autofocus autocapitalize="off" :minlength="mode === 'register' ? 3 : undefined" maxlength="32" required />
      <span v-if="mode === 'register'" class="label">{{ t("auth.usernameHint") }}</span>
    </label>
    <label v-if="mode === 'register'" class="fieldset py-0">
      <span class="fieldset-legend">{{ t("auth.email") }}</span>
      <input v-model="email" type="email" class="input w-full" autocomplete="email" required />
    </label>
    <label class="fieldset py-0">
      <span class="fieldset-legend">{{ t("auth.password") }}</span>
      <input v-model="password" type="password" class="input w-full" :autocomplete="mode === 'register' ? 'new-password' : 'current-password'" :minlength="mode === 'register' ? 8 : undefined" maxlength="128" required />
      <span v-if="mode === 'register'" class="label">{{ t("auth.passwordHint") }}</span>
    </label>
    <p v-if="error" class="rounded-box bg-error/10 px-3 py-2 text-sm text-error" role="alert">{{ error }}</p>
    <button class="btn btn-primary btn-block mt-1" :disabled="busy">
      <LoaderCircle v-if="busy" class="animate-spin" :size="17" />
      {{ mode === "register" ? t("auth.createAccount") : t("auth.signIn") }}
    </button>
    <p class="ax-muted text-center text-sm">
      {{ mode === "register" ? t("auth.alreadyRegistered") : t("auth.noAccount") }}
      <button type="button" class="link link-primary" @click="emit('switch')">{{ mode === "register" ? t("auth.signIn") : t("auth.register") }}</button>
    </p>
  </form>
</template>
