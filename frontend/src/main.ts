import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "./App.vue";
import { router } from "./router";
import { useAuthStore } from "./stores/auth";
import "./style.css";

const pinia = createPinia();
const auth = useAuthStore(pinia);
let restored = false;

router.beforeEach(async to => {
  if (!restored) {
    await auth.restore();
    restored = true;
  }
  if (to.meta.requiresAuth && !auth.authenticated) return { name: "login", query: { redirect: to.fullPath } };
  if (to.meta.guest && auth.authenticated) return { name: "home" };
});

createApp(App).use(pinia).use(router).mount("#app");
