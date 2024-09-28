import {useEffect} from "react";
import {useNavigate} from "react-router-dom";
import loginCheck from "../utils/login_check.ts";
import {Typography} from "@mui/material";
import {AxSkeleton} from "../components/common/skeleton/AxSkeleton.tsx";
import RouteEndpoint from "../config/endpoints/route_endpoint.ts";
import axios from "axios";

export default function IndexPage() {
    const navigate = useNavigate();
    useEffect(() => {
        loginCheck().then((result) => {
            if (!result) {
                navigate(RouteEndpoint.SignIn);
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
