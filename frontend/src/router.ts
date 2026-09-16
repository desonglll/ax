import { createRouter, createWebHistory } from "vue-router";
import HomeView from "./views/HomeView.vue";

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
  scrollBehavior: (_to, _from, saved) => saved || { top: 0 },
});
