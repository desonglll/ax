import {List, ListItem, ListItemButton, ListItemIcon, ListItemText} from "@mui/material";
import {menu_items} from "./menu_items.ts";
import InboxIcon from "@mui/icons-material/Inbox";
import Box from "@mui/material/Box";
import {useNavigate} from "react-router-dom";

export function MenuList({toggleDrawer}: { toggleDrawer: (boolean) => void }) {

    const navigate = useNavigate()
    return (
        <>
            <Box sx={{width: 250}} role="presentation" onClick={() => {
                console.log("click")
                toggleDrawer(false)
            }}>
                <List>
                    {menu_items.map((menu_item) => (
                        <div key={menu_item.id}>
                            <ListItem key={menu_item.id} disablePadding>
                                <ListItemButton onClick={() => {
                                    navigate(menu_item.path)
                                }}>
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
        </>
    );
}