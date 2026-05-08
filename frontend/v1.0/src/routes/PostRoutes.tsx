import { Routes, Route } from "react-router-dom";
import PostPage from "@/pages/PostPage";
import PostList from "@/components/structures/post/PostList";

function PostRoutes() {
  return (
    <Routes>
      <Route index element={<PostList />} />
      <Route path=":id" element={<PostPage />} />
    </Routes>
  );
}

export default PostRoutes;
