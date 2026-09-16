import type { Notification } from "../types";

export const describe = (item: Notification) => {
  switch (item.kind) {
    case "follow":
      return "started following you";
    case "comment":
      return "commented on your post";
    case "reaction":
      return "reacted to your post";
  }
};
