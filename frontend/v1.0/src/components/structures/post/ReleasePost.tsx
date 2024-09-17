import Box from "@mui/material/Box";
import axios from "axios";
import {useEffect} from "react";
import {useNavigate} from "react-router-dom";
import Vditor from "vditor";
import "vditor/dist/index.css";
import {AxiosEndpoint} from "../../../config/endpoints/axios_endpoint.ts";
import type {ApiResponse} from "../../../models/api_response.ts";
import type {File} from "../../../models/file.ts";

export function ReleasePost() {
    const navigate = useNavigate();
    useEffect(() => {
        const vditor = new Vditor("vditor", {
            typewriterMode: true,
            after: () => {
                console.log(vditor.getValue());
            },
            ctrlEnter: (value) => {
                console.log("hello", value);
                const data = {
                    content: value,
                };
                axios.post(`${AxiosEndpoint.CreatePost}`, data).then((resp) => {
                    if (resp.data.code === 200) {
                        console.log("Success");
                        navigate(-1);
                    }
                });
            },
            upload: {
                accept: "image/*,.mp3, .wav, .rar",
                url: `${axios.defaults.baseURL}/${AxiosEndpoint.UploadPubFile}`,
                filename(name) {
                    return name
                        .replace(/[^(a-zA-Z0-9\u4e00-\u9fa5.)]/g, "")
                        .replace(/[?\\/:|<>*\[\]()$%{}@~]/g, "")
                        .replace("/\\s/g", "");
                },
                withCredentials: true,
                format(files, responseText) {
                    const response: ApiResponse<File> = JSON.parse(responseText);
                    console.log(response);
                    console.log(files);

                    const result = {
                        msg: "",
                        code: 0,
                        data: {
                            errFiles: [] as string[],
                            succMap: {} as { [key: string]: string }, // 这里定义键为字符串，值也为字符串的对象
                        },
                    };

                    if (response.code === 200) {
                        console.log("Success!");

                        const data = response.body.data;
                        data.map((file) => {
                            result.data.succMap[
                                file.name
                                ] = `${axios.defaults.baseURL}/${AxiosEndpoint.StreamFile}/${file.id}`;
                        });
                    } else {
                        // 如果上传不成功，可以在 msg 中返回服务器的错误信息
                        result.msg = response.message || "Upload failed!";
                    }

                    return JSON.stringify(result);
                },
            },
        });
    }, [navigate]);

    return (
        <>
            <Box
                sx={{
                    width: "100%",
                    display: "flex",
                    alignItems: "center",
                    flexDirection: "column",
                }}
            >
                <Box sx={{width: "80%", marginTop: "60px"}}>
                    <div id="vditor" className="vditor"/>
                </Box>
            </Box>
        </>
    );
}
