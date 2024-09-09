import { Route, Routes } from "react-router-dom";
import PostData from "../components/Post/PostData.tsx";
import { PostDetail } from "../components/Post/PostDetail.tsx";
import { ReleasePost } from "../components/Post/ReleasePost.tsx";
import { AxSkeleton } from "../components/AxSkeleton.tsx";
import Endpoint from "./common/end_point.ts";

export default function PostRoutes() {
  return (
    <>
      <AxSkeleton>
        <Routes>
          <Route path={Endpoint.List} element={<PostData />} />
          <Route path={`${Endpoint.Detail}/:id`} element={<PostDetail />} />
          <Route path={Endpoint.New} element={<ReleasePost />} />
        </Routes>
      </AxSkeleton>
    </>
  );
}
