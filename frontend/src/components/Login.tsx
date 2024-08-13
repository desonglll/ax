import axios from "axios";
import type React from "react";
import { useEffect, useState } from "react";
import "./Login.sass";
import { Button, TextField } from "@mui/material";
import { useNavigate } from "react-router-dom";
import Card from "@mui/joy/Card";

function Login() {
  const [isLogin, setIsLogin] = useState<boolean>(false);
  const [, setLoginInfo] = useState<string>("Not Login");
  const navigate = useNavigate();

  const login = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const formData = new FormData(e.currentTarget);
    const userName = formData.get("user_name");
    const password = formData.get("password");

    console.log("user_name:", userName);
    console.log("password:", password);

    axios
      .post(
        "login",
        {
          user_name: userName,
          password: password,
        },
        {
          withCredentials: true, // 添加这个选项以确保携带 cookie
        }
      )
      .then((resp) => {
        if (resp.data.code === "Success") {
          setIsLogin(true);
          setLoginInfo(resp.data.body);
          navigate("/index");
        } else {
          setLoginInfo(`Login failed! ${resp.data.message}`);
        }
      });
  };

  const handleLogout = () => {
    axios.post("logout", {}).then((resp) => {
      if (resp.data.code === "Success") {
        setIsLogin(false);
        setLoginInfo(resp.data.message);
      } else {
        setLoginInfo(`Logout failed! ${resp.data.code}`);
      }
    });
  };

  useEffect(() => {
    const check_login = async (): Promise<boolean> => {
      let result = false;
      await axios
        .get("login-check", {
          withCredentials: true, // 添加这个选项以确保携带 cookie
        })
        .then((resp) => {
          if (resp.data.code === "Success") {
            setLoginInfo(resp.data.message);
            result = true;
          } else {
            setLoginInfo(resp.data.message);
            result = false;
          }
        });
      return result;
    };
    check_login().then((r: boolean) => {
      if (r) {
        navigate("/index");
      }
      setIsLogin(r);
    });
  }, [navigate]);
  return (
    <>
      <div className={"container"}>
        <div className={"status"}>
          <h5 className={"status-label"}>Login Status</h5>
          <div className={"login-status"}>
            {isLogin ? <div>✅</div> : <div>❌</div>}
          </div>
        </div>
        <form onSubmit={login}>
          {/*<div className={"endpoint"}>*/}
          {/*    <div className={"endpoint-label"}>*/}
          {/*        <h5>Requested URL:</h5>*/}
          {/*    </div>*/}
          {/*    <div className={"endpoint-url"}>*/}
          {/*        {axios.defaults.baseURL}*/}
          {/*    </div>*/}
          {/*</div>*/}
          {/*<div className={"login-info"}>*/}
          {/*    <h5>{loginInfo}</h5>*/}
          {/*</div>*/}
          <Card
            sx={{
              margin: "50px",
              padding: "40px",
              paddingTop: "80px",
              paddingLeft: "60px",
              paddingRight: "60px",
            }}
          >
            <div className={"log"}>
              <div className={"login-inputs"}>
                <TextField
                  name="user_name"
                  label={"user_name"}
                  type={"text"}
                  variant="standard"
                />
                <TextField
                  name="password"
                  label={"password"}
                  type={"password"}
                  variant="standard"
                />
              </div>
              <div className={"d-flex"}>
                <Button
                  variant="contained"
                  className={"btn btn-primary w-50"}
                  type={"submit"}
                >
                  SignIn
                </Button>
                <Button
                  type={"button"}
                  className={"btn btn-primary w-50"}
                  onClick={handleLogout}
                >
                  SignUp
                </Button>
                <Button
                  type={"button"}
                  className={"btn btn-secondary w-50"}
                  onClick={handleLogout}
                >
                  LogOut
                </Button>
              </div>
            </div>
          </Card>
        </form>
      </div>
    </>
  );
}

export default Login;
