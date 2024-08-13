import { Route, Routes } from "react-router-dom";
import PostData from "../components/Post/PostData.tsx";
import AppMenu from "../components/Navigation/AppMenu.tsx";
import { SideDrawer } from "../components/Navigation/SideDrawer.tsx";
import { Paper } from "@mui/material";
import BottomNav from "../components/Navigation/BottomNav.tsx";
import { useState } from "react";
import Box from "@mui/material/Box";
import { PostDetail } from "../components/Post/PostDetail.tsx";
import { ReleasePost } from "../components/Post/ReleasePost.tsx";

export default function PostRoutes() {
  const [drawerOpen, setDrawerOpen] = useState(false);
  return (
    <>
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
        }}
      >
        <AppMenu drawerOpen={drawerOpen} setDrawerOpen={setDrawerOpen} />
        <SideDrawer open={drawerOpen} setOpen={setDrawerOpen} />
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

        {/* BottomNav 固定在底部 */}
        <Paper
          sx={{ position: "fixed", bottom: 0, left: 0, right: 0 }}
          elevation={3}
        >
          <BottomNav />
        </Paper>
      </Box>
    </>
  );
}
