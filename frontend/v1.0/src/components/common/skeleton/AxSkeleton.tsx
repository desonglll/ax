import {ReactNode, useState} from "react";
import TopMenu from "../../structures/navigation/menu/TopMenu.tsx";
import {SideDrawer} from "../../structures/navigation/sider/SideDrawer.tsx";
import {Paper} from "@mui/material";
import BottomNav from "../../structures/navigation/bottom/BottomNav.tsx";
import {Box} from "@mui/joy";

export function AxSkeleton({children}: { children: ReactNode }) {
    const [drawerOpen, setDrawerOpen] = useState(false);

    return (
        <>
            <Box
                sx={{
                    display: "flex",
                    flexDirection: "column",
                }}
            >
                <TopMenu drawerOpen={drawerOpen} setDrawerOpen={setDrawerOpen}/>
                <SideDrawer open={drawerOpen} setOpen={setDrawerOpen}/>
                <Box
                    sx={{
                        flex: 1, // 填充剩余空间
                        overflowY: "auto", // 添加滚动条
                        paddingBottom: "56px", // 确保内容不被 BottomNav 遮挡
                        paddingTop: "64px",
                    }}
                >
                    {children}
                </Box>
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