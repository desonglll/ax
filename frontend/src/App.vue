<script setup lang="ts">
import AppShell from "./components/AppShell.vue";
import ConfirmDialog from "./components/ConfirmDialog.vue";
import Lightbox from "./components/Lightbox.vue";
import ToastHost from "./components/ToastHost.vue";

/** List pages stay mounted so going back restores their content and scroll position. */
const keepAlive = ["HomeView", "TrendingView", "PeopleView", "ProfileView", "NotificationsView"];
</script>

<template>
  <AppShell>
    <RouterView v-slot="{ Component, route }">
      <Transition name="page" mode="out-in">
        <KeepAlive :include="keepAlive" :max="6">
          <component :is="Component" :key="route.name" />
        </KeepAlive>
      </Transition>
    </RouterView>
  </AppShell>
  <ToastHost />
  <Lightbox />
  <ConfirmDialog />
</template>
