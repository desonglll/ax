import {MenuList} from "./MenuList.tsx";

export function Drawer() {
    const [open, setOpen] = React.useState(false);

    const toggleDrawer = (newOpen: boolean) => () => {
        setOpen(newOpen);
    };
    return (
        <>
            <Drawer open={true} onClose={toggleDrawer(false)}>
                <MenuList/>
            </Drawer>
        </>
    );
}