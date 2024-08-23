import {Route, Routes} from "react-router-dom";
import PostData from "../components/Post/PostData.tsx";
import {PostDetail} from "../components/Post/PostDetail.tsx";
import {ReleasePost} from "../components/Post/ReleasePost.tsx";
import End_points from "./common/end_points.ts";
import {AxSkeleton} from "../components/AxSkeleton.tsx";

export default function PostRoutes() {
    return (
        <>
            <AxSkeleton>
                <Routes>
                    <Route path={End_points.List} element={<PostData/>}/>
                    <Route path={`${End_points.Detail}/:id`} element={<PostDetail/>}/>
                    <Route path={End_points.New} element={<ReleasePost/>}/>
                </Routes>
            </AxSkeleton>
        </>
    );
}
