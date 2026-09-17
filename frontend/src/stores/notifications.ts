import { defineStore } from "pinia";
import { ref } from "vue";
import { notificationApi } from "../api";

/** Unread count shared by the navbar bell, bottom nav and notifications page. */
export const useNotificationStore = defineStore("notifications", () => {
  const unread = ref(0);
  let timer = 0;

  const refresh = async () => {
    try {
      unread.value = (await notificationApi.unread()).body?.data || 0;
    } catch {
      /* signed out or offline: keep the last value */
    }
  };

  const markRead = async (id: number) => {
    await notificationApi.read(id).catch(() => undefined);
    unread.value = Math.max(0, unread.value - 1);
  };

  const markAllRead = async () => {
    await notificationApi.readAll();
    unread.value = 0;
  };

  /** Polls while signed in; also refreshes whenever the tab regains focus. */
  const start = () => {
    stop();
    refresh();
    timer = window.setInterval(refresh, 60_000);
    document.addEventListener("visibilitychange", onVisible);
  };
  const stop = () => {
    window.clearInterval(timer);
    document.removeEventListener("visibilitychange", onVisible);
    unread.value = 0;
  };
  const onVisible = () => {
    if (document.visibilityState === "visible") refresh();
  };

  return { unread, refresh, markRead, markAllRead, start, stop };
});
