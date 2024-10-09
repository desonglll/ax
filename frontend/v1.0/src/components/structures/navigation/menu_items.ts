import type {MenuItem} from "../../../models/menu_item.ts";
import RouteEndpoint from "../../../config/endpoints/route_endpoint.ts";

export const menu_items: MenuItem[] = [
  {
    id: "index",
    name: "Index",
    path: RouteEndpoint.Index,
  },
  {
    id: "login",
    name: "Login",
    path: RouteEndpoint.SignIn,
  },
  {
    id: "post-list-all",
    name: "Post List All",
    path: RouteEndpoint.PostList,
  },
];
