import {useNavigate, useParams} from "react-router-dom";
import {Button, Fade} from "@mui/material";
import {useEffect, useState} from "react";
import getData from "../../../utils/data_fetch.ts";
import type {Post} from "../../../models/post.ts";
import Box from "@mui/material/Box";
import axios from "axios";
import BackButton from "../../common/button/BackButton.tsx";
import {AxiosEndpoint} from "../../../config/endpoints/axios_endpoint.ts";
import {CommentList} from "../comment/CommentList.tsx";
import ReactionItem from "../reaction/ReactionItem.tsx";
import {Editor} from "./Editor.tsx";
import {Previewer} from "./Previewer.tsx";
import {Reaction} from "../../../models/reaction.ts";

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
        likeCount: 0,
        dislikeCount: 0
    });
    const [isUserLike, setIsUserLike] = useState(false)
    const [isUserDislike, setIsUserDislike] = useState(false)
    const [pageLoading, setPageLoading] = useState(true);
    const [editing, setEditing] = useState(false);
    const navigate = useNavigate();
    const [reactionLoading, setReactionLoading] = useState(true)

    useEffect(() => {
        const fetchData = async () => {
            try {
                const postResp = await getData(`${AxiosEndpoint.PostDetail}/${id}`);
                if (postResp.data.code === 200) {
                    setPost(postResp.data.body.data);
                }

                const reactionResp = await axios.get(`${AxiosEndpoint.GetReaction}?toId=${Number(id)}`);
                const reactions_vec: [Reaction] = reactionResp.data.body.data;

                if (reactions_vec.length > 0) {
                    if (reactions_vec[0].reactionName === "Like") {
                        setIsUserLike(true);
                        setIsUserDislike(false);
                    }
                    if (reactions_vec[0].reactionName === "Dislike") {
                        setIsUserLike(false);
                        setIsUserDislike(true);
                    }
                }
            } finally {
                setReactionLoading(false);
                setPageLoading(false);
            }
        };

        fetchData();
    }, [id, navigate]);
    ;


    return (
        <>
            {!pageLoading && (
                <Fade in={!pageLoading}>
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

                        {!reactionLoading && (
                            <div style={{display: "flex", justifyContent: "flex-end"}}>
                                <ReactionItem toId={Number(id)} toType="post" likeCount={post.likeCount}
                                              dislikeCount={post.dislikeCount} isUserLike={isUserLike}
                                              isUserDislike={isUserDislike}/>
                            </div>
                        )}
                        <Box sx={{display: "flex", justifyContent: "center"}}>
                            <CommentList reply_to={Number(id)}/>
                        </Box>
                    </Box>
                </Fade>
            )}
        </>
    );
}
