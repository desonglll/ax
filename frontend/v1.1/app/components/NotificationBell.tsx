import { useEffect, useState } from "react";
import { Link } from "react-router";
import { notificationApi, type Notification } from "../utils/api";

const kindText = (n: Notification): string => {
  const actor = n.actorName || `user #${n.actorId}`;
  switch (n.kind) {
    case "follow":
      return `${actor} followed you`;
    case "comment":
      return `${actor} commented on your post`;
    case "reaction":
      return `${actor} reacted to your post`;
    default:
      return `${actor} did something`;
  }
};

/**
 * Navbar bell with an unread badge and a dropdown of recent notifications.
 * Polls the unread count once a minute while logged in.
 */
export function NotificationBell() {
  const [unread, setUnread] = useState(0);
  const [items, setItems] = useState<Notification[]>([]);
  const [open, setOpen] = useState(false);
  const [loading, setLoading] = useState(false);

  const refreshCount = async () => {
    try {
      const res = await notificationApi.unreadCount();
      if (res.code === 200) setUnread(res.body?.data ?? 0);
    } catch {
      // Not logged in or backend unreachable; keep the badge quiet.
    }
  };

  useEffect(() => {
    refreshCount();
    const timer = setInterval(refreshCount, 60_000);
    return () => clearInterval(timer);
  }, []);

  const toggleOpen = async () => {
    const next = !open;
    setOpen(next);
    if (next) {
      setLoading(true);
      try {
        const res = await notificationApi.list({ limit: 10 });
        if (res.code === 200 && res.body?.data) setItems(res.body.data);
      } catch {
        setItems([]);
      } finally {
        setLoading(false);
      }
    }
  };

  const markAllRead = async () => {
    try {
      await notificationApi.markAllRead();
      setUnread(0);
      setItems((prev) => prev.map((n) => ({ ...n, isRead: true })));
    } catch (e) {
      console.error("Failed to mark notifications read", e);
    }
  };

  return (
    <div className="relative">
      <button
        onClick={toggleOpen}
        className="btn btn-ghost btn-xs relative"
        aria-label="Notifications"
      >
        🔔
        {unread > 0 && (
          <span className="badge badge-error badge-xs absolute -top-1 -right-1 font-mono">
            {unread > 99 ? "99+" : unread}
          </span>
        )}
      </button>
      {open && (
        <div className="absolute right-0 mt-2 w-72 z-50 card card-border bg-base-100 shadow-lg p-2 text-left">
          <div className="flex items-center justify-between px-2 py-1 border-b border-base-300">
            <span className="text-xs font-bold uppercase tracking-wide">Notifications</span>
            {unread > 0 && (
              <button className="btn btn-ghost btn-xs text-primary" onClick={markAllRead}>
                Mark all read
              </button>
            )}
          </div>
          {loading ? (
            <div className="p-3 text-center">
              <span className="loading loading-spinner loading-xs"></span>
            </div>
          ) : items.length === 0 ? (
            <div className="p-3 text-xs text-center opacity-60">No notifications yet.</div>
          ) : (
            <ul className="max-h-72 overflow-y-auto">
              {items.map((n) => (
                <li
                  key={n.id}
                  className={`px-2 py-1.5 text-xs border-b border-base-200 last:border-0 ${
                    n.isRead ? "opacity-60" : "font-semibold"
                  }`}
                >
                  {n.postId ? (
                    <Link to={`/posts/${n.postId}`} onClick={() => setOpen(false)} className="hover:text-primary">
                      {kindText(n)}
                    </Link>
                  ) : (
                    <Link to={`/profile/${n.actorId}`} onClick={() => setOpen(false)} className="hover:text-primary">
                      {kindText(n)}
                    </Link>
                  )}
                  <div className="text-[10px] opacity-50 font-mono">
                    {new Date(n.createdAt).toLocaleString()}
                  </div>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}
    </div>
  );
}
