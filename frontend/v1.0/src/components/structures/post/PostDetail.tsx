import {useNavigate, useParams} from "react-router-dom";
import {Button, Fade} from "@mui/material";
import {useEffect, useState} from "react";
import getData from "../../../utils/data_fetch.ts";
import type {Post} from "../../../models/post.ts";
import Box from "@mui/material/Box";
import axios from "axios";
import Vditor from "vditor";
import BackButton from "../../common/button/BackButton.tsx";
import type {ApiResponse} from "../../../models/api_response.ts";
import type {File} from "../../../models/file.ts";
import {AxiosEndpoint} from "../../../config/endpoints/axios_endpoint.ts";
import {CommentList} from "../comment/CommentList.tsx";
import ReactionItem from "../reaction/ReactionItem.tsx";
import {Editor} from "./Editor.tsx";
import {Previewer} from "./Previewer.tsx";

export function PostDetail() {
    const {id} = useParams(); // 获取路径参数 id
    const [post, setPost] = useState<Post>({
        content: "",
        createdAt: "",
        id: 0,
        reactions: undefined,
        replyTo: undefined,
        updatedAt: "",
        userId: 0,
        userName: "",
    });

    const [loading, setLoading] = useState(true);
    const [editing, setEditing] = useState(false);
    const navigate = useNavigate();

    useEffect(() => {
        getData(`${AxiosEndpoint.PostDetail}/${id}`)
            .then((resp) => {
                if (resp.data.code === 200) {
                    setPost(resp.data.body.data);
                }
            })
            .then(() => {
            })
            .finally(() => {
                setLoading(false);
            });
    }, [id, navigate]);


    return (
        <>
            {!loading && (
                <Fade in={!loading}>
                    <Box
                        sx={{
                            marginTop: "50px",
                            margin: "0px",
                            marginBottom: "200px",
                            padding: "10px",
                        }}
                    >
                        <BackButton/>
                        <div hidden={localStorage.getItem("user_name") !== post.userName}>
                            <Button
                                onClick={() => {
                                    setEditing(!editing);
                                }}
                            >
                                Edit
                            </Button>
                        </div>
                        <div hidden={!editing}>
                            <Editor post={post}/>
                        </div>

                        <div style={{display: "flex", justifyContent: "center"}}>
                            <Box sx={{width: "80%"}} hidden={editing}>
                                <Previewer post={post}/>
                            </Box>
                        </div>

                        <div style={{display: "flex", justifyContent: "flex-end"}}>
                            <ReactionItem toId={Number(id)} toType="post"/>
                        </div>
                        <Box sx={{display: "flex", justifyContent: "center"}}>
                            <CommentList reply_to={Number(id)}/>
                        </Box>
                    </Box>
                </Fade>
            )}
        </>
    );
}
