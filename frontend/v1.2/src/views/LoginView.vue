<script setup lang="ts">
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { ArrowRight, LoaderCircle, LockKeyhole } from "lucide-vue-next";
import { getApiError } from "../api/client";
import { useAuthStore } from "../stores/auth";
import { useToastStore } from "../stores/toast";

const auth = useAuthStore();
const toast = useToastStore();
const router = useRouter();
const route = useRoute();
const userName = ref("");
const password = ref("");
const busy = ref(false);

const submit = async () => {
  busy.value = true;
  try {
    await auth.login(userName.value, password.value);
    toast.show("Welcome back", "success");
    router.push(String(route.query.redirect || "/"));
  } catch (error) { toast.show(getApiError(error, "Login failed"), "error"); }
  finally { busy.value = false; }
};
</script>

<template>
  <div class="ax-container grid min-h-[72vh] place-items-center"><section class="ax-panel w-full max-w-md"><div class="card-body gap-5 p-7"><span class="grid size-12 place-items-center rounded-2xl bg-primary text-primary-content"><LockKeyhole /></span><div><h1 class="text-3xl font-black">Welcome back</h1><p class="ax-muted">Continue your conversations on Ax.</p></div><form class="space-y-4" @submit.prevent="submit"><label class="fieldset"><span class="fieldset-legend">Username</span><input v-model="userName" class="input w-full" autocomplete="username" required /></label><label class="fieldset"><span class="fieldset-legend">Password</span><input v-model="password" type="password" class="input w-full" autocomplete="current-password" required /></label><button class="btn btn-primary btn-block" :disabled="busy"><LoaderCircle v-if="busy" class="animate-spin" :size="17" /><template v-else>Sign in <ArrowRight :size="17" /></template></button></form><p class="text-center text-sm ax-muted">New to Ax? <RouterLink to="/register" class="link link-primary font-bold">Create an account</RouterLink></p></div></section></div>
</template>
