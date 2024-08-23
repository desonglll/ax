import {useEffect} from "react";
import {useNavigate} from "react-router-dom";
import loginCheck from "../utils/login_check.ts";
import {Typography} from "@mui/material";
import End_points from "../routes/common/end_points.ts";
import {AxSkeleton} from "../components/AxSkeleton.tsx";

export default function IndexPage() {
    const navigate = useNavigate();
    useEffect(() => {
        loginCheck().then((resp) => {
            if (resp.data.code === "Unauthorized") {
                navigate(End_points.SignIn);
            }
        });
    }, [navigate]);

    return (
        <>
            <AxSkeleton>
                <Typography>This is Index Page</Typography>
            </AxSkeleton>
        </>
    );
}
