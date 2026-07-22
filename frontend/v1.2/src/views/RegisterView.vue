<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { ArrowRight, LoaderCircle, Sparkles } from "lucide-vue-next";
import { userApi } from "../api";
import { getApiError } from "../api/client";
import { useToastStore } from "../stores/toast";

const router = useRouter();
const toast = useToastStore();
const userName = ref("");
const email = ref("");
const password = ref("");
const busy = ref(false);
const submit = async () => {
  busy.value = true;
  try { await userApi.register({ userName: userName.value, email: email.value, password: password.value }); toast.show("Account created. You can sign in now.", "success"); router.push("/login"); }
  catch (error) { toast.show(getApiError(error, "Registration failed"), "error"); }
  finally { busy.value = false; }
};
</script>

<template>
  <div class="ax-container grid min-h-[72vh] place-items-center"><section class="ax-panel w-full max-w-lg"><div class="card-body gap-5 p-7"><span class="grid size-12 place-items-center rounded-2xl bg-secondary text-secondary-content"><Sparkles /></span><div><h1 class="text-3xl font-black">Join Ax</h1><p class="ax-muted">Build a thoughtful space around your ideas.</p></div><form class="grid gap-4" @submit.prevent="submit"><label class="fieldset"><span class="fieldset-legend">Username</span><input v-model="userName" class="input w-full" minlength="3" maxlength="32" required /></label><label class="fieldset"><span class="fieldset-legend">Email</span><input v-model="email" type="email" class="input w-full" required /></label><label class="fieldset"><span class="fieldset-legend">Password</span><input v-model="password" type="password" class="input w-full" minlength="8" maxlength="128" required /><span class="label">8–128 characters</span></label><button class="btn btn-primary btn-block" :disabled="busy"><LoaderCircle v-if="busy" class="animate-spin" :size="17" /><template v-else>Create account <ArrowRight :size="17" /></template></button></form><p class="text-center text-sm ax-muted">Already a member? <RouterLink to="/login" class="link link-primary font-bold">Sign in</RouterLink></p></div></section></div>
</template>
