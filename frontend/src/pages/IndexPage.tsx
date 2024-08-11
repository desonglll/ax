import {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import loginCheck from "../utils/login_check.ts";
import {List, ListItem, ListItemButton, ListItemIcon, ListItemText, Paper} from "@mui/material";
import InboxIcon from '@mui/icons-material/Inbox';
import {MenuItem} from "../models/menu_item.ts";
import AppMenu from "../components/Navigation/AppMenu.tsx";
import BottomNav from "../components/Navigation/BottomNav.tsx";
import Box from "@mui/material/Box";
import {SideDrawer} from "../components/Navigation/SideDrawer.tsx";

const menu_items: MenuItem[] = [
    {
        id: "index",
        name: "Index",
        path: "/index"
    },
    {
        id: "login",
        name: "Login",
        path: "/login"
    },
    {
        id: "post-list-all",
        name: "Post List All",
        path: "/post/list-all"
    }
]
export default function IndexPage() {
    const [drawerOpen, setDrawerOpen] = useState(false)

    const navigate = useNavigate()
    useEffect(() => {
        loginCheck().then((resp) => {
            if (resp.data.code === "Unauthorized") {
                navigate("/login")
            }
        })
    }, []);

    const handleOnClick = (endpoint: string) => {
        navigate(endpoint)
    }

    return (
        <>
            <AppMenu drawerOpen={drawerOpen} setDrawerOpen={setDrawerOpen}/>
            <SideDrawer open={drawerOpen} setOpen={setDrawerOpen}/>
            <Box
                sx={{
                    flex: 1, // 填充剩余空间
                    overflowY: 'auto', // 添加滚动条
                    paddingBottom: '56px', // 确保内容不被 BottomNav 遮挡
                    paddingTop: '56px'
                }}
            >
                <List>
                    {menu_items.map((menu_item) => (
                        <div key={menu_item.id}>
                            <ListItem disablePadding>
                                <ListItemButton onClick={() => handleOnClick(menu_item.path)}>
                                    <ListItemIcon>
                                        <InboxIcon/>
                                    </ListItemIcon>
                                    <ListItemText primary={menu_item.name}/>
                                </ListItemButton>
                            </ListItem>
                        </div>
                    ))}
                </List>
            </Box>
            <Paper sx={{position: 'fixed', bottom: 0, left: 0, right: 0}} elevation={3}>
                <BottomNav/>
            </Paper>
        </>
    )
}