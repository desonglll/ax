import axios from "axios";
import React, {useEffect, useState} from "react";
import "./Login.sass"

function Login() {

    const [isLogin, setIsLogin] = useState<boolean>(false);
    const [loginInfo, setLoginInfo] = useState<string>("Not Login");

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
                } else {
                    setLoginInfo(`Login failed! ${resp.data.message}`);
                }
                console.log(resp);
            });
    };

    const handleLogout = () => {
        axios
            .post(
                "logout",
                {},
            )
            .then((resp) => {
                if (resp.data.code === "Success") {
                    setIsLogin(false);
                    setLoginInfo(resp.data.message);
                } else {
                    setLoginInfo(`Logout failed! ${resp.data.message}`);
                }
                console.log(resp);
            });

    }

    useEffect(() => {
        const check_login = async (): Promise<boolean> => {
            let result: boolean = false;
            await axios
                .get("login_check", {
                    withCredentials: true, // 添加这个选项以确保携带 cookie
                })
                .then((resp) => {
                    if (resp.data.code === "Success") {
                        setLoginInfo(resp.data.message);
                        result = true
                    } else {
                        setLoginInfo(resp.data.message);
                        result = false
                    }
                });
            return result
        };
        check_login().then((r: boolean) => {
            setIsLogin(r)
        });
    }, []);
    return (
        <>
            <form onSubmit={login}>
                <div>Requested URL: <br/>{axios.defaults.baseURL}</div>
                {loginInfo}
                <div className={"log"}>
                    <input className={"input"} name={"user_name"} type={"text"}/>
                    <input className={"input"} name={"password"} type={"password"}/>
                    <div className={"btn-group"}>
                        <button className={"btn-login"} type={"submit"}>
                            Login
                        </button>
                        <button className={"btn-logout"} onClick={handleLogout}>Logout</button>
                    </div>
                </div>
            </form>
        </>
    )
}

export default Login