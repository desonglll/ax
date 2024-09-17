import { Route, Routes } from "react-router-dom";
import { PostDetail } from "../components/structures/post/PostDetail.tsx";
import { ReleasePost } from "../components/structures/post/ReleasePost.tsx";
import { AxSkeleton } from "../components/common/skeleton/AxSkeleton.tsx";
import RouteEndpoint from "../config/endpoints/route_endpoint.ts";
import PostPage from "../pages/PostPage.tsx";

export default function PostRoutes() {
  return (
    <>
      <AxSkeleton>
        <Routes>
          <Route path={RouteEndpoint.List} element={<PostPage />} />
          <Route path={`${RouteEndpoint.Detail}/:id`} element={<PostDetail />} />
          <Route path={RouteEndpoint.New} element={<ReleasePost />} />
        </Routes>
      </AxSkeleton>
    </>
  );
}
