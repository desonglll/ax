import {useEffect} from "react";
import getData from "../../../utils/data_fetch.ts";
import {AxiosEndpoint} from "../../../config/endpoints/axios_endpoint.ts";
import Vditor from "vditor";
import {ApiResponse} from "../../../models/api_response.ts";
import {File} from "../../../models/file.ts";
import {Post} from "../../../models/post.ts";
import axios from "axios";

export function Editor({post}: { post: Post }) {

    useEffect(() => {
        const vditor = new Vditor("vditor", {
            typewriterMode: true,
            after: () => {
                vditor.setValue(post.content);
            },
            ctrlEnter: (value) => {
                console.log("hello", value);
                const updated_request = {
                    content: value,
                };
                axios
                    .put(`${AxiosEndpoint.UpdatePost}/${post.id}`, updated_request)
                    .then((resp) => {
                        if (resp.data.code === "Success") {
                            console.log(resp.data);
                        }
                    });
            },
            blur(value: string) {
                console.log("hello", value);
                const updated_request = {
                    content: value,
                };
                axios
                    .put(`${AxiosEndpoint.UpdatePost}/${post.id}`, updated_request)
                    .then((resp) => {
                        if (resp.data.code === "Success") {
                            console.log(resp.data);
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


    }, []);
    return (
        <>
            <div id="vditor"/>
        </>
    );
}