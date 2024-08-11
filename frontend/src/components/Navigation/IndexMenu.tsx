import Box from "@mui/material/Box";
import {Button, Menu, Typography} from "@mui/material";
import {MenuItem} from "../../models/menu_item.ts";


export default function IndexMenu() {
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
        }
    ]

    const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
    const open = Boolean(anchorEl);
    const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
        setAnchorEl(event.currentTarget);
    };
    const handleClose = () => {
        setAnchorEl(null);
    };
    return (
        <>
            <Box>
                <Button
                    id="basic-button"
                    aria-controls={open ? 'basic-menu' : undefined}
                    aria-haspopup="true"
                    aria-expanded={open ? 'true' : undefined}
                    onClick={handleClick}
                >
                    Dashboard
                </Button>
                <Menu
                    id="menu-appbar"
                    anchorOrigin={{
                        vertical: 'bottom',
                        horizontal: 'left',
                    }}
                    keepMounted
                    transformOrigin={{
                        vertical: 'top',
                        horizontal: 'left',
                    }}
                    open={open}
                    anchorEl={anchorEl}
                    onClose={handleClose}
                    MenuListProps={{
                        'aria-labelledby': 'basic-button',
                    }}
                    sx={{
                        display: {xs: 'block', md: 'none'},
                    }}
                >
                    {menu_items.map((item) => (
                        <MenuItem key={item.id} onClick={() => {
                        }}>
                            <Typography textAlign="center">{item.name}</Typography>
                        </MenuItem>
                    ))}
                </Menu>
            </Box>
        </>
    )
}