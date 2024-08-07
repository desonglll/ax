import "./App.css";
import type React from "react";
import {useEffect, useState} from "react";
import axios from "axios";

interface ApiResponse {
    status: string;
    message: string;
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    body: any;
}

function App() {
    const [fileResponse, setFileResponse] = useState<ApiResponse>({
        status: "undefined",
        message: "undefined",
        body: null,
    });
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
                    setLoginInfo("Login successful!");
                } else {
                    setLoginInfo(`Login failed! ${resp.data.message}`);
                }
                console.log(resp);
            });
    };

    const uploadFile = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        const formData = new FormData(e.currentTarget);
        try {
            const response = await axios.post("upload", formData, {
                headers: {
                    "Content-Type": "multipart/form-data",
                },
                withCredentials: true, // 添加这个选项以确保携带 cookie
            });
            setFileResponse(response.data);
            setLoginInfo(response.data.message);
            console.log("File upload response:", response.data);
        } catch (error) {
            console.error("Error uploading file:", error);
        }
    };
    useEffect(() => {
        const check_login = async () => {
            await axios
                .get("login_check", {
                    withCredentials: true, // 添加这个选项以确保携带 cookie
                })
                .then((resp) => {
                    console.log(resp.data);
                    if (resp.data.code === "Success") {
                        setIsLogin(true);
                    } else {
                        setIsLogin(false);
                    }
                });
        };
        check_login();
    }, []);

    return (
        <>
            <form onSubmit={login}>
                <div>{axios.defaults.baseURL}</div>
                {isLogin ? <div>Hello</div> : <div>{loginInfo}</div>}
                <div className={"log"}>
                    <input className={"input"} name={"user_name"} type={"text"}/>
                    <input className={"input"} name={"password"} type={"text"}/>
                    <button className={"btn-submit"} type={"submit"}>
                        Login
                    </button>
                </div>
            </form>

            <form onSubmit={uploadFile}>
                <div className={"upload"}>
                    <input className={"input"} name={"file"} type={"file"}/>
                    <button className={"btn-upload"} type={"submit"}>
                        Upload File
                    </button>
                </div>
            </form>

            {fileResponse && (
                <div className="response">
                    <h2>Upload Response</h2>
                    <pre>{fileResponse.message}</pre>
                </div>
            )}
        </>
    );
}

export default App;
