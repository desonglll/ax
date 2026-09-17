<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useRoute, useRouter } from "vue-router";
import AuthForm from "../components/AuthForm.vue";

const { t } = useI18n();
const router = useRouter();
const route = useRoute();

const done = () => {
  const redirect = String(route.query.redirect || "/");
  router.replace(redirect.startsWith("/") ? redirect : "/");
};
</script>

<template>
  <div class="ax-container grid min-h-[60vh] place-items-center">
    <section class="ax-panel w-full max-w-sm">
      <div class="card-body gap-4 p-6">
        <h1 class="text-2xl font-bold">{{ t("auth.signIn") }}</h1>
        <AuthForm mode="login" @done="done" @switch="router.replace({ name: 'register', query: route.query })" />
      </div>
    </section>
  </div>
</template>
