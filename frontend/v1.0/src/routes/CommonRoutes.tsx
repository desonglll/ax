import { Box, Paper } from "@mui/material";
import AppMenu from "../components/navigation/AppMenu";
import { SideDrawer } from "../components/navigation/SideDrawer";
import { useState } from "react";
import { Route, Routes } from "react-router-dom";
import IndexPage from "../pages/IndexPage";
import TestPage from "../pages/TestPage";
import PostRoutes from "./PostRoutes";
import { UserRoutes } from "./UserRoutes";
import BottomNav from "../components/navigation/BottomNav";
import Endpoint from "./common/end_point";

function CommonRoutes() {
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
        <Routes>
          <Route path={"post/*"} element={<PostRoutes />} />
          <Route path={"user/*"} element={<UserRoutes />} />
          <Route path={Endpoint.Index} element={<IndexPage />} />
          <Route path={"test"} element={<TestPage />} />
        </Routes>
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

export default CommonRoutes;
