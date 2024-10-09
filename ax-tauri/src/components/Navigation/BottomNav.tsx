import {BottomNavigation, BottomNavigationAction, Fade} from "@mui/material";
import HomeIcon from "@mui/icons-material/Home";
import {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import ListIcon from "@mui/icons-material/List";
import AccountCircleIcon from "@mui/icons-material/AccountCircle";

const getLocationPath = () => {
  // 获取当前浏览器的 URL
  return window.location.pathname;
};
export default function BottomNav() {
  const [value, setValue] = useState("");
  const navigate = useNavigate();
  const [loading, setLoading] = useState(true);
  useEffect(() => {
    const path = getLocationPath();
    setValue(path);
    setLoading(false);
  }, []);
  return (
    <>
      {loading ? (
        <></>
      ) : (
        <Fade in={!loading}>
          <BottomNavigation
            showLabels
            value={value}
            onChange={(_event, newValue) => {
              navigate(newValue);
              setValue(newValue);
            }}
          >
            <BottomNavigationAction
              value={"/common/index"}
              label="Index"
              icon={<HomeIcon />}
            />
            <BottomNavigationAction
              value={"/common/post/list-all"}
              label="Post"
              icon={<ListIcon />}
            />
            <BottomNavigationAction
              value={"/common/user/profile"}
              label={"My"}
              icon={<AccountCircleIcon />}
            />
          </BottomNavigation>
        </Fade>
      )}
    </>
  );
}
