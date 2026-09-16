import { createRouter, createWebHistory } from "vue-router";
import HomeView from "./views/HomeView.vue";

/** Matches the `.page-*` transition in style.css. */
const PAGE_TRANSITION_MS = 160;

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", name: "home", component: HomeView },
    { path: "/trending", name: "trending", component: () => import("./views/TrendingView.vue") },
    { path: "/posts/:id", name: "post", component: () => import("./views/PostView.vue") },
    { path: "/profile/:id?", name: "profile", component: () => import("./views/ProfileView.vue") },
    { path: "/people", name: "people", component: () => import("./views/PeopleView.vue") },
    { path: "/notifications", name: "notifications", component: () => import("./views/NotificationsView.vue"), meta: { requiresAuth: true } },
    { path: "/login", name: "login", component: () => import("./views/LoginView.vue"), meta: { guest: true } },
    { path: "/register", name: "register", component: () => import("./views/RegisterView.vue"), meta: { guest: true } },
    { path: "/:pathMatch(.*)*", redirect: "/" },
  ],
  // Wait for the outgoing page to fade before restoring the saved position;
  // kept-alive list pages are already rendered at full height by then.
  scrollBehavior: (_to, _from, saved) =>
    new Promise(resolve => setTimeout(() => resolve(saved || { top: 0 }), PAGE_TRANSITION_MS)),
});
