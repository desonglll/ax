import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import loginCheck from "../utils/login_check.ts";
import { Typography } from "@mui/material";
import { AxSkeleton } from "../components/AxSkeleton.tsx";
import Endpoint from "../routes/common/end_point.ts";

export default function IndexPage() {
  const navigate = useNavigate();
  useEffect(() => {
    loginCheck().then((result) => {
      if (!result) {
        navigate(Endpoint.SignIn);
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
