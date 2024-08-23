import type {Post} from "../../models/post.ts";
import Box from "@mui/material/Box";
import {
    Button,
    CardActions,
    CardContent,
    Typography,
} from "@mui/material";
import axios from "axios";
import ThumbUpOffAltIcon from "@mui/icons-material/ThumbUpOffAlt";
import {useEffect, useState} from "react";
import getData from "../../utils/data_fetch.ts";
import ThumbUpAltIcon from "@mui/icons-material/ThumbUpAlt";
import ThumbDownOffAltIcon from "@mui/icons-material/ThumbDownOffAlt";
import ThumbDownAltIcon from "@mui/icons-material/ThumbDown";
import CommentIcon from "@mui/icons-material/Comment";
import {useNavigate} from "react-router-dom";
import End_points from "../../routes/common/end_points.ts";
import {Sheet} from "@mui/joy";
import Vditor from "vditor";

const bull = (
    <Box
        component="span"
        sx={{display: "inline-block", mx: "2px", transform: "scale(0.8)"}}
    >
        •
    </Box>
);

export default function PostListItem({post}: { post: Post }) {
    const [like, setLike] = useState<boolean>(false);
    const [dislike, setDislike] = useState<boolean>(false);
    const navigate = useNavigate();
    const [postItem, setPostItem] = useState<Post>(post);

    useEffect(() => {
        getData(`reaction/post/${post.id}`).then((resp) => {
            const resp_reactions = resp.data.body.data;
            resp_reactions.map((reaction_item: { reaction_name: string }) => {
                if (reaction_item.reaction_name === "like") {
                    setLike(true);
                } else if (reaction_item.reaction_name === "dislike") {
                    setDislike(true);
                }
            });
            Vditor.preview(
                document.getElementById(`pre-${post.id}`) as HTMLDivElement,
                post.content,
                {
                    cdn: "https://unpkg.com/vditor@3.10.5",
                    mode: "light",
                }
            );
        });
    }, [post.id]);

    const handleReaction = async (
        status: boolean,
        setStatus: (arg0: boolean) => void,
        reaction_name: string
    ) => {
        const data = {
            postId: Number(post.id),
            reactionName: reaction_name,
        };
        console.log(data);

        try {
            if (status) {
                await axios.post("reaction/delete", data);
                setStatus(false);
            } else {
                await axios.post("reaction/insert", data);
                setStatus(true);
            }

            const response = await getData(`post/detail/${postItem.id}`);
            if (response.data.code === "Success") {
                setPostItem(response.data.body.data);
                console.log("setPostItem");
            }
        } catch (error) {
            console.error("Error handling reaction", error);
        }
    };

    const handleLike = () => {
        if (like) {
            handleReaction(like, setLike, "like");
        } else {
            if (dislike) {
                handleReaction(dislike, setDislike, "dislike");
            }
            handleReaction(like, setLike, "like");
        }
    };
    const handleDislike = () => {
        if (dislike) {
            handleReaction(dislike, setDislike, "dislike");
        } else {
            if (like) {
                handleReaction(like, setLike, "like");
            }
            handleReaction(dislike, setDislike, "dislike");
        }
    };
    const handleDetail = (id: number) => {
        navigate(`${End_points.PostDetail}/${id}`);
    };

    return (
        <>
            <Sheet
                sx={{
                    width: "70%",
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
            >
                <CardContent sx={{
                    maxHeight: "300px", // 设置最大高度
                    overflow: "hidden", // 隐藏溢出的内容
                    textOverflow: "ellipsis", // 添加省略号
                }}>
                    <Typography sx={{fontSize: 14}} color="text.secondary" gutterBottom>
                        {postItem.userName} {bull} {postItem.createdAt}
                    </Typography>
                    <Button
                        onClick={() => handleDetail(postItem.id)}
                        sx={{color: "black"}}
                    >
                        <p id={`pre-${post.id}`}/>
                    </Button>
                </CardContent>
                <CardActions sx={{justifyContent: "space-between"}}>
                    <Box sx={{display: "flex"}}>
                        <div>
                            <Button size="small" onClick={() => handleLike()}>
                                {like ? <ThumbUpAltIcon/> : <ThumbUpOffAltIcon/>}
                                {postItem.reactions?.like}
                            </Button>
                        </div>
                        <div>
                            <Button size="small" onClick={() => handleDislike()}>
                                {dislike ? <ThumbDownAltIcon/> : <ThumbDownOffAltIcon/>}
                                {postItem.reactions?.dislike}
                            </Button>
                        </div>
                        <Button size="small">
                            <CommentIcon/>
                        </Button>
                    </Box>
                </CardActions>
            </Sheet>
        </>
    );
}
