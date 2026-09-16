<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRoute, useRouter } from "vue-router";
import { LoaderCircle } from "lucide-vue-next";
import { getApiError } from "../api/client";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";

const { t } = useI18n();
const auth = useAuthStore();
const toast = useToastStore();
const router = useRouter();
const route = useRoute();
const userName = ref("");
const password = ref("");
const busy = ref(false);
const error = ref("");

const submit = async () => {
  busy.value = true;
  error.value = "";
  try {
    await auth.login(userName.value.trim(), password.value);
    toast.show(t("auth.signedInAs", { name: auth.user?.userName }), "success");
    const redirect = String(route.query.redirect || "/");
    router.push(redirect.startsWith("/") ? redirect : "/");
  } catch (err) {
    error.value = getApiError(err, t("errors.signIn"));
  } finally {
    busy.value = false;
  }
};
</script>

<template>
  <div class="ax-container grid min-h-[60vh] place-items-center">
    <section class="ax-panel w-full max-w-sm">
      <form class="card-body gap-4 p-6" @submit.prevent="submit">
        <h1 class="text-2xl font-bold">{{ t("auth.signIn") }}</h1>
        <label class="fieldset"><span class="fieldset-legend">{{ t("auth.username") }}</span><input v-model="userName" class="input w-full" autocomplete="username" autofocus required /></label>
        <label class="fieldset"><span class="fieldset-legend">{{ t("auth.password") }}</span><input v-model="password" type="password" class="input w-full" autocomplete="current-password" required /></label>
        <p v-if="error" class="text-sm text-error" role="alert">{{ error }}</p>
        <button class="btn btn-primary btn-block" :disabled="busy"><LoaderCircle v-if="busy" class="animate-spin" :size="17" /><template v-else>{{ t("auth.signIn") }}</template></button>
        <p class="ax-muted text-center text-sm">{{ t("auth.noAccount") }} <RouterLink to="/register" class="link link-primary">{{ t("auth.register") }}</RouterLink></p>
      </form>
    </section>
  </div>
</template>
