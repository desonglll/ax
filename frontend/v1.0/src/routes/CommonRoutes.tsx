import {Box, Paper} from "@mui/material";
import TopMenu from "../components/structures/navigation/menu/TopMenu.tsx";
import {SideDrawer} from "../components/structures/navigation/sider/SideDrawer.tsx";
import {useState} from "react";
import {Route, Routes} from "react-router-dom";
import IndexPage from "../pages/IndexPage";
import TestPage from "../pages/TestPage";
import PostRoutes from "./PostRoutes";
import {UserRoutes} from "./UserRoutes";
import BottomNav from "../components/structures/navigation/bottom/BottomNav.tsx";
import RouteEndpoint from "../config/endpoints/route_endpoint.ts";

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
        <TopMenu drawerOpen={drawerOpen} setDrawerOpen={setDrawerOpen} />
        <SideDrawer open={drawerOpen} setOpen={setDrawerOpen} />
        <Routes>
          <Route path={"post/*"} element={<PostRoutes />} />
          <Route path={"user/*"} element={<UserRoutes />} />
          <Route path={RouteEndpoint.Index} element={<IndexPage />} />
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
