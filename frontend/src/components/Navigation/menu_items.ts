import type { MenuItem } from "../../models/menu_item.ts";

export const menu_items: MenuItem[] = [
  {
    id: "index",
    name: "Index",
    path: "/common/index",
  },
  {
    id: "login",
    name: "Login",
    path: "/login",
  },
  {
    id: "post-list-all",
    name: "Post List All",
    path: "/common/post/list-all",
  },
];
