import {Route, Routes} from "react-router-dom";
import PostData from "../components/Post/PostData.tsx";
import Box from "@mui/material/Box";
import {PostDetail} from "../components/Post/PostDetail.tsx";
import {ReleasePost} from "../components/Post/ReleasePost.tsx";

export default function PostRoutes() {
  return (
    <>
      <Box
        sx={{
          flex: 1, // 填充剩余空间
          overflowY: "auto", // 添加滚动条
          paddingBottom: "56px", // 确保内容不被 BottomNav 遮挡
          paddingTop: "64px",
        }}
      >
        {/* 页面内容 */}
        <Routes>
          <Route path={"list-all"} element={<PostData />} />
          <Route path={"detail/:id"} element={<PostDetail />} />
          <Route path={"new"} element={<ReleasePost />} />
        </Routes>
      </Box>
    </>
  );
}
