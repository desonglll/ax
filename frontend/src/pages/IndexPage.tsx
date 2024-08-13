import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import loginCheck from "../utils/login_check.ts";
import { Typography } from "@mui/material";
import Box from "@mui/material/Box";

export default function IndexPage() {
  const navigate = useNavigate();
  useEffect(() => {
    loginCheck().then((resp) => {
      if (resp.data.code === "Unauthorized") {
        navigate("/login");
      }
    });
  }, [navigate]);

  return (
    <>
      <Box
        sx={{
          flex: 1, // 填充剩余空间
          overflowY: "auto", // 添加滚动条
          padding: "20px",
          paddingBottom: "56px", // 确保内容不被 BottomNav 遮挡
          paddingTop: "66px",
        }}
      >
        <Typography>This is Index Page</Typography>
      </Box>
    </>
  );
}
