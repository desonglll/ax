import { type RouteConfig, index, route } from "@react-router/dev/routes";

export default [
  index("routes/home.tsx"),
  route("trending", "routes/trending.tsx"),
  route("posts/:postId", "routes/post.tsx"),
  route("profile/:userId?", "routes/profile.tsx"),
  route("files", "routes/files.tsx"),
  route("login", "routes/login.tsx"),
  route("register", "routes/register.tsx"),
] satisfies RouteConfig;
