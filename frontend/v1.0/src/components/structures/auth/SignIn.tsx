import axios from "axios";
import type React from "react";
import { useEffect, useState } from "react";
import "./SignIn.sass";
import { useNavigate } from "react-router-dom";
import { Button, FormControl, Input, Link, Sheet, Typography } from "@mui/joy";
import { FormLabel } from "@mui/material";
import RouteEndpoint from "../../../config/endpoints/route_endpoint.ts";
import { AxiosEndpoint } from "../../../config/endpoints/axios_endpoint.ts";
import loginCheck from "../../../utils/login_check.ts";

function SignIn() {
  const [, setIsLogin] = useState<boolean>(false);
  const [, setLoginInfo] = useState<string>("Not SignIn");
  const navigate = useNavigate();

  const signIn = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const formData = new FormData(e.currentTarget);

    const userName = formData.get("user_name");
    const password = formData.get("password");

    console.log("user_name:", userName);
    console.log("password:", password);

    axios
      .post(
        AxiosEndpoint.Login,
        {
          userName,
          password,
        },
        {
          withCredentials: true, // 添加这个选项以确保携带 cookie
        }
      )
      .then((resp) => {
        // ...
        if (resp.data.code === 200) {
          localStorage.setItem("user_name", userName as string);
          setIsLogin(true);
          setLoginInfo(resp.data.body);
          navigate(RouteEndpoint.Index);
        } else {
          setLoginInfo(`Login failed! ${resp.data.message}`);
        }
      });
  };

  // const handleLogout = () => {
  //     axios.post("logout", {}).then((resp) => {
  //         if (resp.data.code === "Success") {
  //             setIsLogin(false);
  //             setLoginInfo(resp.data.message);
  //         } else {
  //             setLoginInfo(`Logout failed! ${resp.data.code}`);
  //         }
  //     });
  // };

  useEffect(() => {
    loginCheck().then((r: boolean) => {
      if (r) {
        navigate(RouteEndpoint.Index);
      }
      setIsLogin(r);
    });
  }, [navigate]);
  return (
    <>
      <Sheet
        sx={{
          width: 300,
          mx: "auto", // margin left & right
          my: 4, // margin top & bottom
          py: 3, // padding top & bottom
          px: 2, // padding left & right
          display: "flex",
          flexDirection: "column",
          gap: 2,
          borderRadius: "sm",
          boxShadow: "md",
        }}
        variant="outlined"
        component={"form"}
        onSubmit={(e) => signIn(e)}
      >
        <div>
          <Typography level="h4" component="h1">
            <b>Welcome!</b>
          </Typography>
          <Typography level="body-sm">Sign in to continue.</Typography>
        </div>
        <FormControl>
          <FormLabel>User Name</FormLabel>
          <Input
            // html input attribute
            name="user_name"
            type="text"
            placeholder="your name"
          />
        </FormControl>
        <FormControl>
          <FormLabel>Password</FormLabel>
          <Input
            // html input attribute
            name="password"
            type="password"
            placeholder="password"
          />
        </FormControl>
        <Button sx={{ mt: 1 /* margin top */ }} type="submit">
          Log in
        </Button>
        <Typography
          endDecorator={<Link href="/sign-up">Sign up</Link>}
          fontSize="sm"
          sx={{ alignSelf: "center" }}
        >
          Don&apos;t have an account?
        </Typography>
      </Sheet>
    </>
  );
}

export default SignIn;
