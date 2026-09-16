<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import { LoaderCircle } from "lucide-vue-next";
import { userApi } from "../api";
import { getApiError } from "../api/client";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";

const { t } = useI18n();
const router = useRouter();
const auth = useAuthStore();
const toast = useToastStore();
const userName = ref("");
const email = ref("");
const password = ref("");
const busy = ref(false);
const error = ref("");

const submit = async () => {
  busy.value = true;
  error.value = "";
  try {
    await userApi.register({ userName: userName.value.trim(), email: email.value.trim(), password: password.value });
    await auth.login(userName.value.trim(), password.value);
    toast.show(t("auth.accountCreated"), "success");
    router.push("/");
  } catch (err) {
    error.value = getApiError(err, t("errors.register"));
  } finally {
    busy.value = false;
  }
};
</script>

<template>
  <div class="ax-container grid min-h-[60vh] place-items-center">
    <section class="ax-panel w-full max-w-sm">
      <form class="card-body gap-4 p-6" @submit.prevent="submit">
        <h1 class="text-2xl font-bold">{{ t("auth.createAccount") }}</h1>
        <label class="fieldset"><span class="fieldset-legend">{{ t("auth.username") }}</span><input v-model="userName" class="input w-full" minlength="3" maxlength="32" autocomplete="username" autofocus required /><span class="label">{{ t("auth.usernameHint") }}</span></label>
        <label class="fieldset"><span class="fieldset-legend">{{ t("auth.email") }}</span><input v-model="email" type="email" class="input w-full" autocomplete="email" required /></label>
        <label class="fieldset"><span class="fieldset-legend">{{ t("auth.password") }}</span><input v-model="password" type="password" class="input w-full" minlength="8" maxlength="128" autocomplete="new-password" required /><span class="label">{{ t("auth.passwordHint") }}</span></label>
        <p v-if="error" class="text-sm text-error" role="alert">{{ error }}</p>
        <button class="btn btn-primary btn-block" :disabled="busy"><LoaderCircle v-if="busy" class="animate-spin" :size="17" /><template v-else>{{ t("auth.createAccount") }}</template></button>
        <p class="ax-muted text-center text-sm">{{ t("auth.alreadyRegistered") }} <RouterLink to="/login" class="link link-primary">{{ t("auth.signIn") }}</RouterLink></p>
      </form>
    </section>
  </div>
</template>
