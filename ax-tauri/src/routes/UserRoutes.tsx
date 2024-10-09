import Box from "@mui/material/Box";
import {Route, Routes} from "react-router-dom";
import Profile from "../components/User/Profile.tsx";

export function UserRoutes() {
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
          <Route path={"profile"} element={<Profile />} />
        </Routes>
      </Box>
    </>
  );
}
