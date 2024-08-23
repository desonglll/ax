import type {MenuItem} from "../../models/menu_item.ts";
import End_points from "../../routes/common/end_points.ts";

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
        path: End_points.Post + End_points.PostList,
    },
];
