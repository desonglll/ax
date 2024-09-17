import { Drawer } from "@mui/material";
import { SideDrawerList } from "./SideDrawerList.tsx";

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
        <SideDrawerList toggleDrawer={setOpen} />
      </Drawer>
    </>
  );
}
