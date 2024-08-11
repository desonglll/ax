import {BottomNavigation, BottomNavigationAction, Collapse, Fade} from "@mui/material";
import HomeIcon from '@mui/icons-material/Home';
import {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import ListIcon from '@mui/icons-material/List';

const getLocationPath = () => {
    // 获取当前浏览器的 URL
    const path = window.location.pathname;
    return path
}
export default function BottomNav() {
    const [value, setValue] = useState("")
    const navigate = useNavigate()
    const [loading, setLoading] = useState(true)
    useEffect(() => {
        const path = getLocationPath()
        setValue(path)
        setLoading(false)
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
                        onChange={(event, newValue) => {
                            navigate(newValue)
                            setValue(newValue);
                        }}
                    >
                        <BottomNavigationAction value={"/index"} label="Index" icon={<HomeIcon/>}/>
                        <BottomNavigationAction value={"/post/list-all"} label="Post" icon={<ListIcon/>}/>
                    </BottomNavigation>
                </Fade>
            )}
        </>
    )
}