import React, {useState} from "react";
import axios from "axios";
import {ApiResponse} from "../models/api_response";
import "./Upload.sass"

function Upload() {

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
            console.log("File upload response:", response.data);
        } catch (error) {
            console.error("Error uploading file:", error);
        }
    };
    const [fileResponse, setFileResponse] = useState<ApiResponse>({
        status: "undefined",
        message: "undefined",
        body: null,
    });
    return (
        <>
            <div className={"container"}>
                <form onSubmit={uploadFile}>
                    <div className={"upload"}>
                        <input className={"input"} name={"file"} type={"file"}/>
                        <button className={"btn-upload"} type={"submit"}>
                            Upload File
                        </button>
                    </div>
                    {fileResponse && (
                        <div className="response">
                            <h4>Upload Response</h4>
                            <pre>{fileResponse.message}</pre>
                        </div>
                    )}
                </form>

            </div>
        </>
    )

}

export default Upload