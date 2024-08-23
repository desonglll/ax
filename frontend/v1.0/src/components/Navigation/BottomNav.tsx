import {BottomNavigation, BottomNavigationAction, Fade} from "@mui/material";
import HomeIcon from "@mui/icons-material/Home";
import {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import ListIcon from "@mui/icons-material/List";
import AccountCircleIcon from "@mui/icons-material/AccountCircle";
import End_points from "../../routes/common/end_points.ts";

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
                            value={"/index"}
                            label="Index"
                            icon={<HomeIcon/>}
                        />
                        <BottomNavigationAction
                            value={End_points.PostList}
                            label="Post"
                            icon={<ListIcon/>}
                        />
                        <BottomNavigationAction
                            value={End_points.Profile}
                            label={"My"}
                            icon={<AccountCircleIcon/>}
                        />
                    </BottomNavigation>
                </Fade>
            )}
        </>
    );
}
