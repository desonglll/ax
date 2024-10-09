import {Drawer} from "@mui/material";
import {MenuList} from "./MenuList.tsx";

export function SideDrawer({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (arg0: boolean) => void;
}) {
  const toggleDrawer = (newOpen: boolean) => () => {
    setOpen(newOpen);
  };

  return (
    <>
      <Drawer open={open} onClose={toggleDrawer(false)}>
        <MenuList toggleDrawer={setOpen} />
      </Drawer>
    </>
  );
}
