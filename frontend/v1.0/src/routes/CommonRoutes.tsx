import {Box, Paper} from "@mui/material";
import AppMenu from "../components/Navigation/AppMenu";
import {SideDrawer} from "../components/Navigation/SideDrawer";
import {useState} from "react";
import {Route, Routes} from "react-router-dom";
import IndexPage from "../pages/IndexPage";
import TestPage from "../pages/TestPage";
import PostRoutes from "./PostRoutes";
import {UserRoutes} from "./UserRoutes";
import BottomNav from "../components/Navigation/BottomNav";
import End_points from "./common/end_points.ts";

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
                <AppMenu drawerOpen={drawerOpen} setDrawerOpen={setDrawerOpen}/>
                <SideDrawer open={drawerOpen} setOpen={setDrawerOpen}/>
                <Routes>
                    <Route path={"post/*"} element={<PostRoutes/>}/>
                    <Route path={"user/*"} element={<UserRoutes/>}/>
                    <Route path={End_points.Index} element={<IndexPage/>}/>
                    <Route path={"test"} element={<TestPage/>}/>
                </Routes>
                {/* BottomNav 固定在底部 */}
                <Paper
                    sx={{position: "fixed", bottom: 0, left: 0, right: 0}}
                    elevation={3}
                >
                    <BottomNav/>
                </Paper>
            </Box>
        </>
    );
}

export default CommonRoutes;
