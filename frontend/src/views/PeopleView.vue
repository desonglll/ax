<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { Search, ShieldCheck, Trash2, UserRound, UsersRound } from "lucide-vue-next";
import { userApi } from "../api";
import { getApiError } from "../api/client";
import Avatar from "../components/Avatar.vue";
import EmptyState from "../components/EmptyState.vue";
import { useAuthStore } from "../stores/auth";
import { useDialogStore } from "../stores/dialog";
import { useToastStore } from "../stores/toast";
import type { User } from "../types";

defineOptions({ name: "PeopleView" });

const { t } = useI18n();
const auth = useAuthStore();
const toast = useToastStore();
const dialog = useDialogStore();
const users = ref<User[]>([]);
const query = ref("");
const loading = ref(true);

const filtered = computed(() => {
  const q = query.value.trim().toLowerCase();
  return q ? users.value.filter(user => `${user.userName} ${user.fullName || ""} ${user.email}`.toLowerCase().includes(q)) : users.value;
});

const load = async () => {
  try {
    users.value = (await userApi.list({ limit: 100 })).body?.data || [];
  } catch (error) {
    toast.show(getApiError(error, t("errors.loadPeople")), "error");
  } finally {
    loading.value = false;
  }
};

const toggle = async (user: User, field: "isAdmin" | "isActive") => {
  try {
    const response = await userApi.update(user.id, { [field]: !user[field] });
    if (response.body?.data) users.value = users.value.map(item => (item.id === user.id ? response.body!.data : item));
    toast.show(t("people.userUpdated"), "success");
  } catch (error) {
    toast.show(getApiError(error, t("errors.updateUser")), "error");
  }
};

const remove = async (user: User) => {
  if (!(await dialog.confirm({ title: t("people.confirmDelete", { name: user.userName }), message: t("common.irreversible"), confirmLabel: t("common.delete"), danger: true }))) return;
  try {
    await userApi.delete(user.id);
    users.value = users.value.filter(item => item.id !== user.id);
    toast.show(t("people.userDeleted"), "success");
  } catch (error) {
    toast.show(getApiError(error, t("errors.delete")), "error");
  }
};

onMounted(load);
</script>

<template>
  <div class="ax-container space-y-5">
    <header class="flex flex-wrap items-center justify-between gap-4">
      <h1 class="ax-page-title">{{ t("people.title") }}</h1>
      <label class="input flex items-center gap-2"><Search :size="16" /><input v-model="query" type="search" :placeholder="t('people.find')" /></label>
    </header>

    <div v-if="loading" class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
      <div v-for="n in 6" :key="n" class="ax-panel"><div class="card-body gap-3"><div class="flex items-center gap-3"><div class="skeleton size-14 rounded-full"></div><div class="flex-1 space-y-2"><div class="skeleton h-3 w-24"></div><div class="skeleton h-3 w-16"></div></div></div><div class="skeleton h-8 w-full"></div></div></div>
    </div>
    <EmptyState v-else-if="!filtered.length" :icon="UsersRound" :title="t('people.noMatches')" />
    <TransitionGroup v-else name="list" tag="div" class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
      <article v-for="person in filtered" :key="person.id" class="ax-panel transition-shadow hover:shadow-sm" :class="{ 'opacity-60': !person.isActive }">
        <div class="card-body gap-3">
          <div class="flex items-start gap-3">
            <RouterLink :to="`/profile/${person.id}`"><Avatar :name="person.userName" size="lg" /></RouterLink>
            <div class="min-w-0 flex-1">
              <RouterLink :to="`/profile/${person.id}`" class="font-bold hover:text-primary">{{ person.fullName || person.userName }}</RouterLink>
              <p class="ax-muted truncate text-sm">@{{ person.userName }}</p>
            </div>
            <span v-if="person.isAdmin" class="badge badge-primary badge-sm"><ShieldCheck :size="12" /> {{ t("nav.admin") }}</span>
            <span v-else-if="!person.isActive" class="badge badge-warning badge-sm">{{ t("people.inactive") }}</span>
          </div>
          <p v-if="auth.user?.isAdmin" class="ax-muted truncate text-sm">{{ person.email }}</p>
          <div class="card-actions mt-1">
            <RouterLink :to="`/profile/${person.id}`" class="btn btn-outline btn-sm flex-1"><UserRound :size="15" /> {{ t("people.profile") }}</RouterLink>
            <div v-if="auth.user?.isAdmin && auth.user.id !== person.id" class="dropdown dropdown-end">
              <button tabindex="0" class="btn btn-ghost btn-sm">{{ t("people.manage") }}</button>
              <ul tabindex="0" class="dropdown-content menu z-20 w-48 rounded-box border border-base-300 bg-base-100 p-2 shadow-xl">
                <li><button @click="toggle(person, 'isAdmin')">{{ person.isAdmin ? t("people.removeAdmin") : t("people.makeAdmin") }}</button></li>
                <li><button @click="toggle(person, 'isActive')">{{ person.isActive ? t("people.deactivate") : t("people.activate") }}</button></li>
                <li><button class="text-error" @click="remove(person)"><Trash2 :size="14" /> {{ t("common.delete") }}</button></li>
              </ul>
            </div>
          </div>
        </div>
      </article>
    </TransitionGroup>
  </div>
</template>
