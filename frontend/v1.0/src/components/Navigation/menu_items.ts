import type { MenuItem } from "../../models/menu_item.ts";
import Endpoint from "../../routes/common/end_point.ts";

export const menu_items: MenuItem[] = [
  {
    id: "index",
    name: "Index",
    path: Endpoint.Index,
  },
  {
    id: "login",
    name: "Login",
    path: Endpoint.SignIn,
  },
  {
    id: "post-list-all",
    name: "Post List All",
    path: Endpoint.PostList,
  },
];
